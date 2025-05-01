<template>
    <div class="card">
      <p><strong>ID:</strong> {{ compra.id }}</p>
      <p><strong>Fecha:</strong> {{ compra.fecha }}</p>
      <p><strong>ID Proveedor:</strong> {{ compra.id_proveedor }}</p>
     
      <p><strong>Total:</strong> {{ compra.total }}€</p>
  
      <button @click="$emit('editar', compra)">Editar</button>
      <button @click="$emit('eliminar', compra)">Eliminar</button>
    </div>
  </template>
  
  <script setup>
  defineProps({
    compra: Object
  });
  </script>
  