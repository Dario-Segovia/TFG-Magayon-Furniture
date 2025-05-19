<template>
  <div style="padding: 20px">
    <h1>Estadísticas de Compras</h1>

    <section>
      <h2>Resumen</h2>
      <ul>
        <li>Total de compras: {{ resumen.total_compras }}</li>
        <li>Total gastado: {{ resumen.total_gastado.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}</li>
        <li>Proveedores distintos: {{ resumen.proveedores_distintos }}</li>
      </ul>
    </section>

    <section>
      <h2>Compras por mes</h2>
      <ul>
        <li v-for="item in comprasPorMes" :key="item.mes">
          {{ item.mes }}: {{ item.total.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}
        </li>
      </ul>
    </section>

    <section>
      <h2>Productos más comprados</h2>
      <ul>
        <li v-for="item in productosMasComprados" :key="item.nombre">
          {{ item.nombre }}: {{ item.cantidad }}
        </li>
      </ul>
    </section>

    <section>
      <h2>Proveedores top</h2>
      <ul>
        <li v-for="item in proveedoresTop" :key="item.nombre">
          {{ item.nombre }}: {{ item.total_pagado.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}
        </li>
      </ul>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const resumen = ref({ total_compras: 0, total_gastado: 0, proveedores_distintos: 0 })
const comprasPorMes = ref([])
const productosMasComprados = ref([])
const proveedoresTop = ref([])

onMounted(async () => {
  resumen.value = await invoke('compras_resumen')
  comprasPorMes.value = await invoke('compras_por_mes')
  productosMasComprados.value = await invoke('productos_mas_comprados')
  proveedoresTop.value = await invoke('proveedores_top')
})
</script>

<style scoped>
h1 {
  font-size: 2em;
  margin-bottom: 1em;
}
section {
  margin-bottom: 2em;
}
ul {
  padding-left: 20px;
}
</style>