<template>
  <div v-if="visible" class="modal-overlay" @click.self="onClose">
    <div class="modal-content modal-eventos">
      <button class="modal-close" @click="onClose" aria-label="Cerrar modal">✖</button>
      <h2><i class="fas fa-calendar-alt"></i> Gestión de Eventos</h2>
      
      <div class="eventos-input-container">
        <div class="input-group">
          <label for="evento-titulo">Título del evento</label>
          <input 
            id="evento-titulo"
            v-model="localNuevoEvento.titulo" 
            class="form-input" 
            placeholder="Ej: Reunión con equipo" 
            @keyup.enter="addEvento"
          />
        </div>
        
        <div class="input-group">
          <label for="evento-fecha">Fecha</label>
          <input 
            id="evento-fecha"
            v-model="localNuevoEvento.fecha" 
            type="date" 
            class="form-input" 
            :min="today"
          />
        </div>
        
        <button class="btn btn-primary" @click="addEvento" :disabled="!isFormValid">
          <i class="fas fa-plus"></i> Añadir Evento
        </button>
      </div>
      
      <div class="calendar-container">
        <vue-cal
          :events="calEvents"
          :disable-views="['years']"
          :default-view="calView"
          style="height: 350px"
          @event-click="onEventClick"
          @cell-click="onCellClick"
          :time="false"
          :hide-title-bar="false"
          :transitions="false"
          class="custom-vue-cal"
          locale="es"
          :selected-date="selectedDate"
        />
       
      </div>
      
      <div class="eventos-list-container">
        <h3><i class="fas fa-list"></i> Próximos Eventos</h3>
        <div class="search-container">
          <input 
            v-model="searchQuery" 
            class="form-input search-input" 
            placeholder="Buscar eventos..." 
          />
          <i class="fas fa-search search-icon"></i>
        </div>
        
        <div v-if="filteredEventos.length === 0" class="empty-state">
          <i class="far fa-calendar-times"></i>
          <p>No hay eventos programados</p>
        </div>
        
        <ul class="eventos-lista">
          <li v-for="(evento, idx) in filteredEventos" :key="idx" @click="focusOnEvent(evento.fecha)">
            <div class="evento-badge">
              <span class="evento-dia">{{ formatDay(evento.fecha) }}</span>
              <span class="evento-mes">{{ formatMonth(evento.fecha) }}</span>
            </div>
            <div class="evento-info">
              <span class="evento-titulo">{{ evento.titulo }}</span>
              <span class="evento-fecha-completa">{{ formatFullDate(evento.fecha) }}</span>
            </div>
            <button class="btn btn-delete" @click.stop="eliminarEvento(idx)" aria-label="Eliminar evento">
              <i class="fas fa-trash-alt"></i>
            </button>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, computed, nextTick } from 'vue'
import VueCal from 'vue-cal'
import 'vue-cal/dist/vuecal.css'


const props = defineProps([
  'visible',
  'onClose',
  'eventosOrdenados',
  'nuevoEvento',
  'agregarEvento',
  'eliminarEvento'
])

const emit = defineEmits(['update:nuevoEvento'])

const localNuevoEvento = ref({ ...props.nuevoEvento })
const calView = ref('month')
const searchQuery = ref('')
const selectedDate = ref(new Date())
const today = new Date().toISOString().split('T')[0]

const isFormValid = computed(() => {
  return localNuevoEvento.value.titulo?.trim() && localNuevoEvento.value.fecha
})

watch(() => props.nuevoEvento, val => {
  localNuevoEvento.value = { ...val }
})

function addEvento() {
  if (!isFormValid.value) return
  
  emit('update:nuevoEvento', { 
    ...localNuevoEvento.value,
    titulo: localNuevoEvento.value.titulo.trim()
  })
  
  nextTick(() => {
    props.agregarEvento()
    localNuevoEvento.value = { titulo: '', fecha: '' }
    selectedDate.value = new Date(localNuevoEvento.value.fecha || new Date())
  })
}

