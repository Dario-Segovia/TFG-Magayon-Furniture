<template>
  <div class="empleados-container">
    <!-- Encabezado -->
    <header class="app-header">
      <div class="header-content">
        <div class="header-text">
          <h1 class="app-title">Gestión de <span>Empleados</span></h1>
          <p class="welcome-message">Administra el personal de tu empresa</p>
        </div>
        <div class="header-actions">
          <button @click="openCreateForm" class="header-action-btn">
            <i class="fas fa-user-plus"></i> Nuevo
          </button>
          <button @click="openCalendar" class="header-action-btn calendar-btn">
            <i class="fas fa-calendar"></i>
          </button>
        </div>
      </div>
    </header>

    <!-- Barra de búsqueda -->
    <div class="search-container">
      <div class="search-wrapper">
        <i class="fas fa-search search-icon"></i>
        <input 
          type="text" 
          v-model="searchQuery" 
          placeholder="Buscar empleados por nombre, puesto..."
          class="search-input"
        >
        <div class="total-employees">
          <span>{{ filteredEmployees.length }}</span> empleados
        </div>
      </div>
    </div>

    <!-- Contenido principal -->
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
        <button @click="openCreateForm" class="empty-state-btn">
          <i class="fas fa-user-plus"></i> Agregar Empleado
        </button>
      </div>

      <!-- Lista de empleados -->
      <div v-else class="employees-grid">
        <div 
          v-for="employee in filteredEmployees" 
          :key="employee.id" 
          class="employee-card"
        >
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
            <button @click.stop="openEditForm(employee)" class="action-btn edit-btn">
              <i class="fas fa-edit"></i>
            </button>
            <button 
              @click.stop="openEmployeeCalendar(employee.id)" 
              class="action-btn calendar-btn"
              title="Ver horario"
            >
              <i class="fas fa-calendar-alt"></i>
            </button>
          </div>
        </div>
      </div>
    </main>

    <!-- Modal para formulario -->
    <transition name="modal">
      <div v-if="showForm" class="employee-form-modal">
        <div class="form-container">
          <div class="form-header">
            <h2>{{ isEditing ? 'Editar Empleado' : 'Nuevo Empleado' }}</h2>
            <button @click="closeForm" class="close-btn">
              <i class="fas fa-times"></i>
            </button>
          </div>

          <form @submit.prevent="submitForm" class="employee-form">
            <div class="form-group" v-for="(value, key) in formFields" :key="key">
              <label :for="key">{{ labels[key] }}</label>
              <input 
                v-if="key !== 'fecha_contratacion'" 
                :type="key === 'email' ? 'email' : (key === 'salario' ? 'number' : 'text')" 
                :id="key" 
                v-model="formData[key]" 
                :required="requiredFields.includes(key)"
                :step="key === 'salario' ? '0.01' : null"
              >
              <input 
                v-else 
                type="date" 
                :id="key" 
                v-model="formData[key]" 
                required
              >
            </div>

            <div class="form-actions">
              <button type="button" @click="closeForm" class="cancel-btn">Cancelar</button>
              <button type="submit" class="submit-btn">{{ isEditing ? 'Actualizar' : 'Guardar' }}</button>
            </div>
          </form>
        </div>
      </div>
    </transition>

    <!-- Modal del Calendario -->
    <transition name="modal">
      <div v-if="showCalendar && selectedEmployeeId" class="calendar-modal">
        <div class="modal-content">
          <div class="modal-header">
            <h2>Horario de {{ getEmployeeName(selectedEmployeeId) }}</h2>
            <button @click="closeCalendar" class="close-btn">
              <i class="fas fa-times"></i>
            </button>
          </div>
          <div class="modal-body">
            <Calendar 
              :key="calendarKey"
              :empleado-id="selectedEmployeeId" 
              @close="closeCalendar" 
            />
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import Calendar from '../components/empleados/horarios.vue';

