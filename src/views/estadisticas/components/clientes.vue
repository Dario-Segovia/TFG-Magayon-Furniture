<template>
  <div style="padding: 20px">
    <h1>Estadísticas de Clientes</h1>

    <!-- Total clientes -->
    <section>
      <h2>Total de clientes</h2>
      <p>{{ totalClientes }}</p>
    </section>

    <!-- Clientes por país -->
    <section>
      <h2>Clientes por país</h2>
      <ul>
        <li v-for="(item, index) in clientesPorPais" :key="index">
          {{ item.categoria }}: {{ item.total }}
        </li>
      </ul>
    </section>

    <!-- Clientes por ciudad -->
    <section>
      <h2>Clientes por ciudad</h2>
      <ul>
        <li v-for="(item, index) in clientesPorCiudad" :key="index">
          {{ item.categoria }}: {{ item.total }}
        </li>
      </ul>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const totalClientes = ref(0)
const clientesPorPais = ref([])
const clientesPorCiudad = ref([])

onMounted(async () => {
  const total = await invoke('obtener_total_clientes')
  totalClientes.value = total.total

  clientesPorPais.value = await invoke('clientes_por_pais')
  clientesPorCiudad.value = await invoke('clientes_por_ciudad')
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