const calEvents = computed(() =>
  props.eventosOrdenados.map(ev => ({
    start: ev.fecha,
    end: ev.fecha,
    title: ev.titulo,
    class: 'custom-event'
  }))
)

const filteredEventos = computed(() => {
  if (!searchQuery.value) return props.eventosOrdenados
  const query = searchQuery.value.toLowerCase()
  return props.eventosOrdenados.filter(ev => 
    ev.titulo.toLowerCase().includes(query) || 
    ev.fecha.includes(query)
  )
})

function onEventClick({ event }) {
  alert(`Evento: ${event.title}\nFecha: ${formatFullDate(event.start)}`)
}

function onCellClick({ date }) {
  let fechaStr = '';
  if (typeof date === 'string') {
    fechaStr = date;
  } else if (date instanceof Date && !isNaN(date)) {
    fechaStr = date.toISOString().slice(0, 10);
  }
  if (fechaStr) {
    localNuevoEvento.value.fecha = fechaStr;
    selectedDate.value = new Date(fechaStr);
  }
}

function changeView(view) {
  calView.value = view
}

function focusOnEvent(date) {
  selectedDate.value = new Date(date)
  calView.value = 'day'
}

function formatDay(date) {
  return new Date(date).getDate()
}

function formatMonth(date) {
  return new Date(date).toLocaleString('es', { month: 'short' }).toUpperCase()
}

function formatFullDate(date) {
  return new Date(date).toLocaleDateString('es', {
    weekday: 'long',
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  })
}
</script>

<style scoped>
@import 'vue-cal/dist/vuecal.css';

