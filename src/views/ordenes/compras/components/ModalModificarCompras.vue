<template>
  <div v-if="visible" class="modal-overlay">
    <div class="modal-container">
      <h2>Modificar Compra</h2>
      <form @submit.prevent="guardarCambios">
        <!-- Proveedor -->
        <div class="form-group">
          <label for="id_proveedor">Proveedor:</label>
          <input 
            v-model="compraParaEditar.id_proveedor" 
            type="number" 
            id="id_proveedor" 
            required
          />
        </div>
        
        <!-- Total -->
        <div class="form-group">
          <label for="total">Total:</label>
          <input 
            v-model="compraParaEditar.total" 
            type="number" 
            step="0.01" 
            id="total" 
            required
          />
        </div>
        
        <!-- Fecha -->
        <div class="form-group">
          <label for="fecha">Fecha:</label>
          <input 
            v-model="compraParaEditar.fecha" 
            type="datetime-local" 
            id="fecha" 
            required
          />
        </div>

        <!-- Producto (mostrar solo los productos actuales) -->
        <div class="form-group">
          <label for="producto">Producto:</label>
          <select v-model="compraParaEditar.productoSeleccionado" required>
            <option 
              v-for="producto in compraParaEditar.productos" 
              :key="producto.id_producto" 
              :value="producto.id_producto"
            >
              {{ producto.nombre }}
            </option>
          </select>
        </div>

        <div class="form-actions">
          <button type="button" @click="cerrarModal">Cancelar</button>
          <button type="submit">Guardar Cambios</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';
import { defineProps, defineEmits } from 'vue';

const props = defineProps({
  visible: Boolean,
  compraSeleccionada: Object
});

const emit = defineEmits(['cerrar', 'guardar']);

const compraParaEditar = ref({
  productos: [],
  productoSeleccionado: null
});

// Copiar la compra y establecer productoSeleccionado al primero
watch(() => props.compraSeleccionada, (nuevaCompra) => {
  if (nuevaCompra) {
    compraParaEditar.value = {
      ...nuevaCompra,
      productoSeleccionado: nuevaCompra.productos?.[0]?.id_producto || null
    };
    console.log("Compra cargada:", compraParaEditar.value);
  }
}, { immediate: true });

// Watch para depurar qué producto se está seleccionando
watch(() => compraParaEditar.value.productoSeleccionado, (nuevoId) => {
  console.log("Producto seleccionado ID:", nuevoId);
  const seleccionado = compraParaEditar.value.productos.find(p => p.id_producto === nuevoId);
  if (seleccionado) {
    console.log("Producto seleccionado:", seleccionado);
  } else {
    console.warn("Producto no encontrado con ID:", nuevoId);
  }
});

const guardarCambios = async () => {
  try {
    console.log("Datos a guardar:", compraParaEditar.value);
    await emit('guardar', compraParaEditar.value);
    cerrarModal();
  } catch (error) {
    console.error('Error guardando los cambios:', error);
  }
};

const cerrarModal = () => {
  emit('cerrar');
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
}

.modal-container {
  background: white;
  padding: 20px;
  border-radius: 8px;
  width: 400px;
}

h2 {
  text-align: center;
}

.form-group {
  margin-bottom: 15px;
}

label {
  display: block;
  font-size: 14px;
}

input, select {
  width: 100%;
  padding: 8px;
  margin-top: 5px;
  font-size: 14px;
}

.form-actions {
  display: flex;
  justify-content: space-between;
  margin-top: 20px;
}

button {
  padding: 10px 20px;
  border: none;
  cursor: pointer;
}

button[type="button"] {
  background-color: #ccc;
}

button[type="submit"] {
  background-color: #4CAF50;
  color: white;
}
</style>
