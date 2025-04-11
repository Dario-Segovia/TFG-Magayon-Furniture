<template>
    <transition name="fade">
      <div v-if="show" class="modal-overlay" @click.self="close">
        <div class="modal-content">
          <h3 class="modal-title">Confirmar eliminación</h3>
          <p class="modal-message">¿Estás seguro que deseas eliminar a <strong>{{ employeeName }}</strong>?</p>
          <div class="modal-actions">
            <button class="btn btn-cancel" @click="close">Cancelar</button>
            <button class="btn btn-confirm" @click="confirm">Eliminar</button>
          </div>
        </div>
      </div>
    </transition>
  </template>
  
  <script setup>
  defineProps({
    show: Boolean,
    employeeName: String
  });
  
  const emit = defineEmits(['close', 'confirm']);
  
  const close = () => emit('close');
  const confirm = () => emit('confirm');
  </script>
  
  <style scoped>
  .modal-overlay {
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
  
  .modal-content {
    background-color: white;
    padding: 25px;
    border-radius: 10px;
    width: 400px;
    max-width: 90%;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  }
  
  .modal-title {
    margin-top: 0;
    margin-bottom: 15px;
    color: #333;
    font-size: 1.3rem;
  }
  
  .modal-message {
    margin-bottom: 25px;
    color: #555;
    line-height: 1.5;
  }
  
  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 20px;
  }
  
  .btn {
    padding: 10px 20px;
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 0.9rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  
  .btn-cancel {
    background-color: #f0f0f0;
    color: #555;
  }
  
  .btn-cancel:hover {
    background-color: #e0e0e0;
    transform: translateY(-1px);
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
  }
  
  .btn-confirm {
    background-color: #e74c3c;
    color: white;
  }
  
  .btn-confirm:hover {
    background-color: #c0392b;
    transform: translateY(-1px);
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
  }
  
  /* Efecto de hover azul para cancelar si lo prefieres */
  /* .btn-cancel:hover {
    background-color: #3498db;
    color: white;
  } */
  
  .fade-enter-active, .fade-leave-active {
    transition: opacity 0.3s;
  }
  
  .fade-enter-from, .fade-leave-to {
    opacity: 0;
  }
  </style>