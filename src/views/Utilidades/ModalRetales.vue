<template>
  <transition name="modal-fade">
    <div v-if="visible" class="fabric-modal-overlay" @click.self="onClose">
      <div class="fabric-modal">
        <!-- Cabecera con tijeras animadas -->
        <div class="fabric-header">
          <div class="scissors-animation">
            <i class="fas fa-cut"></i>
          </div>
          <h2>Calculadora de Retales</h2>
          <button class="close-btn" @click="onClose">
            <i class="fas fa-times"></i>
          </button>
        </div>

        <!-- Cuerpo con efecto de tela -->
        <div class="fabric-body">
          <!-- Inputs con estilo de etiqueta de tela -->
          <div class="fabric-inputs">
            <div class="fabric-label">
              <span class="label-pin">📌</span>
              <label>Largo total (cm)</label>
              <input 
                v-model.number="localLargoTotal" 
                type="number" 
                class="fabric-input" 
                placeholder="250"
                min="0"
              />
            </div>
            
            <div class="fabric-label">
              <span class="label-pin">📌</span>
              <label>Largo retal (cm)</label>
              <input 
                v-model.number="localLargoRetal" 
                type="number" 
                class="fabric-input" 
                placeholder="30"
                min="0"
              />
            </div>
          </div>

          <!-- Visualización de tela siendo cortada -->
          <div class="fabric-visualization" v-if="retalesCalculados !== null && localLargoTotal > 0 && localLargoRetal > 0">
            <div class="fabric-roll">
              <div 
                class="fabric-piece" 
                v-for="(piece, index) in Array(retalesCalculados).fill()" 
                :key="index"
                :style="{ width: (100 * localLargoRetal / localLargoTotal) + '%' }"
              >
                <div class="scissors-cut" v-if="index < retalesCalculados - 1">
                  <i class="fas fa-cut"></i>
                </div>
                <span class="piece-label">{{ localLargoRetal }}cm</span>
              </div>
              <div 
                class="fabric-remaining" 
                v-if="(localLargoTotal - (retalesCalculados * localLargoRetal)) > 0"
                :style="{ width: (100 * (localLargoTotal - (retalesCalculados * localLargoRetal)) / localLargoTotal) + '%' }"
              >
                <span class="remaining-label">Sobra: {{ (localLargoTotal - (retalesCalculados * localLargoRetal)).toFixed(1) }}cm</span>
              </div>
            </div>
          </div>

          <!-- Resultado con animación -->
          <div class="fabric-result" v-if="retalesCalculados !== null">
            <div class="result-bubble">
              <div class="result-number">{{ retalesCalculados }}</div>
              <div class="result-text">
                retales de {{ localLargoRetal }}cm
                <div class="result-subtext" v-if="(localLargoTotal - (retalesCalculados * localLargoRetal)) > 0">
                  (+{{ (localLargoTotal - (retalesCalculados * localLargoRetal)).toFixed(1) }}cm sobrante)
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Patrón de costura en el footer -->
        <div class="fabric-footer">
          <div class="sewing-pattern"></div>
          <div class="hint-text">
            <i class="fas fa-info-circle"></i> Calcula el máximo aprovechamiento de tus materiales
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script setup>
import { ref, watch } from 'vue';

const props = defineProps(['visible', 'onClose', 'largoTotal', 'largoRetal', 'retalesCalculados']);
const emit = defineEmits(['update:largoTotal', 'update:largoRetal']);

const localLargoTotal = ref(props.largoTotal);
const localLargoRetal = ref(props.largoRetal);

watch(localLargoTotal, val => emit('update:largoTotal', val));
watch(localLargoRetal, val => emit('update:largoRetal', val));

// Sincroniza si cambian desde el padre
watch(() => props.largoTotal, val => localLargoTotal.value = val);
watch(() => props.largoRetal, val => localLargoRetal.value = val);
</script>

<style scoped>
/* Animaciones */
@keyframes cutAnimation {
  0% { transform: translateY(-5px) rotate(10deg); }
  50% { transform: translateY(0) rotate(-5deg); }
  100% { transform: translateY(-5px) rotate(10deg); }
}

