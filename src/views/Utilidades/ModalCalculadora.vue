<template>
  <transition name="fade">
    <div v-if="visible" class="modal-overlay" @click.self="onClose">
      <transition name="slide-fade">
        <div class="modal-content modal-calculadora">

          <div class="calculadora">
            <div class="pantalla">
              <div class="expresion">{{ expresion || '0' }}</div>
              <div class="resultado" v-if="resultado !== null">= {{ resultado }}</div>
            </div>
            <div class="botones">
              <button @click="limpiar" class="btn-funcion">C</button>
              <button @click="borrar" class="btn-funcion">←</button>
              <button @click="append('(')">(</button>
              <button @click="append(')')">)</button>

              <button @click="append('7')">7</button>
              <button @click="append('8')">8</button>
              <button @click="append('9')">9</button>
              <button @click="append('/')" class="btn-operador">÷</button>

              <button @click="append('4')">4</button>
              <button @click="append('5')">5</button>
              <button @click="append('6')">6</button>
              <button @click="append('*')" class="btn-operador">×</button>

              <button @click="append('1')">1</button>
              <button @click="append('2')">2</button>
              <button @click="append('3')">3</button>
              <button @click="append('-')" class="btn-operador">−</button>

              <button @click="append('0')" class="btn-cero">0</button>
              <button @click="append('.')">.</button>
              <button @click="calcular" class="btn-igual">=</button>
              <button @click="append('+')" class="btn-operador">+</button>
            </div>
          </div>
        </div>
      </transition>
    </div>
  </transition>
</template>

<script setup>
import { ref } from 'vue'
import { evaluate } from 'mathjs'

const props = defineProps({
  visible: Boolean,
  onClose: Function
})

const expresion = ref('')
const resultado = ref(null)

function append(char) {
  if (/[+\-*/.]/.test(char) && expresion.value.slice(-1).match(/[+\-*/.]/)) return
  expresion.value += char
  resultado.value = null
}

function limpiar() {
  expresion.value = ''
  resultado.value = null
}

function borrar() {
  expresion.value = expresion.value.slice(0, -1)
  resultado.value = null
}

function calcular() {
  try {
    const expr = expresion.value.replace(/÷/g, '/').replace(/×/g, '*')
    resultado.value = evaluate(expr)
  } catch {
    resultado.value = 'Error'
  }
}
</script>

<style scoped>
/* Efectos de transición */
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.3s ease;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
}

.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}
.slide-fade-leave-active {
  transition: all 0.3s cubic-bezier(1, 0.5, 0.8, 1);
}
.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateY(20px);
  opacity: 0;
}

.modal-overlay {
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0,0,0,0.5);
  backdrop-filter: blur(4px);
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal-content {
  position: relative;
 
  border-radius: 16px;
  padding: 0;
  min-width: 350px;
  max-width: 95vw;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 40px rgba(0,0,0,0.25);
  transform: scale(1);
  transition: transform 0.2s ease;
}

.modal-content:hover {
  transform: scale(1.01);
}

/* Estilos de la calculadora */
.modal-calculadora {
  --bg: #222;
  --btn-bg: rgb(255, 255, 255);
  --btn-hover: #e9dede;
  --btn-func-bg: #f44336;
  --btn-func-hover: #d32f2f;
  --btn-oper-bg: #117de9;
  --btn-oper-hover: #1167bd;
  --btn-igual-bg: #1ddf21;
  --btn-igual-hover: #0dc517;
  
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.calculadora {
  background-color: var(--bg);
  border-radius: 16px;
  padding: 20px;
  max-width: 350px;
  box-shadow: 0 0 20px rgba(0,0,0,0.9);
  color: var(--text-color);
  user-select: none;
}

.pantalla {
  background-color: #111;
  border-radius: 10px;
  padding: 16px 20px;
  margin-bottom: 16px;
  min-height: 70px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  justify-content: center;
  font-size: 1.4rem;
  word-break: break-all;
  box-shadow: inset 0 0 10px rgba(0,0,0,0.5);
}

.expresion {
  font-weight: 400;
  color: #ccc;
  transition: color 0.2s ease;
}

.resultado {
  font-weight: 700;
  font-size: 1.8rem;
  margin-top: 6px;
  color: #4caf50;
  transition: all 0.3s ease;
}

.botones {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 14px;
}

button {
  background-color: var(--btn-bg);
  border: none;
  border-radius: 10px;
  padding: 20px 0;
  font-size: 1.4rem;
  color: var(--text-color);
  cursor: pointer;
  transition: all 0.2s ease;
  user-select: none;
  box-shadow: 0 4px 0 rgba(0,0,0,0.3);
}

button:active {
  box-shadow: 0 1px 0 rgba(0,0,0,0.3);
  transform: translateY(3px);
}

button:hover {
  background-color: var(--btn-hover);
  transform: translateY(-1px);
}

.btn-funcion {
  background-color: var(--btn-func-bg);
}

.btn-funcion:hover {
  background-color: var(--btn-func-hover);
}

.btn-operador {
  background-color: var(--btn-oper-bg);
}

.btn-operador:hover {
  background-color: var(--btn-oper-hover);
}

.btn-igual {
  background-color: var(--btn-igual-bg);
  grid-column: 3 / 4;
}

.btn-igual:hover {
  background-color: var(--btn-igual-hover);
}

.btn-cero {
  grid-column: 1 / 3;
}

h2 {
  color: var(--text-color);
  margin-bottom: 16px;
  text-align: center;
  font-size: 1.6rem;
  text-shadow: 0 2px 4px rgba(0,0,0,0.3);
}

i {
  margin-right: 10px;
}
</style>