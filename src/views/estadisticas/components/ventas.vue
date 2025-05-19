<template>
  <div style="padding: 20px">
    <h1>Estadísticas de Ventas</h1>

    <section>
      <h2>Resumen</h2>
      <ul>
        <li>Total de ventas: {{ resumen.total_ventas }}</li>
        <li>Total de ingresos: {{ resumen.total_ingresos.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}</li>
        <li>Clientes distintos: {{ resumen.clientes_distintos }}</li>
      </ul>
    </section>

    <section>
      <h2>Ventas por mes</h2>
      <ul>
        <li v-for="item in ventasPorMes" :key="item.mes">
          {{ item.mes }}: {{ item.total.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}
        </li>
      </ul>
    </section>

    <section>
      <h2>Productos más vendidos</h2>
      <ul>
        <li v-for="item in productosMasVendidos" :key="item.nombre">
          {{ item.nombre }}: {{ item.cantidad }}
        </li>
      </ul>
    </section>

    <section>
      <h2>Clientes top</h2>
      <ul>
        <li v-for="item in clientesTop" :key="item.nombre">
          {{ item.nombre }}: {{ item.total_gastado.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}
        </li>
      </ul>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const resumen = ref({ total_ventas: 0, total_ingresos: 0, clientes_distintos: 0 })
const ventasPorMes = ref([])
const productosMasVendidos = ref([])
const clientesTop = ref([])

onMounted(async () => {
  resumen.value = await invoke('ventas_resumen')
  ventasPorMes.value = await invoke('ventas_por_mes')
  productosMasVendidos.value = await invoke('productos_mas_vendidos')
  clientesTop.value = await invoke('clientes_top')
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