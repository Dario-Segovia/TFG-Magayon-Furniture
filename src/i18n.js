import { createI18n } from 'vue-i18n'
import es from './locales/es.json'
import en from './locales/en.json'
import zh from './locales/zh.json' // <-- Añade esta línea

const i18n = createI18n({
  locale: 'es', // Idioma por defecto
  fallbackLocale: 'en',
  legacy: false, // Necesario para Composition API
  messages: {
    es,
    en,
    zh // <-- Añade aquí
  }
})

export default i18n
