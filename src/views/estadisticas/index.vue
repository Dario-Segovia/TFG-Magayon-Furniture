<template>
  <div style="padding: 32px; max-width: 400px; margin: auto;">
    <h1>Estadísticas</h1>
    <div style="display: flex; flex-direction: column; gap: 16px;">
      <button @click="go('clientes')">Clientes</button>
      <button @click="go('empleados')">Empleados</button>
      <button @click="go('ventas')">Ventas</button>
      <button @click="go('compras')">Compras</button>
      <button @click="go('horarios')">Horarios</button>
      <button @click="go('inventario')">Inventario</button>
      <button @click="go('proveedores')">Proveedores</button>
    </div>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'
const router = useRouter()
function go(tipo) {
  router.push(`/estadisticas/${tipo}`)
}
</script>

<style scoped>
button {
  padding: 12px 0;
  font-size: 1.1em;
  border-radius: 6px;
  border: 1px solid #ccc;
  background: #f8f8f8;
  cursor: pointer;
  transition: background 0.2s;
}
button:hover {
  background: #e0e0e0;
}
</style>