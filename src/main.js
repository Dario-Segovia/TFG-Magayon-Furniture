import { createApp } from "vue";
import App from "./App.vue";
import router from "./router/index.js";
import i18n from './i18n.js';
import 'vue-cal/dist/vuecal.css';
import '@fortawesome/fontawesome-free/css/all.css';
import PrimeVue from 'primevue/config'
import 'primevue/resources/themes/saga-blue/theme.css'
import 'primevue/resources/primevue.min.css'
import 'primeicons/primeicons.css'
import 'primeflex/primeflex.css'

import Card from 'primevue/card'


// IMPORTA LOS COMPONENTES QUE USAS
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Calendar from 'primevue/calendar'
import Checkbox from 'primevue/checkbox'
import InputNumber from 'primevue/inputnumber'
import Dropdown from 'primevue/dropdown'

const app = createApp(App)
  .use(router)
  .use(i18n)
  .use(PrimeVue)

// REGISTRA LOS COMPONENTES GLOBALMENTE
app.component('Dialog', Dialog)
app.component('Button', Button)
app.component('InputText', InputText)
app.component('Textarea', Textarea)
app.component('Calendar', Calendar)
app.component('Checkbox', Checkbox)
app.component('InputNumber', InputNumber)
app.component('Dropdown', Dropdown)
app.component('Card', Card)
app.mount('#app')