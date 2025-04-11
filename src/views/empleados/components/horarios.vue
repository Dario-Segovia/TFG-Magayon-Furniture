<template>
  <div class="horarios-container">
    <!-- Encabezado consistente con el diseño de empleados -->
    <div class="app-header">
      <div class="header-content">
        <div class="header-text">
          <h1 class="app-title">Horarios de <span>{{ empleadoNombre }}</span></h1>
          <p class="welcome-message">Gestiona los turnos y disponibilidad</p>
        </div>
        
      </div>
    </div>

    <!-- Contenedor principal del calendario -->
    <div class="calendar-wrapper">
      <FullCalendar
        :key="calendarKey"
        :options="calendarOptions"
        class="modern-calendar"
      />
    </div>

    <!-- Modal para añadir/editar horario -->
<transition name="modal-fade">
  <div v-if="mostrarFormulario" class="modal-overlay">
    <div class="modal-container">
      <!-- Botón de cerrar -->
      <button @click="cerrarModal" class="modal-close-btn">
        <i class="fas fa-times"></i>
      </button>
      
      <!-- Encabezado del modal -->
      <div class="modal-header">
        <h3 class="modal-title">{{ horarioEditando ? 'Editar Horario' : 'Nuevo Horario' }}</h3>
      </div>
      
      <!-- Formulario -->
      <div class="modal-body">
        <form @submit.prevent="guardarHorario" class="form-horario">
          <div class="form-group">
            <label for="fecha">Fecha</label>
            <input type="date" id="fecha" v-model="nuevoHorario.fecha" required class="form-input">
          </div>
          
          <div class="time-inputs">
            <div class="form-group">
              <label for="hora_inicio">Hora de entrada</label>
              <input type="time" id="hora_inicio" v-model="nuevoHorario.hora_inicio" required class="form-input">
            </div>
            
            <div class="form-group">
              <label for="hora_fin">Hora de salida</label>
              <input type="time" id="hora_fin" v-model="nuevoHorario.hora_fin" required class="form-input">
            </div>
          </div>
          
          <div class="form-group">
            <label for="tipo_turno">Tipo de turno</label>
            <select id="tipo_turno" v-model="nuevoHorario.tipo_turno" class="form-input">
              <option value="Turno normal">Turno normal</option>
              <option value="Turno noche">Turno noche</option>
              <option value="Turno extra">Turno extra</option>
              <option value="Vacaciones">Vacaciones</option>
              <option value="Descanso">Descanso</option>
            </select>
          </div>
          
          <div class="form-group">
            <label for="notas">Notas</label>
            <textarea id="notas" v-model="nuevoHorario.notas" class="form-input" rows="3"></textarea>
          </div>
          
          <div class="form-actions">
            <button type="button" @click="cerrarModal" class="action-btn cancel-btn">
              Cancelar
            </button>
            <button type="submit" class="action-btn submit-btn">
              {{ horarioEditando ? 'Actualizar' : 'Guardar' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</transition>

   <!-- Modal de confirmación para eliminar -->
<transition name="modal-fade">
  <div v-if="mostrarConfirmacion" class="modal-overlay">
    <div class="modal-container">
      <!-- Botón de cerrar en esquina superior derecha -->
      <button @click="mostrarConfirmacion = false" class="modal-close-btn">
        <i class="fas fa-times"></i>
      </button>
      
      <!-- Encabezado del modal -->
      <div class="modal-header">
        <h3 class="modal-title">¿Qué deseas hacer?</h3>
      </div>
      
      <!-- Contenido informativo -->
      <div class="modal-body">
        <div class="info-card">
          <div class="info-item">
            <span class="info-label">Empleado:</span>
            <span class="info-value">{{ empleadoNombre }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Turno:</span>
            <span class="info-value">{{ turnoAEliminar?.title }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Horario:</span>
            <span class="info-value">
              {{ formatTime(turnoAEliminar?.startStr) }} - {{ formatTime(turnoAEliminar?.endStr) }}
            </span>
          </div>
        </div>
      </div>
      
      <!-- Acciones del modal -->
      <div class="modal-actions">
        <button @click="modificarHorario" class="action-btn edit-btn">
          <i class="fas fa-pencil-alt"></i> Modificar
        </button>
        <button @click="confirmarEliminar" class="action-btn delete-btn">
          <i class="fas fa-trash"></i> Eliminar
        </button>
      </div>
    </div>
  </div>
</transition>

  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import FullCalendar from '@fullcalendar/vue3'
import dayGridPlugin from '@fullcalendar/daygrid'
import timeGridPlugin from '@fullcalendar/timegrid'
import interactionPlugin from '@fullcalendar/interaction'
import { invoke } from "@tauri-apps/api/core"

const props = defineProps({
  empleadoId: {
    type: Number,
    required: true
  }
})

const emit = defineEmits(['close'])

// Estado del componente
const calendarKey = ref(0)
const empleadoNombre = ref('')
const eventos = ref([])
const mostrarFormulario = ref(false)
const mostrarConfirmacion = ref(false)
const horarioEditando = ref(false)
const turnoAEliminar = ref(null)


const nuevoHorario = ref({
  fecha: '',
  hora_inicio: '08:00',
  hora_fin: '17:00',
  tipo_turno: 'Turno normal',
  notas: ''
})


function actualizarEventoEnLista(evento) {
  const index = eventos.value.findIndex(e => e.id === evento.id)
  if (index !== -1) {
    eventos.value[index].start = evento.startStr
    eventos.value[index].end = evento.endStr
  }
}

// Configuración del calendario
const calendarOptions = ref({
  plugins: [dayGridPlugin, timeGridPlugin, interactionPlugin],
  initialView: 'timeGridWeek',
  headerToolbar: {
    left: 'prev,next today',
    center: 'title',
    right: 'dayGridMonth,timeGridWeek,timeGridDay'
  },
  slotMinTime: '06:00:00',
  slotMaxTime: '22:00:00',
  allDaySlot: false,
  events: [],
  editable: true,
  selectable: true,
  selectMirror: true,
  dayMaxEvents: true,
  dateClick: handleDateClick,
  eventClick: handleEventClick,
  eventDrop: handleEventDrop,
  eventResize: handleEventResize,
  eventContent: renderEventContent,
  eventClassNames: applyEventClassNames
})

// Cargar datos iniciales
onMounted(() => {
  cargarDatos()
})

async function cargarDatos() {
  try {
    const empleados = await invoke('get_employees')
    const empleado = empleados.find(e => e.id === props.empleadoId)
    empleadoNombre.value = empleado ? `${empleado.nombre} ${empleado.apellido}` : ''

    const data = await invoke('get_horarios')
    
    calendarOptions.value.events = data
      .filter(h => h.empleado_id === props.empleadoId)
      .map(h => ({
        id: h.id,
        title: h.tipo_turno || 'Turno',
        start: `${h.fecha.split('T')[0]}T${formatHora(h.hora_inicio)}`,
        end: `${h.fecha.split('T')[0]}T${formatHora(h.hora_fin)}`,
        extendedProps: {
          notas: h.notas,
          tipo_turno: h.tipo_turno
        },
        backgroundColor: getEventColor(h.tipo_turno),
        borderColor: getEventColor(h.tipo_turno)
      }))
    
    calendarKey.value++ // Forzar actualización
  } catch (error) {
    console.error('Error cargando datos:', error)
  }
}

// Añade este método en tu script
function formatTime(timeStr) {
  if (!timeStr) return '';
  const parts = timeStr.split('T');
  if (parts.length < 2) return timeStr;
  
  const timePart = parts[1].substring(0, 5);
  return timePart;
}

function formatHora(hora) {
  if (typeof hora !== 'string') return '08:00:00'
  if (!hora.includes(':')) return hora + ':00:00'
  const parts = hora.split(':')
  return parts.length === 2 ? `${hora}:00` : hora
}

function getEventColor(tipoTurno) {
  const colors = {
    'Turno normal': '#4e73df',
    'Turno noche': '#6f42c1',
    'Turno extra': '#fd7e14',
    'Vacaciones': '#1cc88a',
    'Descanso': '#858796'
  }
  return colors[tipoTurno] || '#6b7280'
}

function handleDateClick(info) {
  nuevoHorario.value = {
    fecha: info.dateStr,
    hora_inicio: '08:00',
    hora_fin: '17:00',
    tipo_turno: 'Turno normal',
    notas: ''
  }
  horarioEditando.value = false
  mostrarFormulario.value = true
}

function handleEventClick(info) {
  turnoAEliminar.value = info.event; // Guarda el evento seleccionado
  mostrarConfirmacion.value = true; // Muestra el modal de confirmación
}

function handleEventDrop(info) {
  const eventoActualizado = {
    id: info.event.id,
    start: info.event.startStr,
    end: info.event.endStr,
    extendedProps: info.event.extendedProps
  };

  // Actualiza la lista local de eventos
  actualizarEventoEnLista(info.event);

  // Emitir los cambios realizados al componente padre
  emit('changes-made', eventoActualizado);
}


function handleEventResize(info) {
  handleEventDrop(info); // Reutiliza la lógica de handleEventDrop
}

async function actualizarHorario(evento) {
  try {
    const [fecha] = evento.startStr.split('T')
    const hora_inicio = evento.startStr.split('T')[1].substring(0, 5)
    const hora_fin = evento.endStr.split('T')[1].substring(0, 5)
    
    await invoke('update_horario', {
      id: evento.id,
      horario: {
        fecha,
        hora_inicio,
        hora_fin,
        tipo_turno: evento.extendedProps.tipo_turno,
        notas: evento.extendedProps.notas
      }
    })
  } catch (error) {
    console.error('Error actualizando horario:', error)
    cargarDatos() // Recargar para revertir cambios visuales si falla
  }
}

function renderEventContent(info) {
  const { timeText, event } = info

  const container = document.createElement('div')
  const time = document.createElement('div')
  const title = document.createElement('div')

  container.className = 'fc-event-content'
  time.className = 'fc-event-time'
  time.textContent = timeText
  title.className = 'fc-event-title'
  title.textContent = event.title

  container.appendChild(time)
  container.appendChild(title)

  return { domNodes: [container] }
}

function applyEventClassNames(info) {
  return info.event.extendedProps.tipo_turno?.toLowerCase().replace(' ', '-') || ''
}

async function guardarHorario() {
  try {
    const horarioData = {
      empleado_id: props.empleadoId,
      fecha: nuevoHorario.value.fecha,
      hora_inicio: nuevoHorario.value.hora_inicio,
      hora_fin: nuevoHorario.value.hora_fin,
      tipo_turno: nuevoHorario.value.tipo_turno,
      notas: nuevoHorario.value.notas || null
    };

    if (horarioEditando.value) {
      const id = parseInt(nuevoHorario.value.id, 10); // Convierte el ID a entero
      await invoke('update_horario', { 
        id,
        horario: horarioData
      });
    } else {
      await invoke('create_horario', { horario: horarioData });
    }

    mostrarFormulario.value = false;
    await cargarDatos();

    // Emitir evento con los datos actualizados
    emit('changes-made', horarioData);
  } catch (error) {
    console.error('Error al guardar horario:', error);
  }
}
function descartarCambios() {
  cambiosPendientes.value.forEach(cambio => {
    cambio.event.setDates(cambio.originalStart, cambio.originalEnd)

    // Restaurar también en eventos.value
    const index = eventos.value.findIndex(e => e.id === cambio.id)
    if (index !== -1) {
      eventos.value[index].start = cambio.originalStart
      eventos.value[index].end = cambio.originalEnd
    }
  })
  cambiosPendientes.value = []
  mostrandoCambiosPendientes.value = false
}

async function confirmarEliminar() {
  try {
    const id = parseInt(turnoAEliminar.value.id, 10); // Convierte el ID a entero
    await invoke('delete_horario', { id });
    mostrarConfirmacion.value = false;
    await cargarDatos();
  } catch (error) {
    console.error('Error eliminando horario:', error);
  }
}

function cerrarModal() {
  mostrarFormulario.value = false
}

function modificarHorario() {
  const evento = turnoAEliminar.value;

  // Carga los datos del horario en el modal
  nuevoHorario.value = {
    id: evento.id,
    fecha: evento.startStr.split('T')[0],
    hora_inicio: evento.startStr.split('T')[1].substring(0, 5),
    hora_fin: evento.endStr.split('T')[1].substring(0, 5),
    tipo_turno: evento.extendedProps.tipo_turno,
    notas: evento.extendedProps.notas || ''
  };

  horarioEditando.value = true; // Indica que estamos editando un horario
  mostrarFormulario.value = true; // Abre el modal de edición
  mostrarConfirmacion.value = false; // Cierra el modal de confirmación
}
</script>

<style scoped>
.horarios-container {
  background-color: var(--light-bg);
  min-height: 100vh;
  font-family: 'Nunito', sans-serif;
}

/* Encabezado consistente */
.app-header {
  background-color: var(--primary-color);
  
  padding: 1.5rem 2rem;
  box-shadow: 0 0.15rem 1.75rem 0 rgba(58, 59, 69, 0.15);
}

.header-content {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-text h1 {
  margin: 0;
  font-size: 1.8rem;
  font-weight: 700;
}

.header-text h1 span {
  color: var(--secondary-color);
}

.welcome-message {
  margin: 0.25rem 0 0;
  font-size: 0.9rem;
  opacity: 0.8;
}

.header-actions {
  display: flex;
  gap: 0.5rem;
}

.header-action-btn {
  background-color: var(--secondary-color);
  color: #000;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 0.35rem;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s;
  /* Mejorar la visibilidad */
  opacity: 1;
}

.header-action-btn:hover {
  background-color: #e0a800;
  transform: translateY(-1px);
}

/* Contenedor del calendario */
.calendar-wrapper {
  max-width: 1400px;
  margin: 2rem auto;
  padding: 0 2rem;
}

.modern-calendar {
  background: white;
  border-radius: 0.5rem;
  box-shadow: 0 0.15rem 1.75rem 0 rgba(58, 59, 69, 0.1);
  padding: 1rem;
}

/* Modal moderno */
.modern-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1050;
}

.modal-content {
  background-color: white;
  border-radius: 0.5rem;
  width: 100%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 0.5rem 1rem rgba(0, 0, 0, 0.15);
}

.confirm-modal {
  max-width: 400px;
  padding: 2rem;
  text-align: center;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h2 {
  margin: 0;
  color: var(--primary-color);
  font-size: 1.5rem;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.25rem;
  cursor: pointer;
  color: var(--inactive-color);
  transition: color 0.2s;
}

.close-btn:hover {
  color: var(--danger-color);
}

.modal-body {
  padding: 1.5rem;
}

/* Formulario moderno */
.modern-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
}

.form-group {
  margin-bottom: 0;
}

.form-group.span-2 {
  grid-column: span 2;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: var(--text-color);
  font-size: 0.9rem;
}

.modern-input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 0.35rem;
  font-size: 1rem;
  transition: all 0.3s;
  /* Aumentar la visibilidad del foco */
  opacity: 1;
}

.modern-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 0.2rem rgba(78, 115, 223, 0.25);
}

select.modern-input {
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' fill='%23858796' viewBox='0 0 16 16'%3E%3Cpath d='M7.247 11.14 2.451 5.658C1.885 5.013 2.345 4 3.204 4h9.592a1 1 0 0 1 .753 1.659l-4.796 5.48a1 1 0 0 1-1.506 0z'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 0.75rem center;
  background-size: 16px 12px;
}

textarea.modern-input {
  min-height: 80px;
  resize: vertical;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 1rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-color);
}

.cancel-btn {
  padding: 0.75rem 1.5rem;
  border-radius: 0.35rem;
  font-weight: 600;
  cursor: pointer;
  
  border: 1px solid var(--border-color);
  color: var(--text-color);
  transition: all 0.2s;
}

.cancel-btn:hover {
  background-color: #dc2626;
}

.submit-btn {
  padding: 0.75rem 1.5rem;
  border-radius: 0.35rem;
  font-weight: 600;
  cursor: pointer;
  background-color: var(--accent-color);
  border: none;
  color: white;
  transition: all 0.2s;
}

.submit-btn:hover {
  background-color: #3a5bd9;
}

.delete-btn {
  padding: 0.75rem 1.5rem;
  border-radius: 0.35rem;
  font-weight: 600;
  cursor: pointer;
  background-color: var(--danger-color);
  border: none;
 
  transition: all 0.2s;
}

.delete-btn:hover {
  background-color: #dc2626;
}

.edit-btn {
  padding: 0.75rem 1.5rem;
  border-radius: 0.35rem;
  font-weight: 600;
  cursor: pointer;
  background-color: var(--accent-color);
  border: none;
  
  transition: all 0.2s;
}

.edit-btn:hover {
  background-color: #3a5bd9;
}

.confirm-actions {
  display: flex;
  justify-content: center;
  gap: 1rem;
  margin-top: 2rem;
}

/* Estilos para FullCalendar */
:deep(.fc) {
  font-family: 'Nunito', sans-serif;
}

:deep(.fc-header-toolbar) {
  margin-bottom: 1rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 0.5rem; /* Aseguramos que los elementos estén espaciados */
}

:deep(.fc-button) {

  border: none;
 
  font-weight: 600;
  padding: 0.5rem 1rem;
  border-radius: 0.35rem;
  transition: all 0.2s;
  opacity: 1;
}

:deep(.fc-button:hover) {
  background-color: #3a5bd9;
}

:deep(.fc-button-primary:not(:disabled).fc-button-active) {
  
}

:deep(.fc-event) {
  border: none;
  border-radius: 0.25rem;
  padding: 0.25rem 0.5rem;
  cursor: pointer;
  transition: all 0.2s;
}

:deep(.fc-event:hover) {
  filter: brightness(90%);
  transform: translateY(-1px);
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
}

:deep(.fc-event-content) {
  padding: 0.1rem;
}

:deep(.fc-event-time) {
  font-weight: 600;
  margin-bottom: 0.1rem;
  font-size: 0.8rem;
}

:deep(.fc-event-title) {
  font-size: 0.85rem;
}

/* Clases específicas para tipos de turno */
:deep(.turno-normal) {
  background-color: var(--accent-color);
}

:deep(.turno-noche) {
  background-color: #6f42c1;
}

:deep(.turno-extra) {
  background-color: #fd7e14;
}

:deep(.vacaciones) {
  background-color: #1cc88a;
}

:deep(.descanso) {
  background-color: #858796;
}

/* Transiciones */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}


