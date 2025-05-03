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

    <ModalAgregarCompras v-if="showAgregarModal" @close="cerrarAgregarModal" @refresh="listarCompras" />
    
    <ModalModificarCompras 
      :visible="showEditarModal"
      :compraSeleccionada="compraParaEditar"
      @cerrar="cerrarEditarModal"
      @guardar="handleCompraActualizada"
    />

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
const compraParaEditar = ref(null);

const compraSeleccionada = ref(null);
const showEditarModal = ref(false);






function abrirEditarModal(compra) {
  compraParaEditar.value = compra; // Asignar toda la compra, incluida su ID
  showEditarModal.value = true;
}


async function handleCompraActualizada(compraActualizada) {
  const fechaObj = new Date(compraActualizada.fecha);
const fechaFormateada = `${fechaObj.getFullYear()}-${(fechaObj.getMonth() + 1).toString().padStart(2, '0')}-${fechaObj.getDate().toString().padStart(2, '0')} ${fechaObj.getHours().toString().padStart(2, '0')}:${fechaObj.getMinutes().toString().padStart(2, '0')}:${fechaObj.getSeconds().toString().padStart(2, '0')}`;


  try {
    // Verificar que la compra tenga el campo 'id'
    if (!compraActualizada.id) {
      throw new Error("La compra debe tener un ID para actualizarla");
    }

    await invoke('actualizar_compra', {
  id: compraActualizada.id,
  idProveedor: compraActualizada.id_proveedor, // ✅ clave corregida
  total: compraActualizada.total,
  fecha: fechaFormateada // ✅ fecha en formato válido para PostgreSQL
});


    // Después de guardar los cambios, vuelve a listar las compras
    listarCompras();

    // Cerrar el modal de edición
    cerrarEditarModal();
  } catch (error) {
    console.error('Error actualizando la compra:', error);
  }
}




async function listarCompras() {
  try {
    compras.value = await invoke('listar_compras_con_proveedor');
    
    // Log para verificar que se obtienen todas las compras
    console.log("Compras:", compras.value);

    // Aquí recorremos cada compra y, en caso de que sea necesario, se asignan productos
    for (const compra of compras.value) {
      const productos = await invoke('obtener_productos_compra', {
        idCompra: compra.id
      });
      
      // Asignar los productos a la compra
      compra.productos = productos;

      // Log para ver el objeto de cada compra con los productos
      console.log("Compra completa con productos:", compra);
    }
  } catch (error) {
    console.error('Error al listar las compras:', error);
  }
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
