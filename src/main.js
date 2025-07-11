import { createApp } from 'vue'
import { createPinia } from 'pinia'
import Antd from 'ant-design-vue'
import router from './router'
import i18n from './i18n'
import App from './App.vue'

import 'ant-design-vue/dist/reset.css'
import './style.css'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(router)
app.use(Antd)
app.use(i18n)

app.mount('#app') 