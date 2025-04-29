<template>
    <div class="modal">
      <h2>Modificar Compra</h2>
      <form @submit.prevent="modificarCompra">
        <input 
          v-model="id_proveedor" 
          type="number" 
          placeholder="ID Proveedor" 
          required
          min="1"
        />
        <select v-model="estado" required>
          <option value="">Seleccione estado</option>
          <option value="pendiente">Pendiente</option>
          <option value="completado">Completado</option>
          <option value="cancelado">Cancelado</option>
        </select>
        <input 
          v-model="total" 
          type="number" 
          step="0.01" 
          placeholder="Total" 
          min="0"
          required
        />
  
        <div v-if="errorMessage" class="error-message">
          {{ errorMessage }}
        </div>
  
        <div class="buttons">
          <button type="submit">Guardar Cambios</button>
          <button type="button" @click="emit('close')">Cancelar</button>
        </div>
      </form>
    </div>
  </template>
  
  <script setup>
  import { ref, watch } from 'vue';
  import { invoke } from '@tauri-apps/api/core';
  
  const props = defineProps({
    compra: {
      type: Object,
      required: true
    }
  });
  
  const emit = defineEmits(['close', 'compra-actualizada']);
  
  const id_proveedor = ref(props.compra?.id_proveedor ?? 0);
  const estado = ref(props.compra?.estado ?? '');
  const total = ref(props.compra?.total ?? 0);
  const errorMessage = ref('');
  
  watch(() => props.compra, (nuevaCompra) => {
    if (nuevaCompra) {
      id_proveedor.value = nuevaCompra.id_proveedor ?? 0;
      estado.value = nuevaCompra.estado ?? '';
      total.value = nuevaCompra.total ?? 0;
    }
  });
  
  async function modificarCompra() {
    errorMessage.value = '';
  
    // Verifica si el tipo es "proveedor" y si el id_proveedor es correcto
    if (props.compra.tipo === 'proveedor' && !id_proveedor.value) {
      errorMessage.value = 'Debe ingresar un ID de proveedor válido para este tipo de orden';
      return;
    }
  
    if (!props.compra?.id) {
      errorMessage.value = 'No se encontró ID de la compra';
      return;
    }
  
    const idProveedorNumber = parseInt(id_proveedor.value, 10);
    const totalNumber = parseFloat(total.value);
  
    if (isNaN(idProveedorNumber) || idProveedorNumber <= 0) {
      errorMessage.value = 'Debe ingresar un ID de proveedor válido';
      return;
    }
  
    if (!estado.value) {
      errorMessage.value = 'Debe seleccionar un estado';
      return;
    }
  
    if (isNaN(totalNumber) || totalNumber < 0) {
      errorMessage.value = 'Debe ingresar un total válido';
      return;
    }
  
    try {
      // Imprime los datos antes de enviar la actualización para depurar
      console.log('Enviando datos para actualizar la compra:', {
        id: props.compra.id,
        id_proveedor: idProveedorNumber,
        estado: estado.value,
        total: totalNumber
      });
  
      await invoke('actualizar_compra', {
        id: props.compra.id,
        id_proveedor: idProveedorNumber,
        estado: estado.value,
        total: totalNumber
      });
  
      emit('compra-actualizada');
      emit('close');
    } catch (error) {
      console.error('Error updating purchase:', error);
      errorMessage.value = 'Error al actualizar la compra: ' + error.toString();
    }
  }
  console.log('Datos de la compra antes de la actualización:', {
  id: props.compra.id,
  tipo: props.compra.tipo,  // Verifica que tipo sea 'proveedor'
  id_proveedor: id_proveedor.value,
  estado: estado.value,
  total: total.value
});

  </script>
  