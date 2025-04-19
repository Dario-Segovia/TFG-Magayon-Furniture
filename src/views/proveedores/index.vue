<template>
    <div class="proveedores-container">
      <div class="header">
        <div class="header-content">
          <h1>
            <i class="fas fa-users-cog"></i> Administración de Proveedores
          </h1>
          <p class="subtitle">Gestiona tus proveedores y sus productos de manera eficiente</p>
        </div>
        <button @click="showNuevoProveedorModal = true" class="btn-primary">
          <i class="fas fa-plus"></i> Nuevo Proveedor
        </button>
      </div>
  
      <div class="filtros-container">
        <div class="search-box">
          <input 
            v-model="searchTerm" 
            placeholder="Buscar proveedores o productos..." 
           
          >
          <i class="fas fa-search"></i>
        </div>
        
        <div class="filtros-avanzados">
          <button @click="toggleFiltrosAvanzados" class="btn-secondary">
            <i class="fas fa-filter"></i> Filtros
          </button>
          
          <div v-if="mostrarFiltrosAvanzados" class="filtros-content">
            <div class="filtro-group">
              <label>Estado:</label>
              <select v-model="filtroEstado">
                <option value="todos">Todos</option>
                <option value="activo">Activos</option>
                <option value="inactivo">Inactivos</option>
              </select>
            </div>
            
            <div class="filtro-group">
              <label>Contrato vigente:</label>
              <select v-model="filtroContrato">
                <option value="todos">Todos</option>
                <option value="si">Con contrato</option>
                <option value="no">Sin contrato</option>
              </select>
            </div>

            
          </div>
        </div>
      </div>
  
      <!-- Loading state -->
      <div v-if="loading" class="loading-container">
        <i class="fas fa-spinner fa-spin"></i> Cargando proveedores...
      </div>
  
      <!-- Error state -->
      <div v-if="error" class="error-container">
        <i class="fas fa-exclamation-triangle"></i> {{ error }}
        <button @click="cargarProveedores" class="btn-primary small">
          Reintentar
        </button>
      </div>
  
      <div class="proveedores-grid">
        <div v-if="proveedoresFiltrados.length === 0" class="no-results">
          No se encontraron proveedores que coincidan con los criterios de búsqueda
        </div>
        <div v-else class="cards-grid">
          <ProveedorCard
            v-for="proveedor in proveedoresFiltrados"
            :key="proveedor.id"
            :proveedor="proveedor"
            :isOpen="proveedor.id === proveedorAbierto"
            @toggle="toggleAcordeon"
            @edit="abrirEditarProveedor"
            @delete="confirmarEliminarProveedor"
            @view="verDetalleProveedor"
           
          />
        </div>
     
      </div>
  
      <!-- Modales -->
      <NuevoProveedorModal 
        v-if="showNuevoProveedorModal"
        @close="showNuevoProveedorModal = false"
        @save="guardarNuevoProveedor"
      />
      
      <EditarProveedorModal 
        v-if="showEditarProveedorModal"
        :proveedor="proveedorSeleccionado"
        @close="showEditarProveedorModal = false"
        @save="actualizarProveedor"
      />
      
      <DetalleProveedorModal 
        v-if="showDetalleProveedorModal"
        :proveedor="proveedorSeleccionado"
        @close="showDetalleProveedorModal = false"
      />
      
      <ConfirmacionModal 
        v-if="showConfirmacionModal"
        :message="`¿Estás seguro de eliminar a ${proveedorSeleccionado?.nombre}?`"
        @confirm="eliminarProveedor"
        @cancel="showConfirmacionModal = false"
      />
  
      <!-- Modal para productos del proveedor -->
      <ProductosProveedorModal
        v-if="showProductosProveedorModal"
        :proveedor-id="proveedorSeleccionado?.id"
        @close="showProductosProveedorModal = false"
      />
      
      
    </div>
  </template>
  
  <script>
  import { invoke } from '@tauri-apps/api/core';
  import { ref, onMounted, computed } from 'vue';
  import ProveedorCard from './components/ProveedorCard.vue';
  import NuevoProveedorModal from './components/NuevoProveedorModal.vue';
  import EditarProveedorModal from './components/EditarProveedorModal.vue';
  import DetalleProveedorModal from './components/DetalleProveedorModal.vue';
  import ConfirmacionModal from './components/ConfirmacionModal.vue';
  import ProductosProveedorModal from './components/ProductosProveedor.vue';
  
  
  export default {
    components: {
      ProveedorCard,
      NuevoProveedorModal,
      EditarProveedorModal,
      DetalleProveedorModal,
      ConfirmacionModal,
      ProductosProveedorModal,
      
    },
  
    setup() {
      const proveedores = ref([]);
      const proveedorSeleccionado = ref(null);
      const searchTerm = ref('');
      const filtroEstado = ref('todos');
      const filtroContrato = ref('todos');
      const mostrarFiltrosAvanzados = ref(false);
      const loading = ref(false);
      const error = ref(null);
      const proveedorAbierto = ref(null);
  
      // Estados de modales
      const showNuevoProveedorModal = ref(false);
      const showEditarProveedorModal = ref(false);
      const showDetalleProveedorModal = ref(false);
      const showConfirmacionModal = ref(false);
      const showProductosProveedorModal = ref(false);
      
  
      // Obtener proveedores al cargar
      onMounted(async () => {
        await cargarProveedores();
      });
  
      const cargarProveedores = async () => {
  loading.value = true;
  error.value = null;
  try {
    if (searchTerm.value.trim() === '') {
      // Si no hay término de búsqueda, obtener todos los proveedores
      proveedores.value = await invoke('obtener_proveedores');
    } else {
      // Si hay término de búsqueda, buscar tanto proveedores como productos
      const [proveedoresDirectos, proveedoresPorProducto] = await Promise.all([
        invoke('buscar_proveedores', { termino: searchTerm.value.trim() }),
        invoke('buscar_proveedores_por_producto', { 
          nombre_producto: searchTerm.value.trim() 
        })
      ]);
      
      // Combinar y eliminar duplicados
      const todosProveedores = [...proveedoresDirectos, ...proveedoresPorProducto];
      const idsUnicos = [...new Set(todosProveedores.map(p => p.id))];
      proveedores.value = idsUnicos.map(id => 
        todosProveedores.find(p => p.id === id)
      );
    }
  } catch (err) {
    console.error('Error al cargar proveedores:', err);
    error.value = 'Error al cargar los proveedores. Por favor, intente nuevamente.';
  } finally {
    loading.value = false;
  }
};
  
      // Filtros computados
      const proveedoresFiltrados = computed(() => {
        return proveedores.value.filter((proveedor) => {
          // Filtro por búsqueda (incluye productos)
          const matchSearch =
            proveedor.nombre.toLowerCase().includes(searchTerm.value.toLowerCase()) ||
            (proveedor.contacto && proveedor.contacto.toLowerCase().includes(searchTerm.value.toLowerCase())) ||
            (proveedor.email && proveedor.email.toLowerCase().includes(searchTerm.value.toLowerCase())) ||
            (proveedor.productos && proveedor.productos.some(producto =>
              producto.producto.toLowerCase().includes(searchTerm.value.toLowerCase())
            ));
  
          // Filtro por estado
          const matchEstado =
            filtroEstado.value === 'todos' ||
            (filtroEstado.value === 'activo' && proveedor.estado === 'activo') ||
            (filtroEstado.value === 'inactivo' && proveedor.estado !== 'activo');
  
          // Filtro por contrato
          const matchContrato =
            filtroContrato.value === 'todos' ||
            (filtroContrato.value === 'si' && proveedor.contrato_vigente) ||
            (filtroContrato.value === 'no' && !proveedor.contrato_vigente);
  
          return matchSearch && matchEstado && matchContrato;
        });
      });
  
      const aplicarFiltros = () => {
        // Este método se asegura de que los filtros se apliquen correctamente
      };

      const filtrarProveedores = () => {
       

      };
  
      const toggleFiltrosAvanzados = () => {
        mostrarFiltrosAvanzados.value = !mostrarFiltrosAvanzados.value;
      };
  
      // Métodos para acciones
      const abrirEditarProveedor = (proveedor) => {
        console.log('Proveedor seleccionado para editar. ID:', proveedor.id); // Log del ID del proveedor
        proveedorSeleccionado.value = { ...proveedor };
        showEditarProveedorModal.value = true;
      };
  
      const actualizarProveedor = async (proveedorActualizado) => {
        try {
          if (!proveedorActualizado.id) {
            throw new Error('El proveedor no tiene un ID válido.');
          }
  
          console.log('Datos enviados al backend:', {
            id: proveedorActualizado.id,
            proveedor: proveedorActualizado,
          });
  
          await invoke('actualizar_proveedor', {
            id: proveedorActualizado.id,
            proveedor: {
              nombre: proveedorActualizado.nombre,
              contacto: proveedorActualizado.contacto,
              telefono: proveedorActualizado.telefono,
              email: proveedorActualizado.email,
              direccion: proveedorActualizado.direccion,
              pais: proveedorActualizado.pais,
              estado: proveedorActualizado.estado,
              contrato_vigente: proveedorActualizado.contrato_vigente,
            },
          });
  
          await cargarProveedores();
          
          showEditarProveedorModal.value = false;
        } catch (error) {
          console.error('Error al actualizar proveedor:', error);
        }
      };

      const guardarNuevoProveedor = async (nuevoProveedor) => {
        try {
          console.log('Datos enviados al backend para crear proveedor:', nuevoProveedor); // Depuración

          // Elimina el campo `id` si está presente
          const proveedorSinId = { ...nuevoProveedor };
          delete proveedorSinId.id;

          await invoke('crear_proveedor', { proveedor: proveedorSinId });
          await cargarProveedores();
          showNuevoProveedorModal.value = false;
        } catch (error) {
          console.error('Error al guardar el nuevo proveedor:', error);
        }
      };

      const verDetalleProveedor = (proveedor) => {
        console.log('Proveedor seleccionado para ver detalles:', proveedor); // Depuración

        // Cierra cualquier modal abierto
        showDetalleProveedorModal.value = false;
        showProductosProveedorModal.value = false;

        // Abre el modal de detalles para el proveedor seleccionado
        proveedorSeleccionado.value = { ...proveedor };
        showDetalleProveedorModal.value = true;
      };

      const confirmarEliminarProveedor = (proveedor) => {
        console.log('Proveedor seleccionado para eliminar:', proveedor); // Depuración
        proveedorSeleccionado.value = { ...proveedor };
        showConfirmacionModal.value = true;
      };

      const eliminarProveedor = async () => {
        try {
          if (!proveedorSeleccionado.value?.id) {
            throw new Error('No se ha seleccionado un proveedor válido para eliminar.');
          }
      
          console.log('Eliminando proveedor con ID:', proveedorSeleccionado.value.id); // Depuración
      
          // Llamada al backend para eliminar el proveedor
          await invoke('eliminar_proveedor', { id: proveedorSeleccionado.value.id });
      
          // Recargar la lista de proveedores
          await cargarProveedores();
      
          // Cerrar el modal de confirmación
          showConfirmacionModal.value = false;
        } catch (error) {
          console.error('Error al eliminar proveedor:', error);
        }
      };

      const cargarProductos = async () => {
        try {
          productos.value = await invoke('obtener_productos_proveedor', {
            proveedor_id: props.proveedorId,
          });
        } catch (error) {
          console.error('Error al cargar productos:', error);
        }
      };

      const agregarProducto = async (nuevoProducto) => {
        try {
          await invoke('agregar_producto_proveedor', {
            producto: {
              proveedor_id: props.proveedorId,
              producto: nuevoProducto,
            },
          });
          await cargarProductos();
          showAgregarProducto.value = false;
        } catch (error) {
          console.error('Error al agregar producto:', error);
        }
      };

      

      const toggleAcordeon = (id) => {
        proveedorAbierto.value = proveedorAbierto.value === id ? null : id;
      };
  
      return {
        proveedores,
        proveedorSeleccionado,
        searchTerm,
        filtroEstado,
        filtroContrato,
        mostrarFiltrosAvanzados,
        loading,
        error,
        showNuevoProveedorModal,
        showEditarProveedorModal,
        showDetalleProveedorModal,
        showConfirmacionModal,
        showProductosProveedorModal,
       
        proveedoresFiltrados,
        aplicarFiltros,
        
        toggleFiltrosAvanzados,
        abrirEditarProveedor,
        actualizarProveedor,
        guardarNuevoProveedor,
        verDetalleProveedor,
        confirmarEliminarProveedor,
        eliminarProveedor,
        cargarProductos,
        agregarProducto,
        
        proveedorAbierto,
        toggleAcordeon,
      };
    },
  };
  </script>
  
  <style scoped>
