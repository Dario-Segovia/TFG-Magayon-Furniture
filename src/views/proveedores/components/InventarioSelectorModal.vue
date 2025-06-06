<template>
  <div class="modal-overlay">
    <div class="modal-container">
      <div class="modal-header">
        <h3>Selección de productos</h3>
        <div class="header-actions">
          <span v-if="selectedItems.size > 0" class="selected-count">
            {{ selectedItems.size }} seleccionados
          </span>
          <button @click="$emit('close')" class="close-btn" aria-label="Cerrar">✕</button>
        </div>
      </div>
      <div class="modal-body">
        <div class="search-container">
          <input
            v-model="search"
            placeholder="Buscar producto (nombre, categoría, descripción)..."
            class="form-input buscador"
            autofocus
            @keydown.enter="handleEnter"
          />
          <div class="search-actions">
            <button 
              @click="toggleSelectAll" 
              class="btn-select-all"
              :class="{ 'active': allSelected }"
            >
              {{ allSelected ? 'Deseleccionar todos' : 'Seleccionar todos' }}
            </button>
          </div>
        </div>
        
        <div class="inventory-container">
          <div class="inventory-scroller">
            <div 
              v-for="item in paginatedItems" 
              :key="item.id" 
              class="inventory-item"
              :class="{ 'selected': selectedItems.has(item.id) }"
              @click="toggleItemSelection(item)"
            >
              <div class="item-checkbox">
                <input 
                  type="checkbox" 
                  :checked="selectedItems.has(item.id)"
                  @click.stop
                />
              </div>
              <div class="item-info">
                <div class="item-name">{{ item.nombre }}</div>
                <div class="item-details">
                  <span v-if="item.categoria" class="item-category">{{ item.categoria }}</span>
                  <span v-if="item.descripcion" class="item-description">{{ item.descripcion }}</span>
                </div>
              </div>
              <div class="item-meta">
                <span class="item-price">${{ Number(item.precio_unitario).toFixed(2) }}</span>
                <span class="item-stock">{{ item.cantidad }} en stock</span>
              </div>
            </div>
          </div>
          
          <div v-if="filteredItems.length === 0" class="no-results">
            No se encontraron productos
          </div>
          
          <div v-if="filteredItems.length > itemsPerPage" class="pagination-controls">
            <button 
              @click="prevPage" 
              :disabled="currentPage === 1"
              class="pagination-btn"
            >
              Anterior
            </button>
            <span class="page-indicator">
              Página {{ currentPage }} de {{ totalPages }}
            </span>
            <button 
              @click="nextPage" 
              :disabled="currentPage === totalPages"
              class="pagination-btn"
            >
              Siguiente
            </button>
          </div>
        </div>
        
        <div class="modal-footer" v-if="selectedItems.size > 0">
          <button @click="confirmSelection" class="btn-confirm">
            Confirmar selección ({{ selectedItems.size }})
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  initialSelected: {
    type: Array,
    default: () => []
  }
});

const emit = defineEmits(['select', 'select-multiple', 'close']);

const inventario = ref([]);
const search = ref('');
const selectedItems = ref(new Set(props.initialSelected.map(item => item.id)));
const currentPage = ref(1);
const itemsPerPage = 50;

onMounted(async () => {
  inventario.value = await invoke('get_inventory');
});

const filteredItems = computed(() => {
  const searchTerm = search.value.toLowerCase();
  return inventario.value.filter(i =>
    (i.nombre?.toLowerCase() ?? '').includes(searchTerm) ||
    (i.categoria?.toLowerCase() ?? '').includes(searchTerm) ||
    (i.descripcion?.toLowerCase() ?? '').includes(searchTerm)
  );
});

const paginatedItems = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  return filteredItems.value.slice(start, end);
});

const totalPages = computed(() => 
  Math.ceil(filteredItems.value.length / itemsPerPage)
);

const allSelected = computed(() => {
  if (filteredItems.value.length === 0) return false;
  return filteredItems.value.every(item => selectedItems.value.has(item.id));
});

watch(search, () => {
  currentPage.value = 1;
});

function toggleItemSelection(item) {
  if (selectedItems.value.has(item.id)) {
    selectedItems.value.delete(item.id);
  } else {
    selectedItems.value.add(item.id);
  }
  selectedItems.value = new Set(selectedItems.value);
}

