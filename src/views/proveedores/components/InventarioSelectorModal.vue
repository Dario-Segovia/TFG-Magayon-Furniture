<template>
  <div class="modal-overlay">
    <div class="modal-container">
      <div class="modal-header">
        <h3>Selecciona un producto del inventario</h3>
        <button @click="$emit('close')" class="close-btn" aria-label="Cerrar">✕</button>
      </div>
      <div class="modal-body">
        <input
          v-model="search"
          placeholder="Buscar producto por nombre o categoría..."
          class="form-input buscador"
          autofocus
        />
        <ul class="inventario-list">
          <li v-for="item in filtrados" :key="item.id" class="inventario-item">
            <div class="info">
              <span class="nombre">{{ item.nombre }}</span>
              <span v-if="item.categoria" class="categoria">({{ item.categoria }})</span>
              <span v-if="item.descripcion" class="descripcion">- {{ item.descripcion }}</span>
            </div>
            <div class="acciones">
              <span class="stock">Stock: {{ item.cantidad }}</span>
              <span class="precio">${{ Number(item.precio_unitario).toFixed(2) }}</span>
              <button @click="$emit('select', item)" class="btn-mini btn-success">Añadir</button>
            </div>
          </li>
          <li v-if="filtrados.length === 0" class="no-result">No se encontraron productos.</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const inventario = ref([]);
const search = ref('');

onMounted(async () => {
  inventario.value = await invoke('get_inventory');
});

const filtrados = computed(() =>
  inventario.value.filter(i =>
    (i.nombre?.toLowerCase() ?? '').includes(search.value.toLowerCase()) ||
    (i.categoria?.toLowerCase() ?? '').includes(search.value.toLowerCase()) ||
    (i.descripcion?.toLowerCase() ?? '').includes(search.value.toLowerCase())
  )
);
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0,0,0,0.5);
  display: flex; align-items: center; justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(2px);
}
.modal-container {
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0,0,0,0.18);
  width: 95%;
  max-width: 500px;
  max-height: 90vh;
  overflow-y: auto;
  animation: modalFadeIn 0.25s;
}
@keyframes modalFadeIn {
  from { opacity: 0; transform: translateY(-20px);}
  to { opacity: 1; transform: translateY(0);}
}
.modal-header {
  display: flex; justify-content: space-between; align-items: center;
  padding: 18px 24px; border-bottom: 1px solid #f0f0f0;
}
.modal-header h3 {
  margin: 0; font-size: 1.25rem; font-weight: 600;
}
.close-btn {
  background: none; border: none; font-size: 1.5rem; cursor: pointer;
  color: #888; transition: color 0.2s;
}
.close-btn:hover { color: #e74c3c; }
.modal-body {
  padding: 20px 24px;
}
.buscador {
  width: 100%; padding: 10px 12px; margin-bottom: 18px;
  border: 1px solid #ddd; border-radius: 6px;
  font-size: 1rem;
}
.inventario-list {
  list-style: none; padding: 0; margin: 0;
  max-height: 350px; overflow-y: auto;
}
.inventario-item {
  display: flex; justify-content: space-between; align-items: center;
  padding: 10px 0; border-bottom: 1px solid #f4f4f4;
}
.info {
  display: flex; flex-direction: column;
}
.nombre {
  font-weight: 500; font-size: 1.05rem;
}
.categoria {
  color: #888; font-size: 0.95rem; margin-left: 4px;
}
.descripcion {
  color: #aaa; font-size: 0.92rem; margin-left: 2px;
}
.acciones {
  display: flex; align-items: center; gap: 10px;
}
.stock {
  color: #3498db; font-size: 0.95rem;
}
.precio {
  color: #27ae60; font-weight: 500; font-size: 1rem;
}
.btn-mini {
  padding: 4px 10px; border-radius: 5px; border: none;
  background: #27ae60; color: #fff; font-size: 0.95rem; cursor: pointer;
  transition: background 0.2s;
}
.btn-mini:hover { background: #219150; }
.no-result {
  text-align: center; color: #888; padding: 18px 0;
}
</style>