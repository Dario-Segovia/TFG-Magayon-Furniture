<template>
  <!-- Al inicio del template de cada componente de estadísticas -->
<button class="btn-back" @click="$router.back()">
  <i class="fas fa-arrow-left"></i> Atrás
</button>
  <div class="dashboard-container">
    <div class="dashboard-header">
      <h1 class="dashboard-title">Estadísticas de Proveedores</h1>
      <div class="header-divider"></div>
    </div>

    <div class="dashboard-content">
      <!-- KPI Card -->
      <div class="kpi-card">
        <div class="kpi-content">
          <h2 class="kpi-title">Total de Proveedores</h2>
          <p class="kpi-value">{{ resumen.total }}</p>
          <div class="kpi-trend">
            <span class="trend-icon">🏢</span>
            <span class="trend-text">Visión general</span>
          </div>
        </div>
        <div class="kpi-icon">🤝</div>
      </div>

      <div class="kpi-row">
        <div class="kpi-card small">
          <div class="kpi-content">
            <h2 class="kpi-title">Activos</h2>
            <p class="kpi-value">{{ resumen.activos }}</p>
            <div class="kpi-trend">
              <span class="trend-icon">✅</span>
              <span class="trend-text">Proveedores activos</span>
            </div>
          </div>
          <div class="kpi-icon">✔️</div>
        </div>
        <div class="kpi-card small">
          <div class="kpi-content">
            <h2 class="kpi-title">Con contrato vigente</h2>
            <p class="kpi-value">{{ resumen.con_contrato }}</p>
            <div class="kpi-trend">
              <span class="trend-icon">📄</span>
              <span class="trend-text">Contratos vigentes</span>
            </div>
          </div>
          <div class="kpi-icon">📑</div>
        </div>
        <div class="kpi-card small">
          <div class="kpi-content">
            <h2 class="kpi-title">Países diferentes</h2>
            <p class="kpi-value">{{ resumen.paises }}</p>
            <div class="kpi-trend">
              <span class="trend-icon">🌍</span>
              <span class="trend-text">Diversidad geográfica</span>
            </div>
          </div>
          <div class="kpi-icon">🌎</div>
        </div>
      </div>

      <!-- Gráficos y tablas -->
      <div class="chart-row">
        <div class="chart-card">
          <div class="chart-header">
            <h2 class="chart-title">Proveedores por país</h2>
            <div class="chart-actions">
              <button class="chart-action-btn" @click="exportarCSV(proveedoresPorPais, 'proveedores_por_pais.csv')">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <PieChart
              :labels="proveedoresPorPais.map(p => p.pais)"
              :data="proveedoresPorPais.map(p => p.total)"
            />
            <div class="table-container">
              <table class="data-table">
                <thead>
                  <tr>
                    <th>País</th>
                    <th>Cantidad</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="p in proveedoresPorPais" :key="p.pais">
                    <td>{{ p.pais }}</td>
                    <td>{{ p.total }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <div class="chart-card">
          <div class="chart-header">
            <h2 class="chart-title">Nº de productos por proveedor</h2>
            <div class="chart-actions">
              <button class="chart-action-btn" @click="exportarCSV(productosPorProveedor, 'productos_por_proveedor.csv')">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <BarChartHorizontal
              :labels="productosPorProveedor.map(p => p.proveedor)"
              :data="productosPorProveedor.map(p => p.total_productos)"
            />
            <div class="table-container">
              <table class="data-table">
                <thead>
                  <tr>
                    <th>Proveedor</th>
                    <th>Nº Productos</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="p in productosPorProveedor" :key="p.proveedor">
                    <td>{{ p.proveedor }}</td>
                    <td>{{ p.total_productos }}</td>
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
            <h2 class="chart-title">Producto más caro por proveedor</h2>
            <div class="chart-actions">
              <button class="chart-action-btn" @click="exportarCSV(productosMasCarosPorProveedor, 'producto_mas_caro_por_proveedor.csv')">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <div class="table-container">
              <table class="data-table">
                <thead>
                  <tr>
                    <th>Proveedor</th>
                    <th>Producto</th>
                    <th>Precio unitario</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="item in productosMasCarosPorProveedor" :key="item.proveedor + '-' + item.producto">
                    <td>{{ item.proveedor }}</td>
                    <td>{{ item.producto }}</td>
                    <td>{{ item.precio_unitario.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}</td>
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
import BarChartHorizontal from '../../../components/BarChartHorizontal.vue'

const resumen = ref({ total: 0, activos: 0, con_contrato: 0, paises: 0 })
const proveedoresPorPais = ref([])
const productosPorProveedor = ref([])
const productosMasCarosPorProveedor = ref([])

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
  resumen.value = await invoke('proveedores_resumen')
  proveedoresPorPais.value = await invoke('proveedores_por_pais')
  productosPorProveedor.value = await invoke('productos_por_proveedor')
  productosMasCarosPorProveedor.value = await invoke('productos_mas_caros_por_proveedor')
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
  margin-bottom: 16px;
}

.kpi-card {
  flex: 1 1 200px;
  min-width: 220px;
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
  min-width: 180px;
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
  height: 450px;
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