const loading = ref(true);
const employees = ref([]);
const showForm = ref(false);
const searchQuery = ref('');
const isEditing = ref(false);
const currentEmployeeId = ref(null);
const showCalendar = ref(false);
const selectedEmployeeId = ref(null);
const calendarKey = ref(0); // Clave para forzar la recreación del componente Calendar

const labels = {
  nombre: 'Nombre',
  apellido: 'Apellido',
  email: 'Email',
  telefono: 'Teléfono',
  puesto: 'Puesto',
  salario: 'Salario',
  fecha_contratacion: 'Fecha de Contratación'
};
const requiredFields = ['nombre', 'apellido', 'email', 'puesto', 'fecha_contratacion'];

const formFields = ref({
  nombre: '',
  apellido: '',
  email: '',
  telefono: null,
  puesto: '',
  salario: null,
  fecha_contratacion: new Date().toISOString().split('T')[0]
});

const formData = formFields;

onMounted(async () => {
  await loadEmployees();
});

const loadEmployees = async () => {
  try {
    loading.value = true;
    employees.value = await invoke('get_employees');
  } catch (error) {
    console.error('Error al cargar empleados:', error);
  } finally {
    loading.value = false;
  }
};

const filteredEmployees = computed(() => {
  if (!searchQuery.value) return employees.value;
  const query = searchQuery.value.toLowerCase();
  return employees.value.filter(emp =>
    emp.nombre.toLowerCase().includes(query) ||
    emp.apellido.toLowerCase().includes(query) ||
    emp.puesto.toLowerCase().includes(query) ||
    (emp.email && emp.email.toLowerCase().includes(query))
  );
});

const formatDate = (dateString) => new Date(dateString).toLocaleDateString('es-ES');
const formatSalary = (salary) => salary
  ? new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(salary)
  : 'Salario no especificado';

const openCreateForm = () => {
  isEditing.value = false;
  currentEmployeeId.value = null;
  resetForm();
  showForm.value = true;
};

const openEditForm = (employee) => {
  isEditing.value = true;
  currentEmployeeId.value = employee.id;
  formData.value = {
    nombre: employee.nombre,
    apellido: employee.apellido,
    email: employee.email,
    telefono: employee.telefono || null,
    puesto: employee.puesto,
    salario: employee.salario || null,
    fecha_contratacion: (employee.fechaContratacion || employee.fecha_contratacion).split('T')[0]
  };
  showForm.value = true;
};

const submitForm = async () => {
  try {
    const payload = { ...formData.value };
    payload.fechaContratacion = payload.fecha_contratacion;

    if (isEditing.value) {
      await invoke('update_employee', { id: currentEmployeeId.value, ...payload });
    } else {
      await invoke('create_employee', payload);
    }

    await loadEmployees();
    closeForm();
  } catch (error) {
    console.error('Error al guardar empleado:', error);
    alert(`Error al guardar empleado: ${error}`);
  }
};

const resetForm = () => {
  formData.value = {
    nombre: '', apellido: '', email: '', telefono: null,
    puesto: '', salario: null,
    fecha_contratacion: new Date().toISOString().split('T')[0]
  };
};

const closeForm = () => showForm.value = false;

const openEmployeeCalendar = (employeeId) => {
  selectedEmployeeId.value = employeeId;
  calendarKey.value++; // Incrementar la clave para forzar la recreación
  showCalendar.value = true;
};

const openCalendar = () => showCalendar.value = true;
const closeCalendar = () => showCalendar.value = false;
const getEmployeeName = (id) => {
  const emp = employees.value.find(e => e.id === id);
  return emp ? `${emp.nombre} ${emp.apellido}` : '';
};
</script>

<style scoped>

/* Nuevos estilos para los botones de acción */
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
  color: white;
}

.edit-btn:hover {
  background-color: #e0a800;
}

.calendar-btn {
  background-color: var(--accent-color);
  color: white;
}

.calendar-btn:hover {
  background-color: #3a5bd9;
}

