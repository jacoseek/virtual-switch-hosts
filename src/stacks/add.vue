<template>
  <router-stack>
    <div class="pt-[var(--app-bar-height)] px-4">
      <app-header :title="id ? $t('edit') : $t('new')">
        <template #left>
          <app-back />
        </template>
      </app-header>
      <var-form @submit="onSubmit" class="mt-4">
        <var-space direction="column" size="large">
          <var-input :placeholder="$t('hosts_name')" :rules="[(v) => !!v || $t('required')]" v-model="hostsName" />
          <var-radio-group v-model="hostsType">
            <div class="w-full flex justify-between">
              <var-radio :checked-value="HostType.Local">
                <div class="flex items-center" :class="isLocal ? 'text-primary' : ''">
                  <Icon icon="icon-park-outline:file-txt" class="mr-2 text-xl" />
                  {{ $t('hosts_type_local') }}
                </div>
              </var-radio>
              <var-radio :checked-value="HostType.Remote">
                <div class="flex items-center" :class="!isLocal ? 'text-primary' : ''">
                  <Icon icon="icon-park-outline:earth" class="mr-2 text-xl" />
                  {{ $t('hosts_type_remote') }}
                </div>
              </var-radio>
            </div>
          </var-radio-group>
          <div class="">
            <div class="flex justify-between items-center mt-4" v-if="isLocal">
              <div class=""></div>
              <VarSpace>
                <var-button size="small" text outline type="primary">
                  <div class="flex items-center">
                    <Icon icon="icon-park-outline:file-txt" class="text-xl" />
                  </div>
                </var-button>
                <var-button size="small" text outline type="primary">
                  <div class="flex items-center">
                    <Icon icon="icon-park-outline:scan-code" class="text-xl" />
                  </div>
                </var-button>
              </VarSpace>
            </div>
            <div v-else>
              <var-input placeholder="URL" v-model="remoteUrl" :rules="[(v) => !!v || $t('required')]">
                <template #prepend-icon>
                  <Icon icon="icon-park-outline:earth" class="mr-2 text-xl" />
                </template>
                <template #append-icon>
                  <var-button @click.stop="check" size="small" text outline type="primary">{{ $t('check') }}</var-button>
                </template>
              </var-input>
            </div>
            <var-input
              v-model="hostsResult"
              textarea
              variant="outlined"
              :placeholder="$t('hosts_result')"
              class="mt-2 host_area"
              :rules="[(v) => !!v || $t('required')]"
              :disabled="!isLocal"
            >
            </var-input>
          </div>
        </var-space>
        <div class="fixed bottom-4 left-4 right-4">
          <var-button type="primary" native-type="submit" class="w-full" block size="large">
            {{ $t('save') }}
          </var-button>
        </div>
      </var-form>
    </div>
    <ResultPop ref="resultPop" />
  </router-stack>
</template>
<script lang="ts" setup>
  import ResultPop from '@/components/ResultPop.vue';
  import router from '@/router';
  import { useHostsStore, HostType, HostItem } from '@/store/hosts';

  const id = useRoute().query.id as string;

  const { list } = storeToRefs(useHostsStore());

  const hostsType = ref<HostType>(HostType.Local);

  const hostsName = ref<string>('');
  const hostsResult = ref<string>('');
  const isLocal = computed(() => hostsType.value === HostType.Local);
  const remoteUrl = ref<string>('');

  const resultPop = ref<InstanceType<typeof ResultPop>>();

  const check = () => {
    fetch(remoteUrl.value)
      .then((response) => {
        if (!response.ok) {
          throw new Error('Network response was not ok ' + response.statusText);
        }
        return response.text();
      })
      .then((data) => {
        hostsResult.value = data;
      })
      .catch((error) => {
        console.error('There was a problem with the fetch operation:', error);
      });
  };

  const onSubmit = () => {
    if (!hostsName.value) {
      return;
    }
    if (hostsType.value === HostType.Local && hostsResult.value === '') {
      return;
    }
    if (hostsType.value === HostType.Remote && remoteUrl.value === '') {
      return;
    }

    // 存储
    const item: HostItem = {
      id: Date.now().toString(),
      name: hostsName.value,
      type: hostsType.value,
      enabled: false,
    };

    if (isLocal.value) {
      item.hosts = hostsResult.value;
    } else {
      item.remoteUrl = remoteUrl.value;
    }
    if (id) {
      list.value[list.value.findIndex((item) => item.id === id)] = item;
    } else {
      list.value.push(item);
    }

    resultPop.value?.open({
      show: true,
      type: 'success',
      buttonClick: () => {
        router.back();
      },
    });
  };
  onMounted(() => {
    if (id) {
      const currData = list.value.find((item) => item.id === id)!;
      hostsType.value = currData.type;
      hostsName.value = currData.name;
      hostsResult.value = currData.hosts || '';
      remoteUrl.value = currData.remoteUrl || '';
    }
  });
</script>
