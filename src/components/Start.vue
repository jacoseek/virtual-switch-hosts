<template>
  <div id="dragme" class="absolute bottom-20 right-8 w-14 h-14" @click="toggleProxy" :class="{ disabled: isNullHosts }">
    <var-loading :show="isPending">
      <div :class="{ active: isProxying }" class="play-btn flex items-center justify-center bg-primary w-14 h-14">
        <div class="button"></div>
      </div>
    </var-loading>
  </div>
</template>
<script lang="ts" setup>
import { startVpn, stopVpn } from 'tauri-plugin-hosts';

const { isProxying, activeHosts } = storeToRefs(useHostsStore());

const isNullHosts = computed(() => activeHosts.value.length === 0);

const isPending = ref(false);

const { vpnConfig } = storeToRefs(useHostsStore());

const startVpnParams = computed(() => ({
  ...vpnConfig.value,
  httpProxyHost: '127.0.0.1',
  httpProxyPort: 5000,
}));

const toggleProxy = () => {
  if (isNullHosts.value) return;
  if (isPending.value) return;
  if (isProxying.value) {
    stop();
  } else {
    start();
  }
};

const start = async () => {
  isPending.value = true;
  // 开启服务
  const res = await commands.startProxyServer();
  if (res.status == 'error') {
    isPending.value = false;
    Snackbar.error(res.error);
    return;
  }
  try {
    // 开启 VPN
    await startVpn({
      ...startVpnParams.value,
      httpProxyPort: res.data,
    });
    isProxying.value = true;
  } catch (err: any) {
    Snackbar.error(err.message);
    isProxying.value = false;
    commands.stopProxyServer();
  }
  isPending.value = false;
};

const stop = async () => {
  isPending.value = true;
  // 关闭 vpn
  const res = await commands.stopProxyServer();
  if (res.status === 'error') {
    isProxying.value = false;
    Snackbar.error(res.error);
    return;
  }
  try {
    await stopVpn();
    isProxying.value = false;
  } catch (err: any) {
    Snackbar.error(err.message);
  }
  isPending.value = false;
};
</script>

<style lang="scss">
#dragme {
  &.disabled {
    opacity: 0.5;
  }
}
.play-btn {
  border-radius: 50%;
  border: 3px solid rgba(255, 255, 255, 0.8);
  box-shadow: 0 0 50px 13px var(--primary-color);
  box-sizing: border-box;

  .button {
    background: transparent;
    box-sizing: border-box;
    width: 0;
    height: 20px;

    border-color: transparent transparent transparent #fff;
    transition: 100ms all ease;
    cursor: pointer;

    // play state
    border-style: solid;
    border-width: 10px 0 10px 14px;
  }

  &.active {
    .button {
      border-style: double;
      border-width: 0 0 0 14px;
    }

    &.play-btn:before {
      content: '';
      position: absolute;
      width: 200%;
      height: 200%;
      animation-delay: 0s;
      animation: pulsate1 1.5s;
      animation-iteration-count: infinite;
      opacity: 1;
      border-radius: 50%;
      border: 5px solid rgba(24, 160, 88, 0.3);
      background: rgba(198, 16, 0, 0);
    }
  }
}

@keyframes pulsate1 {
  0% {
    transform: scale(0.6);
    opacity: 1;
    box-shadow: inset 0 0 25px 1.5px rgba(24, 160, 88, 0.3), 0 0 25px 5px rgba(24, 160, 88, 0.3);
  }

  100% {
    transform: scale(1);
    opacity: 0;
    box-shadow: none;
  }
}
</style>
