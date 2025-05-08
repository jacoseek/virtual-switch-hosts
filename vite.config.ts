import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import vueDevTools from 'vite-plugin-vue-devtools';
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import { VarletImportResolver } from '@varlet/import-resolver';
import { resolve } from 'node:path';
import vueRouter from 'unplugin-vue-router/vite';
import layouts from 'vite-plugin-vue-layouts';
import topLevelAwait from 'vite-plugin-top-level-await';

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(),
    // vueDevTools(),
    AutoImport({
      dts: 'types/auto-imports.d.ts',
      imports: [
        'vue',
        'pinia',
        'vue-router',
        'vue-i18n',
        {
          '@vueuse/core': [],
        },
      ],
      dirs: ['./src/composables', './src/stores', './src/utils', './src/store', './src/i18n'],
      eslintrc: {
        enabled: true,
      },
      resolvers: [VarletImportResolver({ autoImport: true })],
    }),
    Components({
      resolvers: [VarletImportResolver()],
    }),

    layouts(),

    vueRouter({
      routesFolder: [
        {
          src: 'src/pages',
        },
        {
          src: 'src/stacks',
          path: 'stacks/',
        },
      ],
      exclude: ['**/components/**', '**/composables/**', '**/lib/**'],
      extendRoute,
    }),
    topLevelAwait({
      // The export name of top-level await promise for each chunk module
      promiseExportName: '__tla',
      // The function to generate import names of top-level await promise in each chunk module
      promiseImportName: (i) => `__tla_${i}`,
    }),
  ],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
    },
  },
}));

import { EditableTreeNode } from 'unplugin-vue-router/types';

export interface StackRoute {
  name: string;
  children?: StackRoute[];
}

function extendRoute(route: EditableTreeNode) {
  const stacks = (route.meta?.stacks ?? []) as StackRoute[];

  processStacks(route, stacks);

  function processStacks(route: EditableTreeNode, stacks: (StackRoute | string)[]) {
    stacks.forEach((stack) => {
      const isStringifyStack = typeof stack === 'string';
      const name = isStringifyStack ? stack : stack.name;
      const newRoute = route.insert(name, `/src/stacks/${name}.vue`);

      if (!isStringifyStack && stack.children) {
        processStacks(newRoute, stack.children);
      }
    });
  }
}
