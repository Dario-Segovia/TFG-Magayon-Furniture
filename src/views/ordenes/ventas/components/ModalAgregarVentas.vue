<template>
  <div class="modal-overlay">
    <div class="modal-container">
      <div class="modal-header">
        <h2>Crear Venta</h2>
        <button @click="$emit('cerrar-modal')" class="close-btn">&times;</button>
      </div>

      <div class="modal-body">
        <div class="modal-form">
          <div class="form-group">
            <label>Cliente</label>
            <select v-model="venta.id_cliente" class="form-select">
              <option disabled value="">Seleccione un cliente</option>
              <option v-for="cliente in clientes" :key="cliente.id" :value="cliente.id">
                {{ cliente.nombre }}
              </option>
            </select>
          </div>

          <!-- Fila para agregar producto -->
          <div class="agregar-producto-row">
            <select v-model="productoSeleccionado" class="form-select">
              <option disabled value="">Producto</option>
              <option v-for="producto in inventario" :key="producto.id" :value="producto.id">
                {{ producto.nombre }} ({{ producto.categoria }})
              </option>
            </select>
            <input type="number" v-model.number="cantidad" min="1" class="form-input cantidad-input" placeholder="Cantidad" />
            <button @click="agregarProducto" class="btn btn-primary btn-agregar" :disabled="!productoSeleccionado || cantidad <= 0">
              <span>Agregar</span>
              <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-width="2" d="M12 5v14m7-7H5"/></svg>
            </button>
          </div>

          <!-- Tabla de productos agregados -->
          <div v-if="venta.detalles.length > 0" class="productos-table-container">
            <table class="productos-table">
              <thead>
                <tr>
                  <th>Producto</th>
                  <th>Cant.</th>
                  <th>Precio unit.</th>
                  <th>Subtotal</th>
                  <th></th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(detalle, index) in venta.detalles" :key="index" class="fade-in-row">
                  <td>{{ detalle.nombre }}</td>
                  <td>{{ detalle.cantidad }}</td>
                  <td>{{ detalle.precio_unitario }} €</td>
                  <td>{{ (detalle.cantidad * detalle.precio_unitario).toFixed(2) }} €</td>
                  <td>
                    <button @click="eliminarProducto(index)" class="btn btn-secondary btn-eliminar" title="Eliminar">
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-width="2" d="M6 18 18 6M6 6l12 12"/></svg>
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
            <div class="total-row">
              <span>Total:</span>
              <span class="total-valor">
                {{ totalVenta }} €
              </span>
            </div>
          </div>

          <div class="form-actions">
            <button 
              @click="guardarVenta" 
              class="btn btn-primary"
              :disabled="venta.detalles.length === 0"
            >
              Guardar Venta
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  clientes: Array,
  inventario: Array
});

const emit = defineEmits(['crear-venta']);

const productoSeleccionado = ref("");
const cantidad = ref(1);
const venta = ref({
  id_cliente: '',
  detalles: []
});

const agregarProducto = async () => {
  if (!productoSeleccionado.value || cantidad.value <= 0) return;
  const producto = await invoke('get_inventory_item', { id: productoSeleccionado.value });
  const indexExistente = venta.value.detalles.findIndex(
    d => d.id_producto === producto.id
  );
  if (indexExistente >= 0) {
    venta.value.detalles[indexExistente].cantidad += cantidad.value;
  } else {
    venta.value.detalles.push({
      id_producto: producto.id,
      cantidad: cantidad.value,
      precio_unitario: producto.precio_unitario,
      nombre: producto.nombre
    });
  }
  productoSeleccionado.value = "";
  cantidad.value = 1;
};

const eliminarProducto = (index) => {
  venta.value.detalles.splice(index, 1);
};

const guardarVenta = () => {
  const payload = {
    id_cliente: venta.value.id_cliente ? parseInt(venta.value.id_cliente) : null,
    detalles: venta.value.detalles.map(d => ({
      id_producto: d.id_producto,
      cantidad: d.cantidad,
      precio_unitario: d.precio_unitario
    }))
  };
  emit('crear-venta', payload);
  venta.value = { id_cliente: '', detalles: [] };
};

const totalVenta = computed(() =>
  venta.value.detalles.reduce((acc, d) => acc + d.cantidad * d.precio_unitario, 0).toFixed(2)
);
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

.agregar-producto-row {
  display: flex;
  gap: 12px;
  align-items: flex-end;
  margin-bottom: 12px;
}
.cantidad-input {
  width: 80px;
}
.btn-agregar {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 1rem;
  padding: 10px 18px;
}
.productos-table-container {
  margin-top: 18px;
}
.productos-table {
  width: 100%;
  border-collapse: collapse;
  margin-bottom: 8px;
  background: #f8f9fa;
  border-radius: 8px;
  overflow: hidden;
}
.productos-table th, .productos-table td {
  padding: 8px 10px;
  text-align: left;
}
.productos-table th {
  background: #e0e7ff;
  color: #405890;
  font-weight: 600;
}
.productos-table td {
  background: #fff;
}
.btn-eliminar {
  color: #f44336;
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  border-radius: 50%;
  transition: background 0.2s;
}
.btn-eliminar:hover {
  background: #ffeaea;
}
.total-row {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  gap: 12px;
  font-size: 1.1em;
  font-weight: 600;
  color: #405890;
  margin-top: 6px;
}
.total-valor {
  color: #4caf50;
  font-size: 1.2em;
}
.fade-in-row {
  animation: fadeIn 0.3s;
}
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-10px);}
  to { opacity: 1; transform: translateY(0);}
}
</style>