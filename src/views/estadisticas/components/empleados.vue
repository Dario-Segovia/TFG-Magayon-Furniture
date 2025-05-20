<template>
  <!-- Al inicio del template de cada componente de estadísticas -->
<button class="btn-back" @click="$router.back()">
  <i class="fas fa-arrow-left"></i> Atrás
</button>
  <div class="dashboard-container">
    <div class="dashboard-header">
      <h1 class="dashboard-title">Estadísticas de Empleados</h1>
      <div class="header-divider"></div>
    </div>

    <div class="dashboard-content">
      <!-- KPI Card -->
      <div class="kpi-card">
        <div class="kpi-content">
          <h2 class="kpi-title">Total de Empleados</h2>
          <p class="kpi-value">{{ formatNumber(totalEmpleados) }}</p>
          <div class="kpi-trend">
            <span class="trend-icon">👔</span>
            <span class="trend-text">Visión general</span>
          </div>
        </div>
        <div class="kpi-icon">👥</div>
      </div>

      <!-- Gráficos -->
      <div class="chart-row">
        <div class="chart-card">
          <div class="chart-header">
            <h2 class="chart-title">Empleados por Puesto</h2>
            <div class="chart-actions">
              <button class="chart-action-btn" @click="exportarCSV(empleadosPorPuesto, 'empleados_por_puesto.csv')">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <PieChart
              :labels="empleadosPorPuesto.map(e => e.categoria)"
              :data="empleadosPorPuesto.map(e => e.total)"
            />
            <div class="table-container">
              <table class="data-table">
                <thead>
                  <tr>
                    <th>Puesto</th>
                    <th>Cantidad</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="e in empleadosPorPuesto" :key="e.categoria">
                    <td>{{ e.categoria }}</td>
                    <td>{{ e.total }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <div class="chart-card">
          <div class="chart-header">
            <h2 class="chart-title">Salario Promedio por Puesto</h2>
            <div class="chart-actions">
              <button class="chart-action-btn" @click="exportarCSV(salarioPromedioPorPuesto, 'salario_promedio_por_puesto.csv')">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <BarChartHorizontal
              :labels="salarioPromedioPorPuesto.map(e => e.categoria)"
              :data="salarioPromedioPorPuesto.map(e => e.promedio)"
            />
            <div class="table-container">
              <table class="data-table">
                <thead>
                  <tr>
                    <th>Puesto</th>
                    <th>Salario Promedio (€)</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="e in salarioPromedioPorPuesto" :key="e.categoria">
                    <td>{{ e.categoria }}</td>
                    <td>{{ e.promedio.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <div class="chart-row">
        <div class="chart-card">
          <div class="chart-header">
            <h2 class="chart-title">Contrataciones por Año</h2>
            <div class="chart-actions">
              <button class="chart-action-btn" @click="exportarCSV(contratacionesPorAnio, 'contrataciones_por_anio.csv')">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <BarChart
              :labels="contratacionesPorAnio.map(e => e.categoria)"
              :datasets="[{ label: 'Contrataciones', data: contratacionesPorAnio.map(e => e.total), backgroundColor: '#1976d2' }]"
            />
            <div class="table-container">
              <table class="data-table">
                <thead>
                  <tr>
                    <th>Año</th>
                    <th>Contrataciones</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="e in contratacionesPorAnio" :key="e.categoria">
                    <td>{{ e.categoria }}</td>
                    <td>{{ e.total }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <div class="chart-card">
          <div class="chart-header">
            <h2 class="chart-title">Empleado con Mayor Salario</h2>
          </div>
          <div class="chart-container">
            <div class="table-container">
              <table class="data-table">
                <thead>
                  <tr>
                    <th>Nombre</th>
                    <th>Apellido</th>
                    <th>Salario (€)</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>{{ empleadoMayorSalario.nombre }}</td>
                    <td>{{ empleadoMayorSalario.apellido }}</td>
                    <td>{{ empleadoMayorSalario.salario.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import PieChart from '../../../components/PieChart.vue'
import BarChart from '../../../components/BarChart.vue'
import BarChartHorizontal from '../../../components/BarChartHorizontal.vue'

const totalEmpleados = ref(0)
const empleadosPorPuesto = ref([])
const salarioPromedioPorPuesto = ref([])
const contratacionesPorAnio = ref([])
const empleadoMayorSalario = ref({ nombre: '', apellido: '', salario: 0 })

const formatNumber = (num) => {
  return new Intl.NumberFormat('es-ES').format(num)
}

function exportarCSV(data, filename) {
  if (!data || !data.length) return
  const keys = Object.keys(data[0])
  const csvRows = [
    keys.join(','),
    ...data.map(row => keys.map(k => `"${(row[k] ?? '').toString().replace(/"/g, '""')}"`).join(','))
  ]
  const blob = new Blob([csvRows.join('\n')], { type: 'text/csv' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = filename
  a.click()
  URL.revokeObjectURL(url)
}

onMounted(async () => {
  totalEmpleados.value = await invoke('empleados_totales')
  empleadosPorPuesto.value = await invoke('empleados_por_puesto')
  salarioPromedioPorPuesto.value = await invoke('salario_promedio_por_puesto')
  contratacionesPorAnio.value = await invoke('contrataciones_por_anio')
  empleadoMayorSalario.value = await invoke('empleado_mayor_salario')
})
</script>

<style scoped>
:root {
  --primary-color: #3498db;
  --secondary-color: #2c3e50;
  --accent-color: #e74c3c;
  --light-gray: #ecf0f1;
  --medium-gray: #bdc3c7;
  --dark-gray: #7f8c8d;
  --white: #ffffff;
  --box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  --border-radius: 8px;
}

.dashboard-container {
  padding: 24px;
  font-family: 'Segoe UI', Roboto, 'Helvetica Neue', sans-serif;
  background-color: #f5f7fa;
  min-height: 100vh;
}

.dashboard-header {
  margin-bottom: 32px;
}

.dashboard-title {
  font-size: 28px;
  font-weight: 600;
  color: var(--secondary-color);
  margin: 0;
}

.header-divider {
  height: 3px;
  background: linear-gradient(90deg, var(--primary-color), transparent);
  margin-top: 8px;
  border-radius: 3px;
}

.dashboard-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

/* KPI Card */
.kpi-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: var(--white);
  border-radius: var(--border-radius);
  padding: 24px;
  box-shadow: var(--box-shadow);
  border-left: 4px solid var(--primary-color);
}

.kpi-content {
  flex: 1;
}

.kpi-title {
  font-size: 16px;
  color: var(--dark-gray);
  margin: 0 0 8px 0;
  font-weight: 500;
}

.kpi-value {
  font-size: 36px;
  font-weight: 700;
  color: var(--secondary-color);
  margin: 0 0 12px 0;
}

.kpi-trend {
  display: flex;
  align-items: center;
  gap: 8px;
}

.trend-icon {
  font-size: 18px;
}

.trend-text {
  font-size: 14px;
  color: var(--dark-gray);
}

.kpi-icon {
  font-size: 48px;
  opacity: 0.2;
}

/* Gráficos y tablas */
.chart-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
  gap: 24px;
}

.chart-card {
  background-color: var(--white);
  border-radius: var(--border-radius);
  box-shadow: var(--box-shadow);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  border-bottom: 1px solid var(--light-gray);
}

.chart-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--secondary-color);
  margin: 0;
}

.chart-actions {
  display: flex;
  gap: 8px;
}

.chart-action-btn {
  background-color: var(--light-gray);
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  font-size: 14px;
  color: var(--secondary-color);
  cursor: pointer;
  transition: all 0.2s;
}

.chart-action-btn:hover {
  background-color: var(--medium-gray);
}

.chart-container {
  padding: 16px 24px 24px;
  height: 460px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.table-container {
  overflow-x: auto;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 12px;
  background: var(--white);
  border-radius: 6px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(44, 62, 80, 0.04);
}

.data-table th, .data-table td {
  text-align: left;
  padding: 10px 12px;
}

.data-table th {
  background-color: var(--light-gray);
  font-weight: 600;
}

.data-table tr:nth-child(even) {
  background-color: #f9fafb;
}

/* Responsive */
@media (max-width: 900px) {
  .chart-row {
    grid-template-columns: 1fr;
  }
  .kpi-card {
    flex-direction: column;
    align-items: flex-start;
  }
  .kpi-icon {
    align-self: flex-end;
    margin-top: 16px;
  }
}
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
.btn-back {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  background: #ffffff;
  border: none;
  color: #2b4583;
  font-size: 1.08rem;
  font-weight: 500;
  padding: 8px 16px;
  border-radius: 8px;
  margin-bottom: 18px;
  cursor: pointer;
  transition: background 0.18s, color 0.18s;
}
.btn-back:hover {
  background: #e9ecef;
  color: #1a2b4c;
}
</style>