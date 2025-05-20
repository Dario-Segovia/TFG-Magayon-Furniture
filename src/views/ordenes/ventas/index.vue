<template>
  <button class="btn-back" @click="$router.back()">
  <i class="fas fa-arrow-left"></i> Atrás
</button>
  <div class="inventario-container">
    <!-- Encabezado -->
<div class="header">
  <div class="header-content">
    <div class="header-left">
      <h1>
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
          <polyline points="14 2 14 8 20 8"></polyline>
          <line x1="16" y1="13" x2="8" y2="13"></line>
          <line x1="16" y1="17" x2="8" y2="17"></line>
          <polyline points="10 9 9 9 8 9"></polyline>
        </svg>
        Gestión de Ventas
      </h1>
      <p class="subtitle">Administra y registra todas las transacciones comerciales</p>
    </div>
    <button 
      class="btn-primary nueva-venta-btn"
      @click="abrirModalAgregar"
    >
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <line x1="12" y1="5" x2="12" y2="19"></line>
        <line x1="5" y1="12" x2="19" y2="12"></line>
      </svg>
      Nueva Venta
    </button>
  </div>
</div>


    <!-- Filtros -->
    <div class="filtros-container">
      <div class="search-box">
        <input type="text" placeholder="Buscar ventas..." v-model="filtroTexto" />
        <i>
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="11" cy="11" r="8"></circle>
            <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
          </svg>
        </i>
      </div>
      <div class="filtros-avanzados">
        <button @click="toggleFiltrosAvanzados">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="4" y1="21" x2="4" y2="14"></line>
            <line x1="4" y1="10" x2="4" y2="3"></line>
            <line x1="12" y1="21" x2="12" y2="12"></line>
            <line x1="12" y1="8" x2="12" y2="3"></line>
            <line x1="20" y1="21" x2="20" y2="16"></line>
            <line x1="20" y1="12" x2="20" y2="3"></line>
            <line x1="1" y1="14" x2="7" y2="14"></line>
            <line x1="9" y1="8" x2="15" y2="8"></line>
            <line x1="17" y1="16" x2="23" y2="16"></line>
          </svg>
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
            <label>Cliente</label>
            <select class="filtro-select" v-model="filtroCliente">
              <option value="">Todos</option>
              <option v-for="cliente in clientes" :key="cliente.id" :value="cliente.id">
                {{ cliente.nombre }}
              </option>
            </select>
          </div>
        </div>
      </div>
    </div>

    <!-- Contenido principal -->
    <div class="inventario-grid">
      <!-- Modal para agregar/editar ventas -->
      <div v-if="!ventaEditando" class="fullscreen-modal" v-show="mostrarModalAgregar">
        <div class="modal-content">
          <ModalAgregarVentas 
            :clientes="clientes"
            :inventario="inventario"
            @crear-venta="crearVenta"
            @cerrar-modal="mostrarModalAgregar = false"
          />
        </div>
      </div>
      
      <div v-else class="fullscreen-modal" v-show="mostrarModalEditar">
        <div class="modal-content">
          <ModalModificarVentas 
            :clientes="clientes"
            :inventario="inventario"
            :venta-actual="ventaActual"
            @actualizar-venta="actualizarVenta"
            @cancelar-edicion="cancelarEdicion"
          />
        </div>
      </div>

      <!-- Listado de ventas -->
      <div class="mt-10">
        <h2 class="text-xl font-bold mb-2">Ventas Registradas</h2>
        <div class="cards-grid">
          <template v-if="ventasFiltradas.length > 0">
            <VentasCard 
              v-for="v in ventasFiltradas" 
              :key="v.id" 
              :venta="v"
              @editar-venta="editarVenta"
              @eliminar-venta="eliminarVenta"
            />
          </template>
          <div v-else class="no-results">
            No se encontraron ventas registradas
          </div>
        </div>
      </div>

      <!-- Modal de confirmación para eliminar -->
      <ModalEliminarVentas 
        v-if="ventaAEliminar"
        :venta-id="ventaAEliminar"
        @confirmar-eliminar="confirmarEliminarVenta"
        @cancelar-eliminar="ventaAEliminar = null"
      />
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import VentasCard from './components/VentasCard.vue';
import ModalAgregarVentas from './components/ModalAgregarVentas.vue';
import ModalModificarVentas from './components/ModalModificarVentas.vue';
import ModalEliminarVentas from './components/ModalEliminarVentas.vue';

const clientes = ref([]);
const inventario = ref([]);
const ventas = ref([]);
const ventaEditando = ref(null);
const ventaActual = ref(null);
const ventaAEliminar = ref(null);
const mostrarFiltros = ref(false);
const mostrarModalAgregar = ref(false);
const mostrarModalEditar = ref(false);

