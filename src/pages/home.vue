<template>
  <div class="p-4">
    <app-header>
      <template #left>
        <app-side-menu />
      </template>
      <template #right>
        <var-button text round @click="pushStack('/search')">
          <Icon icon="icon-park-outline:search" class="text-2xl"></Icon>
        </var-button>
        <var-button text round @click="pushStack('/add')">
          <Icon icon="icon-park-outline:plus" class="text-2xl"></Icon>
        </var-button>
      </template>
    </app-header>
    <template v-if="list.length > 0">
      <div class="max-h-60 overflow-y-auto">
        <var-cell @click="handleClick(item)" v-for="(item, index) in list" :key="index" :name="item.name" ripple border class="mt-2">
          <template #icon>
            <Icon
              :icon="item.type === HostType.Local ? 'icon-park-outline:file-txt' : 'icon-park-outline:earth'"
              class="text-2xl text-primary"
            ></Icon>
          </template>
          <template #description>
            <span class="ml-3">{{ item.name }}</span>
          </template>
          <template #extra>
            <div class="flex items-center">
              <var-checkbox v-model="item.enabled" @click.stop></var-checkbox>
              <var-divider vertical hairline />
              <var-button size="small" text @click.stop class="!px-0" @click="itemAction(item)">
                <Icon icon="icon-park-outline:more-one" class="text-2xl" />
              </var-button>
            </div>
          </template>
        </var-cell>
      </div>
      <div class="mt-6 mx-3">
        <var-input
          v-model="activeHosts"
          textarea
          variant="outlined"
          :placeholder="$t('hosts_result')"
          class="mt-4 host_area"
          disabled
          rows="10"
        >
        </var-input>
      </div>
    </template>

    <template v-else>
      <var-result type="empty" :title="$t('empty')" :description="$t('add_content_prompt')">
        <template #footer>
          <var-button plain type="primary" @click="pushStack('/add')">
            <Icon icon="icon-park-outline:plus"></Icon>
            {{ $t('add') }}
          </var-button>
        </template>
      </var-result>
    </template>
  </div>
  <router-stack-view />
</template>

<script lang="ts" setup name="HomePage">
import { HostItem, HostType } from '@/store/hosts';

const { pushStack } = useAppRouter();
const { t } = useI18n();
const { list, activeHosts } = storeToRefs(useHostsStore());

const handleClick = (item: HostItem) => {
  item.enabled = !item.enabled;
};

const itemAction = async (item: HostItem) => {
  const action = await ActionSheet({
    actions: [
      {
        name: t('edit'),
        icon: 'cog-outline',
        color: 'var(--color-warning)',
        className: 'edit',
      },
      {
        name: t('copy'),
        icon: 'content-copy',
        color: 'var(--color-primary)',
        className: 'copy',
      },
      {
        name: t('delete'),
        icon: 'trash-can-outline',
        color: 'var(--color-danger)',
        className: 'delete',
      },
    ],
  });
  if (action !== 'close') {
    switch (action.className) {
      case 'edit':
        pushStack('/add', {
          id: item.id,
        });
        break;
      case 'copy':
        list.value.push({
          ...item,
          id: Date.now().toString(),
          name: `${item.name}-Copy`,
        });
        break;
      case 'delete':
        list.value = list.value.filter((i) => i.id !== item.id);
        break;
    }
  }
};
</script>
<style lang="scss"></style>

<route lang="json">
{
  "meta": {
    "stacks": ["add", "search"]
  }
}
</route>
