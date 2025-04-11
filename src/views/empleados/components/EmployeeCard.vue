<template>
  <div class="employee-card">
    <button 
      @click.stop="$emit('delete', employee)" 
      class="action-btn delete-btn top-right-delete"
      title="Eliminar empleado"
    >
      <i class="fas fa-trash"></i>
    </button>

    <div class="employee-avatar">
      <i class="fas fa-user-circle"></i>
    </div>
    <div class="employee-info">
      <h3>{{ employee.nombre }} {{ employee.apellido }}</h3>
      <p class="position">{{ employee.puesto }}</p>
      <p class="department">{{ formatSalary(employee.salario) }}</p>
      <p class="hire-date">Contratado: {{ formatDate(employee.fecha_contratacion) }}</p>
    </div>
    <div class="employee-contacts">
      <p><i class="fas fa-envelope"></i> {{ employee.email }}</p>
      <p v-if="employee.telefono"><i class="fas fa-phone"></i> {{ employee.telefono }}</p>
    </div>
    <div class="employee-actions">
      <button @click.stop="$emit('edit', employee)" class="action-btn edit-btn">
  <i class="fas fa-edit"></i>
</button>
      <button 
        @click.stop="$emit('show-calendar', employee.id)" 
        class="action-btn calendar-btn"
        title="Ver horario"
      >
        <i class="fas fa-calendar-alt"></i>
      </button>
    </div>
  </div>

  
</template>
  
  <script setup>
  import { computed } from 'vue';
  
  const props = defineProps({
    employee: Object
  });
  
  const formatDate = (dateString) => new Date(dateString).toLocaleDateString('es-ES');
  const formatSalary = (salary) => salary
    ? new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(salary)
    : 'Salario no especificado';
  
  defineEmits(['edit', 'show-calendar', 'delete']);
  </script>
  
  <style scoped>
 .employee-card {
  position: relative;
  background-color: var(--card-bg);
  border-radius: 0.35rem;
  box-shadow: 0 0.15rem 1.75rem 0 rgba(58, 59, 69, 0.1);
  padding: 1.5rem;
  cursor: pointer;
  transition: all 0.3s;
  border-left: 4px solid var(--accent-color);
  display: flex;
  flex-direction: column;
}
  
  .employee-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 0.5rem 1.5rem 0 rgba(58, 59, 69, 0.2);
  }
  
  .employee-avatar {
    font-size: 2.5rem;
    color: var(--inactive-color);
    margin-bottom: 1rem;
  }
  
  .employee-info h3 {
    margin: 0 0 0.5rem;
    font-size: 1.1rem;
    color: var(--primary-color);
  }
  
  .employee-info .position {
    font-weight: 600;
    color: var(--text-color);
    margin: 0 0 0.25rem;
    font-size: 0.95rem;
  }
  
  .employee-info .department {
    color: var(--inactive-color);
    margin: 0 0 0.25rem;
    font-size: 0.85rem;
  }
  
  .employee-info .hire-date {
    color: var(--inactive-color);
    margin: 0.5rem 0 0;
    font-size: 0.8rem;
  }
  
  .employee-contacts {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
  }
  
  .employee-contacts p {
    margin: 0.25rem 0;
    font-size: 0.85rem;
    color: var(--text-color);
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  
  .employee-contacts i {
    color: var(--inactive-color);
    width: 1rem;
  }
  
  .employee-actions {
    display: flex;
    gap: 0.5rem;
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
    justify-content: flex-end;
  }
  
  .action-btn {
    width: 2rem;
    height: 2rem;
    border-radius: 50%;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  .action-btn:hover {
    transform: scale(1.1);
  }
  
  .edit-btn {
    background-color: var(--warning-color);
   
  }
  
  .edit-btn:hover {
    background-color: #e0a800;
  }
  
  .calendar-btn {
    background-color: var(--accent-color);
    
  }
  
  .calendar-btn:hover {
    background-color: #3a5bd9;
  }

  .delete-btn {
    background-color: #ef4444;
    color: white;
  }

  .top-right-delete {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  z-index: 1;
}

  .delete-btn:hover {
    background-color: #dc2626;
  }
  </style>