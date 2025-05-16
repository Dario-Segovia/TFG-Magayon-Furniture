<template>
  <div>
    <Bar :data="chartData" :options="chartOptions" />
  </div>
</template>

<script setup>
import { defineProps, ref, watch } from 'vue';
import { Bar } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js';

// Registrar los elementos de Chart.js necesarios
ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

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
      backgroundColor: 'rgba(75, 192, 192, 0.2)',
      borderColor: 'rgba(75, 192, 192, 1)',
      borderWidth: 1
    }
  ]
});

const chartOptions = ref({
  responsive: true,
  indexAxis: 'y',  // Esto hace que las barras sean horizontales
  scales: {
    x: {
      beginAtZero: true
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
        backgroundColor: 'rgba(75, 192, 192, 0.2)',
        borderColor: 'rgba(75, 192, 192, 1)',
        borderWidth: 1
      }
    ]
  };
});
</script>

<style scoped>
/* Agrega estilo si es necesario */
</style>
