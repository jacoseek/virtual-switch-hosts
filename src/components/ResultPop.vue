<template>
  <var-popup :default-style="false" v-model:show="show">
    <var-result class="result" :type="type" :title="title" :description="description">
      <template #footer>
        <var-button type="primary" @click="buttonClick">{{ buttonText }}</var-button>
      </template>
    </var-result>
  </var-popup>
</template>
<script lang="ts" setup>
import { ResultType } from '@varlet/ui/types/result';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

const show = ref(false);
const type = ref<ResultType>();
const title = ref('');
const description = ref('');
const buttonText = ref('');
let buttonClick = () => {
  show.value = false;
};

defineExpose({
  open: (options: {
    show: boolean;
    type: ResultType;
    title?: string;
    description?: string;
    buttonText?: string;
    buttonClick?: () => void;
  }) => {
    show.value = options.show;
    type.value = options.type;
    title.value = options.title || t(`result_pop_status.${options.type}`);
    description.value = options.description || '';
    buttonText.value = options.buttonText || t('confirm');
    buttonClick = options.buttonClick || buttonClick;
  },
});
</script>

<style>
.result {
  width: 75vw !important;
}
</style>
