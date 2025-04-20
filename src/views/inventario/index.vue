<template>
  <div class="inventario-container">
    <div class="header fade-in">
      <div class="header-content">
        <h1>
          <i class="fas fa-boxes"></i> Inventario
        </h1>
        <p class="subtitle">Gestiona los productos y existencias de tu inventario</p>
      </div>
      <button @click="showAgregar = true" class="btn-primary">
        <i class="fas fa-plus"></i> Nuevo Ítem
      </button>
    </div>

    <div class="filtros-container fade-in">
      <div class="search-box">
        <input v-model="searchTerm" placeholder="Buscar en inventario..." />
        <i class="fas fa-search"></i>
      </div>
    </div>

    <div v-if="loading" class="loading-container">
      <i class="fas fa-spinner fa-spin"></i> Cargando inventario...
    </div>
    <div v-if="error" class="error-container">
      <i class="fas fa-exclamation-triangle"></i> {{ error }}
    </div>

    <div class="inventario-grid">
      <div v-if="itemsFiltrados.length === 0" class="no-results">
        No se encontraron ítems en el inventario.
      </div>
      <div v-else class="cards-grid">
        <InventarioCard
          v-for="item in itemsFiltrados"
          :key="item.id"
          :item="item"
          @edit="abrirEditar"
          @delete="abrirEliminar"
        />
      </div>
    </div>

    <!-- Modales -->
    <ModalAgregarInventario
      v-if="showAgregar"
      @close="showAgregar = false"
      @save="agregarItem"
    />
    <ModalEditarInventario
      v-if="itemEditar"
      :item="itemEditar"
      @close="itemEditar = null"
      @save="editarItem"
    />
    <ModalEliminarInventario
      v-if="itemEliminar"
      :item="itemEliminar"
      @close="itemEliminar = null"
      @confirm="eliminarItem"
    />
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import InventarioCard from './components/InventarioCard.vue';
import ModalAgregarInventario from './components/ModalAgregarInventario.vue';
import ModalEditarInventario from './components/ModalEditarInventario.vue';
import ModalEliminarInventario from './components/ModalEliminarInventario.vue';

const items = ref([]);
const loading = ref(false);
const error = ref(null);

const showAgregar = ref(false);
const itemEditar = ref(null);
const itemEliminar = ref(null);

const searchTerm = ref('');

const cargarInventario = async () => {
  loading.value = true;
  error.value = null;
  console.log('Cargando inventario...');
  try {
    const resItems = await invoke('get_inventory');
    console.log('Respuesta get_inventory:', resItems);
    items.value = resItems;
  } catch (e) {
    console.error('Error al cargar inventario:', e);
    error.value = 'Error al cargar inventario';
  } finally {
    loading.value = false;
    console.log('Carga de inventario finalizada');
  }
};

onMounted(cargarInventario);

const itemsFiltrados = computed(() => {
  return items.value.filter(item =>
    item.nombre.toLowerCase().includes(searchTerm.value.toLowerCase()) ||
    (item.descripcion && item.descripcion.toLowerCase().includes(searchTerm.value.toLowerCase()))
  );
});

const agregarItem = async (nuevo) => {
  console.log('Intentando agregar ítem:', nuevo);
  try {
    await invoke('add_inventory_item', {
      item: {  // Envía los parámetros dentro de un objeto "item"
        nombre: nuevo.nombre,
        descripcion: nuevo.descripcion,
        cantidad: nuevo.cantidad,
        precio_unitario: nuevo.precio_unitario,
        categoria: nuevo.categoria,
      }
    });
    console.log('Ítem agregado correctamente');
    showAgregar.value = false;
    await cargarInventario();
  } catch (e) {
    console.error('Error al agregar ítem:', e);
    error.value = 'Error al agregar ítem';
  }
};

const abrirEditar = (item) => {
  console.log('Abriendo modal de edición para:', item);
  itemEditar.value = { ...item };
};
const editarItem = async (editado) => {
  console.log('Intentando editar ítem:', editado);
  try {
    await invoke('update_inventory_item', {
      id: editado.id,
      item: {  // Envía los demás campos dentro de un objeto 'item'
        nombre: editado.nombre,
        descripcion: editado.descripcion,
        cantidad: editado.cantidad,
        precio_unitario: editado.precio_unitario,
        categoria: editado.categoria
      }
    });
    console.log('Ítem editado correctamente');
    itemEditar.value = null;
    await cargarInventario();
  } catch (e) {
    console.error('Error al editar ítem:', e);
    error.value = 'Error al editar ítem';
  }
};

