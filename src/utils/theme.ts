import { StyleVars, Themes } from '@varlet/ui';
import { usePreferredColorScheme } from '@vueuse/core';

export enum ThemeType {
  Auto,
  Light,
  Dark,
  Unset,
}

export const getThemeOptions = () => {
  const { t } = useI18n();
  return [
    {
      label: t('theme_type.auto'),
      value: ThemeType.Auto,
    },
    {
      label: t('theme_type.light'),
      value: ThemeType.Light,
    },
    {
      label: t('theme_type.dark'),
      value: ThemeType.Dark,
    },
  ];
};

export const changeTheme = (val: ThemeType) => {
  // 清除现有的主题类
  document.documentElement.classList.remove('light', 'dark');

  const applyTheme = (theme: 'dark' | 'light', style: StyleVars | null) => {
    StyleProvider(style);
    document.documentElement.classList.add(theme);
  };

  if (val === ThemeType.Auto) {
    const preferredColor = usePreferredColorScheme();
    if (preferredColor.value === 'dark') {
      applyTheme('dark', themeConfig.dark);
    } else {
      applyTheme('light', themeConfig.light);
    }
  } else if (val === ThemeType.Dark) {
    applyTheme('dark', themeConfig.dark);
  } else if (val === ThemeType.Light) {
    applyTheme('light', themeConfig.light);
  }
  window.$storage.set('theme', val);
};


