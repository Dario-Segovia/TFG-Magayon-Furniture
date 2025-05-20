<template>
  <div class="home-container">
    <header class="app-header">
      <div class="header-content">
        <h1 class="app-title">Magayon <span>Furniture</span></h1>
        <p class="welcome-message">Bienvenido, {{ user?.nombre || 'Usuario' }}</p>
      </div>
      <div class="header-gradient"></div>
    </header>
    
    <div class="modules-container">
      <!-- Módulo Empleados -->
      <div class="module-card" @click="goTo('empleados')" v-if="canView('empleados')">
        <div class="module-icon" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
          <i class="fas fa-users"></i>
        </div>
        <div class="module-content">
          <h3>Empleados</h3>
          <p>Gestión de empleados y horarios</p>
        </div>
        <div class="module-hover-effect"></div>
      </div>

      <!-- Módulo Inventario -->
      <div class="module-card" @click="goTo('inventario')" v-if="canView('inventario')">
        <div class="module-icon" style="background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);">
          <i class="fas fa-boxes"></i>
        </div>
        <div class="module-content">
          <h3>Inventario</h3>
          <p>Gestión de productos en inventario</p>
        </div>
        <div class="module-hover-effect"></div>
      </div>

      <!-- Módulo Clientes -->
      <div class="module-card" @click="goTo('clientes')" v-if="canView('clientes')">
        <div class="module-icon" style="background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);">
          <i class="fas fa-user-tie"></i>
        </div>
        <div class="module-content">
          <h3>Clientes</h3>
          <p>Gestión de clientes y sus datos</p>
        </div>
        <div class="module-hover-effect"></div>
      </div>

      <!-- Módulo Órdenes -->
      <div class="module-card" @click="goTo('ordenes')" v-if="canView('ordenes')">
        <div class="module-icon" style="background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);">
          <i class="fas fa-clipboard-list"></i>
        </div>
        <div class="module-content">
          <h3>Órdenes</h3>
          <p>Gestión de pedidos y ventas</p>
        </div>
        <div class="module-hover-effect"></div>
      </div>

      <!-- Módulo Proveedores -->
      <div class="module-card" @click="goTo('proveedores')" v-if="canView('proveedores')">
        <div class="module-icon" style="background: linear-gradient(135deg, #ff9a9e 0%, #fad0c4 100%);">
          <i class="fas fa-truck-loading"></i>
        </div>
        <div class="module-content">
          <h3>Proveedores</h3>
          <p>Gestión de proveedores y contactos</p>
        </div>
        <div class="module-hover-effect"></div>
      </div>

      <!-- Módulo Conversiones -->
      <div class="module-card" @click="goTo('conversiones')" v-if="canView('conversiones')">
        <div class="module-icon" style="background: linear-gradient(135deg, #a18cd1 0%, #fbc2eb 100%);">
          <i class="fa-solid fa-screwdriver-wrench"></i>
        </div>
        <div class="module-content">
          <h3>Utilidades</h3>
          <p>Herramientas diversas de ayuda</p>
        </div>
        <div class="module-hover-effect"></div>
      </div>

      <!-- Módulo Estadísticas -->
      <div class="module-card" @click="goTo('estadisticas')" v-if="canView('estadisticas')">
        <div class="module-icon" style="background: linear-gradient(135deg, #ffc3a0 0%, #ffafbd 100%);">
          <i class="fas fa-chart-pie"></i>
        </div>
        <div class="module-content">
          <h3>Estadísticas</h3>
          <p>Análisis de datos y rendimiento</p>
        </div>
        <div class="module-hover-effect"></div>
      </div>

      <!-- Botón de Cerrar Sesión -->
      <div class="module-card logout-card" @click="logout">
        <div class="module-icon" style="background: linear-gradient(135deg, #ff5858 0%, #f09819 100%);">
          <i class="fas fa-sign-out-alt"></i>
        </div>
        <div class="module-content">
          <h3>Cerrar Sesión</h3>
          <p>Salir del sistema</p>
        </div>
        <div class="module-hover-effect"></div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router';

const router = useRouter();

