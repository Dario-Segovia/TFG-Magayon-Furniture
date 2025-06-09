<template>
  <div class="login-container">
    <!-- Banderas FUERA del cuadro de login -->
    <div class="lang-switch">
      <img
        src="/banderas/es.png"
        alt="Español"
        @click="changeLang('es')"
        :class="{ active: locale === 'es' }"
      />
      <img
        src="/banderas/en.png"
        alt="English"
        @click="changeLang('en')"
        :class="{ active: locale === 'en' }"
      />
      <img
        src="/banderas/zh.png"
        alt="中文"
        @click="changeLang('zh')"
        :class="{ active: locale === 'zh' }"
      />
    </div>
    <div v-if="isLoading" class="loading-overlay">
    <div class="spinner"></div>
  </div>
    <!-- Cuadro de login -->
    <div class="login-card">
      <h2>{{ $t('login.title') }}</h2>

      <input v-model="usuario" :placeholder="$t('login.username')" />
      <input v-model="password" type="password" :placeholder="$t('login.password')" />

      <button @click="login">{{ $t('login.submit') }}</button>

      <p v-if="error" class="error">{{ error }}</p>
      <p v-if="rol" class="success">{{ $t('login.access') }}: {{ rol }}</p>
 
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";


const usuario = ref('');
const password = ref('');
const rol = ref('');
const error = ref('');
const router = useRouter();
const isLoading = ref(false);


const { locale } = useI18n();

function changeLang(lang) {
  locale.value = lang;
  localStorage.setItem('idioma', lang);
}

onMounted(() => {
  const idiomaGuardado = localStorage.getItem('idioma');
  if (idiomaGuardado) {
    locale.value = idiomaGuardado;
  }
});


async function login() {
  isLoading.value = true;  // Activar el estado de carga
  try {
    // Llamada a la función de backend para autenticar al usuario
    const result = await invoke('login', { usuario: usuario.value, password: password.value });
    rol.value = result;

    // Almacena el usuario y rol en localStorage
    const user = { 
      usuario: usuario.value, 
      rol: result.rol || result, 
      nombre: result.nombre || usuario.value // Usa el nombre real si está disponible
    };
    localStorage.setItem('user', JSON.stringify(user));

    // Redirige al home después de un login exitoso
    if (result === 'admin') {
      router.push({ name: 'home' });
    } else if (result === 'empleado') {
      router.push({ name: 'home' });
    } else {
      error.value = 'Rol no válido';
    }
  } catch (err) {
    // Manejo de errores
    if (err === 'Usuario no encontrado') {
      error.value = 'Usuario no encontrado';
    } else if (err === 'Credenciales incorrectas') {
      error.value = 'Credenciales incorrectas';
    } else {
      error.value = 'Error en el servidor';
    }
  } finally {
    isLoading.value = false;  // Desactivar el estado de carga
  }
}




</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;600&display=swap');

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

.login-container {
  min-height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  background: linear-gradient(145deg, #f0f2f5, #d6d9dd);
  padding: 2rem;
  position: relative;
}

/* Banderas de idioma */
.lang-switch {
  position: absolute;
  top: clamp(1rem, 3vw, 2rem);
  right: clamp(1rem, 3vw, 2rem);
  display: flex;
  gap: clamp(0.5rem, 2vw, 1rem);
  z-index: 10;
}

.lang-switch img {
  width: clamp(30px, 5vw, 40px);
  height: clamp(30px, 5vw, 40px);
  cursor: pointer;
  filter: grayscale(100%);
  transition: 0.3s;
  border-radius: 50%;
  border: 2px solid transparent;
}

.lang-switch img.active,
.lang-switch img:hover {
  filter: none;
  border-color: #007bff;
  transform: scale(1.1);
}

/* Tarjeta de login */
.login-card {
  background-color: white;
  padding: clamp(2rem, 5vw, 3rem);
  border-radius: clamp(1rem, 2vw, 1.5rem);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
  width: 100%;
  max-width: min(90vw, 400px);
  min-width: 280px;
  display: flex;
  flex-direction: column;
  align-items: center;
}

h2 {
  margin-bottom: clamp(1rem, 3vw, 1.5rem);
  color: #333;
  font-size: clamp(1.5rem, 4vw, 2rem);
}

input {
  margin: clamp(0.5rem, 2vw, 1rem) 0;
  padding: clamp(0.75rem, 2vw, 1rem);
  width: 100%;
  border: 1px solid #ccc;
  border-radius: clamp(0.5rem, 1.5vw, 0.8rem);
  font-size: clamp(0.9rem, 2vw, 1rem);
}

button {
  margin-top: clamp(1rem, 3vw, 1.5rem);
  padding: clamp(0.75rem, 2vw, 1rem);
  width: 100%;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: clamp(0.5rem, 1.5vw, 0.8rem);
  font-weight: 600;
  font-size: clamp(0.9rem, 2vw, 1rem);
  cursor: pointer;
  transition: 0.3s;
}

button:hover {
  background-color: #0056b3;
}

.error, .success {
  margin-top: clamp(0.5rem, 2vw, 1rem);
  font-size: clamp(0.8rem, 2vw, 0.9rem);
}

.error {
  color: red;
}

.success {
  color: green;
}

/* Media queries para ajustes específicos */
@media (max-width: 480px) {
  .login-container {
    padding: 1rem;
  }
  
  .login-card {
    padding: 1.5rem;
  }
}

@media (max-height: 600px) {
  .login-container {
    align-items: flex-start;
    padding-top: 4rem;
  }
}


.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 20;
}

.spinner {
  border: 4px solid rgba(0, 0, 0, 0.1);
  border-top: 4px solid #007bff;
  border-radius: 50%;
  width: 50px;
  height: 50px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>