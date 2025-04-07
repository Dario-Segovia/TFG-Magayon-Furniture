<template>
  <div class="p-4">
    <h1 class="text-2xl font-bold mb-4">Horarios de {{ empleadoNombre }}</h1>

    <FullCalendar
      :key="calendarKey"
      :options="calendarOptions"
    />

    <!-- Formulario Modal -->
    <div v-if="mostrarFormulario" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white p-6 rounded-lg max-w-md w-full">
        <h2 class="text-lg font-semibold mb-4">Nuevo Horario</h2>
        <form @submit.prevent="guardarHorario" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700">Fecha</label>
            <input type="date" v-model="nuevoHorario.fecha" required 
                   class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500">
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">Hora inicio</label>
            <input type="time" v-model="nuevoHorario.hora_inicio" required 
                   class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500">
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">Hora fin</label>
            <input type="time" v-model="nuevoHorario.hora_fin" required 
                   class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500">
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">Tipo turno</label>
            <input type="text" v-model="nuevoHorario.tipo_turno" 
                   class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500">
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">Notas</label>
            <input type="text" v-model="nuevoHorario.notas" 
                   class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500">
          </div>
          <div class="flex justify-end space-x-3 pt-4">
            <button type="button" @click="mostrarFormulario = false" 
                    class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50">
              Cancelar
            </button>
            <button type="submit" 
                    class="px-4 py-2 bg-blue-600 rounded-md text-sm font-medium text-white hover:bg-blue-700">
              Guardar
            </button>
          </div>
        </form>
      </div>
    </div>
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

const calendarKey = ref(0)
const empleadoNombre = ref('')
const eventos = ref([])
const mostrarFormulario = ref(false)

const nuevoHorario = ref({
  fecha: '',
  hora_inicio: '08:00',
  hora_fin: '17:00',
  tipo_turno: 'Turno normal',
  notas: ''
})

// En el script setup
const calendarOptions = ref({
  plugins: [dayGridPlugin, timeGridPlugin, interactionPlugin],
  initialView: 'timeGridWeek',
  headerToolbar: {
    left: 'prev,next today',
    center: 'title',
    right: 'dayGridMonth,timeGridWeek,timeGridDay'
  },
  events: [],
  editable: true,
  selectable: true,
  dateClick: handleDateClick,
  eventClick: handleEventClick
})

const cargarDatos = async () => {
  try {
    const empleados = await invoke('get_employees');
    const empleado = empleados.find(e => e.id === props.empleadoId);
    empleadoNombre.value = empleado ? `${empleado.nombre} ${empleado.apellido}` : '';

    const data = await invoke('get_horarios');
    console.log("Datos crudos recibidos:", data); // Depuración
    
    const eventosFiltrados = data
      .filter(h => h.empleado_id === props.empleadoId)
      .map(h => {
        const fecha = h.fecha.split('T')[0]; // Asegura solo la fecha
        const evento = {
          id: h.id,
          title: h.tipo_turno || 'Turno',
          start: `${fecha}T${formatHora(h.hora_inicio)}`,
          end: `${fecha}T${formatHora(h.hora_fin)}`,
          extendedProps: {
            notas: h.notas
          },
          backgroundColor: getEventColor(h.tipo_turno)
        };
        console.log("Evento generado:", evento); // Depuración
        return evento;
      });
    
    calendarOptions.value.events = eventosFiltrados;
    calendarKey.value++; // Forzar actualización
    
  } catch (error) {
    console.error('Error cargando datos:', error);
  }
}

function formatHora(hora) {
  if (typeof hora !== 'string') return '08:00:00';
  if (!hora.includes(':')) return hora + ':00:00';
  const parts = hora.split(':');
  return parts.length === 2 ? `${hora}:00` : hora;
}

function getEventColor(tipoTurno) {
  const colors = {
    'Turno normal': '#3b82f6',
    'Turno noche': '#6366f1',
    'Turno extra': '#f59e0b',
    'Vacaciones': '#10b981'
  }
  return colors[tipoTurno] || '#6b7280'
}

function handleDateClick(info) {
  nuevoHorario.value.fecha = info.dateStr
  mostrarFormulario.value = true
}

function handleEventClick(info) {
  if (confirm(`¿Eliminar este turno (${info.event.title})?`)) {
    eliminarHorario(info.event.id)
  }
}

const guardarHorario = async () => {
  try {
    const horarioData = {
      empleado_id: props.empleadoId,  // Asegúrate de que esto no sea `undefined`
      fecha: nuevoHorario.value.fecha,
      hora_inicio: nuevoHorario.value.hora_inicio,
      hora_fin: nuevoHorario.value.hora_fin,
      tipo_turno: nuevoHorario.value.tipo_turno,
      notas: nuevoHorario.value.notas || null // Si está vacío, envía `null`
    };

    console.log("Datos enviados:", horarioData); // Verifica en la consola

    await invoke('create_horario', { horario: horarioData });
    mostrarFormulario.value = false;
    await cargarDatos();
  } catch (error) {
    console.error('Error al guardar horario:', error);
  }
};

// Eliminar horario
const eliminarHorario = async (id) => {
  try {
    await invoke('delete_horario', { id })
    await cargarDatos()
  } catch (error) {
    console.error('Error eliminando horario:', error)
  }
}

onMounted(() => {
  cargarDatos()
})
</script>

<style scoped>
/* Reemplaza las importaciones CSS con estas */
.fc {
  max-width: 1000px;
  margin: 0 auto;
}
.fc-event {
  cursor: pointer;
}

.fc-daygrid-event {
  padding: 2px 4px;
}
</style>
