<template>
  <button class="btn-back" @click="$router.back()">
    <i class="fas fa-arrow-left"></i> {{ $t('inventario.back') }}
  </button>
  <div class="inventario-container">
    <div class="header fade-in">
      <div class="header-content">
        <h1><i class="fas fa-boxes"></i> {{ $t('inventario.title') }}</h1>
        <p class="subtitle">{{ $t('inventario.subtitle') }}</p>
      </div>
      <div class="header-actions">
        <button @click="showAgregar = true" class="btn-primary">
          <i class="fas fa-plus"></i> {{ $t('inventario.nuevo') }}
        </button>
        <input
          ref="inputArchivo"
          type="file"
          accept=".xml"
          style="display: none"
          @change="onArchivoSeleccionado"
        />
        <button @click="showImportarXml = true" class="btn-icon btn-xml" :title="$t('inventario.importar_xml')">
          <i class="fas fa-file-code"></i>
        </button>
      </div>
    </div>

    <div class="filtros-container fade-in">
      <div class="search-box">
        <input v-model="searchTerm" :placeholder="$t('inventario.buscar')" />
        <i class="fas fa-search"></i>
      </div>
      <div class="filtros-avanzados">
        <button @click="mostrarFiltros = !mostrarFiltros">
          <i class="fas fa-filter"></i> {{ $t('inventario.filtros_avanzados') }}
        </button>
        <div v-if="mostrarFiltros" class="filtros-content">
          <div class="filtro-group">
            <label for="categoria">{{ $t('inventario.categoria') }}:</label>
            <select id="categoria" v-model="categoriaSeleccionada">
              <option value="">{{ $t('inventario.todas') }}</option>
              <option v-for="cat in categorias" :key="cat" :value="cat">{{ cat }}</option>
            </select>
          </div>
          <div class="filtro-group">
            <label>{{ $t('inventario.rango_precio') }}</label>
            <Slider
              v-model="rangoPrecio"
              :min="0"
              :max="1000"
              :step="10"
              :tooltip="'always'"
              :lazy="true"
              :strict="true"
              :format="{ to: v => v, from: v => v }"
            />
          </div>
        </div>
      </div>
    </div>

    <div v-if="loading" class="loading-container">
      <i class="fas fa-spinner fa-spin"></i> {{ $t('inventario.cargando') }}
    </div>
    <div v-if="error" class="error-container">
      <i class="fas fa-exclamation-triangle"></i> {{ error }}
    </div>

    <div class="inventario-grid">
      <div v-if="itemsFiltrados.length === 0" class="no-results">
        {{ $t('inventario.no_resultados') }}
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
    <ModalImportarInventario
      v-if="abrirImportar"
      @close="abrirImportar = false"
      @importar="importarArchivo"
    />
    <ModalImportarXml
      v-if="showImportarXml"
      @close="showImportarXml = false"
      @importar-xml="procesarArchivoXml"
    />
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { writeFile, readFile, BaseDirectory } from '@tauri-apps/plugin-fs';

import '@vueform/slider/themes/default.css'
import Slider from '@vueform/slider'

import InventarioCard from './components/InventarioCard.vue';
import ModalAgregarInventario from './components/ModalAgregarInventario.vue';
import ModalEditarInventario from './components/ModalEditarInventario.vue';
import ModalEliminarInventario from './components/ModalEliminarInventario.vue';
import ModalImportarXml from './components/ModalImportarXml.vue';

const items = ref([]);
const loading = ref(false);
const error = ref(null);

const showAgregar = ref(false);
const itemEditar = ref(null);
const itemEliminar = ref(null);
const abrirImportar = ref(false);
const mostrarFiltros = ref(false);
const showImportarXml = ref(false);

const searchTerm = ref('');
const inputArchivo = ref(null);
const categoriaSeleccionada = ref('');
const rangoPrecio = ref([0, 1000]);

const cargarInventario = async () => {
  loading.value = true;
  error.value = null;
  try {
    const resItems = await invoke('get_inventory');
    items.value = resItems;
  } catch (e) {
    console.error('Error al cargar inventario:', e);
    error.value = 'Error al cargar inventario';
  } finally {
    loading.value = false;
  }
};

onMounted(cargarInventario);

const itemsFiltrados = computed(() => {
  return items.value.filter(item => {
    const coincideBusqueda =
      item.nombre.toLowerCase().includes(searchTerm.value.toLowerCase()) ||
      (item.descripcion && item.descripcion.toLowerCase().includes(searchTerm.value.toLowerCase()));
    const coincideCategoria =
      !categoriaSeleccionada.value || item.categoria === categoriaSeleccionada.value;
    const coincidePrecio =
      item.precio_unitario >= rangoPrecio.value[0] && item.precio_unitario <= rangoPrecio.value[1];
    return coincideBusqueda && coincideCategoria && coincidePrecio;
  });
});