.modal-overlay {
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0,0,0,0.5);
  backdrop-filter: blur(4px);
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: fadeIn 0.3s ease-out;
  padding: 20px;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.modal-content.modal-eventos {
  background: linear-gradient(135deg, #232946 0%, #1a1f36 100%);
  border-radius: 16px;
  padding: 30px;
  width: 100%;
  max-width: 900px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 30px rgba(0,0,0,0.3);
  position: relative;
  color: #fff;
  animation: slideUp 0.4s cubic-bezier(0.22, 1, 0.36, 1);
  border: 1px solid #414670;
}

@keyframes slideUp {
  from { transform: translateY(20px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

.modal-close {
  position: absolute;
  top: 20px;
  right: 20px;
  background: rgba(238, 187, 195, 0.1);
  border: none;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  font-size: 1.2rem;
  cursor: pointer;
  color: #b8c1ec;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.modal-close:hover {
  background: rgba(238, 187, 195, 0.2);
  color: #eebbc3;
  transform: rotate(90deg);
}

h2 {
  color: #eebbc3;
  margin: 0 0 25px 0;
  font-size: 1.8rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
}

h3 {
  color: #b8c1ec;
  margin: 0 0 15px 0;
  font-size: 1.3rem;
  display: flex;
  align-items: center;
  gap: 8px;
}

.eventos-input-container {
  display: grid;
  grid-template-columns: 1fr 1fr auto;
  gap: 15px;
  margin-bottom: 25px;
}

.input-group {
  display: flex;
  flex-direction: column;
}

label {
  color: #b8c1ec;
  margin-bottom: 6px;
  font-size: 0.9rem;
  font-weight: 500;
}

.form-input {
  background: #121629;
  border: 1px solid #414670;
  border-radius: 8px;
  color: #fff;
  padding: 10px 15px;
  font-size: 1rem;
  outline: none;
  transition: all 0.2s;
  height: 42px;
}

.form-input:focus {
  border-color: #eebbc3;
  box-shadow: 0 0 0 2px rgba(238, 187, 195, 0.2);
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  border: none;
  border-radius: 8px;
  padding: 10px 20px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 0.95rem;
}

.btn-primary {
  background: #eebbc3;
  color: #232946;
  align-self: flex-end;
  height: 42px;
}

.btn-primary:hover {
  background: #ffd6e0;
  transform: translateY(-1px);
}

.btn-primary:disabled {
  background: #555b7a;
  color: #b8c1ec;
  cursor: not-allowed;
  transform: none;
}

.calendar-container {
  margin-bottom: 25px;
  background: #121629;
  border-radius: 12px;
  padding: 15px;
  box-shadow: inset 0 0 10px rgba(0,0,0,0.2);
}

.calendar-actions {
  display: flex;
  gap: 10px;
  margin-top: 15px;
}

.btn-view {
  background: transparent;
  color: #b8c1ec;
  border: 1px solid #414670;
  flex: 1;
}

.btn-view:hover {
  background: rgba(184, 193, 236, 0.1);
}

.btn-view.active {
  background: #eebbc3;
  color: #232946;
  border-color: #eebbc3;
}

.eventos-list-container {
  background: #121629;
  border-radius: 12px;
  padding: 20px;
}

.search-container {
  position: relative;
  margin-bottom: 20px;
}

.search-input {
  width: 100%;
  padding-left: 35px;
}

.search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: #b8c1ec;
}

.empty-state {
  text-align: center;
  padding: 30px;
  color: #b8c1ec;
}

.empty-state i {
  font-size: 2.5rem;
  margin-bottom: 10px;
  color: #414670;
}

.empty-state p {
  margin: 0;
  font-size: 1rem;
}

.eventos-lista {
  margin: 0;
  padding: 0;
  list-style: none;
  max-height: 300px;
 
}

.eventos-lista::-webkit-scrollbar {
  width: 6px;
}

.eventos-lista::-webkit-scrollbar-thumb {
  background: #414670;
  border-radius: 3px;
}

.eventos-lista li {
  display: flex;
  align-items: center;
  gap: 15px;
  background: rgba(27, 31, 56, 0.5);
  border-radius: 10px;
  padding: 12px 15px;
  margin-bottom: 10px;
  cursor: pointer;
  transition: all 0.2s;
  border-left: 3px solid #eebbc3;
}

.eventos-lista li:hover {
  background: rgba(184, 193, 236, 0.1);
  transform: translateX(3px);
}

.evento-badge {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: #232946;
  border-radius: 6px;
  width: 50px;
  height: 50px;
  flex-shrink: 0;
}

.evento-dia {
  font-size: 1.3rem;
  font-weight: 700;
  color: #eebbc3;
  line-height: 1;
}

.evento-mes {
  font-size: 0.7rem;
  text-transform: uppercase;
  color: #b8c1ec;
  letter-spacing: 1px;
}

.evento-info {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.evento-titulo {
  font-weight: 500;
  margin-bottom: 3px;
}

.evento-fecha-completa {
  font-size: 0.85rem;
  color: #b8c1ec;
  opacity: 0.8;
}

.btn-delete {
  background: transparent;
  color: #ff6b6b;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  padding: 0;
}

.btn-delete:hover {
  background: rgba(255, 107, 107, 0.1);
}

.custom-vue-cal {
  --vuecal-bg: #121629;
  --vuecal-text: #fff;
  --vuecal-border: #414670;
  --vuecal-today-bg: rgba(238, 187, 195, 0.2);
  --vuecal-selected-bg: #eebbc3;
  --vuecal-selected-text: #232946;
  --vuecal-header-bg: #232946;
  --vuecal-header-text: #b8c1ec;
  --vuecal-cell-divider: rgba(184, 193, 236, 0.1);
  --vuecal-weekday-bg: #232946;
  
  border-radius: 10px;
  border: 1px solid #414670;
}

.custom-vue-cal :deep(.vuecal__event) {
  background-color: rgba(238, 187, 195, 0.2);
  border-left: 3px solid #eebbc3;
  color: #fff;
  border-radius: 3px;
  padding: 3px 6px;
}

.custom-vue-cal :deep(.vuecal__event-title) {
  font-weight: 500;
  font-size: 0.85rem;
}

@media (max-width: 768px) {
  .modal-content.modal-eventos {
    padding: 20px;
    max-height: 95vh;
  }
  
  .eventos-input-container {
    grid-template-columns: 1fr;
  }
  
  .calendar-actions {
    flex-wrap: wrap;
  }
  
  .btn-view {
    flex: 1 0 calc(50% - 5px);
  }
}
</style>