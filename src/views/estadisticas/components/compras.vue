<template>
  <!-- Al inicio del template de cada componente de estadísticas -->
<button class="btn-back" @click="$router.back()">
  <i class="fas fa-arrow-left"></i> Atrás
</button>
  <div class="dashboard-container">
    <div class="dashboard-header">
      <h1 class="dashboard-title">Estadísticas de Compras</h1>
      <div class="header-divider"></div>
    </div>

    <div class="dashboard-content">
      <!-- KPI Card -->
      <div class="kpi-card">
        <div class="kpi-content">
          <h2 class="kpi-title">Total de Compras</h2>
          <p class="kpi-value">{{ formatNumber(resumen.total_compras) }}</p>
          <div class="kpi-trend">
            <span class="trend-icon">💸</span>
            <span class="trend-text">Visión general</span>
          </div>
        </div>
        <div class="kpi-icon">🛒</div>
      </div>

      <!-- Gráficos -->
      <div class="chart-row">
        <div class="chart-card">
          <div class="chart-header">
            <h2 class="chart-title">Compras por Mes</h2>
            <div class="chart-actions">
              <button class="chart-action-btn" @click="exportarCSV(comprasPorMes, 'compras_por_mes.csv')">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <BarChart
              :labels="comprasPorMes.map(c => c.mes)"
              :datasets="[{ label: 'Compras', data: comprasPorMes.map(c => c.total), backgroundColor: '#e64a19' }]"
            />
          </div>
        </div>

        <div class="chart-card">
          <div class="chart-header">
            <h2 class="chart-title">Productos más Comprados</h2>
            <div class="chart-actions">
              <button class="chart-action-btn" @click="exportarCSV(productosMasComprados, 'productos_mas_comprados.csv')">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <BarChartHorizontal
              :labels="productosMasComprados.map(p => p.nombre)"
              :data="productosMasComprados.map(p => p.cantidad)"
            />
          </div>
        </div>
      </div>

      <!-- Tablas de datos -->
      <div class="data-row">
        <div class="data-card">
          <h2 class="data-title">Resumen</h2>
          <table class="data-table">
            <tr>
              <th>Total de compras</th>
              <td>{{ resumen.total_compras }}</td>
            </tr>
            <tr>
              <th>Total gastado</th>
              <td>{{ resumen.total_gastado.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}</td>
            </tr>
            <tr>
              <th>Proveedores distintos</th>
              <td>{{ resumen.proveedores_distintos }}</td>
            </tr>
          </table>
        </div>
        <div class="data-card">
          <h2 class="data-title">Proveedores Top</h2>
          <div class="data-actions">
            <button class="chart-action-btn" @click="exportarCSV(proveedoresTop, 'proveedores_top.csv')">Exportar</button>
          </div>
          <table class="data-table">
            <thead>
              <tr>
                <th>Proveedor</th>
                <th>Total pagado</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="item in proveedoresTop" :key="item.nombre">
                <td>{{ item.nombre }}</td>
                <td>{{ item.total_pagado.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import BarChart from '../../../components/BarChart.vue'
import BarChartHorizontal from '../../../components/BarChartHorizontal.vue'

const resumen = ref({ total_compras: 0, total_gastado: 0, proveedores_distintos: 0 })
const comprasPorMes = ref([])
const productosMasComprados = ref([])
const proveedoresTop = ref([])

const formatNumber = (num) => {
  return new Intl.NumberFormat('es-ES').format(num)
}

function exportarCSV(data, filename) {
  if (!data || !data.length) return;
  const replacer = (key, value) => value === null ? '' : value;
  const header = Object.keys(data[0]);
  const csv = [
    header.join(';'),
    ...data.map(row => header.map(fieldName => JSON.stringify(row[fieldName], replacer)).join(';'))
  ].join('\r\n');

  const blob = new Blob([csv], { type: 'text/csv' });
  const link = document.createElement('a');
  link.href = URL.createObjectURL(blob);
  link.download = filename;
  link.click();
  URL.revokeObjectURL(link.href);
}

onMounted(async () => {
  resumen.value = await invoke('compras_resumen')
  comprasPorMes.value = await invoke('compras_por_mes')
  productosMasComprados.value = await invoke('productos_mas_comprados')
  proveedoresTop.value = await invoke('proveedores_top')
})
</script>

<style scoped>
:root {
  --primary-color: #e64a19;
  --secondary-color: #2c3e50;
  --accent-color: #1976d2;
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

/* Gráficos */
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
  height: 400px;
}

/* Tablas de datos */
.data-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(420px, 1fr));
  gap: 24px;
}

.data-card {
  background-color: var(--white);
  border-radius: var(--border-radius);
  box-shadow: var(--box-shadow);
  padding: 24px;
}

.data-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--secondary-color);
  margin-bottom: 12px;
}

.data-actions {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 8px;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 15px;
}

.data-table th,
.data-table td {
  padding: 8px 12px;
  border-bottom: 1px solid var(--light-gray);
  text-align: left;
}

.data-table th {
  background-color: var(--light-gray);
  color: var(--secondary-color);
  font-weight: 500;
}

.data-table tr:last-child td {
  border-bottom: none;
}

/* Responsive */
@media (max-width: 768px) {
  .chart-row, .data-row {
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