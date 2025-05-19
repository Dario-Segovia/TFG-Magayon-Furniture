<template>
  <div class="app-container">
    <!-- Contenido principal -->
    <div class="main-content">
      <!-- Área de contenido -->
      <main class="content-area">
        <div v-if="vistaActual === 'compras'" class="module-container">
          <Compras />
        </div>

        <div v-else class="module-container">
          <Ventas />
        </div>
      </main>

      <!-- Barra lateral derecha -->
      <aside class="sidebar">
        <button
          @click="vistaActual = 'compras'"
          :class="{ 'active': vistaActual === 'compras' }"
          class="menu-button"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" />
          </svg>
          <span>Compras</span>
        </button>

        <button
          @click="vistaActual = 'ventas'"
          :class="{ 'active': vistaActual === 'ventas' }"
          class="menu-button"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
          </svg>
          <span>Ventas</span>
        </button>
      </aside>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import Compras from './compras/index.vue'
import Ventas from './ventas/index.vue'

const vistaActual = ref('compras')
</script>

<style>
/* Estilos base */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: #f8f9fa;
}

/* Contenido principal */
.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.content-area {
  flex: 1;
  padding: 16px;
  overflow: auto;
}

.module-container {
  background-color: white;
  padding: 16px;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  height: 100%;
  
}

/* Barra lateral */
.sidebar {
  width: 80px;
  background-color: white;
  border-left: 1px solid #e5e7eb;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px 0;
  gap: 16px;
}

.menu-button {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  padding: 12px 0;
  background: none;
  border: none;
  cursor: pointer;
  color: #6b7280;
  transition: all 0.2s ease;
}

.menu-button:hover {
  background-color: #f3f4f6;
  color: #4b5563;
}

.menu-button.active {
  color: #4f46e5;
  background-color: #eef2ff;
}

.menu-button.active svg {
  stroke: #4f46e5;
}

.menu-button svg {
  width: 24px;
  height: 24px;
  stroke: #6b7280;
  margin-bottom: 4px;
}

.menu-button span {
  font-size: 0.75rem;
  font-weight: 500;
}

/* Scrollbar */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: #f1f1f1;
}

::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #a1a1a1;
}
</style>
