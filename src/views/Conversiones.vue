<template>
  <div class="agenda-container">
    <div class="header">
      <div class="header-content">
        <h1>
          <i class="fas fa-calendar-alt"></i> Agenda y Utilidades Profesionales
        </h1>
        <p class="subtitle">
          Herramientas, apuntes y agenda para la gestión diaria de tu empresa de sofás
        </p>
      </div>
    </div>

    <div class="dashboard-grid">
      <!-- Agenda de tareas -->
      <div class="dashboard-card tareas-card">
        <h2><i class="fas fa-tasks"></i> Tareas</h2>
        <div class="tareas-input">
          <input v-model="nuevaTarea" class="form-input" placeholder="Nueva tarea..." @keyup.enter="agregarTarea" />
          <button class="btn btn-primary" @click="agregarTarea">Añadir</button>
        </div>
        <ul class="tareas-lista">
          <li v-for="(tarea, idx) in tareas" :key="idx" :class="{ completada: tarea.completada }">
            <input type="checkbox" v-model="tarea.completada" />
            <span>{{ tarea.texto }}</span>
            <button class="btn btn-delete" @click="eliminarTarea(idx)">🗑️</button>
          </li>
        </ul>
      </div>

      <!-- Agenda de eventos -->
      <div class="dashboard-card eventos-card">
        <h2><i class="fas fa-calendar"></i> Eventos</h2>
        <div class="eventos-input">
          <input v-model="nuevoEvento.titulo" class="form-input" placeholder="Título del evento" />
          <input v-model="nuevoEvento.fecha" type="date" class="form-input" />
          <button class="btn btn-primary" @click="agregarEvento">Añadir</button>
        </div>
        <ul class="eventos-lista">
          <li v-for="(evento, idx) in eventosOrdenados" :key="idx">
            <span class="evento-fecha">{{ evento.fecha }}</span>
            <span class="evento-titulo">{{ evento.titulo }}</span>
            <button class="btn btn-delete" @click="eliminarEvento(idx)">🗑️</button>
          </li>
        </ul>
      </div>

      <!-- Apuntes rápidos con historial y búsqueda -->
      <div class="dashboard-card apuntes-card">
        <h2><i class="fas fa-sticky-note"></i> Apuntes</h2>
        <input v-model="busquedaApuntes" class="form-input" placeholder="Buscar apunte..." />
        <textarea v-model="nuevoApunte" class="form-textarea" rows="3" placeholder="Escribe un apunte y pulsa Guardar"></textarea>
        <button class="btn btn-secondary" @click="guardarApunte">Guardar Apunte</button>
        <ul class="apuntes-lista">
          <li v-for="(apunte, idx) in apuntesFiltrados" :key="idx">
            <span class="apunte-fecha">{{ apunte.fecha }}</span>
            <span class="apunte-texto">{{ apunte.texto }}</span>
            <button class="btn btn-delete" @click="eliminarApunte(idx)">🗑️</button>
          </li>
        </ul>
      </div>

      <!-- Conversor de unidades -->
      <div class="dashboard-card conversiones-card">
        <h2><i class="fas fa-ruler-combined"></i> Conversor de Unidades</h2>
        <div class="form-group">
          <input v-model.number="valorUnidad" type="number" class="form-input" placeholder="Valor" />
          <select v-model="unidadOrigen" class="form-select">
            <option v-for="u in unidades" :key="u" :value="u">{{ u }}</option>
          </select>
          <span class="arrow">→</span>
          <select v-model="unidadDestino" class="form-select">
            <option v-for="u in unidades" :key="u" :value="u">{{ u }}</option>
          </select>
        </div>
        <div class="resultado-conversion">
          <span v-if="conversionValida">{{ valorConvertido }} {{ unidadDestino }}</span>
          <span v-else class="error-text">Conversión no soportada</span>
        </div>
      </div>

      <!-- Calculadora rápida -->
      <div class="dashboard-card calculadora-card">
        <h2><i class="fas fa-calculator"></i> Calculadora Rápida</h2>
        <input v-model="expresion" class="form-input" placeholder="Ej: (2.5*3) + 12/4" @keyup.enter="calcular" />
        <button class="btn btn-primary" @click="calcular">Calcular</button>
        <div class="resultado-calculadora">
          <span v-if="resultadoCalculadora !== null">= {{ resultadoCalculadora }}</span>
        </div>
      </div>

      <!-- Calculadora de retales -->
      <div class="dashboard-card retales-card">
        <h2><i class="fas fa-cut"></i> Calculadora de Retales</h2>
        <div class="form-group">
          <input v-model.number="largoTotal" type="number" class="form-input" placeholder="Largo total (cm)" />
          <input v-model.number="largoRetal" type="number" class="form-input" placeholder="Largo de cada retal (cm)" />
        </div>
        <div class="resultado-retales">
          <span v-if="retalesCalculados !== null">
            Puedes cortar <b>{{ retalesCalculados }}</b> retales de {{ largoRetal }}cm
          </span>
        </div>
      </div>

      <!-- Calculadora de tapizado -->
      <div class="dashboard-card tapizado-card">
        <h2><i class="fas fa-couch"></i> Coste de Tapizado</h2>
        <div class="form-group">
          <input v-model.number="metrosTela" type="number" class="form-input" placeholder="Metros de tela" />
          <input v-model.number="precioTela" type="number" class="form-input" placeholder="Precio por metro (€)" />
          <input v-model.number="manoObra" type="number" class="form-input" placeholder="Mano de obra (€)" />
        </div>
        <div class="resultado-tapizado">
          <span v-if="costeTapizado !== null">
            Coste total: <b>{{ costeTapizado }} €</b>
          </span>
        </div>
      </div>

      <!-- Conversor metros lineales a metros cuadrados -->
      <div class="dashboard-card metros-card">
        <h2><i class="fas fa-ruler-horizontal"></i> ML a M²</h2>
        <div class="form-group">
          <input v-model.number="metrosLineales" type="number" class="form-input" placeholder="Metros lineales" />
          <input v-model.number="anchoTela" type="number" class="form-input" placeholder="Ancho de tela (cm)" />
        </div>
        <div class="resultado-metros">
          <span v-if="metrosCuadrados !== null">
            <b>{{ metrosCuadrados }}</b> m²
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch} from 'vue';

