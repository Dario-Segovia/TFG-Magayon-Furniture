<template>
  <transition name="modal-fade">
    <div v-if="visible" class="elegant-converter-overlay" @click.self="onClose">
      <div class="elegant-converter">
        <!-- Cabecera con estilo clásico -->
        <div class="converter-header">
          <div class="header-icon">
            <i class="fas fa-ruler-combined"></i>
          </div>
          <h2>Conversor de Unidades</h2>
          <button class="close-btn" @click="onClose">
            <i class="fas fa-times"></i>
          </button>
        </div>

        <!-- Cuerpo principal -->
        <div class="converter-body">
          <!-- Panel de conversión -->
          <div class="conversion-panel">
            <div class="input-group">
              <input
                type="number"
                class="elegant-input"
                :value="valorUnidad"
                @input="e => emit('update:valorUnidad', Number(e.target.value))"
                placeholder="Valor"
                min="0"
                step="any"
              />
            </div>

            <div class="unit-selectors">
              <div class="selector-wrapper">
                <select
                  class="elegant-select"
                  :value="unidadOrigen"
                  @change="e => emit('update:unidadOrigen', e.target.value)"
                >
                  <option v-for="u in unidades" :key="u" :value="u">{{ u }}</option>
                </select>
                <div class="select-arrow">
                  <i class="fas fa-chevron-down"></i>
                </div>
              </div>

              <div class="conversion-icon">
                <i class="fas fa-exchange-alt"></i>
              </div>

              <div class="selector-wrapper">
                <select
                  class="elegant-select"
                  :value="unidadDestino"
                  @change="e => emit('update:unidadDestino', e.target.value)"
                >
                  <option v-for="u in unidades" :key="u" :value="u">{{ u }}</option>
                </select>
                <div class="select-arrow">
                  <i class="fas fa-chevron-down"></i>
                </div>
              </div>
            </div>
          </div>

          <!-- Resultado con estilo elegante -->
          <div class="result-panel" v-if="conversionValida">
            <div class="result-content">
              <div class="result-label">Resultado:</div>
              <div class="result-value">
                {{ valorConvertido }} <span class="result-unit">{{ unidadDestino }}</span>
              </div>
              <div class="conversion-formula">
                {{ valorUnidad }} {{ unidadOrigen }} = {{ valorConvertido }} {{ unidadDestino }}
              </div>
            </div>
          </div>
          <div v-else class="error-message">
            <i class="fas fa-exclamation-circle"></i>
            <span>Selecciona unidades compatibles</span>
          </div>
        </div>

        <!-- Pie de página con detalles -->
        <div class="converter-footer">
          <div class="footer-hint">
            <i class="fas fa-info-circle"></i> Selecciona las unidades y escribe el valor a convertir
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script setup>
const props = defineProps([
  'valorUnidad',
  'unidadOrigen',
  'unidadDestino',
  'unidades',
  'conversionValida',
  'valorConvertido',
  'visible',
  'onClose'
]);
const emit = defineEmits(['update:valorUnidad', 'update:unidadOrigen', 'update:unidadDestino']);
</script>

<style scoped>
/* Animaciones suaves */
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}

/* Estilo general */
.elegant-converter-overlay {
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
}

.elegant-converter {
  width: 90%;
  max-width: 500px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
  overflow: hidden;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.elegant-converter-overlay.modal-fade-enter-active .elegant-converter {
  animation: modalSlideUp 0.4s ease-out;
}

@keyframes modalSlideUp {
  from { transform: translateY(30px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

/* Cabecera */
.converter-header {
  background: linear-gradient(135deg, #4a6bff, #6a11cb);
  color: white;
  padding: 25px;
  display: flex;
  align-items: center;
  position: relative;
}

.header-icon {
  width: 45px;
  height: 45px;
  background-color: rgba(255, 255, 255, 0.15);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 15px;
  font-size: 20px;
}

.converter-header h2 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  flex-grow: 1;
}

.close-btn {
  background: none;
  border: none;
  color: white;
  font-size: 1.3rem;
  cursor: pointer;
  opacity: 0.8;
  transition: all 0.3s;
}

.close-btn:hover {
  opacity: 1;
  transform: rotate(90deg);
}

/* Cuerpo del conversor */
.converter-body {
  padding: 30px;
}

.conversion-panel {
  margin-bottom: 30px;
}

.input-group {
  margin-bottom: 25px;
}

.elegant-input {
  width: 100%;
  padding: 15px 20px;
  font-size: 1.2rem;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  outline: none;
  transition: all 0.3s;
  text-align: center;
  font-weight: 500;
  color: #333;
}

.elegant-input:focus {
  border-color: #4a6bff;
  box-shadow: 0 0 0 3px rgba(74, 107, 255, 0.1);
}

.unit-selectors {
  display: flex;
  align-items: center;
  gap: 15px;
}

.selector-wrapper {
  flex: 1;
  position: relative;
}

.elegant-select {
  width: 100%;
  padding: 12px 15px;
  font-size: 1rem;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  background-color: white;
  appearance: none;
  cursor: pointer;
  transition: all 0.3s;
}

.elegant-select:focus {
  border-color: #4a6bff;
  outline: none;
  box-shadow: 0 0 0 3px rgba(74, 107, 255, 0.1);
}

.select-arrow {
  position: absolute;
  right: 15px;
  top: 50%;
  transform: translateY(-50%);
  pointer-events: none;
  color: #666;
}

.conversion-icon {
  color: #4a6bff;
  font-size: 1.2rem;
}

/* Panel de resultados */
.result-panel {
  background-color: #f8f9fa;
  border-radius: 8px;
  padding: 20px;
  margin-top: 20px;
  border-left: 4px solid #4a6bff;
}

.result-content {
  text-align: center;
}

.result-label {
  font-size: 0.9rem;
  color: #666;
  margin-bottom: 5px;
}

.result-value {
  font-size: 1.8rem;
  font-weight: 700;
  color: #4a6bff;
  margin-bottom: 5px;
}

.result-unit {
  font-size: 1.2rem;
  font-weight: 500;
  color: #6a11cb;
}

.conversion-formula {
  font-size: 0.9rem;
  color: #666;
  margin-top: 10px;
  font-style: italic;
}

/* Mensaje de error */
.error-message {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: #ff4757;
  font-weight: 500;
  margin-top: 20px;
  padding: 12px;
  background-color: #fff0f0;
  border-radius: 8px;
}

/* Pie de página */
.converter-footer {
  padding: 15px 25px;
  background-color: #f5f7fa;
  border-top: 1px solid #e0e0e0;
}

.footer-hint {
  font-size: 0.85rem;
  color: #666;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.footer-hint i {
  color: #4a6bff;
}

/* Responsive */
@media (max-width: 768px) {
  .unit-selectors {
    flex-direction: column;
  }
  
  .conversion-icon {
    transform: rotate(90deg);
    margin: 5px 0;
  }
  
  .result-value {
    font-size: 1.5rem;
  }
}

@media (max-width: 480px) {
  .converter-header {
    padding: 20px;
  }
  
  .converter-header h2 {
    font-size: 1.3rem;
  }
  
  .header-icon {
    width: 40px;
    height: 40px;
    font-size: 18px;
  }
  
  .elegant-input {
    padding: 12px 15px;
    font-size: 1.1rem;
  }
}
</style>