<template>
  <!-- Al inicio del template de cada componente de estadísticas -->
<button class="btn-back" @click="$router.back()">
  <i class="fas fa-arrow-left"></i> Atrás
</button>
  <div class="dashboard-container">
    <div class="dashboard-header">
      <h1 class="dashboard-title">Estadísticas de Clientes</h1>
      <div class="header-divider"></div>
    </div>

    <div class="dashboard-content">
      <!-- KPI Card -->
      <div class="kpi-card">
        <div class="kpi-content">
          <h2 class="kpi-title">Total de Clientes</h2>
          <p class="kpi-value">{{ formatNumber(totalClientes) }}</p>
          <div class="kpi-trend">
            <span class="trend-icon">📈</span>
            <span class="trend-text">Visión general</span>
          </div>
        </div>
        <div class="kpi-icon">👥</div>
      </div>

      <!-- Gráficos -->
      <div class="chart-row">
        <div class="chart-card">
          <div class="chart-header">
            <h2 class="chart-title">Clientes por País</h2>
            <div class="chart-actions">
              <button class="chart-action-btn">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <BarChartHorizontal
              :labels="clientesPorPais.map(p => p.categoria)"
              :data="clientesPorPais.map(c => c.total)"
              :colors="['#4e79a7', '#f28e2b', '#e15759', '#76b7b2', '#59a14f']"
            />
          </div>
        </div>

        <div class="chart-card">
          <div class="chart-header">
            <h2 class="chart-title">Clientes por Ciudad</h2>
            <div class="chart-actions">
              <button class="chart-action-btn">Exportar</button>
            </div>
          </div>
          <div class="chart-container">
            <BarChartHorizontal
              :labels="clientesPorCiudad.map(c => c.categoria)"
              :data="clientesPorCiudad.map(c => c.total)"
              :colors="['#4e79a7', '#f28e2b', '#e15759', '#76b7b2', '#59a14f']"
            />
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

const totalClientes = ref(0)
const clientesPorPais = ref([])
const clientesPorCiudad = ref([])

const formatNumber = (num) => {
  return new Intl.NumberFormat('es-ES').format(num)
}

onMounted(async () => {
  const total = await invoke('obtener_total_clientes')
  totalClientes.value = total.total
  clientesPorPais.value = await invoke('clientes_por_pais')
  clientesPorCiudad.value = await invoke('clientes_por_ciudad')
})
</script>

<style scoped>
/* Estilos base */
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

/* Estilos para la tarjeta KPI */
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

/* Estilos para los gráficos */
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

/* Responsive */
@media (max-width: 768px) {
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