/* Variables de diseño */
:root {
  --primary-color: #3a4f6c;
  --secondary-color: #f8b400;
  --accent-color: #4e73df;
  --text-color: #2d3436;
  --light-bg: #f8f9fc;
  --card-bg: #ffffff;
  --border-color: #e3e6f0;
  --success-color: #1cc88a;
  --warning-color: #f6c23e;
  --danger-color: #e74a3b;
  --inactive-color: #858796;
}

.empleados-container {
  background-color: var(--light-bg);
  min-height: 100vh;
  font-family: 'Nunito', sans-serif;
}

/* Encabezado */
.app-header {
  background-color: var(--primary-color);
  color: white;
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

/* Barra de búsqueda */
.search-container {
  background-color: var(--card-bg);
  padding: 1rem 2rem;
  box-shadow: 0 0.15rem 1.75rem 0 rgba(58, 59, 69, 0.1);
}

.search-wrapper {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  position: relative;
}

.search-input {
  flex: 1;
  padding: 0.75rem 1rem 0.75rem 2.5rem;
  border: 1px solid var(--border-color);
  border-radius: 0.35rem;
  font-size: 0.9rem;
  transition: all 0.3s;
}

.search-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 0.2rem rgba(78, 115, 223, 0.25);
}

.search-icon {
  position: absolute;
  left: 1rem;
  color: var(--inactive-color);
}

.total-employees {
  margin-left: 1rem;
  background-color: var(--primary-color);
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 0.35rem;
  font-size: 0.9rem;
  font-weight: 600;
}

.total-employees span {
  font-size: 1.1rem;
  margin-right: 0.25rem;
}

/* Contenido principal */
.main-content {
  max-width: 1400px;
  margin: 2rem auto;
  padding: 0 2rem;
}

/* Grid de empleados */
.employees-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

.employee-card {
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
  z-index: 1050;
}

.form-container {
  background-color: white;
  border-radius: 0.5rem;
  width: 100%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 0.5rem 1rem rgba(0, 0, 0, 0.15);
}

.form-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.form-header h2 {
  margin: 0;
  color: var(--primary-color);
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.25rem;
  cursor: pointer;
  color: var(--inactive-color);
  transition: color 0.2s;
}

.close-btn:hover {
  color: var(--danger-color);
}

.employee-form {
  padding: 1.5rem;
}

.form-group {
  margin-bottom: 1.25rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: var(--text-color);
}

.form-group input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 0.35rem;
  font-size: 1rem;
  transition: all 0.3s;
}

.form-group input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 0.2rem rgba(78, 115, 223, 0.25);
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-color);
}

.cancel-btn, .submit-btn {
  padding: 0.75rem 1.5rem;
  border-radius: 0.35rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.cancel-btn {
  background-color: white;
  border: 1px solid var(--border-color);
  color: var(--text-color);
}

.cancel-btn:hover {
  background-color: #f8f9fa;
}

.submit-btn {
  background-color: var(--accent-color);
  border: none;
  color: white;
}

.submit-btn:hover {
  background-color: #3a5bd9;
}

/* Modal del Calendario */
.calendar-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1050;
}

.modal-content {
  background-color: white;
  border-radius: 0.5rem;
  width: 90%;
  max-width: 1000px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 0.5rem 1rem rgba(0, 0, 0, 0.15);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h2 {
  margin: 0;
  color: var(--primary-color);
}

.modal-body {
  padding: 1.5rem;
}

/* Transiciones del modal */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

/* Responsive */
@media (max-width: 992px) {
  .employees-grid {
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  }
}

@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
  
  .search-container {
    padding: 1rem;
  }
  
  .main-content {
    padding: 0 1rem;
  }
  
  .search-wrapper {
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
  }
  
  .total-employees {
    margin-left: 0;
    text-align: center;
  }
}

@media (max-width: 576px) {
  .employees-grid {
    grid-template-columns: 1fr;
  }
  
  .app-header {
    padding: 1rem;
  }
  
  .form-container {
    margin: 0 1rem;
  }
  
  .form-actions {
    justify-content: space-between;
  }
  
  .cancel-btn, .submit-btn {
    flex: 1;
    text-align: center;
  }
}
</style>