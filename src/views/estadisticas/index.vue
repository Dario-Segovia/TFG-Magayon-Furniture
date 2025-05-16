<template>
  <div class="estadisticas-container">
    <div class="header">
      <div class="header-content">
        <h1>
          <i class="fas fa-chart-bar"></i> Estadísticas
        </h1>
        <p class="subtitle">Visualiza el rendimiento de tu negocio</p>
      </div>
      <div class="filtros-container">
        <input type="date" v-model="fechaDesde" class="filtro-input" />
        <input type="date" v-model="fechaHasta" class="filtro-input" />
        <select v-model="tipoEstadistica" class="filtro-select">
          <option value="ventas">Ventas</option>
          <option value="compras">Compras</option>
        </select>
      </div>
    </div>

    <div class="indicadores-grid">
      <div class="indicador-card">
        <div class="indicador-titulo">Total Ventas</div>
        <div class="indicador-valor">{{ resumen.totalVentas }} €</div>
      </div>
      <div class="indicador-card">
        <div class="indicador-titulo">Total Compras</div>
        <div class="indicador-valor">{{ resumen.totalCompras }} €</div>
      </div>
      <div class="indicador-card">
        <div class="indicador-titulo">Clientes</div>
        <div class="indicador-valor">{{ resumen.totalClientes }}</div>
      </div>
      <div class="indicador-card">
        <div class="indicador-titulo">Productos</div>
        <div class="indicador-valor">{{ resumen.totalProductos }}</div>
      </div>
    </div>

    <div class="graficas-grid">
      <div class="grafica-card">
        <h3>Ventas/Compras por Mes</h3>
        <BarChart :labels="labelsMeses" :data="datosPorMes" />
      </div>
      <div class="grafica-card">
        <h3>Top Productos</h3>
        <BarChartHorizontal :labels="labelsTopProductos" :data="datosTopProductos" />
      </div>
      <div class="grafica-card">
        <h3>Distribución por Cliente/Proveedor</h3>
        <PieChart :labels="labelsPie" :data="datosPie" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed, watch } from 'vue';
// Puedes usar chart.js, apexcharts o vue-chart-3 para los componentes de gráficas
import BarChart from '../../components/BarChart.vue';


import BarChartHorizontal from '../../components/BarChartHorizontal.vue';
import PieChart from '../../components/PieChart.vue';
import { invoke } from '@tauri-apps/api/core';

const fechaDesde = ref('');
const fechaHasta = ref('');
const tipoEstadistica = ref('ventas');
const resumen = ref({
  totalVentas: 0,
  totalCompras: 0,
  totalClientes: 0,
  totalProductos: 0,
});
const labelsMeses = ref([]);
const datosPorMes = ref([]);
const labelsTopProductos = ref([]);
const datosTopProductos = ref([]);
const labelsPie = ref([]);
const datosPie = ref([]);

const cargarEstadisticas = async () => {
  // Llama a tus comandos de Tauri para obtener los datos agregados
  const data = await invoke('obtener_estadisticas', {
    desde: fechaDesde.value,
    hasta: fechaHasta.value,
    tipo: tipoEstadistica.value,
  });
  resumen.value = data.resumen;
  labelsMeses.value = data.labelsMeses;
  datosPorMes.value = data.datosPorMes;
  labelsTopProductos.value = data.labelsTopProductos;
  datosTopProductos.value = data.datosTopProductos;
  labelsPie.value = data.labelsPie;
  datosPie.value = data.datosPie;
};

onMounted(cargarEstadisticas);
watch([fechaDesde, fechaHasta, tipoEstadistica], cargarEstadisticas);
</script>

<style scoped>
.estadisticas-container {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 22px;
  padding: 22px;
  background: linear-gradient(135deg,#2b4583 ,  #d457c3);
  border-radius: 10px;
  color: white;
  box-shadow: 0 4px 12px rgba(0,0,0,0.10);
}
.header-content h1 {
  margin: 0;
  font-size: 1.7rem;
  display: flex;
  align-items: center;
  gap: 10px;
}
.subtitle {
  margin: 0;
  font-size: 1rem;
  color: #e0f7e9;
}
.filtros-container {
  display: flex;
  gap: 12px;
  align-items: center;
}
.filtro-input, .filtro-select {
  padding: 8px 12px;
  border-radius: 8px;
  border: 1px solid #ddd;
  font-size: 1rem;
}
.indicadores-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 18px;
  margin-bottom: 30px;
}
.indicador-card {
  background: #fff;
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  padding: 18px 14px;
  text-align: center;
}
.indicador-titulo {
  color: #888;
  font-size: 1em;
  margin-bottom: 6px;
}
.indicador-valor {
  font-size: 1.5em;
  font-weight: 700;
  color: #2b4583;
}
.graficas-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 24px;
}
.grafica-card {
  background: #fff;
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  padding: 18px 14px;
}
.grafica-card h3 {
  margin: 0 0 12px 0;
  font-size: 1.1em;
  color: #405890;
}
</style>
