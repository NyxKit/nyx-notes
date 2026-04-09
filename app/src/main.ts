import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { NyxKit } from 'nyx-kit'
import { NyxTheme, NyxSize, NyxVariant } from 'nyx-kit/types'
import 'nyx-kit/style.css'
import './shared/assets'
import { installConsoleCapture } from './shared/utils'
import router from './shared/router'
import App from './App.vue'

installConsoleCapture()

createApp(App)
  .use(createPinia())
  .use(router)
  .use(NyxKit, {
    defaults: {
      all: {
        theme: NyxTheme.Primary,
        size: NyxSize.Medium,
        variant: NyxVariant.Subtle,
      },
      button: {
        theme: NyxTheme.Primary,
        size: NyxSize.Medium,
        variant: NyxVariant.Soft,
      }
    },
  })
  .mount('#app')
