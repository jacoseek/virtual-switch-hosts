package com.plugin.hosts

import android.content.Intent
import android.net.IpPrefix
import android.net.ProxyInfo
import android.net.VpnService
import android.os.Build
import android.os.Bundle
import android.os.ParcelFileDescriptor
import android.util.Log
import androidx.annotation.RequiresApi
import app.tauri.plugin.JSObject
import java.net.InetAddress

@RequiresApi(Build.VERSION_CODES.TIRAMISU)
fun stringToIpPrefix(ipPrefixString: String): IpPrefix {
    val parts = ipPrefixString.split("/")
    if (parts.size != 2) throw IllegalArgumentException("Invalid IP prefix string")

    val address = InetAddress.getByName(parts[0])
    val prefixLength = parts[1].toInt()

    return IpPrefix(address, prefixLength)
}

open class MyVpnService : VpnService() {
    companion object {
        @JvmField
        var triggerCallback: (String, JSObject) -> Unit = { _, _ -> }

        @JvmField
        var self: MyVpnService? = null

        const val TAG = "MyVpnService"
        const val ADDRESS = "ADDRESS"
        const val ROUTES = "ROUTES"
        const val DNS_SERVER = "DNS"
        const val ALLOWED_APPLICATIONS = "ALLOWED_APPLICATIONS"
        const val DISALLOWED_APPLICATIONS = "DISALLOWED_APPLICATIONS"
        const val HTTP_PROXY_HOST = "HTTP_PROXY_HOST"
        const val HTTP_PROXY_PORT = "HTTP_PROXY_PORT"
        const val MTU = "MTU"
    }

    private val passList = listOf(
        "localhost",
        "*.local",
        "127.*",
        "10.*",
        "172.16.*",
        "172.17.*",
        "172.18.*",
        "172.19.*",
        "172.2*",
        "172.30.*",
        "172.31.*",
        "192.168.*"
    )

    private lateinit var vpnInterface: ParcelFileDescriptor

    @RequiresApi(Build.VERSION_CODES.TIRAMISU)
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        var args = intent?.getExtras()

        vpnInterface = createVpnInterface(args)
        val eventData = JSObject()
        eventData.put("fd", vpnInterface.fd)

        return START_STICKY
    }

    override fun onCreate() {
        super.onCreate()
        self = this
        Log.i(TAG, "vpn on create")
    }

    override fun onDestroy() {
        Log.i(TAG, "vpn on destroy")
        self = null
        super.onDestroy()
        disconnect()
    }

    override fun onRevoke() {
        Log.i(TAG, "vpn on revoke")
        self = null
        super.onRevoke()
        disconnect()
    }

    private fun disconnect() {
        vpnInterface.close()
    }


    @RequiresApi(Build.VERSION_CODES.TIRAMISU)
    private fun createVpnInterface(args: Bundle?): ParcelFileDescriptor {
        val builder = Builder()
            .setSession("MyVpnService")
            .setBlocking(false)

        val mtu = args?.getInt(MTU) ?: 1500
        val address = args?.getString(ADDRESS) ?: "192.0.2.111/32"
        val dnsServer = args?.getString(DNS_SERVER) ?: "8.8.8.8"
        val routes = args?.getStringArray(ROUTES) ?: emptyArray()
        val allowedApplications = args?.getStringArray(ALLOWED_APPLICATIONS) ?: emptyArray()
        val disallowedApplications = args?.getStringArray(DISALLOWED_APPLICATIONS) ?: emptyArray()
        val httpProxyHost = args?.getString(HTTP_PROXY_HOST)
        val httpProxyPort = args?.getInt(HTTP_PROXY_PORT)

        val ipParts = address.split("/")
        if (ipParts.size != 2) throw IllegalArgumentException("Invalid IP address string")
        builder.addAddress(ipParts[0], ipParts[1].toInt())

        builder.setMtu(mtu)
        builder.addDnsServer(dnsServer)

        for (route in routes) {
            builder.addRoute(stringToIpPrefix(route))
        }

        for (app in allowedApplications) {
            builder.addAllowedApplication(app)
        }

        for (app in disallowedApplications) {
            builder.addDisallowedApplication(app)
        }

        if (httpProxyHost != null && httpProxyPort != null && Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            builder.setHttpProxy(
                ProxyInfo.buildDirectProxy(
                    httpProxyHost,
                    httpProxyPort,
                )
            )
        }

        return builder.also {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                it.setMetered(false)
            }
        }
            .establish()
            ?: throw IllegalStateException("Failed to init VpnService")
    }

}