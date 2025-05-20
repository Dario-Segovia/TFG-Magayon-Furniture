<template>
  <div class="proveedor-card" :class="{ 'contrato-vigente': proveedor.contrato_vigente, 'is-open': isOpen }">
    <div class="card-header">
      <h3>{{ proveedor.nombre }}</h3>
      <span class="estado-badge" :class="proveedor.estado === 'activo' ? 'activo' : 'inactivo'">
        {{ proveedor.estado }}
      </span>
    </div>

    <div class="card-content">
      <div v-if="proveedor.contacto" class="info-row">
        <i class="fas fa-user"></i>
        <span>{{ proveedor.contacto }}</span>
      </div>
      <div v-if="proveedor.telefono" class="info-row">
        <i class="fas fa-phone"></i>
        <a :href="`tel:${proveedor.telefono}`">{{ proveedor.telefono }}</a>
      </div>
      <div v-if="proveedor.email" class="info-row">
        <i class="fas fa-envelope"></i>
        <a :href="`mailto:${proveedor.email}`">{{ proveedor.email }}</a>
      </div>
      <div v-if="proveedor.direccion" class="info-row">
        <i class="fas fa-map-marker-alt"></i>
        <span>{{ proveedor.direccion }}</span>
      </div>
      <div v-if="proveedor.pais" class="info-row">
        <i class="fas fa-flag"></i>
        <span>{{ proveedor.pais }}</span>
      </div>
      <div class="info-row">
        <i class="fas fa-file-contract"></i>
        <span>
          Contrato: 
          <span :class="proveedor.contrato_vigente ? 'contrato-vigente-text' : 'contrato-no-vigente-text'">
            {{ proveedor.contrato_vigente ? 'Vigente' : 'No vigente' }}
          </span>
        </span>
      </div>
    </div>

    <div class="card-actions">
      <button 
        @click="$emit('toggle', proveedor.id)" 
        class="btn-action view" 
        title="Expandir/cerrar" 
        aria-label="Expandir/cerrar"
      >
        <i class="fas" :class="isOpen ? 'fa-chevron-up' : 'fa-chevron-down'"></i>
      </button>
      <button 
        @click="editarProveedor" 
        class="btn-action edit" 
        title="Editar" 
        aria-label="Editar proveedor"
      >
        <i class="fas fa-edit"></i>
      </button>
      <button 
        @click="$emit('delete', proveedor)" 
        class="btn-action delete" 
        title="Eliminar" 
        aria-label="Eliminar proveedor"
      >
        <i class="fas fa-trash"></i>
      </button>
    </div>
    <div v-if="isOpen" class="productos-list">
      <div class="productos-header">
        <strong>Productos</strong>
        <div style="display: flex; gap: 8px;">
          <button class="btn-add-producto" @click="abrirModalProducto()">
            <i class="fas fa-plus"></i> Añadir producto
          </button>
          <button class="btn-add-producto" @click="abrirModalInventario()">
            <i class="fas fa-box"></i> Añadir desde inventario
          </button>
        </div>
      </div>
      <table class="productos-table" v-if="productos && productos.length">
        <thead>
          <tr>
            <th>Nombre</th>
            <th>Descripción</th>
            <th>Precio</th>
            <th>Categoría</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="prod in productos" :key="prod.id">
            <td>{{ prod.producto }}</td>
            <td>{{ prod.descripcion }}</td>
            <td>${{ prod.precio_unitario }}</td>
            <td>{{ prod.categoria }}</td>
            <td>
              <button class="btn-mini" @click="abrirModalProducto(prod)"><i class="fas fa-edit"></i></button>
              <button class="btn-mini btn-danger" @click="confirmarEliminarProducto(prod)"><i class="fas fa-trash"></i></button>
            </td>
          </tr>
        </tbody>
      </table>
      <div v-else class="no-productos">Sin productos registrados.</div>
    </div>
    <ProductoProveedorModal
      v-if="showProductoModal"
      :producto="productoEditando"
      @close="cerrarModalProducto"
      @save="guardarProducto"
    />
    <ConfirmacionModal
      v-if="showEliminarProducto"
      :message="`¿Eliminar el producto '${productoAEliminar?.producto}'?`"
      @confirm="eliminarProducto"
      @cancel="showEliminarProducto = false"
    />
    <InventarioSelectorModal
      v-if="showInventarioModal"
      @close="cerrarModalInventario"
      @select="agregarProductoDesdeInventario"
    />
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import ConfirmacionModal from './ConfirmacionModal.vue';
import ProductoProveedorModal from './ProductoProveedorModal.vue';
import InventarioSelectorModal from './InventarioSelectorModal.vue';

const props = defineProps({
  proveedor: { type: Object, required: true },
  isOpen: { type: Boolean, required: true },
});
const emit = defineEmits(['toggle', 'edit', 'delete']);

const productos = ref([]);
const showProductoModal = ref(false);
const productoEditando = ref({});
const showEliminarProducto = ref(false);
const productoAEliminar = ref(null);
const showInventarioModal = ref(false);

watch(() => props.isOpen, async (open) => {
  if (open) await cargarProductos();
});

async function cargarProductos() {
  productos.value = await invoke('obtener_productos_proveedor', { proveedorId: props.proveedor.id });
}

