<template>
  <div v-if="visible" class="notes-modal-overlay" @click.self="onClose">
    <div class="notebook-modal">
      <div class="notebook-cover">
        <div class="notebook-spine"></div>
        <div class="notebook-header">
          <h2><i class="fas fa-book"></i> Mis Apuntes</h2>
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
                v-model="localBusquedaApuntes" 
                class="search-input" 
                placeholder="Buscar en mis apuntes..." 
              />
            </div>
          </div>
          
          <div class="new-note-section">
            <div class="lined-paper">
              <textarea 
                v-model="localNuevoApunte" 
                class="note-textarea" 
                rows="5" 
                placeholder="Escribe tu nuevo apunte aquí..."
              ></textarea>
              <div class="lines"></div>
            </div>
            <button class="save-btn" @click="guardarApunte">
              <i class="fas fa-save"></i> Guardar Apunte
            </button>
          </div>
          
          <div class="notes-list">
            <div v-for="(apunte, idx) in apuntesFiltrados" :key="idx" class="note-item">
              <div class="note-header">
                <span class="note-date">{{ apunte.fecha }}</span>
                <button class="delete-btn" @click="eliminarApunte(idx)">
                  <i class="fas fa-trash"></i>
                </button>
              </div>
              <div class="note-content">
                {{ apunte.texto }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';

const props = defineProps([
  'visible',
  'onClose',
  'busquedaApuntes',
  'nuevoApunte',
  'guardarApunte',
  'apuntesFiltrados',
  'eliminarApunte'
]);
const emit = defineEmits(['update:busquedaApuntes', 'update:nuevoApunte']);

const localBusquedaApuntes = ref(props.busquedaApuntes);
const localNuevoApunte = ref(props.nuevoApunte);

watch(localBusquedaApuntes, val => emit('update:busquedaApuntes', val));
watch(localNuevoApunte, val => emit('update:nuevoApunte', val));

// Sincroniza si cambian desde el padre
watch(() => props.busquedaApuntes, val => localBusquedaApuntes.value = val);
watch(() => props.nuevoApunte, val => localNuevoApunte.value = val);
</script>

<style scoped>
/* Estilos para el overlay del modal */
.notes-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  animation: fadeIn 0.3s ease-in-out;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

/* Estilo del cuadernillo */
.notebook-modal {
  width: 80%;
  max-width: 800px;
  min-height: 600px;
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

.notebook-paper {
  flex-grow: 1;
  background-color: #f9f7f0;
  padding: 20px;
  position: relative;
  overflow-y: auto;
}

.paper-content {
  max-width: 700px;
  margin: 0 auto;
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
}

/* Sección de búsqueda */
.search-section {
  margin-bottom: 30px;
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

/* Botón de guardar */
.save-btn {
  background-color: #4CAF50;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 20px;
  cursor: pointer;
  font-size: 16px;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 10px;
}

.save-btn:hover {
  background-color: #45a049;
  transform: translateY(-2px);
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
}

/* Lista de apuntes */
.notes-list {
  margin-top: 30px;
}

.note-item {
  background-color: white;
  border-left: 4px solid #8B4513;
  padding: 15px;
  margin-bottom: 15px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  position: relative;
  transition: all 0.3s;
}

.note-item:hover {
  transform: translateX(5px);
  box-shadow: 0 3px 10px rgba(0, 0, 0, 0.15);
}

.note-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  padding-bottom: 5px;
  border-bottom: 1px dashed #eee;
}

.note-date {
  color: #888;
  font-size: 0.9rem;
}

.delete-btn {
  background: none;
  border: none;
  color: #ff6b6b;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.2s;
}

.delete-btn:hover {
  color: #ff0000;
  transform: scale(1.1);
}

.note-content {
  white-space: pre-wrap;
  line-height: 1.6;
}
</style>