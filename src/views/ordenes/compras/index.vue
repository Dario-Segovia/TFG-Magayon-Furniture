<template>
  <div class="inventario-container fade-in">
    <div class="header">
      <div class="header-content">
        <h1>Compras</h1>
        <p class="subtitle">Gestión de todas las compras realizadas</p>
      </div>
      <button class="btn-primary" @click="showAgregarModal = true">
        Agregar Compra
      </button>
    </div>

    <!-- Filtros -->
    <div class="filtros-container">
      <div class="search-box">
        <input type="text" placeholder="Buscar compras..." v-model="filtroTexto" />
        <i>
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="11" cy="11" r="8"></circle>
            <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
          </svg>
        </i>
      </div>
      <div class="filtros-avanzados" style="position:relative;">
        <button @click="mostrarFiltros = !mostrarFiltros">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor"><circle cx="12" cy="12" r="10" stroke-width="2"/><line x1="1" y1="14" x2="7" y2="14" stroke-width="2"/><line x1="9" y1="8" x2="15" y2="8" stroke-width="2"/><line x1="17" y1="16" x2="23" y2="16" stroke-width="2"/></svg>
          Filtros
        </button>
        <div v-if="mostrarFiltros" class="filtros-content">
          <div class="filtro-group">
            <label>Fecha desde</label>
            <input type="date" class="filtro-input" v-model="filtroFechaDesde">
          </div>
          <div class="filtro-group">
            <label>Fecha hasta</label>
            <input type="date" class="filtro-input" v-model="filtroFechaHasta">
          </div>
          <div class="filtro-group">
            <label>Proveedor</label>
            <select class="filtro-select" v-model="filtroProveedor">
              <option value="">Todos</option>
              <option v-for="proveedor in proveedores" :key="proveedor.id" :value="proveedor.id">
                {{ proveedor.nombre }}
              </option>
            </select>
          </div>
        </div>
      </div>
    </div>

    <!-- Lista de compras -->
    <div class="inventario-grid">
      <div class="cards-grid">
        <template v-if="comprasFiltradas.length > 0">
          <ComprasCard
            v-for="compra in comprasFiltradas"
            :key="compra.id"
            :compra="compra"
            @editar="abrirEditarModal"
            @eliminar="abrirEliminarModal"
          />
        </template>
        <div v-else class="no-results">
          No se encontraron compras registradas
        </div>
      </div>
    </div>

    <!-- Modal para agregar compra -->
    <ModalAgregarCompras
      v-if="showAgregarModal"
      @close="cerrarAgregarModal"
      @refresh="listarCompras"
    />

    <!-- Modal para modificar compra -->
    <ModalModificarCompras
      :visible="showEditarModal"
      :compraSeleccionada="compraParaEditar"
      @cerrar="cerrarEditarModal"
      @guardar="handleCompraActualizada"
      @refresh="listarCompras"
    />

    <!-- Modal para eliminar compra -->
    <ModalEliminarCompras
      v-if="showEliminarModal"
      :compra="compraSeleccionada"
      @close="cerrarEliminarModal"
      @compra-eliminada="listarCompras"
    />
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import ComprasCard from './components/ComprasCard.vue';
import ModalAgregarCompras from './components/ModalAgregarCompras.vue';
import ModalModificarCompras from './components/ModalModificarCompras.vue';
import ModalEliminarCompras from './components/ModalEliminarCompras.vue';

const compras = ref([]);
const proveedores = ref([]);
const showAgregarModal = ref(false);
const compraParaEditar = ref(null);
const compraSeleccionada = ref(null);
const showEditarModal = ref(false);
const showEliminarModal = ref(false);

// Filtros
const filtroProveedor = ref('');
const filtroFechaDesde = ref('');
const filtroFechaHasta = ref('');
const filtroTexto = ref('');
const mostrarFiltros = ref(false);

function abrirEditarModal(compra) {
  compraParaEditar.value = { ...compra };
  showEditarModal.value = true;
}