function abrirModalProducto(prod = {}) {
  productoEditando.value = { ...prod };
  showProductoModal.value = true;
}
function cerrarModalProducto() {
  showProductoModal.value = false;
  productoEditando.value = {};
}
async function guardarProducto(producto) {
  if (!producto.producto || !producto.precio_unitario) return;
  if (producto.id) {
    await invoke('eliminar_producto_proveedor', { id: producto.id });
  }
  await invoke('agregar_producto_proveedor', {
    producto: {
      ...producto,
      proveedorId: props.proveedor.id,
    }
  });
  await cargarProductos();
  cerrarModalProducto();
}
function confirmarEliminarProducto(prod) {
  productoAEliminar.value = prod;
  showEliminarProducto.value = true;
}
async function eliminarProducto() {
  await invoke('eliminar_producto_proveedor', { id: productoAEliminar.value.id });
  await cargarProductos();
  showEliminarProducto.value = false;
}
function editarProveedor() {
  emit('edit', props.proveedor);
}
function abrirModalInventario() {
  showInventarioModal.value = true;
}
function cerrarModalInventario() {
  showInventarioModal.value = false;
}
async function agregarProductoDesdeInventario(producto) {
  await invoke('agregar_producto_proveedor', {
    producto: {
      producto: producto.nombre,
      descripcion: producto.descripcion,
      precio_unitario: producto.precio_unitario,
      categoria: producto.categoria,
      proveedorId: props.proveedor.id,
    }
  });
  await cargarProductos();
  cerrarModalInventario();
}
</script>

<style scoped>
.proveedor-card {
  background: white;
  border-radius: 8px;
  padding: 15px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
  position: relative;
  overflow: hidden;
  transition: transform 0.2s, box-shadow 0.2s;
  border-left: 4px solid #4CAF50;
  display: flex;
  flex-direction: column;
  height: auto;
}

.proveedor-card.is-open {
  overflow: hidden;
  grid-row-end: span 2;
}

.proveedor-card.contrato-vigente {
  border-left-color: #2196F3;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  padding-bottom: 10px;
  border-bottom: 1px solid #eee;
}

.card-header h3 {
  margin: 0;
  font-size: 1.2em;
  color: #333;
}

.estado-badge {
  padding: 3px 8px;
  border-radius: 12px;
  font-size: 0.8em;
  font-weight: 500;
}

.estado-badge.activo {
  background-color: #e8f5e9;
  color: #2e7d32;
}

.estado-badge.inactivo {
  background-color: #ffebee;
  color: #c62828;
}

.card-content {
  margin-bottom: 15px;
}

.info-row {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 8px;
  font-size: 0.95em;
}

.info-row i {
  color: #666;
  width: 16px;
  text-align: center;
}

.info-row a {
  color: #2196F3;
  text-decoration: none;
}

.info-row a:hover {
  text-decoration: underline;
}

.card-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.btn-action {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-action.view {
  background: #e3f2fd;
  color: #2196F3;
}

.btn-action.edit {
  background: #e8f5e9;
  color: #4CAF50;
}

.btn-action.delete {
  background: #ffebee;
  color: #f44336;
}

.btn-action:hover {
  opacity: 0.8;
}

.contrato-tag {
  position: absolute;
  top: 10px;
  right: 10px;
  background: #e3f2fd;
  color: #2196F3;
  padding: 3px 8px;
  border-radius: 12px;
  font-size: 0.75em;
  display: flex;
  align-items: center;
  gap: 5px;
}

.productos-container {
  margin-top: 10px;
  padding: 10px;
  background: #f9f9f9;
  border-radius: 6px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
  transition: max-height 0.3s ease, opacity 0.3s ease;
  overflow: hidden;
  max-height: 0;
  opacity: 0;
  position: relative;
  z-index: 1;
}

.productos-container.open {
  max-height: 500px;
  opacity: 1;
}

.contrato-vigente-text {
  color: #2196F3;
  font-weight: bold;
}

.contrato-no-vigente-text {
  color: #c62828;
  font-weight: bold;
}

.productos-list {
  margin-top: 15px;
  background: #f9f9f9;
  border-radius: 6px;
  padding: 10px;
}

.productos-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.productos-table {
  width: 100%;
  border-collapse: collapse;
  margin-bottom: 10px;
}

.productos-table th,
.productos-table td {
  border: 1px solid #eee;
  padding: 6px 10px;
  font-size: 0.90em;
}

.btn-add-producto {
  background: #4caf50;
  color: white;
  border: none;
  border-radius: 6px;
  padding: 5px 12px;
  cursor: pointer;
  font-size: 0.73em;
  display: flex;
  align-items: center;
  gap: 5px;
}

.btn-add-producto:hover {
  background: #388e3c;
}

.btn-mini {
  background: #e0e0e0;
  border: none;
  border-radius: 4px;
  padding: 3px 8px;
  margin-left: 2px;
  cursor: pointer;
  font-size: 0.95em;
}

.btn-mini.btn-danger {
  background: #ffebee;
  color: #c62828;
}

.btn-mini.btn-success {
  background: #e8f5e9;
  color: #2e7d32;
}

.producto-form {
  display: flex;
  gap: 8px;
  margin-bottom: 10px;
}

.producto-form input {
  padding: 4px 8px;
  border-radius: 4px;
  border: 1px solid #ccc;
  min-width: 90px;
}

.no-productos {
  color: #888;
  font-size: 0.95em;
  padding: 8px 0;
}
</style>