/* Contenedor principal */
.proveedores-container {
  padding: 20px;
  max-width: 95%;
  margin: 0 auto;
 
  border-radius: 8px;
 
}

/* Encabezado */
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding: 20px;
  background: linear-gradient(135deg, #4caf50, #22c9be); /* Degradado verde-azul */
  border-radius: 8px;
  color: white;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.header-content {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.header h1 {
  margin: 0;
  font-size: 1.8rem;
  display: flex;
  align-items: center;
  gap: 10px;
}

.header h1 i {
  font-size: 1.5rem;
}

.subtitle {
  margin: 0;
  font-size: 1rem;
  color: #d6eaf8; /* Color más claro para el subtítulo */
}

/* Filtros */
.filtros-container {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  margin-bottom: 20px;
  align-items: center;
  
}

.search-box {
  position: relative;
  flex-grow: 1;
  min-width: 300px;
  max-width: 1650px;
  margin-right: auto;
  
}

.search-box input {
  width: 100%;
  padding: 10px 15px 10px 35px;
  border-radius: 5px;
  border: 1px solid #ddd;
}

.search-box i {
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%);
  color: #777;
}

.filtros-avanzados {
  position: relative;
  flex-shrink: 0;
  
}

.filtros-content {
  position: absolute;
  right: 0;
  top: 100%;
  background: white;
  padding: 15px;
  border-radius: 5px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  z-index: 10;
  width: 250px;
  margin-top: 5px;
}

.filtro-group {
  margin-bottom: 10px;
}

.filtro-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: 500;
}

