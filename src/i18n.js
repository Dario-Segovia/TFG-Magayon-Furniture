import { createI18n } from 'vue-i18n'
import es from './locales/es.json'
import en from './locales/en.json'

const i18n = createI18n({
  locale: 'es', // Idioma por defecto
  fallbackLocale: 'en',
  legacy: false, // Necesario para Composition API
  messages: {
    es,
    en
  }
})

export default i18n
