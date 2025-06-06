<template>
  <transition name="modal-fade">
    <div v-if="visible" class="upholstery-modal-overlay" @click.self="onClose">
      <div class="upholstery-modal">
        <!-- Cabecera con sofá animado -->
        <div class="upholstery-header">
          <div class="sofa-animation">
            <i class="fas fa-couch"></i>
          </div>
          <h2>Coste de Tapizado</h2>
          <button class="close-btn" @click="onClose">
            <i class="fas fa-times"></i>
          </button>
        </div>

        <!-- Cuerpo con visualización de tela -->
        <div class="upholstery-body">
          <!-- Inputs con estilo de muestra de tela -->
          <div class="fabric-samples">
            <div class="sample-input">
              <div class="sample-swatch" :style="{ background: 'linear-gradient(135deg, #a8edea, #fed6e3)' }"></div>
              <label>Metros de tela</label>
              <input 
                v-model.number="localMetrosTela" 
                type="number" 
                class="fabric-input" 
                placeholder="Ej: 5.5"
                min="0"
                step="0.1"
              />
            </div>
            
            <div class="sample-input">
              <div class="sample-swatch" :style="{ background: 'linear-gradient(135deg, #f5f7fa, #c3cfe2)' }"></div>
              <label>Precio por metro (€)</label>
              <input 
                v-model.number="localPrecioTela" 
                type="number" 
                class="fabric-input" 
                placeholder="Ej: 24.99"
                min="0"
                step="0.01"
              />
            </div>
            
            <div class="sample-input">
              <div class="sample-swatch" :style="{ background: 'linear-gradient(135deg, #e0c3fc, #8ec5fc)' }"></div>
              <label>Mano de obra (€)</label>
              <input 
                v-model.number="localManoObra" 
                type="number" 
                class="fabric-input" 
                placeholder="Ej: 150"
                min="0"
              />
            </div>
          </div>

          <!-- Visualización del sofá con tela -->
          <div class="sofa-visualization" v-if="costeTapizado !== null">
            <div class="sofa-wrapper">
              <div class="sofa-base">
                <div class="sofa-cushion" v-for="n in 3" :key="n">
                  <div class="fabric-pattern" :style="getFabricStyle(n)"></div>
                </div>
              </div>
            </div>
          </div>

          <!-- Resultado con desglose -->
          <div class="upholstery-result" v-if="costeTapizado !== null">
            <div class="cost-breakdown">
              <div class="breakdown-item">
                <span class="breakdown-label">Tela ({{ localMetrosTela }}m × {{ localPrecioTela }}€)</span>
                <span class="breakdown-value">{{ (localMetrosTela * localPrecioTela).toFixed(2) }} €</span>
              </div>
              <div class="breakdown-item">
                <span class="breakdown-label">Mano de obra</span>
                <span class="breakdown-value">{{ localManoObra.toFixed(2) }} €</span>
              </div>
              <div class="breakdown-total">
                <span class="total-label">COSTE TOTAL</span>
                <span class="total-value">
                  {{ typeof costeTapizado === 'number' ? costeTapizado.toFixed(2) : '—' }} €
                </span>
              </div>
            </div>
          </div>
        </div>

        <!-- Footer con herramientas de tapicero -->
        <div class="upholstery-footer">
          <div class="tools-decoration">
            <i class="fas fa-ruler"></i>
            <i class="fas fa-scissors"></i>
            <i class="fas fa-tools"></i>
            <i class="fas fa-thumbtack"></i>
          </div>
          <div class="hint-text">
            <i class="fas fa-info-circle"></i> Calcula el coste exacto de tus proyectos de tapicería
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script setup>
import { ref, watch, computed } from 'vue';

const props = defineProps(['visible', 'onClose', 'metrosTela', 'precioTela', 'manoObra', 'costeTapizado']);
const emit = defineEmits(['update:metrosTela', 'update:precioTela', 'update:manoObra']);

const localMetrosTela = ref(props.metrosTela);
const localPrecioTela = ref(props.precioTela);
const localManoObra = ref(props.manoObra);

watch(localMetrosTela, val => emit('update:metrosTela', val));
watch(localPrecioTela, val => emit('update:precioTela', val));
watch(localManoObra, val => emit('update:manoObra', val));

// Sincroniza si cambian desde el padre
watch(() => props.metrosTela, val => localMetrosTela.value = val);
watch(() => props.precioTela, val => localPrecioTela.value = val);
watch(() => props.manoObra, val => localManoObra.value = val);

const getFabricStyle = (n) => {
  const patterns = [
    'repeating-linear-gradient(45deg, #a8edea 0px, #a8edea 10px, #fed6e3 10px, #fed6e3 20px)',
    'repeating-linear-gradient(-45deg, #f5f7fa 0px, #f5f7fa 10px, #c3cfe2 10px, #c3cfe2 20px)',
    'repeating-radial-gradient(circle, #e0c3fc 0px, #e0c3fc 10px, #8ec5fc 10px, #8ec5fc 20px)'
  ];
  return { background: patterns[n % patterns.length] };
};
</script>

<style scoped>
/* Animaciones */
@keyframes sofaBounce {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-5px); }
}

