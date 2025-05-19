<template>
  <div style="padding: 20px">
    <h1>Estadísticas de Proveedores</h1>

    <section>
      <h2>Resumen</h2>
      <ul>
        <li>Total de proveedores: {{ resumen.total }}</li>
        <li>Activos: {{ resumen.activos }}</li>
        <li>Con contrato vigente: {{ resumen.con_contrato }}</li>
        <li>Países diferentes: {{ resumen.paises }}</li>
      </ul>
    </section>

    <section>
      <h2>Proveedores por país</h2>
      <ul>
        <li v-for="item in proveedoresPorPais" :key="item.pais">
          {{ item.pais }}: {{ item.total }}
        </li>
      </ul>
    </section>

    <section>
      <h2>Número de productos por proveedor</h2>
      <ul>
        <li v-for="item in productosPorProveedor" :key="item.proveedor">
          {{ item.proveedor }}: {{ item.total_productos }}
        </li>
      </ul>
    </section>

    <section>
      <h2>Producto más caro por proveedor</h2>
      <ul>
        <li v-for="item in productosMasCarosPorProveedor" :key="item.proveedor + '-' + item.producto">
          {{ item.proveedor }} - {{ item.producto }}: {{ item.precio_unitario.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}
        </li>
      </ul>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const resumen = ref({ total: 0, activos: 0, con_contrato: 0, paises: 0 })
const proveedoresPorPais = ref([])
const productosPorProveedor = ref([])
const productosMasCarosPorProveedor = ref([])

onMounted(async () => {
  resumen.value = await invoke('proveedores_resumen')
  proveedoresPorPais.value = await invoke('proveedores_por_pais')
  productosPorProveedor.value = await invoke('productos_por_proveedor')
  productosMasCarosPorProveedor.value = await invoke('productos_mas_caros_por_proveedor')
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