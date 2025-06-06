<template>
  <transition name="modal-fade">
    <div v-if="visible" class="fabric-converter-overlay" @click.self="onClose">
      <div class="fabric-converter-modal">
        <!-- Cabecera con regla animada -->
        <div class="converter-header">
          <div class="ruler-animation">
            <i class="fas fa-ruler-combined"></i>
          </div>
          <h2>Conversor Textil</h2>
          <h3>Metros Lineales a Metros Cuadrados</h3>
          <button class="close-btn" @click="onClose">
            <i class="fas fa-times"></i>
          </button>
        </div>

        <!-- Cuerpo con visualización interactiva -->
        <div class="converter-body">
          <!-- Inputs con estilo textil -->
          <div class="fabric-measurements">
            <div class="measurement-input">
              <div class="measurement-visual">
                <div class="fabric-roll-visual"></div>
                <span class="measurement-icon">📏</span>
              </div>
              <label>Metros Lineales</label>
              <input 
                type="number" 
                :value="metrosLineales" 
                @input="$emit('update:metrosLineales', $event.target.valueAsNumber)" 
                placeholder="Ej: 5.0"
                min="0"
                step="0.1"
                class="fabric-input"
              />
            </div>
            
            <div class="measurement-input">
              <div class="measurement-visual">
                <div class="fabric-width-visual"></div>
                <span class="measurement-icon">📐</span>
              </div>
              <label>Ancho de Tela (cm)</label>
              <input 
                type="number" 
                :value="anchoTela" 
                @input="$emit('update:anchoTela', $event.target.valueAsNumber)" 
                placeholder="Ej: 140"
                min="0"
                class="fabric-input"
              />
            </div>
          </div>

          <!-- Visualización de la tela desplegada -->
          <div class="fabric-unfolding" v-if="metrosCuadrados !== null && metrosLineales > 0 && anchoTela > 0">
            <div class="fabric-roll-container">
              <div class="fabric-roll" :style="{ '--rotation': rollRotation }">
                <div class="fabric-roll-core"></div>
                <div class="fabric-roll-layer" v-for="n in 10" :key="n"></div>
              </div>
            </div>
            
            <div class="fabric-unfolded" :style="{ width: `${unfoldedWidth}%` }">
              <div class="fabric-grid"></div>
              <div class="size-indicator length">
                {{ metrosLineales }} ml
              </div>
              <div class="size-indicator width">
                {{ (anchoTela / 100).toFixed(2) }} m
              </div>
            </div>
          </div>

          <!-- Resultado con efecto desplegable -->
          <div class="converter-result" v-if="metrosCuadrados !== null">
            <div class="result-container">
              <div class="result-value">
                {{ metrosCuadrados !== null ? metrosCuadrados.toFixed(2) : '' }}
              </div>
              <div class="result-unit">metros cuadrados</div>
              <div class="result-equivalence">
                = {{ metrosLineales }} ml × {{ (anchoTela / 100).toFixed(2) }} m
              </div>
            </div>
          </div>
        </div>

        <!-- Footer con herramientas de medición -->
        <div class="converter-footer">
          <div class="tools-decoration">
            <i class="fas fa-tape"></i>
            <i class="fas fa-square"></i>
            <i class="fas fa-calculator"></i>
            <i class="fas fa-text-width"></i>
          </div>
          <div class="hint-text">
            <i class="fas fa-lightbulb"></i> Convierte entre medidas lineales y de superficie para tus proyectos textiles
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

const props = defineProps(['visible', 'onClose', 'metrosLineales', 'anchoTela', 'metrosCuadrados']);
const emit = defineEmits(['update:metrosLineales', 'update:anchoTela']);

const rollRotation = ref(0);
const isUnfolding = ref(false);

const unfoldedWidth = computed(() => {
  if (!props.metrosLineales || !props.anchoTela) return 0;
  const maxWidth = 80; // Porcentaje máximo del ancho del contenedor
  const relativeLength = Math.min(props.metrosLineales / 10, 1); // Normalizado a 10m
  return maxWidth * relativeLength;
});

