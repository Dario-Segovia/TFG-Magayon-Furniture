<template>
  <div v-if="visible" class="modal-overlay">
    <div class="modal-container">
      <div class="modal-header">
        <h2>Modificar Compra</h2>
        <button class="close-btn" @click="cerrarModal">×</button>
      </div>

      <div class="modal-body">
        <form @submit.prevent="guardarCambios" class="modal-form">

          <div class="form-grid">
            <!-- Proveedor (nombre) -->
            <div class="form-group">
              <label for="id_proveedor">Proveedor:</label>
              <select 
                v-model="compraParaEditar.id_proveedor" 
                id="id_proveedor" 
                class="form-select" 
                required
              >
                <option 
                  v-for="proveedor in compraParaEditar.proveedores" 
                  :key="proveedor.id" 
                  :value="proveedor.id"
                >
                  {{ proveedor.nombre }}
                </option>
              </select>
            </div>

            <!-- Producto -->
            <div class="form-group">
              <label for="producto">Producto:</label>
              <select 
                v-model="compraParaEditar.productoSeleccionado" 
                id="producto"
                class="form-select"
                required
              >
                <option 
                  v-for="producto in compraParaEditar.productos" 
                  :key="producto.nombre" 
                  :value="producto.nombre" 
                >
                  {{ producto.nombre }}
                </option>
              </select>
            </div>

            <!-- Cantidad editable -->
            <div class="form-group">
              <label for="cantidad">Cantidad:</label>
              <input 
                v-model.number="compraParaEditar.cantidad" 
                type="number" 
                id="cantidad" 
                class="form-input"
                min="1"
                required
              />
            </div>

            <!-- Total (solo lectura) -->
            <div class="form-group">
              <label for="total">Total:</label>
              <input 
                :value="calcularTotal()" 
                type="number" 
                id="total" 
                class="form-input"
                readonly
              />
            </div>

            <!-- Fecha -->
            <div class="form-group">
              <label for="fecha">Fecha:</label>
              <input 
                v-model="compraParaEditar.fecha" 
                type="datetime-local" 
                id="fecha" 
                class="form-input"
                required
              />
            </div>
          </div>

          <div class="form-actions">
            <button type="button" class="btn btn-secondary" @click="cerrarModal">Cancelar</button>
            <button type="submit" class="btn btn-primary">Guardar Cambios</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>



<script setup>
import { ref, watch } from 'vue';
import { defineProps, defineEmits } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  visible: Boolean,
  compraSeleccionada: Object
});

const emit = defineEmits(['cerrar', 'guardar']);

const compraParaEditar = ref({
  productos: [],
  productoSeleccionado: null
});

const calcularTotal = () => {
  const producto = compraParaEditar.value.productos.find(
    p => p.nombre === compraParaEditar.value.productoSeleccionado
  );
  if (!producto) return 0;

  const cantidad = compraParaEditar.value.cantidad || 0;
  return (producto.precio_unitario * cantidad).toFixed(2);
};


watch(() => props.compraSeleccionada, (nuevaCompra) => {
  if (nuevaCompra) {
    console.log("Cargando compra seleccionada:", nuevaCompra);

    // Convierte '2025-05-13 11:07:12.919417' a '2025-05-13T11:07'
    let fechaFormateada = '';
    if (typeof nuevaCompra.fecha === 'string') {
      const [fecha, hora] = nuevaCompra.fecha.split(' ');
      if (fecha && hora) {
        const horaLimpiada = hora.slice(0, 5); // HH:MM
        fechaFormateada = `${fecha}T${horaLimpiada}`;
      }
    } else if (nuevaCompra.fecha instanceof Date) {
      fechaFormateada = nuevaCompra.fecha.toISOString().slice(0, 16);
    }

    compraParaEditar.value = {
      ...nuevaCompra,
      proveedores: nuevaCompra.proveedores || [
        { id: nuevaCompra.id_proveedor, nombre: nuevaCompra.nombre_proveedor }
      ],
      productoSeleccionado: nuevaCompra.productos?.[0]?.nombre || null,
      cantidad: nuevaCompra.productos?.[0]?.cantidad || 1,
      productos: nuevaCompra.productos?.map(producto => ({
        ...producto,
        id_producto: producto.id_producto || producto.id || null
      })) || [],
      fecha: fechaFormateada
    };

    console.log("Productos cargados en la compra:", compraParaEditar.value.productos);
  }
}, { immediate: true });




// Watch para depurar qué producto se está seleccionando
watch(() => compraParaEditar.value.productoSeleccionado, (nuevoNombre) => {
  console.log("Producto seleccionado nombre:", nuevoNombre);

  const seleccionado = compraParaEditar.value.productos.find(
    p => p.nombre === nuevoNombre
  );

  if (seleccionado) {
    console.log("Producto seleccionado:", seleccionado);  // Log de producto seleccionado
  } else {
    console.warn("Producto no encontrado con nombre:", nuevoNombre);
  }
});

