use crate::{
  body::Body, certificate_authority::CertificateAuthority, rewind::Rewind, HttpContext,
  HttpHandler, RequestOrResponse, WebSocketContext, WebSocketHandler,
};
use futures::{Sink, Stream, StreamExt};
use http::uri::{Authority, Scheme};
use hyper::{
  body::{Bytes, Incoming},
  header::Entry,
  service::service_fn,
  upgrade::Upgraded,
  Method, Request, Response, StatusCode, Uri,
};
use hyper_util::{
  client::legacy::{connect::Connect, Client},
  rt::{TokioExecutor, TokioIo},
  server,
};
use std::{convert::Infallible, future::Future, net::SocketAddr, sync::Arc};
use tokio::{io::AsyncReadExt, net::TcpStream, task::JoinHandle};
use tokio_rustls::TlsAcceptor;
use tokio_tungstenite::{
  tungstenite::{self, Message},
  Connector, WebSocketStream,
};
use tracing::{error, info_span, warn, Instrument, Span};

fn bad_request() -> Response<Body> {
  Response::builder()
    .status(StatusCode::BAD_REQUEST)
    .body(Body::empty())
    .expect("Failed to build response")
}

pub(crate) struct InternalProxy<C, CA, H, W> {
  pub ca: Arc<CA>,
  pub client: Client<C, Body>,
  pub server: server::conn::auto::Builder<TokioExecutor>,
  pub http_handler: H,
  pub websocket_handler: W,
  pub websocket_connector: Option<Connector>,
  pub client_addr: SocketAddr,
  pub should_intercept: bool,
  pub enable_tls: bool,
}

impl<C, CA, H, W> Clone for InternalProxy<C, CA, H, W>
where
  C: Clone,
  H: Clone,
  W: Clone,
{
  fn clone(&self) -> Self {
    InternalProxy {
      ca: Arc::clone(&self.ca),
      client: self.client.clone(),
      server: self.server.clone(),
      http_handler: self.http_handler.clone(),
      websocket_handler: self.websocket_handler.clone(),
      websocket_connector: self.websocket_connector.clone(),
      client_addr: self.client_addr,
      should_intercept: self.should_intercept,
      enable_tls: self.enable_tls,
    }
  }
}