// Filtros
const filtroCliente = ref('');
const filtroFechaDesde = ref('');
const filtroFechaHasta = ref('');
const filtroTexto = ref('');

// Carga inicial de datos
onMounted(async () => {
  clientes.value = await invoke('obtener_clientes');
  inventario.value = await invoke('get_inventory');
  await cargarVentas();
});

const cargarVentas = async () => {
  ventas.value = await invoke('get_ventas');
};

// Computed para ventas filtradas
const ventasFiltradas = computed(() => {
  return ventas.value.filter(v => {
    // Filtro por cliente
    if (filtroCliente.value && String(v.id_cliente) !== String(filtroCliente.value)) return false;
    // Filtro por fecha desde
    if (filtroFechaDesde.value && new Date(v.fecha) < new Date(filtroFechaDesde.value)) return false;
    // Filtro por fecha hasta
    if (filtroFechaHasta.value && new Date(v.fecha) > new Date(filtroFechaHasta.value + 'T23:59:59')) return false;
    // Filtro por texto (cliente o producto)
    if (filtroTexto.value) {
      const texto = filtroTexto.value.toLowerCase();
      const cliente = (v.nombre_cliente || '').toLowerCase();
      const productos = (v.detalles || []).map(d => d.nombre_producto?.toLowerCase() || '').join(' ');
      if (!cliente.includes(texto) && !productos.includes(texto)) return false;
    }
    return true;
  });
});

const toggleFiltrosAvanzados = () => {
  mostrarFiltros.value = !mostrarFiltros.value;
};

const abrirModalAgregar = () => {
  mostrarModalAgregar.value = true;
};

// Métodos para manejar ventas
const crearVenta = async (ventaData) => {
  try {
    const id = await invoke('crear_venta', { data: ventaData });
    alert(`Venta creada con ID: ${id}`);
    mostrarModalAgregar.value = false;
    await cargarVentas();
  } catch (e) {
    alert('Error al crear la venta: ' + e);
  }
};

const editarVenta = (venta) => {
  ventaEditando.value = venta.id;
  ventaActual.value = {
    id_cliente: venta.id_cliente,
    detalles: venta.detalles.map(d => ({
      id_producto: d.id_producto,
      cantidad: d.cantidad,
      precio_unitario: d.precio_unitario,
      nombre: d.nombre_producto
    }))
  };
  mostrarModalEditar.value = true;
};

const actualizarVenta = async (ventaData) => {
  try {
    await invoke('update_venta', {
      params: {
        idVenta: ventaEditando.value,
        data: ventaData
      }
    });
    alert('Venta actualizada correctamente');
    mostrarModalEditar.value = false;
    ventaEditando.value = null;
    ventaActual.value = null;
    await cargarVentas();
  } catch (e) {
    alert('Error al actualizar la venta: ' + e);
  }
};

const eliminarVenta = (ventaId) => {
  ventaAEliminar.value = ventaId;
};

const confirmarEliminarVenta = async () => {
  try {
    await invoke('delete_venta', { idVenta: ventaAEliminar.value });
    alert('Venta eliminada correctamente');
    ventaAEliminar.value = null;
    await cargarVentas();
  } catch (e) {
    alert('Error al eliminar la venta: ' + e);
  }
};

const cancelarEdicion = () => {
  mostrarModalEditar.value = false;
  ventaEditando.value = null;
  ventaActual.value = null;
};
</script>

<style scoped>
/* Estilos base */
.inventario-container {
  padding: 20px;
  max-width: 1800px;
  margin: 0 auto;
  background: #f4f7f9;
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
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
  background: linear-gradient(135deg,#2b4583 ,  #d457c3);
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

.filtros-avanzados {
  position: relative;
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

.filtro-group {
  margin-bottom: 12px;
}

.filtro-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: 500;
  color: #333;
}

.filtro-group select,
.filtro-group input {
  width: 100%;
  padding: 8px;
  border-radius: 8px;
  border: 1px solid #ddd;
  font-size: 1rem;
  box-shadow: 0 2px 4px rgba(0,0,0,0.05);
  transition: border-color 0.3s, box-shadow 0.3s;
}

.filtro-group select:focus,
.filtro-group input:focus {
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
  margin-bottom: 20px;
}

.btn-primary:hover {
  background: linear-gradient(135deg,#c32caf, #405890);
  transform: translateY(-2px);
  box-shadow: 0 6px 10px rgba(0,0,0,0.15);
}





.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.nueva-venta-btn {
  margin-bottom: 0;
}

</style>