const abrirEliminar = (item) => {
  console.log('Abriendo modal de eliminación para:', item);
  itemEliminar.value = item;
};
const eliminarItem = async () => {
  console.log('Intentando eliminar ítem:', itemEliminar.value);
  try {
    await invoke('delete_inventory_item', { id: itemEliminar.value.id });
    console.log('Ítem eliminado correctamente');
    itemEliminar.value = null;
    await cargarInventario();
  } catch (e) {
    console.error('Error al eliminar ítem:', e);
    error.value = 'Error al eliminar ítem';
  }
};
</script>

<style scoped>
.inventario-container {
  padding: 20px;
  max-width: 1100px;
  margin: 0 auto;
  background: #f4f7f9;
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
}
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 22px;
  padding: 22px;
  background: linear-gradient(135deg, #DB5375, #B3FFB3); /* Cambiado aquí */
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
  color: #e0f7e9;
}
.fade-in {
  animation: fadeIn 0.5s;
}
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}
.filtros-container {
  display: flex;
  gap: 18px;
  margin-bottom: 20px;
  align-items: center;
  background: #fff;
  padding: 14px 18px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.07);
  animation: fadeIn 0.5s;
}
.search-box {
  position: relative;
  flex-grow: 1;
  min-width: 240px;
}
.search-box input {
  width: 96%;
  padding: 10px 15px 10px 35px;
  border-radius: 8px;
  border: 1px solid #ddd;
  font-size: 1rem;
  box-shadow: 0 2px 4px rgba(0,0,0,0.05);
  transition: border-color 0.3s, box-shadow 0.3s;
}
.search-box input:focus {
  border-color: #4caf50;
  box-shadow: 0 4px 8px rgba(0,0,0,0.10);
  outline: none;
}
.search-box i {
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%);
  color: #777;
  font-size: 1.2rem;
}
.filtros-avanzados button {
  padding: 8px 15px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 500;
  font-size: 0.95rem;
  background: linear-gradient(135deg, #be3fa3, #1ed06e);
  color: white;
  box-shadow: 0 2px 4px rgba(0,0,0,0.10);
  transition: transform 0.2s, box-shadow 0.2s, background 0.3s;
}
.filtros-avanzados button:hover {
  background: linear-gradient(135deg, #be3fa3, #1ed06e);
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0,0,0,0.15);
}
.filtros-content {
  position: absolute;
  right: 0;
  top: 100%;
  background: white;
  padding: 15px;
  border-radius: 8px;
  box-shadow: 0 4px 8px rgba(0,0,0,0.10);
  z-index: 10;
  width: 220px;
  margin-top: 5px;
}
.filtro-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: 500;
  color: #333;
}
.filtro-group select {
  width: 100%;
  padding: 8px;
  border-radius: 8px;
  border: 1px solid #ddd;
  font-size: 1rem;
  box-shadow: 0 2px 4px rgba(0,0,0,0.05);
  transition: border-color 0.3s, box-shadow 0.3s;
}
.filtro-group select:focus {
  border-color: #4caf50;
  box-shadow: 0 4px 8px rgba(0,0,0,0.10);
  outline: none;
}
.inventario-grid {
  margin-top: 10px;
}
.cards-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
  gap: 18px;
  align-items: start;
}
.loading-container,
.error-container,
.no-results {
  grid-column: 1 / -1;
  padding: 20px;
  text-align: center;
  background: #f8f9fa;
  border-radius: 5px;
  margin: 20px 0;
}
.error-container {
  background: #ffe6e6;
  color: #d32f2f;
}
.no-results {
  color: #6c757d;
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
  background: linear-gradient(135deg, #EA6E6E ,  #9375FE);
  color: white;
  box-shadow: 0 4px 6px rgba(0,0,0,0.10);
  transition: transform 0.2s, box-shadow 0.2s, background 0.3s;
}
.btn-primary:hover {
  background: linear-gradient(135deg, #9375FE, #EA6E6E);
  transform: translateY(-2px);
  box-shadow: 0 6px 10px rgba(0,0,0,0.15);
}
</style>