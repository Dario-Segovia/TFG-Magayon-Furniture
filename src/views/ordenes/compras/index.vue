<template>
  <div class="container">
    <h1>Compras</h1>

    <button @click="showAgregarModal = true">Agregar Compra</button>

    <div class="compras-list">
      <ComprasCard
        v-for="compra in compras"
        :key="compra.id"
        :compra="compra"
        @editar="abrirEditarModal"
        @eliminar="abrirEliminarModal"
      />
    </div>

    <ModalAgregarCompras v-if="showAgregarModal" @close="cerrarAgregarModal" @compra-agregada="listarCompras" />
    <ModalModificarCompras v-if="showEditarModal" :compra="compraSeleccionada" @close="cerrarEditarModal" @compra-actualizada="listarCompras" />
    <ModalEliminarCompras v-if="showEliminarModal" :compra="compraSeleccionada" @close="cerrarEliminarModal" @compra-eliminada="listarCompras" />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import ComprasCard from './components/ComprasCard.vue';
import ModalAgregarCompras from './components/ModalAgregarCompras.vue';
import ModalModificarCompras from './components/ModalModificarCompras.vue';
import ModalEliminarCompras from './components/ModalEliminarCompras.vue';

const compras = ref([]);
const showAgregarModal = ref(false);
const showEditarModal = ref(false);
const showEliminarModal = ref(false);
const compraSeleccionada = ref(null);

async function listarCompras() {
  try {
    compras.value = await invoke('listar_compras');
  } catch (error) {
    console.error(error);
  }
}

function abrirEditarModal(compra) {
  showEditarModal.value = true;
}

function abrirEliminarModal(compra) {
  compraSeleccionada.value = compra;
  showEliminarModal.value = true;
}

function cerrarAgregarModal() {
  showAgregarModal.value = false;
}

function cerrarEditarModal() {
  showEditarModal.value = false;
}

function cerrarEliminarModal() {
  showEliminarModal.value = false;
}

onMounted(() => {
  listarCompras();
});
</script>
