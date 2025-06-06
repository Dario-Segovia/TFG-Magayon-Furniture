<template>
  <div v-if="visible" class="notes-modal-overlay" @click.self="onClose">
    <div class="notebook-modal tareas-notebook">
      <div class="notebook-cover">
        <div class="notebook-spine"></div>
        <div class="notebook-header">
          <h2><i class="fas fa-tasks"></i> Mis Tareas</h2>
          <button class="close-btn" @click="onClose">
            <i class="fas fa-times"></i>
          </button>
        </div>
      </div>
      <div class="notebook-paper">
        <div class="paper-content">
          <div class="search-section">
            <div class="search-container">
              <i class="fas fa-search search-icon"></i>
              <input 
                v-model="busqueda"
                class="search-input"
                placeholder="Buscar tareas…"
                aria-label="Buscar tareas"
              />
            </div>
          </div>
          <div class="lined-paper">
            <div class="tareas-input-row">
              <textarea
                v-model="localNuevaTarea"
                class="note-textarea tarea-input"
                placeholder="Nueva tarea…"
                @keyup.enter.exact="agregarTarea"
                aria-label="Nueva tarea"
                autofocus
                rows="2"
              />
              <button class="save-btn" @click="agregarTarea" :disabled="!localNuevaTarea.trim()">
                <i class="fas fa-plus"></i>
              </button>
            </div>
            <div class="lines"></div>
          </div>
          <div class="tareas-actions">
            <button class="btn btn-secondary" @click="marcarTodas(true)" :disabled="tareas.length === 0">
              <i class="fas fa-check-double"></i> Marcar todas
            </button>
            <button class="btn btn-secondary" @click="marcarTodas(false)" :disabled="tareas.length === 0">
              <i class="fas fa-times-circle"></i> Desmarcar todas
            </button>
            <button class="btn btn-delete" @click="borrarCompletadas" :disabled="tareas.filter(t => t.completada).length === 0">
              <i class="fas fa-trash"></i> Borrar completadas
            </button>
            <span class="tareas-contador">
              ({{ tareas.length }} total, {{ tareas.filter(t => t.completada).length }} completadas)
            </span>
          </div>
          <transition-group name="fade" tag="ul" class="tareas-lista">
            <li
              v-for="(tarea, idx) in tareasFiltradas"
              :key="tarea.id"
              :class="{ completada: tarea.completada, editando: editando === tarea.id }"
              tabindex="0"
              @dblclick="editarTarea(tarea)"
            >
              <input
                type="checkbox"
                v-model="tarea.completada"
                @change="guardarTareas"
                :aria-label="`Marcar completada ${tarea.texto}`"
              />
              <span v-if="editando !== tarea.id" @click="editarTarea(tarea)">
                {{ tarea.texto }}
              </span>
              <input
                v-else
                v-model="tareaEditada"
                class="form-input tarea-edit"
                @keyup.enter="guardarEdicion(tarea)"
                @blur="guardarEdicion(tarea)"
                @keyup.escape="editando = null"
                ref="inputEdit"
              />
              <button class="btn btn-delete" @click="eliminarTarea(idx)" aria-label="Eliminar tarea">
                <i class="fas fa-trash-alt"></i>
              </button>
            </li>
          </transition-group>
          <div v-if="tareasFiltradas.length === 0" class="empty-state">
            <i class="far fa-check-square"></i>
            <p>No hay tareas pendientes</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, computed, nextTick } from 'vue';

const props = defineProps(['visible', 'onClose', 'tareas', 'nuevaTarea', 'agregarTarea', 'eliminarTarea']);
const emit = defineEmits(['update:nuevaTarea', 'guardarTareas']);

const localNuevaTarea = ref(props.nuevaTarea);
const busqueda = ref('');
const editando = ref(null);
const tareaEditada = ref('');
const inputEdit = ref(null);

watch(localNuevaTarea, val => emit('update:nuevaTarea', val));
watch(() => props.nuevaTarea, val => localNuevaTarea.value = val);

const tareasFiltradas = computed(() =>
  props.tareas.filter(t =>
    t.texto.toLowerCase().includes(busqueda.value.toLowerCase())
  )
);

function agregarTarea() {
  const texto = localNuevaTarea.value.trim();
  if (!texto) return;
  props.agregarTarea(texto);
  localNuevaTarea.value = '';
}

function eliminarTarea(idx) {
  props.eliminarTarea(idx);
}

function marcarTodas(valor) {
  props.tareas.forEach(t => (t.completada = valor));
  guardarTareas();
}

function borrarCompletadas() {
  for (let i = props.tareas.length - 1; i >= 0; i--) {
    if (props.tareas[i].completada) props.tareas.splice(i, 1);
  }
  guardarTareas();
}

function editarTarea(tarea) {
  editando.value = tarea.id;
  tareaEditada.value = tarea.texto;
  nextTick(() => {
    if (inputEdit.value && inputEdit.value[0]) inputEdit.value[0].focus();
  });
}

function guardarEdicion(tarea) {
  if (tareaEditada.value.trim()) {
    tarea.texto = tareaEditada.value.trim();
  }
  editando.value = null;
  guardarTareas();
}

function guardarTareas() {
  emit('guardarTareas');
}
</script>

