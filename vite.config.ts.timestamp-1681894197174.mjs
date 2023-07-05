// vite.config.ts
import { defineConfig } from "file:///e:/RustProject/ChatGPT-Desktop-master/node_modules/vite/dist/node/index.js";
import vue from "file:///e:/RustProject/ChatGPT-Desktop-master/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import autoprefixer from "file:///e:/RustProject/ChatGPT-Desktop-master/node_modules/autoprefixer/lib/autoprefixer.js";
import Unocss from "file:///e:/RustProject/ChatGPT-Desktop-master/node_modules/unocss/dist/vite.mjs";
import { presetUno, presetIcons, transformerDirectives } from "file:///e:/RustProject/ChatGPT-Desktop-master/node_modules/unocss/dist/index.mjs";
import presetAutoprefixer from "file:///e:/RustProject/ChatGPT-Desktop-master/node_modules/unocss-preset-autoprefixer/dist/index.mjs";
import AutoImport from "file:///e:/RustProject/ChatGPT-Desktop-master/node_modules/unplugin-auto-import/dist/vite.js";
import Components from "file:///e:/RustProject/ChatGPT-Desktop-master/node_modules/unplugin-vue-components/dist/vite.mjs";
import { ArcoResolver } from "file:///e:/RustProject/ChatGPT-Desktop-master/node_modules/unplugin-vue-components/dist/resolvers.mjs";
import { visualizer } from "file:///e:/RustProject/ChatGPT-Desktop-master/node_modules/rollup-plugin-visualizer/dist/plugin/index.js";
import topLevelAwait from "file:///e:/RustProject/ChatGPT-Desktop-master/node_modules/vite-plugin-top-level-await/exports/import.mjs";
import VueI18nPlugin from "file:///e:/RustProject/ChatGPT-Desktop-master/node_modules/@intlify/unplugin-vue-i18n/lib/vite.mjs";
var vite_config_default = defineConfig(async () => ({
  plugins: [
    vue(),
    VueI18nPlugin({}),
    Unocss({
      presets: [presetUno(), presetIcons(), presetAutoprefixer()],
      transformers: [
        transformerDirectives({
          applyVariable: ["--uno"]
        })
      ]
    }),
    AutoImport({
      dts: "./src/types/auto-import.d.ts",
      eslintrc: {
        enabled: true
      },
      imports: [
        "vue",
        "pinia",
        "vue-i18n",
        {
          "@arco-design/web-vue": ["Message"]
        }
      ],
      resolvers: [ArcoResolver()],
      vueTemplate: true,
      dirs: [
        "./src/api/*",
        "./src/constants/*",
        "./src/hooks/*",
        "./src/sqls/*",
        "./src/stores/*",
        "./src/utils/*"
      ]
    }),
    Components({
      dts: "./src/types/components.d.ts",
      resolvers: [
        ArcoResolver({
          resolveIcons: true
        })
      ]
    }),
    visualizer(),
    topLevelAwait({
      // The export name of top-level await promise for each chunk module
      promiseExportName: "__tla",
      // The function to generate import names of top-level await promise in each chunk module
      promiseImportName: (i) => `__tla_${i}`
    })
  ],
  resolve: {
    alias: {
      "@": "/src"
    }
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: "0.0.0.0"
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG
  },
  css: {
    postcss: {
      plugins: [
        autoprefixer({
          overrideBrowserslist: [
            "Android 4.1",
            "iOS 7.1",
            "Chrome > 31",
            "ff > 31",
            "ie >= 8",
            "last 10 versions"
          ]
        })
      ]
    }
  }
}));
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJlOlxcXFxSdXN0UHJvamVjdFxcXFxDaGF0R1BULURlc2t0b3AtbWFzdGVyXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ZpbGVuYW1lID0gXCJlOlxcXFxSdXN0UHJvamVjdFxcXFxDaGF0R1BULURlc2t0b3AtbWFzdGVyXFxcXHZpdGUuY29uZmlnLnRzXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ltcG9ydF9tZXRhX3VybCA9IFwiZmlsZTovLy9lOi9SdXN0UHJvamVjdC9DaGF0R1BULURlc2t0b3AtbWFzdGVyL3ZpdGUuY29uZmlnLnRzXCI7aW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSAndml0ZSdcbmltcG9ydCB2dWUgZnJvbSAnQHZpdGVqcy9wbHVnaW4tdnVlJ1xuaW1wb3J0IGF1dG9wcmVmaXhlciBmcm9tICdhdXRvcHJlZml4ZXInXG5pbXBvcnQgVW5vY3NzIGZyb20gJ3Vub2Nzcy92aXRlJ1xuaW1wb3J0IHsgcHJlc2V0VW5vLCBwcmVzZXRJY29ucywgdHJhbnNmb3JtZXJEaXJlY3RpdmVzIH0gZnJvbSAndW5vY3NzJ1xuaW1wb3J0IHByZXNldEF1dG9wcmVmaXhlciBmcm9tICd1bm9jc3MtcHJlc2V0LWF1dG9wcmVmaXhlcidcbmltcG9ydCBBdXRvSW1wb3J0IGZyb20gJ3VucGx1Z2luLWF1dG8taW1wb3J0L3ZpdGUnXG5pbXBvcnQgQ29tcG9uZW50cyBmcm9tICd1bnBsdWdpbi12dWUtY29tcG9uZW50cy92aXRlJ1xuaW1wb3J0IHsgQXJjb1Jlc29sdmVyIH0gZnJvbSAndW5wbHVnaW4tdnVlLWNvbXBvbmVudHMvcmVzb2x2ZXJzJ1xuaW1wb3J0IHsgdmlzdWFsaXplciB9IGZyb20gJ3JvbGx1cC1wbHVnaW4tdmlzdWFsaXplcidcbmltcG9ydCB0b3BMZXZlbEF3YWl0IGZyb20gJ3ZpdGUtcGx1Z2luLXRvcC1sZXZlbC1hd2FpdCdcbmltcG9ydCBWdWVJMThuUGx1Z2luIGZyb20gJ0BpbnRsaWZ5L3VucGx1Z2luLXZ1ZS1pMThuL3ZpdGUnXG5cbmV4cG9ydCBkZWZhdWx0IGRlZmluZUNvbmZpZyhhc3luYyAoKSA9PiAoe1xuICBwbHVnaW5zOiBbXG4gICAgdnVlKCksXG4gICAgVnVlSTE4blBsdWdpbih7fSksXG4gICAgVW5vY3NzKHtcbiAgICAgIHByZXNldHM6IFtwcmVzZXRVbm8oKSwgcHJlc2V0SWNvbnMoKSwgcHJlc2V0QXV0b3ByZWZpeGVyKCldLFxuICAgICAgdHJhbnNmb3JtZXJzOiBbXG4gICAgICAgIHRyYW5zZm9ybWVyRGlyZWN0aXZlcyh7XG4gICAgICAgICAgYXBwbHlWYXJpYWJsZTogWyctLXVubyddXG4gICAgICAgIH0pXG4gICAgICBdXG4gICAgfSksXG4gICAgQXV0b0ltcG9ydCh7XG4gICAgICBkdHM6ICcuL3NyYy90eXBlcy9hdXRvLWltcG9ydC5kLnRzJyxcbiAgICAgIGVzbGludHJjOiB7XG4gICAgICAgIGVuYWJsZWQ6IHRydWVcbiAgICAgIH0sXG4gICAgICBpbXBvcnRzOiBbXG4gICAgICAgICd2dWUnLFxuICAgICAgICAncGluaWEnLFxuICAgICAgICAndnVlLWkxOG4nLFxuICAgICAgICB7XG4gICAgICAgICAgJ0BhcmNvLWRlc2lnbi93ZWItdnVlJzogWydNZXNzYWdlJ11cbiAgICAgICAgfVxuICAgICAgXSxcbiAgICAgIHJlc29sdmVyczogW0FyY29SZXNvbHZlcigpXSxcbiAgICAgIHZ1ZVRlbXBsYXRlOiB0cnVlLFxuICAgICAgZGlyczogW1xuICAgICAgICAnLi9zcmMvYXBpLyonLFxuICAgICAgICAnLi9zcmMvY29uc3RhbnRzLyonLFxuICAgICAgICAnLi9zcmMvaG9va3MvKicsXG4gICAgICAgICcuL3NyYy9zcWxzLyonLFxuICAgICAgICAnLi9zcmMvc3RvcmVzLyonLFxuICAgICAgICAnLi9zcmMvdXRpbHMvKidcbiAgICAgIF1cbiAgICB9KSxcbiAgICBDb21wb25lbnRzKHtcbiAgICAgIGR0czogJy4vc3JjL3R5cGVzL2NvbXBvbmVudHMuZC50cycsXG4gICAgICByZXNvbHZlcnM6IFtcbiAgICAgICAgQXJjb1Jlc29sdmVyKHtcbiAgICAgICAgICByZXNvbHZlSWNvbnM6IHRydWVcbiAgICAgICAgfSlcbiAgICAgIF1cbiAgICB9KSxcbiAgICB2aXN1YWxpemVyKCksXG4gICAgdG9wTGV2ZWxBd2FpdCh7XG4gICAgICAvLyBUaGUgZXhwb3J0IG5hbWUgb2YgdG9wLWxldmVsIGF3YWl0IHByb21pc2UgZm9yIGVhY2ggY2h1bmsgbW9kdWxlXG4gICAgICBwcm9taXNlRXhwb3J0TmFtZTogJ19fdGxhJyxcbiAgICAgIC8vIFRoZSBmdW5jdGlvbiB0byBnZW5lcmF0ZSBpbXBvcnQgbmFtZXMgb2YgdG9wLWxldmVsIGF3YWl0IHByb21pc2UgaW4gZWFjaCBjaHVuayBtb2R1bGVcbiAgICAgIHByb21pc2VJbXBvcnROYW1lOiAoaSkgPT4gYF9fdGxhXyR7aX1gXG4gICAgfSlcbiAgXSxcbiAgcmVzb2x2ZToge1xuICAgIGFsaWFzOiB7XG4gICAgICAnQCc6ICcvc3JjJ1xuICAgIH1cbiAgfSxcbiAgY2xlYXJTY3JlZW46IGZhbHNlLFxuICBzZXJ2ZXI6IHtcbiAgICBwb3J0OiAxNDIwLFxuICAgIHN0cmljdFBvcnQ6IHRydWUsXG4gICAgaG9zdDogJzAuMC4wLjAnXG4gIH0sXG4gIGVudlByZWZpeDogWydWSVRFXycsICdUQVVSSV8nXSxcbiAgYnVpbGQ6IHtcbiAgICB0YXJnZXQ6IHByb2Nlc3MuZW52LlRBVVJJX1BMQVRGT1JNID09ICd3aW5kb3dzJyA/ICdjaHJvbWUxMDUnIDogJ3NhZmFyaTEzJyxcbiAgICBtaW5pZnk6ICFwcm9jZXNzLmVudi5UQVVSSV9ERUJVRyA/ICdlc2J1aWxkJyA6IGZhbHNlLFxuICAgIHNvdXJjZW1hcDogISFwcm9jZXNzLmVudi5UQVVSSV9ERUJVR1xuICB9LFxuICBjc3M6IHtcbiAgICBwb3N0Y3NzOiB7XG4gICAgICBwbHVnaW5zOiBbXG4gICAgICAgIGF1dG9wcmVmaXhlcih7XG4gICAgICAgICAgb3ZlcnJpZGVCcm93c2Vyc2xpc3Q6IFtcbiAgICAgICAgICAgICdBbmRyb2lkIDQuMScsXG4gICAgICAgICAgICAnaU9TIDcuMScsXG4gICAgICAgICAgICAnQ2hyb21lID4gMzEnLFxuICAgICAgICAgICAgJ2ZmID4gMzEnLFxuICAgICAgICAgICAgJ2llID49IDgnLFxuICAgICAgICAgICAgJ2xhc3QgMTAgdmVyc2lvbnMnXG4gICAgICAgICAgXVxuICAgICAgICB9KVxuICAgICAgXVxuICAgIH1cbiAgfVxufSkpXG4iXSwKICAibWFwcGluZ3MiOiAiO0FBQXVTLFNBQVMsb0JBQW9CO0FBQ3BVLE9BQU8sU0FBUztBQUNoQixPQUFPLGtCQUFrQjtBQUN6QixPQUFPLFlBQVk7QUFDbkIsU0FBUyxXQUFXLGFBQWEsNkJBQTZCO0FBQzlELE9BQU8sd0JBQXdCO0FBQy9CLE9BQU8sZ0JBQWdCO0FBQ3ZCLE9BQU8sZ0JBQWdCO0FBQ3ZCLFNBQVMsb0JBQW9CO0FBQzdCLFNBQVMsa0JBQWtCO0FBQzNCLE9BQU8sbUJBQW1CO0FBQzFCLE9BQU8sbUJBQW1CO0FBRTFCLElBQU8sc0JBQVEsYUFBYSxhQUFhO0FBQUEsRUFDdkMsU0FBUztBQUFBLElBQ1AsSUFBSTtBQUFBLElBQ0osY0FBYyxDQUFDLENBQUM7QUFBQSxJQUNoQixPQUFPO0FBQUEsTUFDTCxTQUFTLENBQUMsVUFBVSxHQUFHLFlBQVksR0FBRyxtQkFBbUIsQ0FBQztBQUFBLE1BQzFELGNBQWM7QUFBQSxRQUNaLHNCQUFzQjtBQUFBLFVBQ3BCLGVBQWUsQ0FBQyxPQUFPO0FBQUEsUUFDekIsQ0FBQztBQUFBLE1BQ0g7QUFBQSxJQUNGLENBQUM7QUFBQSxJQUNELFdBQVc7QUFBQSxNQUNULEtBQUs7QUFBQSxNQUNMLFVBQVU7QUFBQSxRQUNSLFNBQVM7QUFBQSxNQUNYO0FBQUEsTUFDQSxTQUFTO0FBQUEsUUFDUDtBQUFBLFFBQ0E7QUFBQSxRQUNBO0FBQUEsUUFDQTtBQUFBLFVBQ0Usd0JBQXdCLENBQUMsU0FBUztBQUFBLFFBQ3BDO0FBQUEsTUFDRjtBQUFBLE1BQ0EsV0FBVyxDQUFDLGFBQWEsQ0FBQztBQUFBLE1BQzFCLGFBQWE7QUFBQSxNQUNiLE1BQU07QUFBQSxRQUNKO0FBQUEsUUFDQTtBQUFBLFFBQ0E7QUFBQSxRQUNBO0FBQUEsUUFDQTtBQUFBLFFBQ0E7QUFBQSxNQUNGO0FBQUEsSUFDRixDQUFDO0FBQUEsSUFDRCxXQUFXO0FBQUEsTUFDVCxLQUFLO0FBQUEsTUFDTCxXQUFXO0FBQUEsUUFDVCxhQUFhO0FBQUEsVUFDWCxjQUFjO0FBQUEsUUFDaEIsQ0FBQztBQUFBLE1BQ0g7QUFBQSxJQUNGLENBQUM7QUFBQSxJQUNELFdBQVc7QUFBQSxJQUNYLGNBQWM7QUFBQTtBQUFBLE1BRVosbUJBQW1CO0FBQUE7QUFBQSxNQUVuQixtQkFBbUIsQ0FBQyxNQUFNLFNBQVM7QUFBQSxJQUNyQyxDQUFDO0FBQUEsRUFDSDtBQUFBLEVBQ0EsU0FBUztBQUFBLElBQ1AsT0FBTztBQUFBLE1BQ0wsS0FBSztBQUFBLElBQ1A7QUFBQSxFQUNGO0FBQUEsRUFDQSxhQUFhO0FBQUEsRUFDYixRQUFRO0FBQUEsSUFDTixNQUFNO0FBQUEsSUFDTixZQUFZO0FBQUEsSUFDWixNQUFNO0FBQUEsRUFDUjtBQUFBLEVBQ0EsV0FBVyxDQUFDLFNBQVMsUUFBUTtBQUFBLEVBQzdCLE9BQU87QUFBQSxJQUNMLFFBQVEsUUFBUSxJQUFJLGtCQUFrQixZQUFZLGNBQWM7QUFBQSxJQUNoRSxRQUFRLENBQUMsUUFBUSxJQUFJLGNBQWMsWUFBWTtBQUFBLElBQy9DLFdBQVcsQ0FBQyxDQUFDLFFBQVEsSUFBSTtBQUFBLEVBQzNCO0FBQUEsRUFDQSxLQUFLO0FBQUEsSUFDSCxTQUFTO0FBQUEsTUFDUCxTQUFTO0FBQUEsUUFDUCxhQUFhO0FBQUEsVUFDWCxzQkFBc0I7QUFBQSxZQUNwQjtBQUFBLFlBQ0E7QUFBQSxZQUNBO0FBQUEsWUFDQTtBQUFBLFlBQ0E7QUFBQSxZQUNBO0FBQUEsVUFDRjtBQUFBLFFBQ0YsQ0FBQztBQUFBLE1BQ0g7QUFBQSxJQUNGO0FBQUEsRUFDRjtBQUNGLEVBQUU7IiwKICAibmFtZXMiOiBbXQp9Cg==
