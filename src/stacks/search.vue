<template>
  <router-stack>
    <div class="pt-[var(--app-bar-height)] px-4 h-full">
      <app-header>
        <input v-model="search" ref="inputField" class="text-white border-none bg-transparent focus:outline-none" placeholder="Search">
        </input>
      </app-header>
      <div class="p-2 h-full flex flex-col overflow-hidden">
        <div class="flex-1 mt-4 overflow-scroll">
          <div v-for="item in tableData" :key="item.ip" class="h-8">
            <div class="h-6 flex">
              <span class="inline-block w-36 text-primary"> {{ item.ip }}</span>
              <span class="inline-block truncate flex-1" v-html="highlightHosts(item.hostname)"></span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </router-stack>
</template>

<script lang="ts" setup>
import { useFocus } from '@vueuse/core';

const search = ref('');
const { activeHosts } = storeToRefs(useHostsStore());

const inputField = ref()

const tableData = computed(() => {
  let hosts = activeHosts.value.split('\n');
  let hostsArr: { ip: string; hostname: string }[] = [];

  hosts.forEach(function (line) {
    var hashIndex, matched, ip: string, hostnames;
    hashIndex = line.indexOf('#');
    if (hashIndex > -1) {
      line = line.slice(0, hashIndex);
    }

    matched = line.trim().split(/\s+/);

    if (matched.length < 2) {
      return;
    }

    ip = matched[0];
    hostnames = matched.slice(1);

    hostnames.forEach(function (hostname) {
      hostsArr.push({
        ip: ip,
        hostname: hostname,
      });
    });
  });
  if (search.value) {
    return hostsArr.filter((v) => v.hostname.includes(search.value));
  } else {
    return hostsArr;
  }
});

const highlightHosts = (hostname: string) => {
  return hostname.replace(search.value, `<span class="text-white bg-primary rounded">${search.value}</span>`);
};

onMounted(() => {
  useFocus(inputField, { initialValue: true })
});

</script>
<style lang="scss"></style>