// --- Agenda de tareas ---
const tareas = ref(JSON.parse(localStorage.getItem('tareas_sofa') || '[]'));
const nuevaTarea = ref('');
function agregarTarea() {
  if (nuevaTarea.value.trim()) {
    tareas.value.push({ texto: nuevaTarea.value, completada: false });
    nuevaTarea.value = '';
    guardarTareas();
  }
}
function eliminarTarea(idx) {
  tareas.value.splice(idx, 1);
  guardarTareas();
}
function guardarTareas() {
  localStorage.setItem('tareas_sofa', JSON.stringify(tareas.value));
}
watch(tareas, guardarTareas, { deep: true });

// --- Agenda de eventos ---
const eventos = ref(JSON.parse(localStorage.getItem('eventos_sofa') || '[]'));
const nuevoEvento = ref({ titulo: '', fecha: '' });
function agregarEvento() {
  if (nuevoEvento.value.titulo && nuevoEvento.value.fecha) {
    eventos.value.push({ ...nuevoEvento.value });
    nuevoEvento.value = { titulo: '', fecha: '' };
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
watch(eventos, guardarEventos, { deep: true });

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
const conversionValida = computed(() => unidadOrigen.value !== unidadDestino.value);
const conversiones = {
  mm: { mm: 1, cm: 0.1, m: 0.001, pulgadas: 0.0393701, pies: 0.00328084, yardas: 0.00109361 },
  cm: { mm: 10, cm: 1, m: 0.01, pulgadas: 0.393701, pies: 0.0328084, yardas: 0.0109361 },
  m: { mm: 1000, cm: 100, m: 1, pulgadas: 39.3701, pies: 3.28084, yardas: 1.09361 },
  pulgadas: { mm: 25.4, cm: 2.54, m: 0.0254, pulgadas: 1, pies: 0.0833333, yardas: 0.0277778 },
  pies: { mm: 304.8, cm: 30.48, m: 0.3048, pulgadas: 12, pies: 1, yardas: 0.333333 },
  yardas: { mm: 914.4, cm: 91.44, m: 0.9144, pulgadas: 36, pies: 3, yardas: 1 }
};
const valorConvertido = computed(() => {
  if (conversiones[unidadOrigen.value] && conversiones[unidadOrigen.value][unidadDestino.value]) {
    return (valorUnidad.value * conversiones[unidadOrigen.value][unidadDestino.value]).toFixed(4);
  }
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
const metrosTela = ref(null);
const precioTela = ref(null);
const manoObra = ref(null);
const costeTapizado = computed(() => {
  if (metrosTela.value > 0 && precioTela.value > 0 && manoObra.value >= 0) {
    return (metrosTela.value * precioTela.value + manoObra.value).toFixed(2);
  }
  return null;
});

// --- Conversor metros lineales a metros cuadrados ---
const metrosLineales = ref(null);
const anchoTela = ref(null);
const metrosCuadrados = computed(() => {
  if (metrosLineales.value > 0 && anchoTela.value > 0) {
    return ((metrosLineales.value * anchoTela.value) / 100).toFixed(2);
  }
  return null;
});
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
.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 28px;
}
.dashboard-card {
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  padding: 22px 18px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-height: 220px;
}
.dashboard-card h2 {
  margin: 0 0 10px 0;
  font-size: 1.25em;
  color: #405890;
  display: flex;
  align-items: center;
  gap: 8px;
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
</style>