function toggleSelectAll() {
  if (allSelected.value) {
    filteredItems.value.forEach(item => {
      selectedItems.value.delete(item.id);
    });
  } else {
    filteredItems.value.forEach(item => {
      selectedItems.value.add(item.id);
    });
  }
  selectedItems.value = new Set(selectedItems.value);
}

function confirmSelection() {
  const selected = inventario.value.filter(item => 
    selectedItems.value.has(item.id)
  );
  emit('select-multiple', selected);
  emit('close');
}

function nextPage() {
  if (currentPage.value < totalPages.value) {
    currentPage.value++;
  }
}

function prevPage() {
  if (currentPage.value > 1) {
    currentPage.value--;
  }
}

function handleEnter() {
  if (filteredItems.value.length === 1) {
    toggleItemSelection(filteredItems.value[0]);
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.modal-container {
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  width: 95%;
  max-width: 800px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  animation: modalFadeIn 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

@keyframes modalFadeIn {
  from { opacity: 0; transform: translateY(-30px) scale(0.95); }
  to { opacity: 1; transform: translateY(0) scale(1); }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  border-bottom: 1px solid #f0f0f0;
  background: #f9fafb;
  border-radius: 12px 12px 0 0;
}

.modal-header h3 {
  margin: 0;
  font-size: 1.3rem;
  font-weight: 600;
  color: #2d3748;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 16px;
}

.selected-count {
  font-size: 0.9rem;
  color: #4a5568;
  background: #edf2f7;
  padding: 4px 10px;
  border-radius: 20px;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #718096;
  transition: color 0.2s;
  padding: 4px;
  line-height: 1;
}

.close-btn:hover {
  color: #e53e3e;
}

.modal-body {
  padding: 0;
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.search-container {
  padding: 16px 24px;
  border-bottom: 1px solid #f0f0f0;
  background: #fff;
}

.buscador {
  width: 100%;
  padding: 12px 16px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  transition: border-color 0.2s, box-shadow 0.2s;
  margin-bottom: 12px;
}

.buscador:focus {
  outline: none;
  border-color: #4299e1;
  box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.2);
}

.search-actions {
  display: flex;
  justify-content: flex-end;
}

.btn-select-all {
  background: none;
  border: none;
  color: #4299e1;
  font-size: 0.9rem;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  transition: background 0.2s;
}

.btn-select-all:hover {
  background: #ebf8ff;
}

.btn-select-all.active {
  color: #3182ce;
  font-weight: 500;
}

.inventory-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.inventory-scroller {
  flex: 1;
  overflow-y: auto;
  padding: 0 16px;
}

.inventory-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  transition: background 0.2s;
  gap: 12px;
}

.inventory-item:hover {
  background: #f8fafc;
}

.inventory-item.selected {
  background: #ebf8ff;
  border-left: 3px solid #4299e1;
}

.item-checkbox {
  flex-shrink: 0;
}

.item-checkbox input {
  cursor: pointer;
}

.item-info {
  flex: 1;
  min-width: 0;
}

.item-name {
  font-weight: 500;
  font-size: 1rem;
  color: #2d3748;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-details {
  display: flex;
  gap: 8px;
  font-size: 0.85rem;
  color: #718096;
  margin-top: 4px;
}

.item-category {
  background: #edf2f7;
  padding: 2px 8px;
  border-radius: 4px;
}

.item-description {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-meta {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 4px;
  flex-shrink: 0;
  margin-left: 12px;
}

.item-price {
  font-weight: 600;
  color: #2f855a;
}

.item-stock {
  font-size: 0.8rem;
  color: #718096;
}

.no-results {
  padding: 32px;
  text-align: center;
  color: #a0aec0;
  font-size: 0.95rem;
}

.pagination-controls {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 16px;
  padding: 16px;
  border-top: 1px solid #f0f0f0;
  background: #f9fafb;
}

.pagination-btn {
  padding: 6px 12px;
  background: #fff;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.2s, border-color 0.2s;
}

.pagination-btn:hover {
  background: #edf2f7;
  border-color: #cbd5e0;
}

.pagination-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.page-indicator {
  font-size: 0.9rem;
  color: #4a5568;
}

.modal-footer {
  padding: 16px 24px;
  border-top: 1px solid #f0f0f0;
  display: flex;
  justify-content: flex-end;
  background: #f9fafb;
  border-radius: 0 0 12px 12px;
}

.btn-confirm {
  padding: 10px 20px;
  background: #4299e1;
  color: white;
  border: none;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-confirm:hover {
  background: #3182ce;
}
</style>