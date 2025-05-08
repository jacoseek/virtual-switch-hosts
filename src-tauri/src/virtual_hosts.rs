use mitm::{
  certificate_authority::CertificateAuthority,
  rustls::{crypto::aws_lc_rs, ServerConfig},
  Body, HttpContext, HttpHandler, Proxy,
};

use http::uri::Authority;
use hyper::Request;
use log::{error, info};
use std::{
  collections::HashMap,
  net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
  sync::Arc,
};
use tokio::{
  net::TcpListener,
  sync::{oneshot::Sender, Mutex},
  task::JoinHandle,
};

use crate::tauri_error::TauriError;

type ProxyState = Mutex<Option<(Sender<()>, JoinHandle<Result<(), mitm::Error>>)>>;

lazy_static::lazy_static! {
    // 外面传入的配置 hosts
    pub static ref CONFIG_HOSTS_MAP:Mutex<HashMap<String, IpAddr>> = Mutex::new(HashMap::new());
    // 缓存的 hosts
    pub static ref HOSTS_MAP_CACHE: Mutex<HashMap<String, IpAddr>> = Mutex::new(HashMap::new());
    static ref PROXY_STATE: ProxyState =  Mutex::new(None);
}

struct NoCa;

impl CertificateAuthority for NoCa {
  async fn gen_server_config(&self, _authority: &Authority) -> Arc<ServerConfig> {
    unreachable!();
  }
}

#[derive(Clone)]
struct ProxyHandler;

impl HttpHandler for ProxyHandler {
  async fn handle_authority(&mut self, authority: Authority) -> Authority {
    let host = authority.host();
    if let Some(ip) = CONFIG_HOSTS_MAP.lock().await.get(host) {
      let port = authority.port_u16();
      let new_authority = if let Some(port) = port {
        format!("{}:{}", ip, port)
      } else {
        ip.to_string()
      };
      new_authority
        .parse()
        .expect("Failed to parse new Authority")
    } else {
      authority.clone()
    }
  }

  fn enable_tls(&mut self, _ctx: &HttpContext, _req: &Request<Body>) -> bool {
    false
  }
}

async fn start_server(addr: SocketAddr) -> Result<(), TauriError> {
  info!("Starting proxy server at {addr}");

  let mut proxy = PROXY_STATE.lock().await;
  if proxy.is_some() {
    return Ok(());
  }

  let (close_tx, close_rx) = tokio::sync::oneshot::channel();

  let listener = TcpListener::bind(SocketAddr::from(addr)).await?;

  let pb = Proxy::builder()
    .with_listener(listener)
    .with_ca(NoCa)
    .with_rustls_client(aws_lc_rs::default_provider())
    .with_http_handler(ProxyHandler)
    .with_graceful_shutdown(async {
      close_rx.await.unwrap_or_default();
    })
    .build()
    .expect("Failed to create proxy");

  let thread = tokio::task::spawn(pb.start());

  proxy.replace((close_tx, thread));

  Ok(())
}

fn hosts_parse(content: &str) -> HashMap<String, IpAddr> {
  let mut hosts_map: HashMap<String, IpAddr> = HashMap::new();
  let lines: Vec<&str> = content.lines().collect();

  for line in lines {
    let line = line.trim();
    if line.is_empty() || line.starts_with("#") {
      continue;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 2 {
      // 尝试解析为 IPv4 或 IPv6 地址
      if let Ok(ipv4) = parts[0].parse::<Ipv4Addr>() {
        let domain = parts[1].trim();
        hosts_map.insert(domain.to_string(), IpAddr::V4(ipv4));
      } else if let Ok(ipv6) = parts[0].parse::<Ipv6Addr>() {
        let domain = parts[1].trim();
        hosts_map.insert(domain.to_string(), IpAddr::V6(ipv6));
      } else {
        // 处理无法解析的 IP 地址格式
        error!("Invalid IP address: {:?}", parts);
      }
    }
  }
  hosts_map
}

#[tauri::command]
#[specta::specta]
pub async fn start_proxy_server() -> Result<u16, TauriError> {
  let port = match portpicker::pick_unused_port() {
    Some(p) => p,
    None => 17890,
  };
  let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
  start_server(addr).await?;
  info!("Start proxy success");
  Ok(port)
}

#[tauri::command]
#[specta::specta]
pub async fn stop_proxy_server() -> Result<(), TauriError> {
  let mut proxy = PROXY_STATE.lock().await;
  if proxy.is_none() {
    return Ok(());
  }
  let (close_tx, thread) = {
    proxy
      .take()
      .ok_or_else(|| TauriError::StringError("Proxy is not running".to_string()))?
  };
  assert!(!close_tx.is_closed());
  close_tx
    .send(())
    .map_err(|_| TauriError::StringError("Failed to send close signal".to_string()))?;
  // 等待进程关闭
  let _ = thread
    .await
    .map_err(|_| TauriError::StringError("Failed to await proxy thread".to_string()))?;

  info!("Proxy stopped");
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn proxy_server_status() -> bool {
  PROXY_STATE.lock().await.is_some()
}

#[tauri::command]
#[specta::specta]
pub async fn set_hosts(content: String) {
  let mut config = CONFIG_HOSTS_MAP.lock().await;
  *config = hosts_parse(content.as_str());
}
