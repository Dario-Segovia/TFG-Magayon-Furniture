<template>
  <div>
    <Pie :data="chartData" :options="chartOptions" />
  </div>
</template>

<script setup>
import { defineProps, ref, watch } from 'vue';
import { Pie } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, ArcElement, CategoryScale, LinearScale } from 'chart.js';

// Registrar los elementos de Chart.js necesarios
ChartJS.register(Title, Tooltip, Legend, ArcElement, CategoryScale, LinearScale);

const props = defineProps({
  labels: {
    type: Array,
    required: true
  },
  data: {
    type: Array,
    required: true
  }
});

const chartData = ref({
  labels: props.labels,
  datasets: [
    {
      label: 'Datos',
      data: props.data,
      backgroundColor: ['#FF6384', '#36A2EB', '#FFCE56'], // Colores para las secciones del pastel
      hoverOffset: 4
    }
  ]
});

const chartOptions = ref({
  responsive: true,
  plugins: {
    legend: {
      position: 'top'
    },
    tooltip: {
      callbacks: {
        label: function(tooltipItem) {
          return `${tooltipItem.label}: ${tooltipItem.raw} €`; // Formateo de tooltip
        }
      }
    }
  }
});

watch([props.labels, props.data], () => {
  chartData.value = {
    labels: props.labels,
    datasets: [
      {
        label: 'Datos',
        data: props.data,
        backgroundColor: ['#FF6384', '#36A2EB', '#FFCE56'],
        hoverOffset: 4
      }
    ]
  };
});
</script>

<style scoped>
/* Agrega estilo si es necesario */
</style>
