<template>
  <div style="padding: 12px;">
    <!-- Botón atrás -->
    <button class="btn-back" @click="goBack">
      <i class="fas fa-arrow-left"></i> {{ $t('estadisticas.back') }}
    </button>
    <h1>{{ $t('estadisticas.resumen') }}</h1>
    <div class="kpi-row">
      <div class="kpi-card">
        <div class="kpi-label">{{ $t('estadisticas.beneficio_neto') }}</div>
        <div class="kpi-value" :class="beneficioNeto >= 0 ? 'positivo' : 'negativo'">
          {{ beneficioNeto.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}
        </div>
        <div class="kpi-sub">{{ $t('estadisticas.ingresos') }}: {{ ingresosTotales.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}</div>
        <div class="kpi-sub">{{ $t('estadisticas.costes') }}: {{ costesTotales.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}</div>
      </div>
      <div class="kpi-card">
        <div class="kpi-label">{{ $t('estadisticas.mayor_venta') }}</div>
        <div class="kpi-value">{{ mayorVenta.total.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}</div>
        <div class="kpi-sub">{{ mayorVenta.mes }}</div>
      </div>
      <div class="kpi-card">
        <div class="kpi-label">{{ $t('estadisticas.mayor_compra') }}</div>
        <div class="kpi-value">{{ mayorCompra.total.toLocaleString('es-ES', { style: 'currency', currency: 'EUR' }) }}</div>
        <div class="kpi-sub">{{ mayorCompra.mes }}</div>
      </div>
      <div class="kpi-card">
        <div class="kpi-label">{{ $t('estadisticas.empleado_mes') }}</div>
        <div class="kpi-value">{{ empleadoMes.nombre }}</div>
        <div class="kpi-sub">{{ $t('estadisticas.horas_trabajadas') }}: {{ empleadoMes.total_horas?.toFixed(2) ?? 0 }}</div>
      </div>
    </div>

    <div class="charts-row">
      <div class="chart-box">
        <h3>{{ $t('estadisticas.ingresos_vs_costes') }}</h3>
        <div v-if="labelsMeses.length === 0">
          <em>{{ $t('estadisticas.no_datos_grafico') }}</em>
        </div>
        <BarChart
          v-else
          :labels="labelsMeses"
          :datasets="[
            { label: $t('estadisticas.ingresos'), data: datosVentasMes, backgroundColor: 'rgba(46, 125, 50, 0.7)' },
            { label: $t('estadisticas.costes'), data: datosComprasMes, backgroundColor: 'rgba(198, 40, 40, 0.7)' }
          ]"
        />
      </div>
      <div class="chart-box">
        <h3>{{ $t('estadisticas.distribucion_horas') }}</h3>
        <PieChart
          :labels="labelsEmpleados"
          :data="datosHorasEmpleados"
        />
      </div>
    </div>

    <div class="nav-grid">
      <router-link
        v-for="area in navAreas"
        :key="area.key"
        :to="area.to"
        class="nav-card"
        :style="{ '--card-color': area.color }"
      >
        <div class="nav-icon">
          <span :class="area.icon"></span>
        </div>
        <div class="nav-title">{{ $t(`estadisticas.${area.key}`) }}</div>
        <div class="nav-desc">{{ $t(`estadisticas.desc_${area.key}`) }}</div>
      </router-link>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import BarChart from '../../Components/BarChart.vue'
import PieChart from '../../Components/PieChart.vue'

const router = useRouter()
function goBack() {
  router.back()
}

const ingresosTotales = ref(0)
const costesTotales = ref(0)
const beneficioNeto = ref(0)
const mayorVenta = ref({ mes: '', total: 0 })
const mayorCompra = ref({ mes: '', total: 0 })
const empleadoMes = ref({ nombre: '', total_horas: 0 })

const labelsMeses = ref([])
const datosVentasMes = ref([])
const datosComprasMes = ref([])

const labelsEmpleados = ref([])
const datosHorasEmpleados = ref([])

const navAreas = [
  {
    key: 'clientes',
    to: '/estadisticas/clientes',
    title: 'Clientes',
    desc: 'Análisis de clientes y su distribución.',
    icon: 'mdi mdi-account-group',
    color: '#1976d2'
  },
  {
    key: 'empleados',
    to: '/estadisticas/empleados',
    title: 'Empleados',
    desc: 'Estadísticas de personal y puestos.',
    icon: 'mdi mdi-account-tie',
    color: '#388e3c'
  },
  {
    key: 'ventas',
    to: '/estadisticas/ventas',
    title: 'Ventas',
    desc: 'Evolución y productos más vendidos.',
    icon: 'mdi mdi-cash-register',
    color: '#fbc02d'
  },
  {
    key: 'compras',
    to: '/estadisticas/compras',
    title: 'Compras',
    desc: 'Histórico y proveedores principales.',
    icon: 'mdi mdi-cart-arrow-down',
    color: '#e64a19'
  },
  {
    key: 'horarios',
    to: '/estadisticas/horarios',
    title: 'Horarios',
    desc: 'Control y análisis de horas trabajadas.',
    icon: 'mdi mdi-clock-outline',
    color: '#7b1fa2'
  },
  {
    key: 'inventario',
    to: '/estadisticas/inventario',
    title: 'Inventario',
    desc: 'Stock, categorías y productos críticos.',
    icon: 'mdi mdi-warehouse',
    color: '#0288d1'
  },
  {
    key: 'proveedores',
    to: '/estadisticas/proveedores',
    title: 'Proveedores',
    desc: 'Relación y productos por proveedor.',
    icon: 'mdi mdi-truck',
    color: '#c2185b'
  }
]

onMounted(async () => {
  // Costes vs Beneficios
  const ventas = await invoke('ventas_resumen')
  const compras = await invoke('compras_resumen')
  ingresosTotales.value = ventas.total_ingresos ?? 0
  costesTotales.value = compras.total_gastado ?? 0
  beneficioNeto.value = ingresosTotales.value - costesTotales.value

  // Mayor venta
  const ventasMes = await invoke('ventas_por_mes')
  // Mayor compra
  const comprasMes = await invoke('compras_por_mes')

  // Unifica meses para los gráficos
  const mesesSet = new Set([
    ...ventasMes.map(v => v.mes),
    ...comprasMes.map(c => c.mes)
  ]);
  const mesesUnificados = Array.from(mesesSet).sort();

  labelsMeses.value = mesesUnificados;
  datosVentasMes.value = mesesUnificados.map(mes => {
    const found = ventasMes.find(v => v.mes === mes);
    return found ? found.total : 0;
  });
  datosComprasMes.value = mesesUnificados.map(mes => {
    const found = comprasMes.find(c => c.mes === mes);
    return found ? found.total : 0;
  });

  if (ventasMes.length > 0) {
    const mayor = ventasMes.reduce((a, b) => (a.total > b.total ? a : b))
    mayorVenta.value = mayor
  }
  if (comprasMes.length > 0) {
    const mayor = comprasMes.reduce((a, b) => (a.total > b.total ? a : b))
    mayorCompra.value = mayor
  }

  // Empleado del mes (más horas)
  const horas = await invoke('horas_por_empleado')
  if (horas.length > 0) {
    empleadoMes.value = horas[0]
    labelsEmpleados.value = horas.map(e => e.nombre)
    datosHorasEmpleados.value = horas.map(e => e.total_horas)
  }

  console.log('labelsMeses', labelsMeses.value)
  console.log('datosVentasMes', datosVentasMes.value)
  console.log('datosComprasMes', datosComprasMes.value)
  console.log('labelsEmpleados', labelsEmpleados.value)
  console.log('datosHorasEmpleados', datosHorasEmpleados.value)
})
</script>

<style scoped>
.kpi-row {
  display: flex;
  gap: 24px;
  margin-bottom: 32px;
  flex-wrap: wrap;
}
.kpi-card {
  background: #f8f8f8;
  border-radius: 8px;
  padding: 18px 32px;
  min-width: 220px;
  box-shadow: 0 2px 8px #0001;
  text-align: center;
  flex: 1 1 220px;
}
.kpi-label {
  font-size: 1.1em;
  color: #555;
}
.kpi-value {
  font-size: 2em;
  font-weight: bold;
  color: #222;
}
.kpi-value.positivo { color: #2e7d32; }
.kpi-value.negativo { color: #c62828; }
.kpi-sub {
  font-size: 0.95em;
  color: #888;
  margin-top: 4px;
}
.charts-row {
  display: flex;
  gap: 32px;
  margin-bottom: 32px;
  flex-wrap: wrap;
}
.chart-box {
  flex: 1 1 350px;
  min-width: 320px;
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 8px #0001;
  padding: 16px;
}
.nav-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 28px;
  margin-bottom: 32px;
}
.nav-card {
  background: var(--card-color, #1976d2);
  color: #fff;
  border-radius: 14px;
  box-shadow: 0 4px 16px #0002;
  padding: 28px 20px 20px 20px;
  text-decoration: none;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  position: relative;
  transition: transform 0.15s, box-shadow 0.15s, background 0.2s;
  cursor: pointer;
  min-height: 170px;
  overflow: hidden;
}
.nav-card:hover {
  transform: translateY(-6px) scale(1.03);
  box-shadow: 0 8px 32px #0003;
  background: linear-gradient(120deg, var(--card-color) 80%, #fff2 100%);
}
.nav-icon {
  font-size: 2.5em;
  margin-bottom: 12px;
  opacity: 0.92;
  filter: drop-shadow(0 2px 2px #0003);
}
.nav-title {
  font-size: 1.25em;
  font-weight: 600;
  margin-bottom: 6px;
  letter-spacing: 0.5px;
}
.nav-desc {
  font-size: 1em;
  opacity: 0.92;
  margin-bottom: 0;
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

<!--
Requiere Material Design Icons (mdi) en tu proyecto.
Puedes añadir en tu index.html:
<link href="https://cdn.jsdelivr.net/npm/@mdi/font/css/materialdesignicons.min.css" rel="stylesheet">
-->