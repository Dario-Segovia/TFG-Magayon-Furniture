<template>
 <button class="btn-back" @click="$router.back()">
  <i class="fas fa-arrow-left"></i> Atrás
</button>
  <div class="clientes-page">
    <div class="clientes-header">
      <h1 class="clientes-title">Gestión de Clientes</h1>
    </div>
    <div class="clientes-content">
      <ClientesTabla />
    </div>
  </div>
</template>
  
<script setup>
import { ref, onMounted } from 'vue'
import ClientesTabla from './components/ClientesTabla.vue'
import { invoke } from '@tauri-apps/api/core'

const clientes = ref([])

const fetchClientes = async () => {
  clientes.value = await invoke('obtener_clientes')
}

onMounted(fetchClientes)
</script>

<style scoped>
.clientes-page {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  padding: 2rem;
  background-color: #f9fafb; /* Fondo claro */
  min-height: 100vh; /* Asegura que ocupe toda la pantalla */
}

.clientes-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.clientes-title {
  font-size: 1.875rem; /* Tamaño de texto grande */
  font-weight: 700;
  color: #111827; /* Texto oscuro */
  margin: 0;
}

.clientes-content {
  padding: 1.5rem;
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}


.btn-back {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  background: #ffffff;
  border: none;
  color: #2b4583;
  font-size: 1.08rem;
  font-weight: 500;
  padding: 8px 16px;
  border-radius: 8px;
  margin-bottom: 18px;
  cursor: pointer;
  transition: background 0.18s, color 0.18s;
}
.btn-back:hover {
  background: #e9ecef;
  color: #1a2b4c;
}
</style>