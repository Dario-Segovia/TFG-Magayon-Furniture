<template>
  <!-- Al inicio del template de cada componente de estadísticas -->
<button class="btn-back" @click="$router.back()">
  <i class="fas fa-arrow-left"></i> Atrás
</button>
  <div class="dashboard-container">
    <div class="dashboard-header">
      <h1 class="dashboard-title">Estadísticas de Horarios</h1>
      <div class="header-divider"></div>
    </div>

    <div class="dashboard-content">
      <!-- KPI Card -->
      <div class="kpi-card">
        <div class="kpi-content">
          <h2 class="kpi-title">Total de Registros</h2>
          <p class="kpi-value">{{ resumen.total_registros }}</p>
          <div class="kpi-trend">
            <span class="trend-icon">⏱️</span>
            <span class="trend-text">Registros totales</span>
          </div>
        </div>
        <div class="kpi-icon">📋</div>
      </div>
      <div class="kpi-row">
        <div class="kpi-card small">
          <div class="kpi-content">
            <h2 class="kpi-title">Total de Horas</h2>
            <p class="kpi-value">{{ resumen.total_horas.toFixed(2) }} h</p>
            <div class="kpi-trend">
              <span class="trend-icon">🕒</span>
              <span class="trend-text">Horas trabajadas</span>
            </div>
          </div>
          <div class="kpi-icon">⏳</div>
        </div>
        <div class="kpi-card small">
          <div class="kpi-content">
            <h2 class="kpi-title">Empleados distintos</h2>
            <p class="kpi-value">{{ resumen.empleados_distintos }}</p>
            <div class="kpi-trend">
              <span class="trend-icon">👤</span>
              <span class="trend-text">Empleados únicos</span>
            </div>
          </div>
          <div class="kpi-icon">👥</div>
        </div>
      </div>

      <!-- Gráficos y tablas -->
      <div class="chart-row">
        <div class="chart-card">
          <div class="chart-header">
            <h2 class="chart-title">Horas trabajadas por empleado</h2>
            <div class="chart-actions">
              <button class="chart-action-btn" @click="exportarCSV(horasPorEmpleado, 'horas_por_empleado.csv')">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <BarChartHorizontal
              :labels="horasPorEmpleado.map(e => e.nombre)"
              :data="horasPorEmpleado.map(e => e.total_horas)"
            />
            <div class="table-container">
              <table class="data-table">
                <thead>
                  <tr>
                    <th>Empleado</th>
                    <th>Horas trabajadas</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="e in horasPorEmpleado" :key="e.nombre">
                    <td>{{ e.nombre }}</td>
                    <td>{{ e.total_horas.toFixed(2) }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <div class="chart-card">
          <div class="chart-header">
            <h2 class="chart-title">Turnos por tipo</h2>
            <div class="chart-actions">
              <button class="chart-action-btn" @click="exportarCSV(turnosPorTipo, 'turnos_por_tipo.csv')">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <PieChart
              :labels="turnosPorTipo.map(t => t.categoria)"
              :data="turnosPorTipo.map(t => t.total)"
            />
            <div class="table-container">
              <table class="data-table">
                <thead>
                  <tr>
                    <th>Tipo de turno</th>
                    <th>Cantidad</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="t in turnosPorTipo" :key="t.categoria">
                    <td>{{ t.categoria }}</td>
                    <td>{{ t.total }}</td>
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
            <h2 class="chart-title">Turnos por empleado y tipo</h2>
            <div class="chart-actions">
              <button class="chart-action-btn" @click="exportarCSV(turnosPorEmpleado, 'turnos_por_empleado.csv')">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <div class="table-container">
              <table class="data-table">
                <thead>
                  <tr>
                    <th>Empleado</th>
                    <th>Tipo de turno</th>
                    <th>Cantidad</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="item in turnosPorEmpleado" :key="item.nombre + '-' + item.tipo_turno">
                    <td>{{ item.nombre }}</td>
                    <td>{{ item.tipo_turno }}</td>
                    <td>{{ item.cantidad }}</td>
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
import BarChartHorizontal from '../../../components/BarChartHorizontal.vue'
import PieChart from '../../../components/PieChart.vue'

const resumen = ref({ total_registros: 0, total_horas: 0, empleados_distintos: 0 })
const horasPorEmpleado = ref([])
const turnosPorTipo = ref([])
const turnosPorEmpleado = ref([])

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
  resumen.value = await invoke('resumen_horarios')
  horasPorEmpleado.value = await invoke('horas_por_empleado')
  turnosPorTipo.value = await invoke('turnos_por_tipo')
  turnosPorEmpleado.value = await invoke('turnos_por_empleado')
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

/* KPI Cards */
.kpi-row {
  display: flex;
  gap: 24px;
  flex-wrap: wrap;
  margin-top: 16px;
}

.kpi-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: var(--white);
  border-radius: var(--border-radius);
  padding: 24px;
  box-shadow: var(--box-shadow);
  border-left: 4px solid var(--primary-color);
  margin-bottom: 0;
}

.kpi-card.small {
  flex: 1 1 200px;
  min-width: 220px;
  padding: 18px 24px;
  font-size: 0.95em;
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
  font-size: 32px;
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
  gap: 20px;
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
  margin-top: 10px;
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
  .kpi-row {
    flex-direction: column;
  }
  .kpi-card.small {
    min-width: 0;
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