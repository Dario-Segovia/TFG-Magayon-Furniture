<template>
  <div class="pie-container">
    <Pie :data="chartData" :options="chartOptions" />
  </div>
</template>

<script setup>
import { defineProps, ref, watch } from 'vue';
import { Pie } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, ArcElement } from 'chart.js';

// Registrar los elementos de Chart.js necesarios
ChartJS.register(Title, Tooltip, Legend, ArcElement);

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

const defaultColors = [
  '#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF', '#FF9F40',
  '#8BC34A', '#F44336', '#00BCD4', '#E91E63', '#607D8B', '#795548'
];

const chartData = ref({
  labels: props.labels,
  datasets: [
    {
      label: 'Horas trabajadas',
      data: props.data,
      backgroundColor: props.labels.map((_, i) => defaultColors[i % defaultColors.length]),
      hoverOffset: 4
    }
  ]
});




const chartOptions = ref({
  responsive: true,
  plugins: {
    legend: {
      position: 'bottom'
    }
  }
});

watch([() => props.labels, () => props.data], () => {
  chartData.value = {
    labels: props.labels,
    datasets: [
      {
        label: 'Horas trabajadas',
        data: props.data,
        backgroundColor: props.labels.map((_, i) => defaultColors[i % defaultColors.length]),
        hoverOffset: 4
      }
    ]
  };
});
</script>

<style scoped>
.pie-container {
  width: 400px;
  height: 400px;
  margin: 0 auto; /* esto centra horizontalmente */
}


</style>
