import { createApp } from "vue";
import App from "./App.vue";
import router from "./router/index.js";
import i18n from './i18n.js';
import '@fortawesome/fontawesome-free/css/all.css';

createApp(App)
.use(router)
.use(i18n)
.mount('#app')