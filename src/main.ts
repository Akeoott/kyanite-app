import App from '@/App.vue'
import router from '@/router'
import '@/styles/styles.css'
import { createApp } from 'vue'

const app = createApp(App)

app.use(router)
app.mount('#app')

if ('scrollRestoration' in history) history.scrollRestoration = 'manual'