.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
 
 
  position: relative;
}

.close-btn {
  background-color: transparent;
  border: none;
  font-size: 1.2rem;
  cursor: pointer;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: background-color 0.3s;
  margin-left: auto;
}

.close-btn:hover {
  background-color: #eee;
}

.close-btn i {
  pointer-events: none;
}


/* Responsive */
@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
  
  .calendar-wrapper {
    padding: 0 1rem;
  }
  
  .form-grid {
    grid-template-columns: 1fr;
  }
  
  .form-group.span-2 {
    grid-column: span 1;
  }
  
  :deep(.fc-header-toolbar) {
    flex-direction: column;
    gap: 0.5rem;
  }
  
  :deep(.fc-toolbar-chunk) {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    justify-content: center;
  }
}

@media (max-width: 576px) {
  .modal-content {
    margin: 0 1rem;
  }
  
  .form-actions {
    flex-direction: column;
  }
  
  .cancel-btn,
  .submit-btn,
  .delete-btn {
    width: 100%;
  }
}


/* Estilos para los modales */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(3px);
}

.modal-container {
  background-color: white;
  border-radius: 12px;
  width: 100%;
  max-width: 500px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
  position: relative;
  animation: modal-enter 0.3s ease-out;
}

.modal-close-btn {
  position: absolute;
  top: 16px;
  right: 16px;
  width: 32px;
  height: 32px;
  border: none;
  background: none;
  cursor: pointer;
  font-size: 1.2rem;
  color: #6b7280;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.modal-close-btn:hover {
  background-color: #f3f4f6;
  color: #111827;
}