// Obtener el rol del usuario desde localStorage
const userStr = localStorage.getItem('user');
const user = userStr ? JSON.parse(userStr) : null;
const rol = user ? user.rol : null;

function logout() {
  localStorage.removeItem('user'); // Eliminar datos del usuario al cerrar sesión
  router.push('/'); // Redirigir al login
}

// Función para verificar si el módulo es accesible según el rol del usuario
function canView(module) {
  // Si el rol es 'admin', el usuario puede ver todos los módulos
  if (rol === 'admin') {
    return true;
  }
  // Si el rol es 'empleado', algunos módulos no serán visibles
  const restrictedModules = ['empleados', 'estadisticas']; // Módulos restringidos para empleados
  return !restrictedModules.includes(module);
}

function goTo(module) {
  router.push({ name: module });
}
</script>

<style scoped>
/* Estilos generales */
:root {
  --primary-color: #3a4f6c;
  --secondary-color: #f8b400;
  --text-color: #2d3436;
  --light-bg: #f9f9f9;
  --card-shadow: 0 10px 30px -5px rgba(0, 0, 0, 0.1);
}

.home-container {
  padding: 0;
  background-color: var(--light-bg);
  min-height: 100vh;
  font-family: 'Poppins', sans-serif;
}

/* Encabezado */
.app-header {
  background-color: var(--primary-color);
  color: white;
  padding: 2rem 2rem 4rem;
  position: relative;
  overflow: hidden;
  margin-bottom: 2rem;
}

.header-content {
  position: relative;
  z-index: 2;
  max-width: 1200px;
  margin: 0 auto;
}

.header-gradient {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(58, 79, 108, 0.8) 0%, rgba(26, 42, 64, 0.9) 100%);
  z-index: 1;
}

.app-title {
  font-size: 2.5rem;
  font-weight: 700;
  margin: 0;
  letter-spacing: 1px;
}

.app-title span {
  color: var(--secondary-color);
  font-weight: 600;
}

.welcome-message {
  font-size: 1.1rem;
  opacity: 0.9;
  margin-top: 0.5rem;
}

/* Contenedor de módulos */
.modules-container {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 25px;
  padding: 0 2rem 2rem;
  max-width: 1200px;
  margin: -3rem auto 0;
  position: relative;
  z-index: 3;
}

/* Tarjetas de módulo */
.module-card {
  background-color: white;
  border-radius: 12px;
  box-shadow: var(--card-shadow);
  padding: 25px;
  cursor: pointer;
  transition: all 0.3s ease;
  overflow: hidden;
  position: relative;
  height: 220px;
  display: flex;
  flex-direction: column;
  border: 1px solid rgba(0, 0, 0, 0.05);
}

.module-card.logout-card {
  border: 2px dashed rgba(0, 0, 0, 0.1);
}

.module-icon {
  width: 70px;
  height: 70px;
  border-radius: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 20px;
  color: white;
  font-size: 28px;
  transition: all 0.3s ease;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
}

.module-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.module-content h3 {
  margin: 0 0 10px;
  font-size: 1.3rem;
  color: var(--text-color);
  font-weight: 600;
}

.module-content p {
  margin: 0;
  font-size: 0.95rem;
  color: #636e72;
  line-height: 1.5;
}

.module-hover-effect {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0) 100%);
  opacity: 0;
  transition: opacity 0.3s ease;
}

/* Efectos hover */
.module-card:hover {
  transform: translateY(-8px);
  box-shadow: 0 15px 30px -5px rgba(0, 0, 0, 0.15);
}

.module-card:hover .module-hover-effect {
  opacity: 1;
}

.module-card:hover .module-icon {
  transform: scale(1.1);
}

.module-card:active {
  transform: translateY(-2px);
}

/* Responsive */
@media (max-width: 768px) {
  .modules-container {
    grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
    padding: 0 1rem 2rem;
  }
  
  .app-header {
    padding: 1.5rem 1rem 3rem;
  }
  
  .app-title {
    font-size: 2rem;
  }
}

@media (max-width: 480px) {
  .modules-container {
    grid-template-columns: 1fr;
  }
}
</style>