impl<C, CA, H, W> InternalProxy<C, CA, H, W>
where
  C: Connect + Clone + Send + Sync + 'static,
  CA: CertificateAuthority,
  H: HttpHandler,
  W: WebSocketHandler,
{
  fn context(&self) -> HttpContext {
    HttpContext {
      client_addr: self.client_addr,
      should_intercept: self.should_intercept,
      enable_tls: self.enable_tls,
    }
  }

  pub(crate) async fn proxy(
    mut self,
    req: Request<Incoming>,
  ) -> Result<Response<Body>, Infallible> {
    let ctx = self.context();
    let req = req.map(Body::from);
    let should_intercept = self.http_handler.should_intercept(&ctx, &req).await;
    let enable_tls = self.http_handler.enable_tls(&ctx, &req);
    // 给 ctx 设置一些默认值
    self.should_intercept = should_intercept;
    self.enable_tls = enable_tls;

    let req = if should_intercept {
      match self.http_handler.handle_request(&ctx, req).await {
        RequestOrResponse::Request(req) => req,
        RequestOrResponse::Response(res) => return Ok(res),
      }
    } else {
      req
    };

    if req.method() == Method::CONNECT {
      let res = self.clone().process_connect(req);
      // 如果没有开启 tls 那么需要直接返回
      if should_intercept && !enable_tls {
        Ok(self.http_handler.handle_response(&ctx, res).await)
      } else {
        Ok(res)
      }
    } else if hyper_tungstenite::is_upgrade_request(&req) {
      Ok(self.upgrade_websocket(req))
    } else {
      let res = self.client.request(normalize_request(req)).await;
      match res {
        Ok(res) => {
          let res = res.map(Body::from);
          if should_intercept {
            Ok(self.http_handler.handle_response(&ctx, res).await)
          } else {
            Ok(res)
          }
        }
        Err(err) => Ok(self.http_handler.handle_error(&ctx, err).await),
      }
    }
  }

  fn process_connect(mut self, mut req: Request<Body>) -> Response<Body> {
    match req.uri().authority().cloned() {
      Some(authority) => {
        // let span = info_span!("process_connect");
        let fut = async move {
          match hyper::upgrade::on(&mut req).await {
            Ok(upgraded) => {
              let mut upgraded = TokioIo::new(upgraded);

              if self.should_intercept && self.enable_tls {
                let mut buffer = [0; 4];
                let bytes_read = match upgraded.read(&mut buffer).await {
                  Ok(bytes_read) => bytes_read,
                  Err(e) => {
                    error!("Failed to read from upgraded connection: {}", e);
                    return;
                  }
                };

                let upgraded = Rewind::new(
                  upgraded,
                  Bytes::copy_from_slice(buffer[..bytes_read].as_ref()),
                );

                if buffer == *b"GET " {
                  if let Err(e) = self
                    .serve_stream(TokioIo::new(upgraded), Scheme::HTTP, authority)
                    .await
                  {
                    error!("WebSocket connect error: {}, {}", e, req.uri());
                  }
                  return;
                } else if buffer[..2] == *b"\x16\x03" {
                  let server_config = self.ca.gen_server_config(&authority).await;

                  let stream = match TlsAcceptor::from(server_config).accept(upgraded).await {
                    Ok(stream) => TokioIo::new(stream),
                    Err(_e) => {
                      // error!("Failed to establish TLS connection: {}, {}", e, req.uri());
                      return;
                    }
                  };

                  if let Err(e) = self.serve_stream(stream, Scheme::HTTPS, authority).await {
                    if !e.to_string().starts_with("error shutting down connection") {
                      error!("HTTPS connect error: {}, {}", e, req.uri());
                    }
                  }
                  return;
                } else {
                  warn!(
                    "Unknown protocol, read '{:02X?}' from upgraded connection",
                    &buffer[..bytes_read]
                  );
                }
              } else {
                let mut server = match TcpStream::connect(
                  self
                    .http_handler
                    .handle_authority(authority.clone())
                    .await
                    .as_ref(),
                )
                .await
                {
                  Ok(server) => server,
                  Err(e) => {
                    error!("Failed to connect to: {}, {}", e, req.uri());
                    return;
                  }
                };

                if let Err(e) = tokio::io::copy_bidirectional(&mut upgraded, &mut server).await {
                  error!("Failed to tunnel to {}: {}", authority, e);
                }
              }
            }
            Err(e) => error!("Upgrade error: {}, {}", e, req.uri()),
          };
        };

        tokio::spawn(fut);
        Response::new(Body::empty())
      }
      None => bad_request(),
    }
  }

  fn upgrade_websocket(self, req: Request<Body>) -> Response<Body> {
    let mut req = {
      let (mut parts, _) = req.into_parts();
      parts.uri = {
        let mut parts = parts.uri.into_parts();

        parts.scheme = if parts.scheme.unwrap_or(Scheme::HTTP) == Scheme::HTTP {
          Some("ws".try_into().expect("Failed to convert scheme"))
        } else {
          Some("wss".try_into().expect("Failed to convert scheme"))
        };

        match Uri::from_parts(parts) {
          Ok(uri) => uri,
          Err(_) => {
            return bad_request();
          }
        }
      };
      Request::from_parts(parts, ())
    };

    match hyper_tungstenite::upgrade(&mut req, None) {
      Ok((res, websocket)) => {
        let fut = async move {
          match websocket.await {
            Ok(ws) => {
              if let Err(e) = self.handle_websocket(ws, req).await {
                error!("Failed to handle WebSocket: {}", e);
              }
            }
            Err(e) => {
              error!("Failed to upgrade to WebSocket: {}", e);
            }
          }
        };

        tokio::spawn(fut);
        res.map(Body::from)
      }
      Err(_) => bad_request(),
    }
  }

  async fn handle_websocket(
    self,
    client_socket: WebSocketStream<TokioIo<Upgraded>>,
    req: Request<()>,
  ) -> Result<(), tungstenite::Error> {
    let uri = req.uri().clone();
    let (server_socket, _) = tokio_tungstenite::connect_async_tls_with_config(
      req.clone(),
      None,
      false,
      self.websocket_connector,
    )
    .await?;
    // 中间人
    let (server_sink, server_stream) = server_socket.split();
    let (client_sink, client_stream) = client_socket.split();

    let InternalProxy {
      websocket_handler, ..
    } = self;

    spawn_message_forwarder(
      server_stream,
      client_sink,
      websocket_handler.clone(),
      WebSocketContext::ServerToClient {
        src: uri.clone(),
        dst: self.client_addr,
      },
    );

    spawn_message_forwarder(
      client_stream,
      server_sink,
      websocket_handler,
      WebSocketContext::ClientToServer {
        src: self.client_addr,
        dst: uri,
      },
    );
    Ok(())
  }

  async fn serve_stream<I>(
    self,
    stream: I,
    scheme: Scheme,
    authority: Authority,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
  where
    I: hyper::rt::Read + hyper::rt::Write + Unpin + Send + 'static,
  {
    let service = service_fn(|mut req| {
      if req.version() == hyper::Version::HTTP_10 || req.version() == hyper::Version::HTTP_11 {
        let (mut parts, body) = req.into_parts();
        parts.uri = {
          let mut parts = parts.uri.into_parts();
          parts.scheme = Some(scheme.clone());
          parts.authority = Some(authority.clone());
          Uri::from_parts(parts).expect("Failed to build URI")
        };

        req = Request::from_parts(parts, body);
      };

      self.clone().proxy(req)
    });

    self
      .server
      .serve_connection_with_upgrades(stream, service)
      .await
  }
}

fn spawn_with_trace<T: Send + Sync + 'static>(
  fut: impl Future<Output = T> + Send + 'static,
  span: Span,
) -> JoinHandle<T> {
  tokio::spawn(fut.instrument(span))
}

fn spawn_message_forwarder(
  stream: impl Stream<Item = Result<Message, tungstenite::Error>> + Unpin + Send + 'static,
  sink: impl Sink<Message, Error = tungstenite::Error> + Unpin + Send + 'static,
  handler: impl WebSocketHandler,
  ctx: WebSocketContext,
) {
  let span = info_span!("message_forwarder", context = ?ctx);
  let fut = handler.handle_websocket(ctx, stream, sink);
  spawn_with_trace(fut, span);
}

fn normalize_request<T>(mut req: Request<T>) -> Request<T> {
  // Hyper will automatically add a Host header if needed.
  req.headers_mut().remove(hyper::header::HOST);

  // HTTP/2 supports multiple cookie headers, but HTTP/1.x only supports one.
  if let Entry::Occupied(mut cookies) = req.headers_mut().entry(hyper::header::COOKIE) {
    let joined_cookies = bstr::join(b"; ", cookies.iter());
    cookies.insert(joined_cookies.try_into().expect("Failed to join cookies"));
  }

  *req.version_mut() = hyper::Version::HTTP_11;
  req
}