watch(() => props.visible, (visible) => {
  if (visible) {
    rollRotation.value = 0;
    isUnfolding.value = true;
    const interval = setInterval(() => {
      rollRotation.value += 10;
      if (rollRotation.value >= 360) {
        rollRotation.value = 0;
      }
    }, 100);
    
    return () => clearInterval(interval);
  }
});
</script>

<style scoped>
/* Animaciones */
@keyframes rollSpin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

@keyframes unfoldFabric {
  from { width: 0; opacity: 0; }
  to { width: var(--final-width); opacity: 1; }
}

@keyframes pulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.05); }
}

.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.4s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}

/* Estilo general */
.fabric-converter-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  backdrop-filter: blur(5px);
}

.fabric-converter-modal {
  width: 90%;
  max-width: 600px;
  background: white;
  border-radius: 15px;
  box-shadow: 0 15px 40px rgba(0, 0, 0, 0.3);
  overflow: hidden;
  transform: translateY(0);
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.fabric-converter-overlay.modal-fade-enter-active .fabric-converter-modal {
  animation: modalSlideUp 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

@keyframes modalSlideUp {
  from { transform: translateY(50px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

/* Cabecera */
.converter-header {
  background: linear-gradient(135deg, #00c6fb, #005bea);
  color: white;
  padding: 25px;
  position: relative;
  border-bottom: 3px solid #0084ff;
  text-align: center;
}

.ruler-animation {
  width: 60px;
  height: 60px;
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 15px;
  font-size: 28px;
  animation: pulse 2s infinite ease-in-out;
}

.converter-header h2 {
  margin: 0;
  font-size: 1.8rem;
  font-weight: 700;
  text-shadow: 1px 1px 3px rgba(0, 0, 0, 0.2);
}

.converter-header h3 {
  margin: 5px 0 0;
  font-size: 1.1rem;
  font-weight: 400;
  opacity: 0.9;
}

.close-btn {
  position: absolute;
  top: 20px;
  right: 20px;
  background: none;
  border: none;
  color: white;
  font-size: 1.5rem;
  cursor: pointer;
  transition: all 0.3s;
}

.close-btn:hover {
  transform: rotate(90deg);
}

/* Cuerpo del modal */
.converter-body {
  padding: 30px;
  background-color: #f8fafc;
}

/* Inputs de medición */
.fabric-measurements {
  display: flex;
  gap: 20px;
  margin-bottom: 30px;
  flex-wrap: wrap;
}

.measurement-input {
  flex: 1;
  min-width: 200px;
}

.measurement-visual {
  position: relative;
  height: 60px;
  margin-bottom: 15px;
}

.fabric-roll-visual {
  width: 80px;
  height: 40px;
  background: linear-gradient(135deg, #a1c4fd, #c2e9fb);
  border-radius: 20px;
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
  box-shadow: 0 3px 10px rgba(0,0,0,0.1);
}

.fabric-width-visual {
  width: 100px;
  height: 20px;
  background: linear-gradient(135deg, #84fab0, #8fd3f4);
  position: absolute;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  box-shadow: 0 3px 10px rgba(0,0,0,0.1);
}

.measurement-icon {
  position: absolute;
  top: -10px;
  right: calc(50% - 60px);
  font-size: 1.5rem;
  background: white;
  width: 30px;
  height: 30px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 5px rgba(0,0,0,0.1);
}

.measurement-input label {
  display: block;
  margin-bottom: 8px;
  font-size: 0.9rem;
  color: #005bea;
  font-weight: 600;
  text-align: center;
}

.fabric-input {
  width: 100%;
  padding: 12px 15px;
  font-size: 1rem;
  border: 2px solid #e0e8ff;
  border-radius: 8px;
  transition: all 0.3s;
  text-align: center;
}

.fabric-input:focus {
  border-color: #00c6fb;
  outline: none;
  box-shadow: 0 0 0 3px rgba(0, 198, 251, 0.2);
}

/* Visualización de tela desplegada */
.fabric-unfolding {
  margin: 30px 0;
  height: 200px;
  position: relative;
  display: flex;
  align-items: center;
}

.fabric-roll-container {
  width: 20%;
  display: flex;
  justify-content: center;
}

.fabric-roll {
  width: 60px;
  height: 60px;
  position: relative;
  transform: rotate(var(--rotation));
  transition: transform 0.3s ease;
}

.fabric-roll-core {
  position: absolute;
  width: 20px;
  height: 20px;
  background: #888;
  border-radius: 50%;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 2;
}

.fabric-roll-layer {
  position: absolute;
  border-radius: 50%;
  background: linear-gradient(135deg, #a1c4fd, #c2e9fb);
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 1;
}

.fabric-roll-layer:nth-child(2) { width: 30px; height: 30px; }
.fabric-roll-layer:nth-child(3) { width: 40px; height: 40px; }
.fabric-roll-layer:nth-child(4) { width: 50px; height: 50px; }
.fabric-roll-layer:nth-child(5) { width: 60px; height: 60px; }
.fabric-roll-layer:nth-child(6) { width: 70px; height: 70px; }
.fabric-roll-layer:nth-child(7) { width: 80px; height: 80px; }
.fabric-roll-layer:nth-child(8) { width: 90px; height: 90px; }
.fabric-roll-layer:nth-child(9) { width: 100px; height: 100px; }
.fabric-roll-layer:nth-child(10) { width: 110px; height: 110px; }

.fabric-unfolded {
  height: 100px;
  background: linear-gradient(135deg, #a1c4fd, #c2e9fb);
  border-radius: 5px;
  position: relative;
  overflow: hidden;
  box-shadow: 0 3px 15px rgba(0,0,0,0.1);
  transition: width 1s ease-out;
}

.fabric-grid {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: 
    linear-gradient(rgba(255,255,255,0.3) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255,255,255,0.3) 1px, transparent 1px);
  background-size: 20px 20px;
}

.size-indicator {
  position: absolute;
  background: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 3px 8px;
  border-radius: 15px;
  font-size: 0.8rem;
  font-weight: 500;
}

.size-indicator.length {
  bottom: -25px;
  left: 50%;
  transform: translateX(-50%);
}

.size-indicator.width {
  right: -50px;
  top: 50%;
  transform: translateY(-50%) rotate(90deg);
}

/* Resultado */
.converter-result {
  margin-top: 30px;
}

.result-container {
  background: linear-gradient(135deg, #00c6fb, #005bea);
  color: white;
  padding: 25px;
  border-radius: 10px;
  text-align: center;
  box-shadow: 0 5px 20px rgba(0, 198, 251, 0.3);
}

.result-value {
  font-size: 3.5rem;
  font-weight: 800;
  line-height: 1;
}

.result-unit {
  font-size: 1.3rem;
  font-weight: 600;
  margin-bottom: 10px;
}

.result-equivalence {
  font-size: 0.9rem;
  opacity: 0.9;
}

/* Footer con herramientas */
.converter-footer {
  position: relative;
  padding: 20px;
  background: white;
  border-top: 1px solid #f1f1f1;
}

.tools-decoration {
  display: flex;
  justify-content: center;
  gap: 25px;
  color: #00c6fb;
  font-size: 1.2rem;
  margin-bottom: 15px;
}

.tools-decoration i {
  animation: pulse 2s infinite ease-in-out;
}

.tools-decoration i:nth-child(1) { animation-delay: 0s; }
.tools-decoration i:nth-child(2) { animation-delay: 0.2s; }
.tools-decoration i:nth-child(3) { animation-delay: 0.4s; }
.tools-decoration i:nth-child(4) { animation-delay: 0.6s; }

.hint-text {
  font-size: 0.9rem;
  color: #666;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.hint-text i {
  color: #00c6fb;
}

/* Responsive */
@media (max-width: 768px) {
  .fabric-measurements {
    flex-direction: column;
    gap: 25px;
  }
  
  .measurement-input {
    min-width: 100%;
  }
  
  .result-value {
    font-size: 2.8rem;
  }
  
  .result-unit {
    font-size: 1.1rem;
  }
}

@media (max-width: 480px) {
  .converter-header h2 {
    font-size: 1.5rem;
  }
  
  .converter-header h3 {
    font-size: 1rem;
  }
  
  .fabric-unfolding {
    height: 150px;
  }
  
  .size-indicator.width {
    right: -40px;
    font-size: 0.7rem;
  }
}
</style>