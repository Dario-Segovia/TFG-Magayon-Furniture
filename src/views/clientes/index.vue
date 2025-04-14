<template>
    <div class="p-6 space-y-4">
      <h1 class="text-2xl font-bold">Gestión de Clientes</h1>
      <ClientesTabla />
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