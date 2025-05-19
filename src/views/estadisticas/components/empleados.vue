<template>
  <div style="padding: 20px">
    <h1>Estadísticas de Empleados</h1>

    <section>
      <h2>Total de empleados</h2>
      <p>{{ totalEmpleados }}</p>
    </section>

    <section>
      <h2>Empleados por puesto</h2>
      <ul>
        <li v-for="(item, index) in empleadosPorPuesto" :key="index">
          {{ item.categoria }}: {{ item.total }}
        </li>
      </ul>
    </section>

    <section>
      <h2>Salario promedio por puesto</h2>
      <ul>
        <li v-for="(item, index) in salarioPromedioPorPuesto" :key="index">
          {{ item.categoria }}: {{ item.promedio.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}
        </li>
      </ul>
    </section>

    <section>
      <h2>Salario total</h2>
      <p>{{ salarioTotal }} €</p>
    </section>

    <section>
      <h2>Antigüedad promedio</h2>
      <p>{{ antiguedadPromedio }} años</p>
    </section>

    <section>
      <h2>Contrataciones por año</h2>
      <ul>
        <li v-for="(item, index) in contratacionesPorAnio" :key="index">
          {{ item.categoria }}: {{ item.total }}
        </li>
      </ul>
    </section>

    <section>
      <h2>Empleado con mayor salario</h2>
      <p>{{ empleadoMayorSalario.nombre }} {{ empleadoMayorSalario.apellido }} - {{ empleadoMayorSalario.salario }} €</p>
    </section>

    <section>
      <h2>Contrataciones en los últimos 12 meses</h2>
      <p>{{ contratacionesRecientes }}</p>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const totalEmpleados = ref(0)
const empleadosPorPuesto = ref([])
const salarioPromedioPorPuesto = ref([])
const salarioTotal = ref(0)
const antiguedadPromedio = ref(0)
const contratacionesPorAnio = ref([])
const empleadoMayorSalario = ref({ nombre: '', apellido: '', salario: 0 })
const contratacionesPorMes = ref([])
const contratacionesRecientes = ref(0)

onMounted(async () => {
  totalEmpleados.value = await invoke('empleados_totales')
  empleadosPorPuesto.value = await invoke('empleados_por_puesto')
  salarioPromedioPorPuesto.value = await invoke('salario_promedio_por_puesto')
  salarioTotal.value = await invoke('salario_total')
  antiguedadPromedio.value = await invoke('antiguedad_promedio')
  contratacionesPorAnio.value = await invoke('contrataciones_por_anio')
  empleadoMayorSalario.value = await invoke('empleado_mayor_salario')
  contratacionesPorMes.value = await invoke('contrataciones_ultimos_meses')
  contratacionesRecientes.value = contratacionesPorMes.value.reduce((acc, item) => acc + (item.total || 0), 0)
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