<style scoped>
/* Reutiliza estilos de ModalApuntes y añade detalles para tareas */
.notes-modal-overlay {
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex; justify-content: center; align-items: center;
  z-index: 1000;
  animation: fadeIn 0.3s ease-in-out;
}
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}
.notebook-modal {
  width: 90%;
  max-width: 700px;
  min-height: 500px;
  display: flex;
  background-color: #fff;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
  border-radius: 5px;
  overflow: hidden;
  position: relative;
  animation: slideUp 0.3s ease-out;
}
@keyframes slideUp {
  from { transform: translateY(20px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}
.notebook-cover {
  width: 50px;
  background: linear-gradient(to bottom, #8B4513, #A0522D);
  position: relative;
  display: flex;
  flex-direction: column;
}
.notebook-spine {
  position: absolute;
  right: -5px;
  top: 0;
  bottom: 0;
  width: 10px;
  background: linear-gradient(to right, #654321, #8B4513);
  box-shadow: 2px 0 5px rgba(0, 0, 0, 0.2);
}
.notebook-header {
  padding: 20px 10px;
  color: white;
  writing-mode: vertical-rl;
  text-orientation: mixed;
  transform: rotate(180deg);
  text-align: center;
  flex-grow: 1;
  display: flex;
  justify-content: center;
  align-items: center;
}
.notebook-header h2 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: bold;
}
.close-btn {
  position: absolute;
  top: 10px;
  right: 10px;
  background: none;
  border: none;
  color: white;
  font-size: 1.2rem;
  cursor: pointer;
  z-index: 10;
}



.lines {
  position: absolute;
  top: 0;
  left: 30px;
  right: 0;
  bottom: 0;
  background: repeating-linear-gradient(
    #f9f7f0,
    #f9f7f0 24px,
    #e0e0e0 25px,
    #e0e0e0 26px
  );
  z-index: 0;
  pointer-events: none;
}
.tareas-input-row {
  display: flex;
  gap: 8px;
  align-items: center;
  position: relative;
  z-index: 1;
}
.tarea-input {
  flex: 1;
  border: none;
  background: transparent;
  font-size: 1.1rem;
  outline: none;
  padding: 0 10px;
  min-height: 100px;
}
.save-btn {
  background-color: #4CAF50;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 20px;
  cursor: pointer;
  font-size: 1.1rem;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  gap: 8px;
}
.save-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
.save-btn:hover:not(:disabled) {
  background-color: #45a049;
  transform: translateY(-2px);
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
}
.search-section {
  margin-bottom: 20px;
}
.search-container {
  position: relative;
}
.search-icon {
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%);
  color: #888;
}
.search-input {
  width: 100%;
  padding: 10px 10px 10px 35px;
  border: 1px solid #ddd;
  border-radius: 20px;
  font-size: 16px;
  background-color: white;
}
.tareas-actions {
  display: flex;
  gap: 8px;
  margin-bottom: 10px;
  align-items: center;
  flex-wrap: wrap;
}
.tareas-contador {
  font-size: 1rem;
  color: #7e8ba3;
  margin-left: 12px;
}
.tareas-lista {
  list-style: none;
  padding: 0;
  margin: 0 0 8px 0;
  max-height: 220px;
  overflow-y: auto;
}
.tareas-lista li {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 7px 0;
  border-bottom: 1px solid #f0f0f0;
  transition: background 0.2s;
  background: transparent;
}
.tareas-lista li.completada span {
  text-decoration: line-through;
  color: #aaa;
}
.tareas-lista li.editando {
  background: #f0f4fa;
}
.tarea-edit {
  width: 100%;
  border: 1px solid #ddd;
  border-radius: 4px;
  padding: 4px 8px;
  font-size: 1rem;
}
.btn {
  padding: 6px 10px;
  border: none;
  border-radius: 8px;
  font-size: 0.95rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  transition: background-color 0.2s, transform 0.1s;
}
.btn-delete {
  background-color: #ff4757;
  color: white;
}
.btn-delete:hover:not(:disabled) {
  background-color: #e8414d;
}
.fade-enter-active, .fade-leave-active {
  transition: all 0.3s;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
  transform: translateY(10px);
}
.empty-state {
  text-align: center;
  color: #b0b8c9;
  margin-top: 18px;
}


.notebook-paper {
  flex-grow: 1;
  background-color: #f9f7f0;
  padding: 20px;
  position: relative;
  overflow-y: auto;

 
  box-sizing: border-box;
}

.paper-content {
  max-width: 700px;
  margin: 0 auto;


  overflow-y: auto;
}


/* Efecto de papel con líneas */
.lined-paper {
  position: relative;
  background-color: #fff;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  border-radius: 3px;
  border-left: 1px solid #f0e6d2;
   max-width: 600px;
  width: 100%;
  word-wrap: break-word;
}


.note-textarea {
  position: relative;
  z-index: 1;
  width: 100%;
  border: none;
  background: transparent;
  resize: none;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  font-size: 16px;
  line-height: 26px;
  padding: 0 10px;
  outline: none;

  /* Estas son las líneas nuevas que necesitas */
  word-wrap: break-word;
  overflow-wrap: break-word;
  white-space: pre-wrap;
}
</style>