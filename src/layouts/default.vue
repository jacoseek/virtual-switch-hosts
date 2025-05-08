<template>
  <div class="h-[var(--app-height)] pb-[51px] overflow-y-auto">
    <div class="pt-[var(--app-bar-height)]">
      <router-view v-slot="{ Component }">
        <keep-alive>
          <component :is="Component" />
        </keep-alive>
      </router-view>
    </div>
    <var-bottom-navigation fixed safe-area border v-model:active="active">
      <var-bottom-navigation-item v-for="item in tabs" :key="item.key" @click="() => to(item.route)">
        <template #icon>
          <Icon :icon="item.icon" class="text-2xl"></Icon>
        </template>
        {{ $t(`tabbar.${item.key}`) }}
      </var-bottom-navigation-item>
    </var-bottom-navigation>
  </div>
</template>

<script lang="ts" setup name="BasicLayoutPage">
const { router, route } = useAppRouter();
const active = ref();

const tabs = [
  { key: 'home', icon: 'icon-park-twotone:home', route: '/home' },
  { key: 'setting', icon: 'icon-park-twotone:setting', route: '/setting' },
];

watch(
  () => route.path,
  (val) => {
    active.value = tabs.findIndex((item) => item.route === val);
  },
  { immediate: true },
);

function to(path: string) {
  router.replace(path);
}
</script>

<style scoped lang="scss"></style>
