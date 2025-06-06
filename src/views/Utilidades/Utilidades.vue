<template>
  <!-- Botón atrás -->
  <button class="btn-back" @click="goBack">
    <i class="fas fa-arrow-left"></i> Atrás
  </button>
  <div class="agenda-container">
    <div class="header">
      <div class="header-content">
        <h1>
          <i class="fas fa-calendar-alt"></i> Utilidades Profesionales
        </h1>
        <p class="subtitle">
          Herramientas, apuntes y agenda para la gestión diaria de tu empresa de sofás
        </p>
      </div>
    </div>

    <div class="dashboard-grid">
      <div class="dashboard-card card-btn" @click="modalAbierto = 'tareas'">
        <i class="fas fa-tasks card-icon"></i>
        <span>Tareas</span>
      </div>
      <div class="dashboard-card card-btn" @click="modalAbierto = 'eventos'">
        <i class="fas fa-calendar card-icon"></i>
        <span>Eventos</span>
      </div>
     
      <div class="dashboard-card card-btn" @click="modalAbierto = 'unidades'">
        <i class="fas fa-ruler-combined card-icon"></i>
        <span>Conversor de Unidades</span>
      </div>
      <div class="dashboard-card card-btn" @click="modalAbierto = 'calculadora'">
        <i class="fas fa-calculator card-icon"></i>
        <span>Calculadora Rápida</span>
      </div>
      <div class="dashboard-card card-btn" @click="modalAbierto = 'retales'">
        <i class="fas fa-cut card-icon"></i>
        <span>Calculadora de Retales</span>
      </div>
      <div class="dashboard-card card-btn" @click="modalAbierto = 'tapizado'">
        <i class="fas fa-couch card-icon"></i>
        <span>Coste de Tapizado</span>
      </div>
      <div class="dashboard-card card-btn" @click="modalAbierto = 'metros'">
        <i class="fas fa-ruler-horizontal card-icon"></i>
        <span>ML a M²</span>
      </div>
    </div>

    <!-- MODALES INDIVIDUALES -->
<ModalTareas
  v-if="modalAbierto === 'tareas'"
  :visible="modalAbierto === 'tareas'"
  :onClose="() => modalAbierto = null"
  :tareas="tareas"
  :nuevaTarea="nuevaTarea"
  :agregarTarea="agregarTarea"
  :eliminarTarea="eliminarTarea"
/>
<ModalEventos
  v-if="modalAbierto === 'eventos'"
  :visible="modalAbierto === 'eventos'"
  :onClose="() => modalAbierto = null"
  :eventosOrdenados="eventosOrdenados"
  v-model:nuevoEvento="nuevoEvento"
  :agregarEvento="agregarEvento"
  :eliminarEvento="eliminarEvento"
/>
<ModalApuntes
  v-if="modalAbierto === 'apuntes'"
  :visible="modalAbierto === 'apuntes'"
  :onClose="() => modalAbierto = null"
  v-model:busquedaApuntes="busquedaApuntes"
  v-model:nuevoApunte="nuevoApunte"
  :guardarApunte="guardarApunte"
  :apuntesFiltrados="apuntesFiltrados"
  :eliminarApunte="eliminarApunte"
/>
<ModalUnidades
  v-if="modalAbierto === 'unidades'"
  :visible="modalAbierto === 'unidades'"
  :onClose="() => modalAbierto = null"
  v-model:valorUnidad="valorUnidad"
  v-model:unidadOrigen="unidadOrigen"
  v-model:unidadDestino="unidadDestino"
  :unidades="unidades"
  :conversionValida="conversionValida"
  :valorConvertido="valorConvertido"
/>
<ModalCalculadora
  v-if="modalAbierto === 'calculadora'"
  :visible="modalAbierto === 'calculadora'"
  :onClose="() => modalAbierto = null"
  :expresion="expresion"
  :calcular="calcular"
  :resultadoCalculadora="resultadoCalculadora"
