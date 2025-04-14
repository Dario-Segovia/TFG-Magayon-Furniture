<template>
    <form @submit.prevent="crear" class="clientes-form">
      <h2 class="clientes-form-title">Nuevo Cliente</h2>
      
      <input 
        v-model="nombre" 
        placeholder="Nombre completo" 
        class="clientes-input" 
        required 
      />
      <input 
        v-model="email" 
        placeholder="Correo electrónico" 
        class="clientes-input" 
        required 
      />
      <input 
        v-model="telefono" 
        placeholder="Teléfono" 
        class="clientes-input" 
      />
      <textarea 
        v-model="direccion" 
        placeholder="Dirección" 
        class="clientes-input clientes-textarea"
      ></textarea>
  
      <div class="clientes-actions">
        <button type="submit" class="clientes-submit-btn">
          Crear Cliente
        </button>
        <button type="button" @click="cancelar" class="clientes-cancel-btn">
          Cancelar
        </button>
      </div>
    </form>
  </template>
  
  <script setup>
  import { ref } from 'vue'
  import { invoke } from '@tauri-apps/api/core'
  
  const emit = defineEmits(['cliente-creado', 'cancelar'])
  
  const nombre = ref('')
  const email = ref('')
  const telefono = ref('')
  const direccion = ref('')
  
  const crear = async () => {
    await invoke('crear_cliente', {
      nombre: nombre.value,
      email: email.value,
      telefono: telefono.value,
      direccion: direccion.value,
    })
    nombre.value = ''
    email.value = ''
    telefono.value = ''
    direccion.value = ''
    emit('cliente-creado')
  }
  
  const cancelar = () => {
    // Limpia los campos y emite el evento de cancelar
    nombre.value = ''
    email.value = ''
    telefono.value = ''
    direccion.value = ''
    emit('cerrar') 
  }
  </script>
  
  <style scoped>
  .clientes-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 500px;
    margin: 0 auto;
    padding: 2rem;
    background-color: #f9fafb;
    border-radius: 0.75rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  }
  
  .clientes-form-title {
    font-size: 1.25rem;
    font-weight: bold;
    color: #111827;
    text-align: center;
    margin-bottom: 1rem;
  }
  
  .clientes-input {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-size: 1rem;
    background-color: #fff;
    transition: border-color 0.2s, box-shadow 0.2s;
  }
  
  .clientes-input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
  }
  
  .clientes-textarea {
    min-height: 100px;
    resize: vertical;
  }
  
  .clientes-actions {
    display: flex;
    justify-content: space-between;
    margin-top: 1rem;
  }
  
  .clientes-submit-btn {
    padding: 0.75rem 1.5rem;
    background-color: #2563eb;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .clientes-submit-btn:hover {
    background-color: #1d4ed8;
  }
  
  .clientes-submit-btn:active {
    background-color: #1e40af;
  }
  
  .clientes-cancel-btn {
    padding: 0.75rem 1.5rem;
    background-color: #9e9e9e;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .clientes-cancel-btn:hover {
    background-color: #757575;
  }
  
  .clientes-cancel-btn:active {
    background-color: #616161;
  }
  </style>
  