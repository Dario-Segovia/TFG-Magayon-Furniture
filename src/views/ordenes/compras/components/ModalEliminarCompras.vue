<template>
    <div class="modal">
      <h2>Eliminar Compra</h2>
      <p>¿Estás seguro de que quieres eliminar esta compra?</p>
      <p><strong>ID:</strong> {{ compra.id }}</p>
      <p><strong>Proveedor:</strong> {{ compra.id_proveedor }}</p>
  
      <button @click="eliminarCompra">Eliminar</button>
      <button @click="$emit('close')">Cancelar</button>
    </div>
  </template>
  
  <script setup>
  import { invoke } from '@tauri-apps/api/core';
  
  const props = defineProps({
    compra: Object
  });
  
  async function eliminarCompra() {
    try {
      await invoke('eliminar_compra', {
        id: props.compra.id
      });
      $emit('compra-eliminada');
      $emit('close');
    } catch (error) {
      console.error(error);
    }
  }
  </script>
  