.filtro-group select {
  width: 100%;
  padding: 8px;
  border-radius: 5px;
  border: 1px solid #ddd;
}

/* Botón de filtros avanzados */
.filtros-avanzados button {
  padding: 8px 13px; /* Tamaño compacto */
  border: none;
  border-radius: 8px; /* Bordes redondeados */
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 500;
  font-size: 0.95rem; /* Tamaño de texto ligeramente más pequeño */
  background: linear-gradient(135deg, #4caf50, #1a958d); /* Degradado verde-azul */
  color: white;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); /* Sombra ligera */
  transition: transform 0.2s ease, box-shadow 0.2s ease, background 0.3s ease;
}

.filtros-avanzados button i {
  font-size: 1rem; /* Tamaño del ícono */
}

.filtros-avanzados button:hover {
  background: linear-gradient(135deg, #388e3c, #1ba89f); /* Degradado más oscuro */
  transform: translateY(-2px); /* Efecto de elevación */
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15); /* Sombra más pronunciada */
}

.filtros-avanzados button:active {
  transform: translateY(0); /* Elimina la elevación al hacer clic */
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); /* Sombra más suave */
}

/* Tarjetas */
.cards-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 20px;
  align-items: start;
}

.proveedor-card {
  transition: all 0.3s ease;
  min-height: 150px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  overflow: hidden;
  background: white;
}

