<template>
  <form @submit.prevent="mostrarConfirmacion" class="clientes-form">
    <h2 class="clientes-form-title">Editar Cliente</h2>

    <div class="form-group">
      <label class="form-label">Nombre completo</label>
      <input 
        v-model="nombre" 
        class="clientes-input" 
        required 
      />
    </div>
    
    <div class="form-group">
      <label class="form-label">Correo electrónico</label>
      <input 
        v-model="email" 
        type="email"
        class="clientes-input" 
        required 
      />
    </div>
    
    <div class="form-group">
      <label class="form-label">Teléfono</label>
      <input 
        v-model="telefono" 
        type="tel"
        class="clientes-input" 
      />
    </div>

    <h3 class="section-title">Dirección</h3>
    <div class="clientes-direccion-grid">
      <div class="form-group">
        <label class="form-label">Calle/Avenida</label>
        <input 
          v-model="via" 
          class="clientes-input" 
        />
      </div>
      <div class="form-group">
        <label class="form-label">Número</label>
        <input 
          v-model="numero" 
          class="clientes-input" 
        />
      </div>
      <div class="form-group">
        <label class="form-label">Ciudad</label>
        <input 
          v-model="ciudad" 
          class="clientes-input" 
        />
      </div>
      <div class="form-group">
        <label class="form-label">Provincia</label>
        <input 
          v-model="provincia" 
          class="clientes-input" 
        />
      </div>
      <div class="form-group">
        <label class="form-label">País</label>
        <input 
          v-model="pais" 
          class="clientes-input" 
        />
      </div>
    </div>

    <div class="clientes-actions">
      <button type="button" @click="emit('cerrar')" class="clientes-cancel-btn">
        Cancelar
      </button>
      <button type="submit" class="clientes-submit-btn">
        <svg class="btn-icon" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
        </svg>
        Actualizar Cliente
      </button>
    </div>

    <!-- Modal de confirmación -->
    <div v-if="showConfirm" class="modal-overlay">
      <div class="modal-container fade-in" style="max-width: 350px;">
        <div class="modal-header">
          <h3>Confirmar actualización</h3>
        </div>
        <div class="modal-body">
          <p>¿Estás seguro de que deseas actualizar este cliente?</p>
        </div>
        <div class="clientes-actions">
          <button @click="confirmarActualizar" class="clientes-submit-btn">Sí</button>
          <button @click="cancelarActualizar" class="clientes-cancel-btn">No</button>
        </div>
      </div>
    </div>
  </form>
</template>

<script setup>
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({ cliente: Object });
const emit = defineEmits(['actualizado', 'cerrar']);

// Inicializar propiedades del formulario
const nombre = ref(props.cliente.nombre || '');
const email = ref(props.cliente.email || '');
const telefono = ref(props.cliente.telefono || '');
const via = ref(props.cliente.via || '');
const numero = ref(props.cliente.numero || '');
const ciudad = ref(props.cliente.ciudad || '');
const provincia = ref(props.cliente.provincia || '');
const pais = ref(props.cliente.pais || '');
const showConfirm = ref(false);
let actualizarResolve = null;

const actualizar = async () => {
  showConfirm.value = true;
  await new Promise((resolve) => (actualizarResolve = resolve));
};

// Observar cambios en props.cliente
watch(
  () => props.cliente,
  (newCliente) => {
    nombre.value = newCliente.nombre || '';
    email.value = newCliente.email || '';
    telefono.value = newCliente.telefono || '';
    via.value = newCliente.via || '';
    numero.value = newCliente.numero || '';
    ciudad.value = newCliente.ciudad || '';
    provincia.value = newCliente.provincia || '';
    pais.value = newCliente.pais || '';
  },
  { immediate: true } // Ejecutar inmediatamente al cargar el componente
);

// Función para mostrar el modal de confirmación
const mostrarConfirmacion = () => {
  showConfirm.value = true;
};

// Función para confirmar la actualización del cliente
const confirmarActualizar = async () => {
  await invoke('actualizar_cliente', {
    id: props.cliente.id,
    nombre: nombre.value,
    email: email.value,
    telefono: telefono.value,
    via: via.value,
    numero: numero.value,
    ciudad: ciudad.value,
    provincia: provincia.value,
    pais: pais.value
  });
  emit('actualizado');
  emit('cerrar');
  showConfirm.value = false;
  if (actualizarResolve) actualizarResolve();
};

// Función para cancelar la actualización
const cancelarActualizar = () => {
  showConfirm.value = false;
  if (actualizarResolve) actualizarResolve();
};
</script>

<style scoped>
.clientes-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  max-width: 600px;
  margin: 0 auto;
  padding: 2.5rem;
  background-color: white;
  border-radius: 1rem;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.08);
}

.clientes-form-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #111827;
  text-align: center;
  margin-bottom: 0.5rem;
  position: relative;
  padding-bottom: 0.75rem;
}

.clientes-form-title::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 25%;
  right: 25%;
  height: 2px;
  background: linear-gradient(90deg, rgba(59,130,246,0) 0%, rgba(59,130,246,1) 50%, rgba(59,130,246,0) 100%);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: #4b5563;
}

.clientes-input {
  width: 100%;
  max-width: 90%;
  padding: 0.875rem;
  border: 1px solid #e5e7eb;
  border-radius: 0.5rem;
  font-size: 0.9375rem;
  background-color: #f9fafb;
  transition: all 0.2s ease;
}

.clientes-input:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  background-color: white;
}

.section-title {
  font-size: 1.125rem;
  font-weight: 500;
  color: #111827;
  margin: 0.5rem 0;
}

.clientes-direccion-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.clientes-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 1.5rem;
}

.clientes-submit-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.875rem 1.75rem;
  background-color: #2563eb;
  color: white;
  border: none;
  border-radius: 0.5rem;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.clientes-submit-btn:hover {
  background-color: #1d4ed8;
  transform: translateY(-1px);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.clientes-submit-btn:active {
  transform: translateY(0);
}

.clientes-cancel-btn {
  padding: 0.875rem 1.75rem;
  background-color: white;
  color: #4b5563;
  border: 1px solid #e5e7eb;
  border-radius: 0.5rem;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.clientes-cancel-btn:hover {
  background-color: #f9fafb;
  border-color: #d1d5db;
}

.btn-icon {
  width: 1.25rem;
  height: 1.25rem;
}

/* Estilos para el modal de confirmación */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0,0,0,0.35);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  backdrop-filter: blur(2px);
}

.modal-container {
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.modal-header {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.modal-header h3 {
  font-size: 1.125rem;
  font-weight: 600;
  color: #111827;
  margin: 0;
}

.modal-body {
  font-size: 0.9375rem;
  color: #6b7280;
}

@media (max-width: 640px) {
  .clientes-direccion-grid {
    grid-template-columns: 1fr;
  }

  .clientes-form {
    padding: 2rem;
    gap: 1rem;
  }

  .form-group {
    gap: 0.25rem;
  }
}

/* Animaciones para el modal */
.modal-container.fade-in {
  animation: modalFadeIn 0.25s;
}

@keyframes modalFadeIn {
  from { opacity: 0; transform: scale(0.95);}
  to { opacity: 1; transform: scale(1);}
}
</style>