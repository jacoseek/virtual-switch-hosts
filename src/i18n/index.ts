import { createI18n, useI18n } from 'vue-i18n';
import { Locale } from '@varlet/ui';

export enum LanguageType {
  zhCN = 'zh-CN',
  enUS = 'en-US',
}

export function loadLang() {
  const modules: Record<string, any> = import.meta.glob('./lang/*.ts', {
    eager: true,
  });
  const langs: Record<string, any> = {};

  for (const path in modules) {
    if (path === './lang/base.ts') {
      continue;
    }
    const name = path.replace(/(\.\/lang\/|\.ts)/g, '');
    langs[name] = modules[path].default;
  }
  return langs;
}

export const i18n = createI18n({
  // 传统模式
  legacy: false,
  // 当前的语言模式
  locale: LanguageType.enUS,
  // 无匹配语言模式的默认语言模式
  fallbackLocale: LanguageType.enUS,
  // 所有的
  messages: loadLang(),
  // 全局注入, 可以使用 $ 开始的变量
  globalInjection: true,
});

export const getLangOptions = () => {
  const { messages, locale } = useI18n();
  return Object.entries(messages.value[locale.value].lang).map(([value, text]) => ({
    label: text,
    value,
  }));
};

export function setLang(locale: LanguageType) {
  i18n.global.locale.value = locale;

  if (locale === LanguageType.zhCN) {
    Locale.add('zh-CN', Locale.zhCN);
  } else {
    Locale.add('en-US', Locale.enUS);
  }

  window.$storage.set('language', locale);
}