.proveedor-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transform: translateY(-2px);
}

/* Estados de carga y error */
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
  background: #ffe6e6; /* Fondo rojo claro */
  color: #d32f2f; /* Texto rojo */
}

.no-results {
  color: #6c757d;
}




/* Botones */
.btn-primary {
  padding: 12px 20px;
  border: none;
  border-radius: 8px; /* Bordes más redondeados */
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 10px;
  font-weight: 600;
  font-size: 1rem;
  background: linear-gradient(135deg, #4caf50, #1a958d); /* Degradado verde-azul */
  color: white;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); /* Sombra para darle profundidad */
  transition: transform 0.2s ease, box-shadow 0.2s ease, background 0.3s ease;
}

.btn-primary i {
  font-size: 1.2rem; /* Tamaño del ícono */
}

.btn-primary:hover {
  background: linear-gradient(135deg, #388e3c, #1ba89f); /* Degradado más oscuro */
  transform: translateY(-2px); /* Efecto de elevación */
  box-shadow: 0 6px 10px rgba(0, 0, 0, 0.15); /* Sombra más pronunciada */
}

.btn-primary:active {
  transform: translateY(0); /* Elimina la elevación al hacer clic */
  box-shadow: 0 3px 6px rgba(0, 0, 0, 0.1); /* Sombra más suave */
}

.btn-primary.small {
  padding: 8px 12px;
  font-size: 0.9em;
}

.btn-secondary {
  background-color: #f0f0f0; /* Gris claro */
  color: #333;
}

.btn-secondary:hover {
  background-color: #e0e0e0;
}

/* Animación de carga */
.fa-spin {
  animation: fa-spin 2s infinite linear;
}

@keyframes fa-spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(359deg);
  }
}
</style>