/>
<ModalRetales
  v-if="modalAbierto === 'retales'"
  :visible="modalAbierto === 'retales'"
  :onClose="() => modalAbierto = null"
  v-model:largoTotal="largoTotal"
  v-model:largoRetal="largoRetal"
  :retalesCalculados="retalesCalculados"
/>
<ModalTapizado
  v-if="modalAbierto === 'tapizado'"
  :visible="modalAbierto === 'tapizado'"
  :onClose="() => modalAbierto = null"
  v-model:metrosTela="metrosTela"
  v-model:precioTela="precioTela"
    
  v-model:manoObra="manoObra"
  :costeTapizado="costeTapizado"
/>
<ModalMetros
  v-if="modalAbierto === 'metros'"
  :visible="modalAbierto === 'metros'"
  :onClose="() => modalAbierto = null"
  v-model:metrosLineales="metrosLineales"
  v-model:anchoTela="anchoTela"
  :metrosCuadrados="metrosCuadrados"
/>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';
import { useRouter } from 'vue-router';
import ModalTareas from './ModalTareas.vue';
import ModalEventos from './ModalEventos.vue';

import ModalUnidades from './ModalUnidades.vue';
import ModalCalculadora from './ModalCalculadora.vue';
import ModalRetales from './ModalRetales.vue';
import ModalTapizado from './ModalTapizado.vue';
import ModalMetros from './ModalMetros.vue';

// Variables de visibilidad para cada modal
const showTareas = ref(false);
const showEventos = ref(false);
const showApuntes = ref(false);
const showUnidades = ref(false);
const showCalculadora = ref(false);
const showRetales = ref(false);
const showTapizado = ref(false);
const showMetros = ref(false);
const modalAbierto = ref(null); // Puede ser: 'tareas', 'eventos', 'apuntes', 'unidades', 'calculadora', 'retales', 'tapizado', 'metros'

// --- Agenda de tareas ---
const tareas = ref(JSON.parse(localStorage.getItem('tareas_sofa') || '[]'));
const nuevaTarea = ref('');
function agregarTarea(texto) {
  tareas.value.push({ id: Date.now() + Math.random(), texto, completada: false });
  guardarTareas();
}

function eliminarTarea(idx) {
  tareas.value.splice(idx, 1);
  guardarTareas();
}
function guardarTareas() {
  localStorage.setItem('tareas_sofa', JSON.stringify(tareas.value));
}

// --- Agenda de eventos ---
const eventos = ref(JSON.parse(localStorage.getItem('eventos_sofa') || '[]'));
const nuevoEvento = ref({ titulo: '', fecha: '' });
function agregarEvento() {
  if (nuevoEvento.value && nuevoEvento.value.titulo && nuevoEvento.value.fecha) {
    eventos.value.push({ ...nuevoEvento.value });
    // En vez de reasignar el objeto, resetea sus propiedades:
    nuevoEvento.value.titulo = '';
    nuevoEvento.value.fecha = '';
    guardarEventos();
  }
}
function eliminarEvento(idx) {
  eventos.value.splice(idx, 1);
  guardarEventos();
}
function guardarEventos() {
  localStorage.setItem('eventos_sofa', JSON.stringify(eventos.value));
}
const eventosOrdenados = computed(() =>
  [...eventos.value].sort((a, b) => a.fecha.localeCompare(b.fecha))
);

// --- Apuntes rápidos con historial y búsqueda ---
const apuntes = ref(JSON.parse(localStorage.getItem('apuntes_sofa_hist') || '[]'));
const nuevoApunte = ref('');
const busquedaApuntes = ref('');
function guardarApunte() {
  if (nuevoApunte.value.trim()) {
    apuntes.value.unshift({
      texto: nuevoApunte.value,
      fecha: new Date().toLocaleString('es-ES')
    });
    nuevoApunte.value = '';
    guardarApuntes();
  }
}
function eliminarApunte(idx) {
  apuntes.value.splice(idx, 1);
  guardarApuntes();
}
function guardarApuntes() {
  localStorage.setItem('apuntes_sofa_hist', JSON.stringify(apuntes.value));
}
const apuntesFiltrados = computed(() =>
  apuntes.value.filter(a =>
    a.texto.toLowerCase().includes(busquedaApuntes.value.toLowerCase())
  )
);