@keyframes fabricAppear {
  from { transform: scaleX(0); }
  to { transform: scaleX(1); }
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
.fabric-modal-overlay {
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

.fabric-modal {
  width: 90%;
  max-width: 600px;
  background: white;
  border-radius: 15px;
  box-shadow: 0 15px 40px rgba(0, 0, 0, 0.3);
  overflow: hidden;
  transform: translateY(0);
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.fabric-modal-overlay.modal-fade-enter-active .fabric-modal {
  animation: modalSlideUp 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

@keyframes modalSlideUp {
  from { transform: translateY(50px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

/* Cabecera con tijeras animadas */
.fabric-header {
  background: linear-gradient(135deg, #ff6b6b, #ff4757);
  color: white;
  padding: 25px;
  display: flex;
  align-items: center;
  position: relative;
  border-bottom: 3px solid #ff8e8e;
}

.scissors-animation {
  width: 50px;
  height: 50px;
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 20px;
  font-size: 22px;
  animation: cutAnimation 1.5s infinite ease-in-out;
  transform-origin: center 70%;
}

.fabric-header h2 {
  margin: 0;
  font-size: 1.8rem;
  font-weight: 700;
  flex-grow: 1;
  text-shadow: 1px 1px 3px rgba(0, 0, 0, 0.2);
}

.close-btn {
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
.fabric-body {
  padding: 30px;
  background-color: #fff9f9;
}

/* Inputs con estilo de etiqueta de tela */
.fabric-inputs {
  display: flex;
  gap: 20px;
  margin-bottom: 30px;
  flex-wrap: wrap;
}

.fabric-label {
  flex: 1;
  min-width: 200px;
  position: relative;
  background: white;
  padding: 15px;
  border-radius: 8px;
  box-shadow: 0 3px 10px rgba(255, 107, 107, 0.1);
  border-left: 5px solid #ff6b6b;
}

.label-pin {
  position: absolute;
  top: -10px;
  left: 10px;
  font-size: 1.2rem;
  transform: rotate(-15deg);
}

.fabric-label label {
  display: block;
  margin-bottom: 8px;
  font-size: 0.9rem;
  color: #ff6b6b;
  font-weight: 600;
}

.fabric-input {
  width: 100%;
  padding: 12px 15px;
  font-size: 1.1rem;
  border: 2px solid #ffecec;
  border-radius: 6px;
  transition: all 0.3s;
}

.fabric-input:focus {
  border-color: #ff6b6b;
  outline: none;
  box-shadow: 0 0 0 3px rgba(255, 107, 107, 0.2);
}

/* Visualización de tela */
.fabric-visualization {
  margin: 30px 0;
  background: #f8f8f8;
  padding: 20px;
  border-radius: 10px;
  box-shadow: inset 0 0 10px rgba(0,0,0,0.05);
}

.fabric-roll {
  display: flex;
  height: 80px;
  background: linear-gradient(135deg, #74b9ff, #0984e3);
  border-radius: 5px;
  overflow: hidden;
  position: relative;
  box-shadow: 0 3px 10px rgba(0, 0, 0, 0.1);
}

.fabric-piece {
  height: 100%;
  background: linear-gradient(135deg, #a29bfe, #6c5ce7);
  position: relative;
  display: flex;
  justify-content: center;
  align-items: center;
  animation: fabricAppear 0.5s ease-out;
  border-right: 2px dashed rgba(255, 255, 255, 0.3);
}

.fabric-piece:last-child {
  border-right: none;
}

.scissors-cut {
  position: absolute;
  right: -12px;
  background: white;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  display: flex;
  justify-content: center;
  align-items: center;
  color: #ff6b6b;
  font-size: 12px;
  box-shadow: 0 2px 5px rgba(0,0,0,0.2);
  z-index: 2;
}

.piece-label {
  background: rgba(0, 0, 0, 0.6);
  color: white;
  padding: 3px 8px;
  border-radius: 15px;
  font-size: 0.8rem;
  font-weight: 500;
}

.fabric-remaining {
  height: 100%;
  background: linear-gradient(135deg, #ffeaa7, #fdcb6e);
  display: flex;
  justify-content: center;
  align-items: center;
}

.remaining-label {
  background: rgba(0, 0, 0, 0.6);
  color: white;
  padding: 3px 8px;
  border-radius: 15px;
  font-size: 0.8rem;
  font-weight: 500;
}

/* Resultado */
.fabric-result {
  margin-top: 20px;
}

.result-bubble {
  background: linear-gradient(135deg, #ff7675, #d63031);
  color: white;
  padding: 20px;
  border-radius: 15px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 5px 15px rgba(214, 48, 49, 0.3);
  animation: bounceIn 0.6s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

@keyframes bounceIn {
  0% { transform: scale(0.8); opacity: 0; }
  50% { transform: scale(1.05); }
  100% { transform: scale(1); opacity: 1; }
}

.result-number {
  font-size: 3.5rem;
  font-weight: 800;
  margin-right: 15px;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
}

.result-text {
  font-size: 1.3rem;
  font-weight: 600;
}

.result-subtext {
  font-size: 0.9rem;
  opacity: 0.9;
  margin-top: 5px;
}

/* Footer con patrón de costura */
.fabric-footer {
  position: relative;
  padding: 20px;
  background: white;
  border-top: 1px solid #f1f1f1;
}

.sewing-pattern {
  position: absolute;
  top: -10px;
  left: 0;
  right: 0;
  height: 20px;
  background-image: radial-gradient(circle, #ff6b6b 1px, transparent 1px);
  background-size: 15px 15px;
  opacity: 0.3;
}

.hint-text {
  font-size: 0.9rem;
  color: #666;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.hint-text i {
  color: #ff6b6b;
}

/* Responsive */
@media (max-width: 768px) {
  .fabric-inputs {
    flex-direction: column;
    gap: 15px;
  }
  
  .fabric-label {
    min-width: 100%;
  }
  
  .result-bubble {
    flex-direction: column;
    text-align: center;
  }
  
  .result-number {
    margin-right: 0;
    margin-bottom: 10px;
  }
}

@media (max-width: 480px) {
  .fabric-header h2 {
    font-size: 1.4rem;
  }
  
  .result-number {
    font-size: 2.5rem;
  }
  
  .result-text {
    font-size: 1.1rem;
  }
}
</style>