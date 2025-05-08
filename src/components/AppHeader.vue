<script setup lang="ts">
import { AppBarProps } from '@varlet/ui';

withDefaults(
  defineProps<{
    title?: AppBarProps['title'];
    titlePosition?: AppBarProps['titlePosition'];
  }>(),
  {
    titlePosition: 'left',
  },
);
</script>

<template>
  <var-app-bar safe-area-top fixed :z-index="100" :title="title" :title-position="titlePosition" color="hsl(151, 100%, 21%)">
    <template #left>
      <slot name="left" v-if="$slots.left" />
      <app-back v-else />
    </template>

    <template #right>
      <var-space class="pr-[8px]" :size="[0, '1.4vmin']">
        <slot name="right" />
      </var-space>
    </template>

    <template #content>
      <slot name="content" />
    </template>

    <template #default>
      <slot name="default" />
    </template>
  </var-app-bar>
</template>
