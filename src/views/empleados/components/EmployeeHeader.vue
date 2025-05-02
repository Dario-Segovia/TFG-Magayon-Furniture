<template>
    <header class="app-header">
      <div class="header-content">
        <div class="header-text">
          <h1 class="app-title">Gestión de <span>Empleados</span></h1>
          <p class="welcome-message">Administra el personal de tu empresa</p>
        </div>
        <div class="header-actions">
          <button @click="$emit('create')" class="header-action-btn">
            <i class="fas fa-user-plus"></i> Nuevo
          </button>
          
        </div>
      </div>
    </header>
  </template>
  
  <script setup>
  defineEmits(['create', 'show-calendar']);
  </script>
  
  <style scoped>
  .app-header {
  background-color: var(--primary-color);
  color: var(--text-color); /* Añade esto para el texto */
  padding: 1.5rem 2rem;
  box-shadow: 0 0.15rem 1.75rem 0 rgba(58, 59, 69, 0.15);
}
  
  .header-content {
    max-width: 1400px;
    margin: 0 auto;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .header-text h1 {
    margin: 0;
    font-size: 1.8rem;
    font-weight: 700;
  }
  
  .header-text h1 span {
    color: var(--secondary-color);
  }
  
  .welcome-message {
    margin: 0.25rem 0 0;
    font-size: 0.9rem;
    opacity: 0.8;
  }
  
  .header-actions {
    display: flex;
    gap: 0.5rem;
  }
  
  .header-action-btn {
    background-color: var(--secondary-color);
    color: #000;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 0.35rem;
    font-weight: 600;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: all 0.2s;
  }
  
  .header-action-btn:hover {
    background-color: #e0a800;
    transform: translateY(-1px);
  }
  
  .calendar-btn {
    background-color: var(--accent-color);
    color: white;
    padding: 0.5rem;
    width: 2.5rem;
    justify-content: center;
  }
  
  .calendar-btn:hover {
    background-color: #3a5bd9;
  }
  
  @media (max-width: 768px) {
    .header-content {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }
  }
  </style>