@keyframes toolsSpin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
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
.upholstery-modal-overlay {
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

.upholstery-modal {
  width: 90%;
  max-width: 650px;
  background: white;
  border-radius: 15px;
  box-shadow: 0 15px 40px rgba(0, 0, 0, 0.3);
  overflow: hidden;
  transform: translateY(0);
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.upholstery-modal-overlay.modal-fade-enter-active .upholstery-modal {
  animation: modalSlideUp 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

@keyframes modalSlideUp {
  from { transform: translateY(50px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

/* Cabecera con sofá animado */
.upholstery-header {
  background: linear-gradient(135deg, #667eea, #764ba2);
  color: white;
  padding: 25px;
  display: flex;
  align-items: center;
  position: relative;
  border-bottom: 3px solid #8a63d2;
}

.sofa-animation {
  width: 50px;
  height: 50px;
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 20px;
  font-size: 22px;
  animation: sofaBounce 2s infinite ease-in-out;
}

.upholstery-header h2 {
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
.upholstery-body {
  padding: 30px;
  background-color: #faf9ff;
}

/* Inputs con muestras de tela */
.fabric-samples {
  display: flex;
  gap: 20px;
  margin-bottom: 30px;
  flex-wrap: wrap;
}

.sample-input {
  flex: 1;
  min-width: 150px;
}

.sample-swatch {
  height: 30px;
  border-radius: 6px;
  margin-bottom: 10px;
  box-shadow: 0 3px 10px rgba(0,0,0,0.1);
  border: 1px solid #eee;
}

.sample-input label {
  display: block;
  margin-bottom: 8px;
  font-size: 0.9rem;
  color: #667eea;
  font-weight: 600;
}

.fabric-input {
  width: 100%;
  padding: 12px 15px;
  font-size: 1rem;
  border: 2px solid #e6e6fa;
  border-radius: 6px;
  transition: all 0.3s;
  background: white;
}

.fabric-input:focus {
  border-color: #667eea;
  outline: none;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.2);
}

/* Visualización del sofá */
.sofa-visualization {
  margin: 30px 0;
  padding: 20px;
  background: white;
  border-radius: 10px;
  box-shadow: 0 3px 15px rgba(0,0,0,0.05);
}

.sofa-wrapper {
  display: flex;
  justify-content: center;
}

.sofa-base {
  display: flex;
  width: 100%;
  max-width: 400px;
  height: 120px;
  background: #d4b483;
  border-radius: 10px 10px 0 0;
  padding: 10px;
  box-shadow: inset 0 -10px 20px rgba(0,0,0,0.1);
}

.sofa-cushion {
  flex: 1;
  margin: 0 5px;
  background: white;
  border-radius: 5px;
  overflow: hidden;
  position: relative;
  box-shadow: 0 2px 5px rgba(0,0,0,0.1);
}

.fabric-pattern {
  width: 100%;
  height: 100%;
  animation: fabricAppear 0.5s ease-out;
}

@keyframes fabricAppear {
  from { transform: scaleY(0); }
  to { transform: scaleY(1); }
}

/* Resultado con desglose */
.upholstery-result {
  margin-top: 20px;
}

.cost-breakdown {
  background: white;
  border-radius: 10px;
  padding: 20px;
  box-shadow: 0 3px 15px rgba(0,0,0,0.05);
}

.breakdown-item {
  display: flex;
  justify-content: space-between;
  padding: 10px 0;
  border-bottom: 1px dashed #eee;
}

.breakdown-item:last-child {
  border-bottom: none;
}

.breakdown-label {
  color: #666;
  font-size: 0.95rem;
}

.breakdown-value {
  font-weight: 600;
  color: #444;
}

.breakdown-total {
  display: flex;
  justify-content: space-between;
  margin-top: 15px;
  padding-top: 15px;
  border-top: 2px solid #667eea;
}

.total-label {
  font-weight: 700;
  color: #667eea;
  text-transform: uppercase;
  letter-spacing: 1px;
  font-size: 0.9rem;
}

.total-value {
  font-weight: 800;
  font-size: 1.4rem;
  color: #764ba2;
}

/* Footer con herramientas */
.upholstery-footer {
  position: relative;
  padding: 20px;
  background: white;
  border-top: 1px solid #f1f1f1;
}

.tools-decoration {
  display: flex;
  justify-content: center;
  gap: 25px;
  color: #a78bfa;
  font-size: 1.2rem;
  margin-bottom: 15px;
}

.tools-decoration i:nth-child(1) {
  animation: toolsSpin 8s linear infinite;
}
.tools-decoration i:nth-child(2) {
  animation: toolsSpin 6s linear infinite reverse;
}
.tools-decoration i:nth-child(3) {
  animation: toolsSpin 10s linear infinite;
}
.tools-decoration i:nth-child(4) {
  animation: toolsSpin 7s linear infinite reverse;
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
  color: #667eea;
}

/* Responsive */
@media (max-width: 768px) {
  .fabric-samples {
    flex-direction: column;
    gap: 15px;
  }
  
  .sample-input {
    min-width: 100%;
  }
  
  .sofa-base {
    height: 100px;
  }
}

@media (max-width: 480px) {
  .upholstery-header h2 {
    font-size: 1.4rem;
  }
  
  .total-value {
    font-size: 1.2rem;
  }
  
  .tools-decoration {
    gap: 15px;
    font-size: 1rem;
  }
}
</style>