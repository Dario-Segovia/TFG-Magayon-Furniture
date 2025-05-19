<template>
  <div style="padding: 20px">
    <h1>Estadísticas de Inventario</h1>

    <section>
      <h2>Resumen</h2>
      <ul>
        <li>Total de productos: {{ resumen.total_productos }}</li>
        <li>Stock total: {{ resumen.stock_total }}</li>
        <li>Valor total del inventario: {{ resumen.valor_total.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}</li>
      </ul>
    </section>

    <section>
      <h2>Stock por categoría</h2>
      <ul>
        <li v-for="item in stockPorCategoria" :key="item.categoria">
          {{ item.categoria }}: {{ item.total }}
        </li>
      </ul>
    </section>

    <section>
      <h2>Productos críticos (menos de 10 en stock)</h2>
      <ul>
        <li v-for="item in productosCriticos" :key="item.nombre">
          {{ item.nombre }}: {{ item.cantidad }}
        </li>
      </ul>
    </section>

    <section>
      <h2>Productos más caros</h2>
      <ul>
        <li v-for="item in productosMasCaros" :key="item.nombre">
          {{ item.nombre }}: {{ item.precio_unitario.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}
        </li>
      </ul>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const resumen = ref({ total_productos: 0, stock_total: 0, valor_total: 0 })
const stockPorCategoria = ref([])
const productosCriticos = ref([])
const productosMasCaros = ref([])

onMounted(async () => {
  resumen.value = await invoke('inventario_resumen')
  stockPorCategoria.value = await invoke('stock_por_categoria')
  productosCriticos.value = await invoke('productos_criticos')
  productosMasCaros.value = await invoke('productos_mas_caros')
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