.modal-header {
  padding: 24px 24px 16px;
  border-bottom: 1px solid #e5e7eb;
}

.modal-title {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: #111827;
  text-align: center;
}

.modal-body {
  padding: 20px 24px;
}

/* Estilo para la tarjeta de información */
.info-card {
  background-color: #f9fafb;
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 20px;
}

.info-item {
  display: flex;
  margin-bottom: 12px;
}

.info-item:last-child {
  margin-bottom: 0;
}

.info-label {
  font-weight: 600;
  color: #374151;
  min-width: 100px;
}

.info-value {
  color: #111827;
}

/* Estilos para los botones de acción */
.modal-actions {
  display: flex;
  justify-content: center;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid #e5e7eb;
}

.action-btn {
  padding: 10px 16px;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  transition: all 0.2s;
  border: none;
  font-size: 0.9rem;
}

.edit-btn {
  background-color: #3b82f6;
  color: white;
}

.edit-btn:hover {
  background-color: #2563eb;
}

.delete-btn {
  background-color: #ef4444;
  color: white;
}

.delete-btn:hover {
  background-color: #dc2626;
}

.cancel-btn {
  background-color: #f3f4f6;
  color: #374151;
}

.cancel-btn:hover {
  background-color: #dc2626;
}

.submit-btn {
  background-color: #3b82f6;
  color: white;
}

.submit-btn:hover {
  background-color: #2563eb;
}

/* Estilos para el formulario */
.form-horario {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-group label {
  font-size: 0.9rem;
  color: #374151;
  font-weight: 500;
}

.form-input {
  padding: 10px 12px;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 0.95rem;
  transition: border-color 0.2s;
}

.form-input:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.time-inputs {
  display: flex;
  gap: 16px;
}

.time-inputs .form-group {
  flex: 1;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid #e5e7eb;
}

/* Animaciones */
@keyframes modal-enter {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}

.modal-close-btn {
  z-index: 10;
}
/* Responsive */
@media (max-width: 600px) {
  .modal-container {
    margin: 0 16px;
  }
  
  .time-inputs {
    flex-direction: column;
    gap: 16px;
  }
  
  .modal-actions {
    flex-direction: column;
  }
  
  .action-btn {
    justify-content: center;
  }
}


</style>