// --- Conversor de unidades ---
const unidades = [
  'mm', 'cm', 'm', 'pulgadas', 'pies', 'yardas'
];
const valorUnidad = ref(1);
const unidadOrigen = ref('cm');
const unidadDestino = ref('m');

const conversionValida = computed(() =>
  unidades.includes(unidadOrigen.value) &&
  unidades.includes(unidadDestino.value) &&
  valorUnidad.value !== null &&
  valorUnidad.value !== '' &&
  !isNaN(valorUnidad.value)
);

const conversiones = {
  mm: { mm: 1, cm: 0.1, m: 0.001, pulgadas: 0.0393701, pies: 0.00328084, yardas: 0.00109361 },
  cm: { mm: 10, cm: 1, m: 0.01, pulgadas: 0.393701, pies: 0.0328084, yardas: 0.0109361 },
  m: { mm: 1000, cm: 100, m: 1, pulgadas: 39.3701, pies: 3.28084, yardas: 1.09361 },
  pulgadas: { mm: 25.4, cm: 2.54, m: 0.0254, pulgadas: 1, pies: 0.0833333, yardas: 0.0277778 },
  pies: { mm: 304.8, cm: 30.48, m: 0.3048, pulgadas: 12, pies: 1, yardas: 0.333333 },
  yardas: { mm: 914.4, cm: 91.44, m: 0.9144, pulgadas: 36, pies: 3, yardas: 1 }
};

const valorConvertido = computed(() => {
  if (!conversionValida.value) return '';
  const origen = unidadOrigen.value;
  const destino = unidadDestino.value;
  const valor = parseFloat(valorUnidad.value);
  if (isNaN(valor)) return '';
  if (conversiones[origen] && conversiones[origen][destino]) {
    return (valor * conversiones[origen][destino]).toFixed(4);
  }
  // Si origen y destino son iguales, devuelve el mismo valor
  if (origen === destino) return valor.toFixed(4);
  return '';
});

// --- Calculadora rápida ---
const expresion = ref('');
const resultadoCalculadora = ref(null);
function calcular() {
  try {
    if (!/^[\d+\-*/().\s]+$/.test(expresion.value)) throw new Error('Expresión inválida');
    // eslint-disable-next-line no-eval
    resultadoCalculadora.value = eval(expresion.value);
  } catch {
    resultadoCalculadora.value = 'Error';
  }
}

// --- Calculadora de retales ---
const largoTotal = ref(null);
const largoRetal = ref(null);
const retalesCalculados = computed(() => {
  if (largoTotal.value > 0 && largoRetal.value > 0) {
    return Math.floor(largoTotal.value / largoRetal.value);
  }
  return null;
});

// --- Calculadora de tapizado ---
const metrosTela = ref(0);
const precioTela = ref(0);
const manoObra = ref(0);

const costeTapizado = computed(() => {
  const tela = Number(metrosTela.value) * Number(precioTela.value);
  const mano = Number(manoObra.value);
  if (isNaN(tela) || isNaN(mano)) return 0;
  return tela + mano;
});

// --- Conversor metros lineales a metros cuadrados ---
const metrosLineales = ref(null);
const anchoTela = ref(null);
const metrosCuadrados = computed(() => {
  if (metrosLineales.value > 0 && anchoTela.value > 0) {
    return (metrosLineales.value * anchoTela.value) / 100;
  }
  return null;
});

const router = useRouter();
function goBack() {
  router.back();
}
</script>

