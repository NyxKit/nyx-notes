import { createApp } from 'vue'
import 'nyx-kit/style.css'
import './assets/theme.css'
import router from './router'
import App from './App.vue'

createApp(App).use(router).mount('#app')
