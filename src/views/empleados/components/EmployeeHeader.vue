<template>
  <header class="app-header fade-in">
    <div class="header-content">
      <div class="header-text">
        <h1 class="app-title">
          <i class="fas fa-user-tie"></i> Gestión de <span>Empleados</span>
        </h1>
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
  background: linear-gradient(135deg, #EA6E6E, #9375FE);
  color: #fff;
  padding: 22px; /* Igual que clientes/inventario */
  border-radius: 10px; /* Igual que clientes/inventario */
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  margin-bottom: 22px;
  animation: fadeIn 0.5s;
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
  font-size: 1.7rem;
  font-weight: 700;
  display: flex;
  align-items: center;
  gap: 10px;
}

.header-text h1 span {
  color: #e0f7e9;
}

.welcome-message {
  margin: 0.25rem 0 0;
  font-size: 1rem;
  color: #e0f7e9;
  opacity: 0.95;
}

.header-actions {
  display: flex;
  gap: 0.5rem;
}

.header-action-btn {
  background: linear-gradient(135deg, #EA6E6E, #9375FE);
  color: #fff;
  border: none;
  padding: 0.7rem 1.3rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 1rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  transition:
    transform 0.2s,
    box-shadow 0.2s,
    background 0.3s;
}

.header-action-btn:hover {
  background: linear-gradient(135deg, #9375FE, #EA6E6E);
  transform: translateY(-2px);
  box-shadow: 0 6px 10px rgba(0, 0, 0, 0.15);
}

@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
}

.fade-in {
  animation: fadeIn 0.5s;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}
</style>