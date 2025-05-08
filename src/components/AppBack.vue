<script setup lang="ts">
const { router } = useAppRouter();
</script>

<template>
  <var-button color="transparent" round text @click="router.back">
    <Icon icon="icon-park-outline:left" class="text-2xl"></Icon>
  </var-button>
</template>
