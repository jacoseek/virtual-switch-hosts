<template>
  <router-view />
  <Start />
</template>
<script setup lang="ts">

useGlobalStore().init();
useHostsStore().init();

const setupAppHeight = () => {
  window.addEventListener('resize', updateAppHeight);
  window.addEventListener('orientationchange', updateAppHeight);
  updateAppHeight();

  function updateAppHeight() {
    document.documentElement.style.setProperty('--app-height', `${window.innerHeight}px`);
  }
};

setupAppHeight();
</script>

<style></style>
