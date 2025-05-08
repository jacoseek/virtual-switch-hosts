<script setup lang="ts">
const { pushStack } = useAppRouter();
const show = ref(false);
</script>

<template>
  <var-button class="ml-4px" text round @click="show = !show" v-bind="$attrs">
    <Icon icon="icon-park-outline:hamburger-button" class="text-2xl"></Icon>
  </var-button>

  <var-popup position="left" v-model:show="show">
    <div class="w-[300px]">
      <!-- TODO -->
      <div class="flex flex-col justify-center px-[20px] bg-cover">
        <img src="@/assets/img/icon.png" alt="" class="rounded-full border w-20 mt-8" />
        <div class="mt-4 fw-[500] text-xl">virtual-switch-hosts</div>
        <div class="mt-2 text-md">0.1.0</div>
        <div class="mt-4 text-sm">{{ $t('one_words_desc') }}</div>
      </div>

      <div class="py-[10px] px-[4px] mt-2">
        <var-space direction="column">
          <var-cell class="rounded-[4px]" :title="$t('issue')" v-ripple @click="pushStack('/sign-in')">
            <template #icon>
              <Icon icon="icon-park-outline:helpcenter" class="text-xl mr-2" />
            </template>
          </var-cell>
          <var-divider />

          <var-cell class="rounded-[4px]" :title="$t('close')" v-ripple @click="show = false">
            <template #icon>
              <var-icon class="mr-2" name="window-close" />
            </template>
          </var-cell>
        </var-space>
      </div>
    </div>
  </var-popup>
</template>
