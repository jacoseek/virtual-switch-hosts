// 必须先导入一下
import "@/utils/storage";

import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import { i18n } from '@/i18n';
import router from '@/router';
import store from '@/store';
import { Icon } from '@iconify/vue';

const app = createApp(App);

// 路由
app.use(router);

// 国际化
app.use(i18n);

// 状态管理
app.use(store);

app.component('Icon', Icon);

app.mount('#app');