const categorias = computed(() => {
  const set = new Set();
  items.value.forEach(item => {
    if (item.categoria) set.add(item.categoria);
  });
  return Array.from(set);
});

const agregarItem = async (nuevo) => {
  try {
    await invoke('add_inventory_item', {
      item: {
        nombre: nuevo.nombre,
        descripcion: nuevo.descripcion,
        cantidad: nuevo.cantidad,
        precio_unitario: nuevo.precio_unitario,
        categoria: nuevo.categoria,
      }
    });
    showAgregar.value = false;
    await cargarInventario();
  } catch (e) {
    console.error('Error al agregar ítem:', e);
    error.value = 'Error al agregar ítem';
  }
};

const abrirEditar = (item) => {
  itemEditar.value = { ...item };
};

const editarItem = async (editado) => {
  try {
    await invoke('update_inventory_item', {
      id: editado.id,
      item: {
        nombre: editado.nombre,
        descripcion: editado.descripcion,
        cantidad: editado.cantidad,
        precio_unitario: editado.precio_unitario,
        categoria: editado.categoria
      }
    });
    itemEditar.value = null;
    await cargarInventario();
  } catch (e) {
    console.error('Error al editar ítem:', e);
    error.value = 'Error al editar ítem';
  }
};

const abrirEliminar = (item) => {
  itemEliminar.value = item;
};

const eliminarItem = async () => {
  try {
    await invoke('delete_inventory_item', { id: itemEliminar.value.id });
    itemEliminar.value = null;
    await cargarInventario();
  } catch (e) {
    console.error('Error al eliminar ítem:', e);
    error.value = 'Error al eliminar ítem';
  }
};

const importarArchivo = async (archivo) => {
  try {
    loading.value = true;
    error.value = null;

    const text = await archivo.text();

    await invoke('importar_inventario_xml', { xmlData: text });

    abrirImportar.value = false;
    await cargarInventario();
  } catch (e) {
    console.error('Error al importar archivo:', e);
    error.value = 'Error al importar archivo';
  } finally {
    loading.value = false;
  }
};

const abrirSelectorArchivo = () => {
  inputArchivo.value.click();
};

const onArchivoSeleccionado = (e) => {
  const archivo = e.target.files[0];
  if (archivo) {
    importarArchivo(archivo);
  }
};

const aplicarFiltros = () => {
  mostrarFiltros.value = false;
};

const procesarArchivoXml = async (xmlData) => {
  try {
    loading.value = true;
    error.value = null;

    await invoke('importar_inventario_xml', { xmlData });

    showImportarXml.value = false;
    await cargarInventario();
  } catch (e) {
    console.error('Error al procesar archivo XML:', e);
    error.value = 'Error al procesar archivo XML';
  } finally {
    loading.value = false;
  }
};
</script>


<style scoped>
.inventario-container {
  padding: 20px;
  max-width: 1800px;
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
  background: linear-gradient(135deg, #DB5375, #B3FFB3);
  border-radius: 10px;
  color: white;
  box-shadow: 0 4px 12px rgba(0,0,0,0.10);
  animation: fadeIn 0.5s;
}
.header-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
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
  display: flex;
  flex-direction: column;
  min-width: 160px;
}
.filtro-group label {
  margin-bottom: 4px;
  font-size: 0.95rem;
}
.filtro-group select {
  padding: 8px;
  border-radius: 8px;
  border: 1px solid #ddd;
  font-size: 1rem;
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

.header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.btn-icon {
  background: linear-gradient(135deg, #EA6E6E ,  #9375FE);
  border: none;
  border-radius: 8px;
  width: 48px;         /* cuadrado */
  height: 48px;        /* cuadrado */
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  cursor: pointer;
  font-size: 1.6rem;   /* icono más grande */
  box-shadow: 0 4px 6px rgba(0,0,0,0.10);
  transition: transform 0.2s, box-shadow 0.2s, background 0.3s;
  padding: 0;          /* sin padding extra */
}
.btn-icon:hover {
  background: linear-gradient(135deg, #9375FE, #EA6E6E);
  transform: translateY(-2px) scale(1.08);
  box-shadow: 0 6px 10px rgba(0,0,0,0.15);
}
.btn-xml i {
  font-size: 1.4em;
  margin: 0;
}
</style>