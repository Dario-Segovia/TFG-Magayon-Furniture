<template>
  <button class="btn-back" @click="$router.back()">
    <i class="fas fa-arrow-left"></i> {{ $t('clientes.back') }}
  </button>
  <div class="clientes-page">
    <div class="clientes-header fade-in">
      <div class="header-content">
        <h1><i class="fas fa-users"></i> {{ $t('clientes.titulo') }}</h1>
        <p class="subtitle">{{ $t('clientes.bienvenida') }}</p>
      </div>
      <!-- <button class="btn-primary"><i class="fas fa-plus"></i> {{ $t('clientes.nuevo') }}</button> -->
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
  padding: 20px;
  max-width: 1800px;
  margin: 0 auto;
  background: #f4f7f9;
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
}
.clientes-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 22px;
  padding: 22px;
  background: linear-gradient(135deg, #5A8DEE, #A084EE); /* Gradiente azul-morado */
  border-radius: 10px;
  color: white;
  box-shadow: 0 4px 12px rgba(0,0,0,0.10);
  animation: fadeIn 0.5s;
}
.header-content h1 {
  margin: 0;
  font-size: 1.7rem;
  display: flex;
  align-items: center;
  gap: 10px;
}
.subtitle {
  margin: 0;
  font-size: 1rem;
  color: #e0e7ff;
}
.fade-in {
  animation: fadeIn 0.5s;
}
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}
.clientes-content {
  padding: 1.5rem;
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.btn-primary {
  padding: 12px 20px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 10px;
  font-weight: 600;
  font-size: 1rem;
  background: linear-gradient(135deg, #5A8DEE, #A084EE);
  color: white;
  box-shadow: 0 4px 6px rgba(0,0,0,0.10);
  transition: transform 0.2s, box-shadow 0.2s, background 0.3s;
}
.btn-primary:hover {
  background: linear-gradient(135deg, #A084EE, #5A8DEE);
  transform: translateY(-2px);
  box-shadow: 0 6px 10px rgba(0,0,0,0.15);
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