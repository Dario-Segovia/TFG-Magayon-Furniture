<template>
  <main class="main-content">
    <!-- Estado de carga -->
    <div v-if="loading" class="loading-state">
      <i class="fas fa-spinner fa-spin"></i> Cargando empleados...
    </div>

    <!-- Mensaje cuando no hay empleados -->
    <div v-else-if="employees.length === 0" class="empty-state">
      <i class="fas fa-users-slash"></i>
      <h3>No hay empleados registrados</h3>
      <p>Comienza agregando un nuevo empleado</p>
      <button @click="$emit('create')" class="empty-state-btn">
        <i class="fas fa-user-plus"></i> Agregar Empleado
      </button>
    </div>
    
    <!-- Lista de empleados -->
    <div v-else class="employees-grid">
      <EmployeeCard 
        v-for="employee in employees" 
        :key="employee.id" 
        :employee="employee"
        @edit="$emit('edit', employee)" 
        @show-calendar="$emit('show-calendar', employee.id)"
        @delete="$emit('delete', employee)"
      />
    </div>
  </main>
  
  
</template>

<script setup>
import { defineProps } from 'vue';
import EmployeeCard from './EmployeeCard.vue';

defineProps({
  employees: Array, // Recibe la lista de empleados filtrados
  loading: Boolean
});
</script>

<style scoped>
.main-content {
  max-width: 1400px;
  margin: 2rem auto;
  padding: 0 2rem;
}

.employees-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

/* Estados de carga y vacío */
.loading-state, .empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  background-color: var(--card-bg);
  border-radius: 0.35rem;
  box-shadow: 0 0.15rem 1.75rem 0 rgba(58, 59, 69, 0.1);
}

.loading-state i {
  font-size: 2rem;
  margin-bottom: 1rem;
  color: var(--accent-color);
}

.empty-state i {
  font-size: 3rem;
  margin-bottom: 1.5rem;
  color: var(--inactive-color);
}

.empty-state h3 {
  margin: 0 0 0.5rem;
  color: var(--primary-color);
}

.empty-state p {
  color: var(--inactive-color);
  margin: 0 0 1.5rem;
  text-align: center;
}

.empty-state-btn {
  background-color: var(--accent-color);
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 0.35rem;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s;
}

.empty-state-btn:hover {
  background-color: #3a5bd9;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  opacity: 0;
  animation: fadeIn 0.3s ease-in-out forwards;
}

.modal {
  background-color: white;
  padding: 2rem;
  border-radius: 0.5rem;
  width: 90%;
  max-width: 400px;
  text-align: center;
  box-shadow: 0 10px 25px rgba(0,0,0,0.2);
}

.modal h3 {
  margin-bottom: 1rem;
  color: var(--primary-color);
}

.modal p {
  margin-bottom: 1rem;
  color: #555;
}

.modal-actions {
  display: flex;
  justify-content: center;
  gap: 1rem;
  margin-top: 1.5rem;
}

.cancel-btn,
.confirm-btn {
  padding: 0.6rem 1.2rem;
  border: none;
  border-radius: 0.35rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.cancel-btn {
  background-color: #e0e0e0;
  color: #333;
}

.cancel-btn:hover {
  background-color: #d0d0d0;
}

.confirm-btn {
  background-color: #f44336;
  color: white;
}

.confirm-btn:hover {
  background-color: #d32f2f;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@media (max-width: 992px) {
  .employees-grid {
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  }
}

@media (max-width: 768px) {
  .main-content {
    padding: 0 1rem;
  }
}

@media (max-width: 576px) {
  .employees-grid {
    grid-template-columns: 1fr;
  }
}


</style>