<template>
  <div style="padding: 20px">
    <h1>Estadísticas de Horarios</h1>

    <section>
      <h2>Resumen</h2>
      <ul>
        <li>Total de registros: {{ resumen.total_registros }}</li>
        <li>Total de horas trabajadas: {{ resumen.total_horas.toFixed(2) }} h</li>
        <li>Empleados distintos: {{ resumen.empleados_distintos }}</li>
      </ul>
    </section>

    <section>
      <h2>Horas trabajadas por empleado</h2>
      <ul>
        <li v-for="item in horasPorEmpleado" :key="item.nombre">
          {{ item.nombre }}: {{ item.total_horas.toFixed(2) }} h
        </li>
      </ul>
    </section>

    <section>
      <h2>Turnos por tipo</h2>
      <ul>
        <li v-for="item in turnosPorTipo" :key="item.categoria">
          {{ item.categoria }}: {{ item.total }}
        </li>
      </ul>
    </section>

    <section>
      <h2>Turnos por empleado y tipo</h2>
      <ul>
        <li v-for="item in turnosPorEmpleado" :key="item.nombre + '-' + item.tipo_turno">
          {{ item.nombre }} - {{ item.tipo_turno }}: {{ item.cantidad }}
        </li>
      </ul>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const resumen = ref({ total_registros: 0, total_horas: 0, empleados_distintos: 0 })
const horasPorEmpleado = ref([])
const turnosPorTipo = ref([])
const turnosPorEmpleado = ref([])

onMounted(async () => {
  resumen.value = await invoke('resumen_horarios')
  horasPorEmpleado.value = await invoke('horas_por_empleado')
  turnosPorTipo.value = await invoke('turnos_por_tipo')
  turnosPorEmpleado.value = await invoke('turnos_por_empleado')
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