async function handleCompraActualizada(compraActualizada) {
  const fechaObj = new Date(compraActualizada.fecha);
  const fechaFormateada = `${fechaObj.getFullYear()}-${(fechaObj.getMonth() + 1).toString().padStart(2, '0')}-${fechaObj.getDate().toString().padStart(2, '0')} ${fechaObj.getHours().toString().padStart(2, '0')}:${fechaObj.getMinutes().toString().padStart(2, '0')}:${fechaObj.getSeconds().toString().padStart(2, '0')}`;

  try {
    if (!compraActualizada.id) {
      throw new Error("La compra debe tener un ID para actualizarla");
    }
    await invoke('actualizar_compra', {
      id: compraActualizada.id,
      idProveedor: compraActualizada.id_proveedor,
      total: compraActualizada.total,
      fecha: fechaFormateada
    });
    const index = compras.value.findIndex(c => c.id === compraActualizada.id);
    if (index !== -1) {
      compras.value[index] = { ...compraActualizada };
    }
    cerrarEditarModal();
    listarCompras();
  } catch (error) {
    console.error('Error actualizando la compra:', error);
  }
}

async function listarCompras() {
  try {
    const lista = await invoke('listar_compras_con_proveedor');
    const proveedoresList = await invoke('listar_proveedores');
    proveedores.value = proveedoresList;
    for (const compra of lista) {
      const productos = await invoke('obtener_productos_compra', {
        idCompra: compra.id
      });
      compra.productos = productos;
    }
    compras.value = lista;
  } catch (error) {
    console.error('Error al listar las compras:', error);
  }
}

function abrirEliminarModal(compra) {
  compraSeleccionada.value = compra;
  showEliminarModal.value = true;
}
function cerrarAgregarModal() { showAgregarModal.value = false; }
function cerrarEditarModal() { showEditarModal.value = false; }
function cerrarEliminarModal() { showEliminarModal.value = false; }

onMounted(() => {
  listarCompras();
});

// Filtro computado
const comprasFiltradas = computed(() => {
  return compras.value.filter(c => {
    // Filtro por proveedor
    if (filtroProveedor.value && String(c.id_proveedor) !== String(filtroProveedor.value)) return false;
    // Filtro por fecha desde
    if (filtroFechaDesde.value && new Date(c.fecha) < new Date(filtroFechaDesde.value)) return false;
    // Filtro por fecha hasta
    if (filtroFechaHasta.value && new Date(c.fecha) > new Date(filtroFechaHasta.value + 'T23:59:59')) return false;
    // Filtro por texto (proveedor o producto)
    if (filtroTexto.value) {
      const texto = filtroTexto.value.toLowerCase();
      const proveedor = (c.nombre_proveedor || '').toLowerCase();
      const productos = (c.productos || []).map(p => p.nombre?.toLowerCase() || '').join(' ');
      if (!proveedor.includes(texto) && !productos.includes(texto)) return false;
    }
    return true;
  });
});
</script>

<style scoped>
/* Estilos base */
.inventario-container {
  padding: 20px;
  max-width: 1100px;
  margin: 0 auto;
  background: #f4f7f9;
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
}

/* Hacer que el modal ocupe toda la pantalla */
.fullscreen-modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5); /* Fondo semitransparente */
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 9999; /* Asegúrate de que esté por encima de otros elementos */
  animation: fadeIn 0.5s;
}

.fullscreen-modal .modal-content {
  width: 90%; /* Ajusta el ancho según lo desees */
  height: 90%; /* Ajusta la altura según lo desees */
  background: white;
  border-radius: 10px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  overflow-y: auto;
  padding: 20px;
}

/* Animación de desvanecimiento */
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

/* Estilos para el encabezado */
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 22px;
  padding: 22px;
  background: linear-gradient(135deg,#2b4583 ,  #d457c3); /* Cambiado aquí */
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
  background: linear-gradient(135deg, #405890 ,  #c32caf);
  color: white;
  box-shadow: 0 4px 6px rgba(0,0,0,0.10);
  transition: transform 0.2s, box-shadow 0.2s, background 0.3s;
}

.btn-primary:hover {
  background: linear-gradient(135deg,#c32caf, #405890);
  transform: translateY(-2px);
  box-shadow: 0 6px 10px rgba(0,0,0,0.15);
}
</style>
