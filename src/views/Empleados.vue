<template>
  <div class="empleados-container">
    <!-- Encabezado estilo Home -->
    <header class="app-header">
      <div class="header-content">
        <h1 class="app-title">Gestión de <span>Empleados</span></h1>
        <p class="welcome-message">Administra el personal de tu empresa</p>
      </div>
      <div class="header-gradient"></div>
    </header>
    
    <!-- Barra de acciones superior -->
    <div class="action-bar">
      <div class="action-buttons">
        <button @click="openCreateForm" class="action-btn create">
          <i class="fas fa-user-plus"></i> Nuevo Empleado
        </button>
        <button @click="refreshList" class="action-btn refresh">
          <i class="fas fa-sync-alt"></i> Actualizar Lista
        </button>
      </div>
      <div class="search-box">
        <input 
          type="text" 
          v-model="searchQuery" 
          placeholder="Buscar empleados..."
          class="search-input"
        >
        <i class="fas fa-search search-icon"></i>
      </div>
    </div>

    <!-- Contenedor principal con lista de empleados -->
    <div class="main-content">
      <!-- Lista de empleados -->
      <EmployeeList 
        ref="employeeList"
        @edit-employee="openEditForm"
        :search-query="searchQuery"
      />

      <!-- Modal para formulario -->
      <transition name="modal">
        <EmployeeForm 
          v-if="showForm"
          :employeeData="selectedEmployee"
          @close="closeForm"
          @saved="handleEmployeeSaved"
          class="employee-form-modal"
        />
      </transition>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import EmployeeForm from '../components/EmployeeForm.vue';
import EmployeeList from '../components/EmployeeList.vue';

const employeeList = ref(null);
const showForm = ref(false);
const selectedEmployee = ref(null);
const searchQuery = ref('');

const openCreateForm = () => {
  selectedEmployee.value = null;
  showForm.value = true;
};

const openEditForm = (employee) => {
  selectedEmployee.value = employee;
  showForm.value = true;
};

const closeForm = () => {
  showForm.value = false;
};

const handleEmployeeSaved = () => {
  closeForm();
  refreshList();
};

const refreshList = () => {
  if (employeeList.value) {
    employeeList.value.refresh();
  }
};
</script>

<style scoped>
/* Variables y estilos base del Home */
:root {
  --primary-color: #3a4f6c;
  --secondary-color: #f8b400;
  --text-color: #2d3436;
  --light-bg: #f9f9f9;
  --card-shadow: 0 10px 30px -5px rgba(0, 0, 0, 0.1);
}

.empleados-container {
  padding: 0;
  background-color: var(--light-bg);
  min-height: 100vh;
  font-family: 'Poppins', sans-serif;
}

/* Encabezado igual al Home */
.app-header {
  background-color: var(--primary-color);
  color: white;
  padding: 2rem 2rem 4rem;
  position: relative;
  overflow: hidden;
  margin-bottom: 2rem;
}

.header-content {
  position: relative;
  z-index: 2;
  max-width: 1200px;
  margin: 0 auto;
}

.header-gradient {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(58, 79, 108, 0.8) 0%, rgba(26, 42, 64, 0.9) 100%);
  z-index: 1;
}

.app-title {
  font-size: 2.5rem;
  font-weight: 700;
  margin: 0;
  letter-spacing: 1px;
}

.app-title span {
  color: var(--secondary-color);
  font-weight: 600;
}

.welcome-message {
  font-size: 1.1rem;
  opacity: 0.9;
  margin-top: 0.5rem;
}

/* Barra de acciones */
.action-bar {
  max-width: 1200px;
  margin: -3rem auto 20px;
  padding: 0 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 15px;
  position: relative;
  z-index: 3;
}

.action-buttons {
  display: flex;
  gap: 15px;
}

.action-btn {
  padding: 10px 20px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.2s ease;
  border: none;
  color: white;
}

.action-btn.create {
  background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
}

.action-btn.refresh {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.action-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
}

/* Buscador */
.search-box {
  position: relative;
  min-width: 250px;
}

.search-input {
  width: 100%;
  padding: 10px 15px 10px 40px;
  border-radius: 6px;
  border: 1px solid #ddd;
  font-size: 14px;
  transition: all 0.3s;
}

.search-input:focus {
  outline: none;
  border-color: var(--secondary-color);
  box-shadow: 0 0 0 2px rgba(248, 180, 0, 0.2);
}

.search-icon {
  position: absolute;
  left: 15px;
  top: 50%;
  transform: translateY(-50%);
  color: #7f8c8d;
}

/* Contenido principal */
.main-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem 2rem;
}

/* Modal para formulario */
.employee-form-modal {
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
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

/* Responsive */
@media (max-width: 768px) {
  .action-bar {
    flex-direction: column;
    align-items: stretch;
    padding: 0 1rem;
  }
  
  .app-header {
    padding: 1.5rem 1rem 3rem;
  }
  
  .app-title {
    font-size: 2rem;
  }
  
  .main-content {
    padding: 0 1rem 2rem;
  }
}

@media (max-width: 480px) {
  .action-buttons {
    flex-direction: column;
  }
  
  .search-box {
    width: 100%;
  }
}
</style>