export const themeConfig: { light: StyleVars, dark: StyleVars } = {
  "light": {
    ...Themes.md3Light,
    "--hsl-primary": "151, 100%, 21%",
    "--color-primary": "hsla(var(--hsl-primary), 1)",
    "--hsl-on-primary": "0, 0%, 100%",
    "--color-on-primary": "hsla(var(--hsl-on-primary), 1)",
    "--hsl-primary-container": "139, 94%, 75%",
    "--color-primary-container": "hsla(var(--hsl-primary-container), 1)",
    "--hsl-on-primary-container": "144, 100%, 6%",
    "--color-on-primary-container": "hsla(var(--hsl-on-primary-container), 1)",
    "--hsl-info": "129, 11%, 35%",
    "--color-info": "hsla(var(--hsl-info), 1)",
    "--hsl-on-info": "0, 0%, 100%",
    "--color-on-info": "hsla(var(--hsl-on-info), 1)",
    "--hsl-info-container": "123, 32%, 87%",
    "--color-info-container": "hsla(var(--hsl-info-container), 1)",
    "--hsl-on-info-container": "137, 41%, 9%",
    "--color-on-info-container": "hsla(var(--hsl-on-info-container), 1)",
    "--hsl-warning": "37,87%,53%",
    "--color-warning": "hsla(var(--hsl-warning), 1)",
    "--hsl-on-warning": "0, 0%, 100%",
    "--color-on-warning": "hsla(var(--hsl-on-warning), 1)",
    "--hsl-warning-container": "193, 76%, 85%",
    "--color-warning-container": "hsla(var(--hsl-warning-container), 1)",
    "--hsl-on-warning-container": "190, 100%, 7%",
    "--color-on-warning-container": "hsla(var(--hsl-on-warning-container), 1)",
    "--hsl-danger": "0, 75%, 42%",
    "--color-danger": "hsla(var(--hsl-danger), 1)",
    "--hsl-on-danger": "0, 0%, 100%",
    "--color-on-danger": "hsla(var(--hsl-on-danger), 1)",
    "--hsl-danger-container": "6, 100%, 92%",
    "--color-danger-container": "hsla(var(--hsl-danger-container), 1)",
    "--hsl-on-danger-container": "358, 100%, 13%",
    "--color-on-danger-container": "hsla(var(--hsl-on-danger-container), 1)",
    "--hsl-body": "80, 60%, 98%",
    "--color-body": "hsla(var(--hsl-body), 1)",
    "--hsl-text": "120, 6%, 10%",
    "--color-text": "hsla(var(--hsl-text), 1)",
    "--hsl-on-surface-variant": "120, 6%, 27%",
    "--color-on-surface-variant": "hsla(var(--hsl-on-surface-variant), 1)",
    "--hsl-outline": "120, 3%, 46%",
    "--color-outline": "hsla(var(--hsl-outline), 1)",
    "--hsl-inverse-surface": "120, 3%, 19%",
    "--color-inverse-surface": "hsla(var(--hsl-inverse-surface), 1)",
    "--hsl-surface-container": "80, 60%, 98%",
    "--hsl-surface-container-high": "80, 60%, 98%",
    "--color-surface-container-high": "hsla(var(--hsl-body), 1)",
  },
  "dark": {
    ...Themes.md3Dark,
    "--hsl-primary": "141, 64%, 63%",
    "--color-primary": "hsla(var(--hsl-primary), 1)",
    "--hsl-on-primary": "148, 100%, 11%",
    "--color-on-primary": "hsla(var(--hsl-on-primary), 1)",
    "--hsl-primary-container": "150, 100%, 16%",
    "--color-primary-container": "hsla(var(--hsl-primary-container), 1)",
    "--hsl-on-primary-container": "139, 94%, 75%",
    "--color-on-primary-container": "hsla(var(--hsl-on-primary-container), 1)",
    "--hsl-info": "125, 18%, 76%",
    "--color-info": "hsla(var(--hsl-info), 1)",
    "--hsl-on-info": "133, 22%, 17%",
    "--color-on-info": "hsla(var(--hsl-on-info), 1)",
    "--hsl-info-container": "133, 15%, 26%",
    "--color-info-container": "hsla(var(--hsl-info-container), 1)",
    "--hsl-on-info-container": "123, 32%, 87%",
    "--color-on-info-container": "hsla(var(--hsl-on-info-container), 1)",
    "--hsl-warning": "37, 87%, 53%",
    "--color-warning": "hsla(var(--hsl-warning), 1)",
    "--hsl-on-warning": "189, 97%, 13%",
    "--color-on-warning": "hsla(var(--hsl-on-warning), 1)",
    "--hsl-warning-container": "190, 46%, 23%",
    "--color-warning-container": "hsla(var(--hsl-warning-container), 1)",
    "--hsl-on-warning-container": "193, 76%, 85%",
    "--color-on-warning-container": "hsla(var(--hsl-on-warning-container), 1)",
    "--hsl-danger": "6, 100%, 84%",
    "--color-danger": "hsla(var(--hsl-danger), 1)",
    "--hsl-on-danger": "357, 100%, 21%",
    "--color-on-danger": "hsla(var(--hsl-on-danger), 1)",
    "--hsl-danger-container": "356, 100%, 29%",
    "--color-danger-container": "hsla(var(--hsl-danger-container), 1)",
    "--hsl-on-danger-container": "6, 100%, 84%",
    "--color-on-danger-container": "hsla(var(--hsl-on-danger-container), 1)",
    "--hsl-body": "120, 6%, 10%",
    "--color-body": "hsla(var(--hsl-body), 1)",
    "--hsl-text": "84, 8%, 88%",
    "--color-text": "hsla(var(--hsl-text), 1)",
    "--hsl-on-surface-variant": "108, 8%, 77%",
    "--color-on-surface-variant": "hsla(var(--hsl-on-surface-variant), 1)",
    "--hsl-outline": "113, 4%, 56%",
    "--color-outline": "hsla(var(--hsl-outline), 1)",
    "--hsl-inverse-surface": "84, 8%, 88%",
    "--color-inverse-surface": "hsla(var(--hsl-inverse-surface), 1)",
    "--hsl-surface-container": "120, 6%, 10%",
    "--hsl-surface-container-high": "120, 6%, 10%",
    "--color-surface-container-high": "hsla(var(--hsl-body), 1)",
  }
}
