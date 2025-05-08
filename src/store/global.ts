export const useGlobalStore = defineStore('global', () => {
  const theme = ref<ThemeType>(ThemeType.Unset);
  watch(theme, (val) => {
    changeTheme(val);
  });

  const language = ref<LanguageType>(LanguageType.enUS);

  watch(language, (val) => {
    setLang(val);
  });

  // action
  const init = async () => {
    // init theme
    theme.value = (await window.$storage.get('theme')) || ThemeType.Auto;
    // init language
    language.value = (await window.$storage.get('language')) || LanguageType.enUS;
  };

  return {
    theme,
    language,
    // action
    init,
  };
});