<style scoped>
.agenda-container {
  padding: 20px;
  max-width: 1400px;
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
.header-content {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.header-content h1 {
  margin: 0;
  font-size: 2rem;
  display: flex;
  align-items: center;
  gap: 10px;
}
.subtitle {
  margin: 0;
  font-size: 1.1rem;
  color: #e0f7e9;
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
.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 28px;
}
.dashboard-card {
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  padding: 0;
  display: flex;
  flex-direction: column;
  min-height: 220px;
  overflow: hidden;
}
.dashboard-card.card-btn {
  justify-content: center;
  align-items: center;
  cursor: pointer;
  min-height: 140px;
  text-align: center;
  transition: box-shadow 0.2s, transform 0.2s;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.dashboard-card.card-btn:hover {
  box-shadow: 0 4px 16px rgba(52,152,219,0.15);
  transform: translateY(-2px) scale(1.03);
  background: #f8f9fa;
}
.dashboard-card details {
  width: 100%;
}
.dashboard-card summary {
  font-size: 1.15em;
  font-weight: 600;
  color: #405890;
  padding: 18px 18px 12px 18px;
  cursor: pointer;
  outline: none;
  display: flex;
  align-items: center;
  gap: 8px;
  border-bottom: 1px solid #f0f0f0;
  background: #f8f9fa;
  border-radius: 12px 12px 0 0;
  transition: background 0.2s;
}
.dashboard-card details[open] summary {
  background: #e9ecef;
}
.card-content {
  padding: 16px 18px 18px 18px;
  max-height: 320px;
  overflow-y: auto;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.tareas-lista, .apuntes-lista, .eventos-lista {
  max-height: 120px;
  overflow-y: auto;
}
.form-group {
  display: flex;
  gap: 10px;
  align-items: center;
}
.form-input, .form-select, .form-textarea {
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 1rem;
  transition: border-color 0.2s, box-shadow 0.2s;
}
.form-input:focus, .form-select:focus, .form-textarea:focus {
  outline: none;
  border-color: #3498db;
  box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1);
}
.arrow {
  font-size: 1.5em;
  color: #888;
}
.resultado-conversion, .resultado-calculadora, .resultado-retales, .resultado-tapizado, .resultado-metros {
  font-size: 1.1em;
  font-weight: 600;
  color: #2b4583;
  margin-top: 8px;
}
.error-text {
  color: #e74c3c;
}
.success-text {
  color: #27ae60;
  margin-left: 10px;
}
.btn {
  padding: 10px 18px;
  border-radius: 8px;
  font-weight: 500;
  cursor: pointer;
  border: none;
  margin-top: 8px;
}
.btn-primary {
  background-color: #3498db;
  color: white;
}
.btn-primary:hover {
  background-color: #2980b9;
}
.btn-secondary {
  background-color: #f8f9fa;
  color: #34495e;
}
.btn-secondary:hover {
  background-color: #e9ecef;
}
.btn-delete {
  background: none;
  color: #e74c3c;
  font-size: 1.1em;
  margin-left: 8px;
  border: none;
  cursor: pointer;
}
.form-textarea {
  width: 100%;
  min-height: 60px;
  resize: vertical;
}
.tareas-lista, .apuntes-lista, .eventos-lista {
  list-style: none;
  padding: 0;
  margin: 0;
  max-height: 160px;
  overflow-y: auto;
}
.tareas-lista li, .apuntes-lista li, .eventos-lista li {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 6px 0;
  border-bottom: 1px solid #f0f0f0;
}
.tareas-lista li.completada span {
  text-decoration: line-through;
  color: #aaa;
}
.evento-fecha {
  color: #405890;
  font-weight: 600;
  min-width: 90px;
}
.evento-titulo {
  flex: 1;
}
.apunte-fecha {
  color: #888;
  font-size: 0.95em;
  min-width: 110px;
}
.apunte-texto {
  flex: 1;
}
.tareas-input, .eventos-input {
  display: flex;
  gap: 8px;
  margin-bottom: 8px;
}
.dashboard-card.card-btn {
  justify-content: center;
  align-items: center;
  cursor: pointer;
  min-height: 140px;
  text-align: center;
  transition: box-shadow 0.2s, transform 0.2s;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.dashboard-card.card-btn:hover {
  box-shadow: 0 4px 16px rgba(52,152,219,0.15);
  transform: translateY(-2px) scale(1.03);
  background: #f8f9fa;
}
.card-icon {
  font-size: 2.5rem;
  color: #2b4583;
}
</style>