package com.plugin.hosts

import android.Manifest
import android.annotation.SuppressLint
import android.app.Activity
import android.content.Intent
import android.content.pm.PackageInfo
import android.content.pm.PackageManager
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.drawable.BitmapDrawable
import android.graphics.drawable.Drawable
import android.net.VpnService
import android.os.Build
import android.util.Base64
import android.util.Log
import android.webkit.WebView
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.Permission
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import com.plugin.hosts.MyVpnService.Companion.TAG
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.ByteArrayOutputStream

@InvokeArg
class StartVpnArgs {
    var address: String? = null
    var routes: Array<String>? = emptyArray()
    var dnsServer: String? = null
    var allowedApplications: Array<String>? = emptyArray()
    var disallowedApplications: Array<String>? = emptyArray()
    var httpProxyHost: String? = null
    var httpProxyPort: Int? = null
    var mtu: Int? = null
}

@TauriPlugin(
    permissions = [
        Permission(
            strings = [Manifest.permission.ACCESS_FINE_LOCATION],
            alias = "accessFileLocation"
        )
    ]
)
class HostsPlugin(private val activity: Activity) : Plugin(activity) {
    override fun load(webView: WebView) {
        Log.i(TAG, "load vpn service plugin")
        MyVpnService.triggerCallback = { event, data ->
            Log.i(TAG, "vpn: triggerCallback $event $data")
            trigger(event, data)
        }
    }

    @Command
    fun startVpn(invoke: Invoke) {
        val args = invoke.parseArgs(StartVpnArgs::class.java)
        MyVpnService.self?.onRevoke()

        val it = VpnService.prepare(activity)
        if (it != null) {
            activity.startActivityForResult(it, 0x0f)
            invoke.reject("授权成功后，请重新点一下开始")
            return
        } else {
            val intent = Intent(activity, MyVpnService::class.java)
            intent.putExtra(MyVpnService.ADDRESS, args.address)
            intent.putExtra(MyVpnService.ROUTES, args.routes)
            intent.putExtra(MyVpnService.DNS_SERVER, args.dnsServer)
            intent.putExtra(MyVpnService.ALLOWED_APPLICATIONS, args.allowedApplications)
            intent.putExtra(MyVpnService.DISALLOWED_APPLICATIONS, args.disallowedApplications)
            intent.putExtra(MyVpnService.HTTP_PROXY_HOST, args.httpProxyHost)
            intent.putExtra(MyVpnService.HTTP_PROXY_PORT, args.httpProxyPort)
            intent.putExtra(MyVpnService.MTU, args.mtu)

            activity.startService(intent)
        }
        invoke.resolve()
    }

    @Command
    fun stopVpn(invoke: Invoke) {
        Log.i(TAG, "stop vpn in plugin")
        MyVpnService.self?.onRevoke()
        activity.stopService(Intent(activity, MyVpnService::class.java))
        Log.i(TAG, "stop vpn in plugin end")
        invoke.resolve()
    }

    @SuppressLint("QueryPermissionsNeeded")
    @Command
    fun getAppList(invoke: Invoke) {
        GlobalScope.launch(Dispatchers.IO) {
            // 获取 PackageManager
            val packageManager: PackageManager = activity.packageManager

            // 获取已安装应用列表
            val packages: List<PackageInfo> =
                packageManager.getInstalledPackages(PackageManager.GET_META_DATA)

            // 创建一个用于存储应用名称、包名和图标的列表
            val appList: MutableList<JSObject> = mutableListOf()

            for (packageInfo in packages) {
                val appName =
                    packageManager.getApplicationLabel(packageInfo.applicationInfo).toString()
                val packageName = packageInfo.packageName
                val appIcon = packageManager.getApplicationIcon(packageInfo.applicationInfo)
                val appInfo = JSObject();
                appInfo.put("appName", appName)
                appInfo.put("packageName", packageName)
                appInfo.put("appIcon", encodeBitmapToBase64Drawable(appIcon))
                appList.add(appInfo)
            }
            withContext(Dispatchers.Main) {
                val ret = JSObject()
                ret.put("value", JSArray(appList))
                invoke.resolve(ret)
            }
        }
    }

    private fun encodeBitmapToBase64Drawable(icon: Drawable): String {
        val bitmap = createBitmapFromDrawable(icon)
        val byteArrayOutputStream = ByteArrayOutputStream()
        bitmap.compress(Bitmap.CompressFormat.PNG, 100, byteArrayOutputStream)
        val byteArray = byteArrayOutputStream.toByteArray()
        return Base64.encodeToString(byteArray, Base64.DEFAULT)
    }

    private fun createBitmapFromDrawable(drawable: Drawable): Bitmap {
        val bitmap: Bitmap
        if (drawable is BitmapDrawable) {
            bitmap = drawable.bitmap
        } else {
            // 如果是AdaptiveIconDrawable，则创建一个新的Bitmap
            val width = drawable.intrinsicWidth
            val height = drawable.intrinsicHeight
            bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
            val canvas = Canvas(bitmap)
            drawable.setBounds(0, 0, canvas.width, canvas.height)
            drawable.draw(canvas)
        }
        return bitmap
    }
}