const guardarCambios = async () => {
  // Actualiza la cantidad en el producto seleccionado
  const producto = compraParaEditar.value.productos.find(
    p => p.nombre === compraParaEditar.value.productoSeleccionado
  );
  if (producto) {
    producto.cantidad = compraParaEditar.value.cantidad;
  }

  const detalles = compraParaEditar.value.productos.map(p => {
    console.log("Producto antes de procesar:", p);

    const idProducto = p.id_producto || p.id;
    console.log("id_producto para el producto:", idProducto);

    if (!idProducto) {
      console.error("Falta id_producto para el producto:", p);
      return null;
    }

    const cantidad = Number(p.cantidad);
    const precioUnitario = Number(p.precio_unitario);
    if (isNaN(cantidad) || isNaN(precioUnitario)) {
      console.error("Cantidad o precio_unitario no válidos para el producto:", p);
      return null;
    }

    // Eliminado el formateo de la fecha - se usa el valor directamente
   return {
  id_compra: compraParaEditar.value.id,
  fecha: compraParaEditar.value.fecha.replace('T', ' ') + ':00',
  id_proveedor: compraParaEditar.value.id_proveedor,
  total: Number(compraParaEditar.value.total),
  id_producto: idProducto,
  nombre_producto: p.nombre,
  cantidad: cantidad,
  precio_unitario: precioUnitario
};

  }).filter(d => d !== null);

  if (detalles.length === 0) {
    console.error("No hay detalles válidos para actualizar.");
    return;
  }

  console.log("Enviando detalles actualizados:", detalles);

  try {
    await invoke('actualizar_compra', { detalles });
    cerrarModal();
    emit('refresh');
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
  z-index: 1000;
  backdrop-filter: blur(3px);
}

.modal-container {
  background-color: white;
  border-radius: 12px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
  width: 90%;
  max-width: 700px;
  max-height: 90vh;
  overflow-y: auto;
  animation: modalFadeIn 0.3s ease-out;
}

@keyframes modalFadeIn {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 25px;
  border-bottom: 1px solid #f0f0f0;
}

.modal-header h2 {
  margin: 0;
  font-size: 1.5rem;
  color: #2c3e50;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.2rem;
  color: #7f8c8d;
  cursor: pointer;
  transition: color 0.2s;
  padding: 5px;
}

.close-btn:hover {
  color: #e74c3c;
}

.modal-body {
  padding: 25px;
}

.modal-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 20px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-group.full-width {
  grid-column: 1 / -1;
}

.form-group label {
  font-size: 0.9rem;
  color: #34495e;
  font-weight: 500;
}

.form-input, .form-select {
  padding: 12px 15px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 0.95rem;
  transition: border-color 0.2s, box-shadow 0.2s;
}

.form-input:focus, .form-select:focus {
  outline: none;
  border-color: #3498db;
  box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1);
}

.checkbox-group {
  flex-direction: row;
  align-items: center;
  margin-top: 10px;
}

.checkbox-container {
  display: flex;
  align-items: center;
  cursor: pointer;
  position: relative;
  user-select: none;
}

.checkbox-container input {
  position: absolute;
  opacity: 0;
  cursor: pointer;
  height: 0;
  width: 0;
}

.checkmark {
  height: 20px;
  width: 20px;
  background-color: white;
  border: 1px solid #ddd;
  border-radius: 4px;
  margin-right: 10px;
  transition: background-color 0.2s;
}

.checkbox-container:hover input ~ .checkmark {
  background-color: #f8f9fa;
}

.checkbox-container input:checked ~ .checkmark {
  background-color: #3498db;
  border-color: #3498db;
}

.checkmark:after {
  content: "";
  position: absolute;
  display: none;
}

.checkbox-container input:checked ~ .checkmark:after {
  display: block;
}

.checkbox-container .checkmark:after {
  left: 7px;
  top: 3px;
  width: 5px;
  height: 10px;
  border: solid white;
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 15px;
  padding-top: 20px;
  border-top: 1px solid #f0f0f0;
  margin-top: 10px;
}

.btn {
  padding: 12px 20px;
  border-radius: 8px;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.2s;
  border: none;
}

.btn-primary {
  background-color: #3498db;
  color: white;
}

.btn-primary:hover {
  background-color: #2980b9;
  transform: translateY(-1px);
}

.btn-secondary {
  background-color: #f8f9fa;
  color: #34495e;
}

.btn-secondary:hover {
  background-color: #e9ecef;
}

@media (max-width: 600px) {
  .form-grid {
    grid-template-columns: 1fr;
  }
  
  .form-actions {
    flex-direction: column-reverse;
  }
  
  .btn {
    width: 100%;
    justify-content: center;
  }
}
</style>