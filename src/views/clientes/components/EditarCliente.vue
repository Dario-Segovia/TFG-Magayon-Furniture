<template>
    <form @submit.prevent="actualizar" class="clientes-edit-form">
      <input 
        v-model="nombre" 
        class="clientes-edit-input" 
        required 
      />
      <input 
        v-model="email" 
        class="clientes-edit-input" 
        required 
      />
      <input 
        v-model="telefono" 
        class="clientes-edit-input" 
      />
      <textarea 
        v-model="direccion" 
        class="clientes-edit-input clientes-edit-textarea"
      ></textarea>
      <div class="clientes-edit-buttons">
        <button type="submit" class="clientes-edit-submit">
          Actualizar
        </button>
        <button @click.prevent="emit('cerrar')" class="clientes-edit-cancel">
          Cancelar
        </button>
      </div>
    </form>
  </template>
  
  <script setup>
  import { ref } from 'vue'
  import { invoke } from '@tauri-apps/api/core'
  
  const props = defineProps({ cliente: Object })
  const emit = defineEmits(['actualizado', 'cerrar'])
  
  const nombre = ref(props.cliente.nombre)
  const email = ref(props.cliente.email)
  const telefono = ref(props.cliente.telefono)
  const direccion = ref(props.cliente.direccion)
  
  const actualizar = async () => {
    await invoke('actualizar_cliente', {
      id: props.cliente.id,
      nombre: nombre.value,
      email: email.value,
      telefono: telefono.value,
      direccion: direccion.value
    })
    emit('actualizado')
    emit('cerrar')
  }
  </script>
  
  <style scoped>
  .clientes-edit-form {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-width: 500px;
    margin: 0.5rem auto 0;
    padding: 1.5rem;
    background-color: white;
    border-radius: 0.5rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
  
  .clientes-edit-input {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-size: 1rem;
    transition: border-color 0.2s, box-shadow 0.2s;
  }
  
  .clientes-edit-input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
  }
  
  .clientes-edit-textarea {
    min-height: 100px;
    resize: vertical;
  }
  
  .clientes-edit-buttons {
    display: flex;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }
  
  .clientes-edit-submit,
  .clientes-edit-cancel {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 0.375rem;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
    flex: 1;
  }
  
  .clientes-edit-submit {
    background-color: #16a34a;
    color: white;
  }
  
  .clientes-edit-submit:hover {
    background-color: #15803d;
  }
  
  .clientes-edit-submit:active {
    background-color: #166534;
  }
  
  .clientes-edit-cancel {
    background-color: #6b7280;
    color: white;
  }
  
  .clientes-edit-cancel:hover {
    background-color: #4b5563;
  }
  
  .clientes-edit-cancel:active {
    background-color: #374151;
  }
  </style>