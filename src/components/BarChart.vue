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
  datasets: {
    type: Array,
    required: true
  }
});

const chartData = ref({
  labels: props.labels,
  datasets: props.datasets
});

const chartOptions = ref({
  responsive: true,
  scales: {
    y: {
      beginAtZero: true
    }
  }
});

watch([() => props.labels, () => props.datasets], () => {
  chartData.value = {
    labels: props.labels,
    datasets: props.datasets
  };
});
</script>

<style scoped>
.pie-container {
  width: 300px;
  height: 300px;
  position: relative;
}

</style>
