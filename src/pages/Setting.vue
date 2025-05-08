<template>
  <div class="p-4">
    <app-header>
      <template #left>
        <app-side-menu />
      </template>
      <template #right>
        <app-locale-switch />
        <app-theme-switch />
      </template>
    </app-header>
    <var-space direction="column" size="large">
      <div class="text-primary font-bold">{{ $t('base') }}</div>
      <var-select :placeholder="$t('theme')" v-model="theme" :options="themeOptions"> </var-select>
      <var-select :placeholder="$t('language')" v-model="language" :options="langOptions"> </var-select>
    </var-space>
    <var-space direction="column" size="large" class="mt-8">
      <div class="flex justify-between items-center">
        <div class="text-primary font-bold">{{ $t('vpn_config') }}</div>
        <var-button @click="vpnConfigReset" text type="primary">
          {{ $t('reset') }}
        </var-button>
      </div>
      <var-input placeholder="address" v-model="vpnConfig.address" :options="langOptions"> </var-input>
      <var-input placeholder="dnsServer" v-model="vpnConfig.dnsServer" :options="langOptions"> </var-input>
      <var-input placeholder="mtu" v-model="vpnConfig.mtu" type="number" :options="langOptions"> </var-input>
    </var-space>
  </div>
</template>
<script lang="ts" setup>
const { theme, language } = storeToRefs(useGlobalStore());
import { defaultVpnConfig } from '@/store/hosts';

const langOptions = getLangOptions();
const themeOptions = getThemeOptions();

const { vpnConfig } = storeToRefs(useHostsStore());

function vpnConfigReset() {
  vpnConfig.value = { ...defaultVpnConfig };
}
</script>
