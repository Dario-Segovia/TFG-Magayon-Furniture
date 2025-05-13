<template>
  <div class="modal-overlay">
    <div class="modal-container confirm-modal">
      <div class="modal-header">
        <h3>Eliminar Compra</h3>
        <button class="close-btn" @click="$emit('close')">×</button>
      </div>

      <div class="modal-body">
        <div class="message-container">
          <span class="icon-warning">⚠️</span>
          <p>¿Estás seguro de que quieres eliminar esta compra?</p>
        </div>

        
        <p><strong>Proveedor:</strong> {{ compra.nombre_proveedor }}</p>
        <p><strong>Total:</strong> {{ compra.total }} €</p>
        <p><strong>Fecha:</strong> {{ compra.fecha }}</p>

        <div class="modal-actions">
          <button class="btn btn-cancel" @click="$emit('close')">Cancelar</button>
          <button class="btn btn-confirm" @click="eliminarCompra">Eliminar</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  compra: Object
});

const emit = defineEmits(['compra-eliminada', 'close']);

async function eliminarCompra() {
  try {
    await invoke('eliminar_compra', {
      id: props.compra.id
    });
    emit('compra-eliminada');
    emit('close');
  } catch (error) {
    console.error(error);
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-container.confirm-modal {
  background: white;
  border-radius: 8px;
  width: 90%;
  max-width: 450px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  animation: modalFadeIn 0.3s ease-out;
}

@keyframes modalFadeIn {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 20px;
  border-bottom: 1px solid #eee;
}

.modal-header h3 {
  margin: 0;
  font-size: 1.3em;
  color: #333;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.2em;
  cursor: pointer;
  color: #777;
  padding: 5px;
}

.close-btn:hover {
  color: #333;
}

.modal-body {
  padding: 20px;
}

.message-container {
  display: flex;
  align-items: center;
  gap: 15px;
  margin-bottom: 25px;
}

.message-container p {
  margin: 0;
  font-size: 1em;
  color: #555;
  line-height: 1.5;
}

.icon-warning {
  font-size: 2em;
  color: #FFA500;
  flex-shrink: 0;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 20px;
}

.btn {
  padding: 10px 20px;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s;
  min-width: 100px;
}

.btn-cancel {
  background: #f5f5f5;
  border: 1px solid #ddd;
  color: #333;
}

.btn-cancel:hover {
  background: #e0e0e0;
}

.btn-confirm {
  background: #f44336;
  border: 1px solid #d32f2f;
  color: white;
}

.btn-confirm:hover {
  background: #d32f2f;
}
</style>
