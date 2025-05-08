import { presetVarlet } from '@varlet/preset-tailwindcss';

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx,vue}'],
  theme: {
    extend: {
      colors: {
        // primary: '#1989FA',
        // info: '#1989FA',
        // success: '#07C160',
        // warning: '#FF976A',
        // error: '#EE0A24',
      },
    },
  },
  corePlugins: {
    // preflight: false,
  },
  darkMode: 'selector',
  presets: [presetVarlet()],
};
