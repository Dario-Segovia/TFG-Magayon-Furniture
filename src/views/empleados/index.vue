<template>
  <div class="empleados-container">
    <EmployeeHeader 
      @create="openCreateForm" 
      @show-calendar="openCalendar" 
    />
    
    <EmployeeSearch 
      v-model="searchQuery" 
      :count="filteredEmployees.length" 
    />
    
    <EmployeeList
      :employees="filteredEmployees"
      :loading="loading"
      @edit="openEditForm"
      @show-calendar="openEmployeeCalendar"
      @delete="prepareDelete"
    />
    
    <EmployeeForm
      v-model:show="showForm"
      :is-editing="isEditing"
      :form-data="formData"
      :labels="labels"
      :required-fields="requiredFields"
      @submit="submitForm"
      @close="closeForm"
    />
    
    <CalendarModal
      v-model:show="showCalendar"
      :employee-id="selectedEmployeeId"
      :employee-name="getEmployeeName(selectedEmployeeId)"
      @close="closeCalendar"
    />
    
    <ConfirmModal
      v-model:show="showDeleteModal"
      :employee-name="employeeToDeleteName"
      @close="closeDeleteModal"
      @confirm="confirmDelete"
    />
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import EmployeeHeader from './components/EmployeeHeader.vue';
import EmployeeSearch from './components/EmployeeSearch.vue';
import EmployeeList from './components/EmployeeList.vue';
import EmployeeForm from './components/EmployeeForm.vue';
import CalendarModal from './components/CalendarModal.vue';
import ConfirmModal from './components/ConfirmModal.vue';

const loading = ref(true);
const employees = ref([]);
const searchQuery = ref('');
const showForm = ref(false);
const isEditing = ref(false);
const currentEmployeeId = ref(null);
const showCalendar = ref(false);
const selectedEmployeeId = ref(null);
const showDeleteModal = ref(false);
const employeeToDelete = ref(null);

const employeeToDeleteName = computed(() => {
  return employeeToDelete.value 
    ? `${employeeToDelete.value.nombre} ${employeeToDelete.value.apellido}`
    : '';
});

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

const prepareDelete = (employee) => {
  employeeToDelete.value = employee;
  showDeleteModal.value = true;
};

const confirmDelete = async () => {
  if (!employeeToDelete.value) return;
  
  try {
    await invoke('delete_employee', { id: employeeToDelete.value.id });
    await loadEmployees();
    closeDeleteModal();
  } catch (error) {
    console.error('Error al eliminar empleado:', error);
    alert('Error al eliminar empleado');
  }
};

const closeDeleteModal = () => {
  showDeleteModal.value = false;
  employeeToDelete.value = null;
};

// Resto del código se mantiene igual...
const formData = ref({
  nombre: '',
  apellido: '',
  email: '',
  telefono: null,
  puesto: '',
  salario: null,
  fecha_contratacion: new Date().toISOString().split('T')[0]
});

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
    fecha_contratacion: employee.fecha_contratacion.split('T')[0]
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
  showCalendar.value = true;
};

const getEmployeeName = (id) => {
  const emp = employees.value.find(e => e.id === id);
  return emp ? `${emp.nombre} ${emp.apellido}` : '';
};

onMounted(loadEmployees);
</script>

<style scoped>
.empleados-container {
  background-color: var(--light-bg);
  min-height: 100vh;
  font-family: 'Nunito', sans-serif;
}
</style>