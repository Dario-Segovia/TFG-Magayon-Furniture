<template>
    <div>
      <h2>Lista de Empleados</h2>
      <ul>
        <li v-for="(employee, index) in employees" :key="employee.id">
          <div>
            <span>{{ employee.nombre }} {{ employee.apellido }}</span>
            <button @click="editEmployee(employee)">Editar</button>
            <button @click="deleteEmployee(employee.id)">Eliminar</button>
          </div>
        </li>
      </ul>
    </div>
  </template>
  
  <script>
  import { ref } from 'vue';
  import { invoke } from "@tauri-apps/api/core";
  
  export default {
    data() {
      return {
        employees: [],
      };
    },
    created() {
      this.fetchEmployees();
    },
    methods: {
      async fetchEmployees() {
        try {
          const employees = await invoke('get_employees');
          this.employees = employees;
        } catch (error) {
          console.error(error);
          alert('Error al obtener los empleados.');
        }
      },
      async deleteEmployee(id) {
        if (confirm('¿Estás seguro de que quieres eliminar este empleado?')) {
          try {
            const response = await invoke('delete_employee', { id });
            alert(response);
            this.fetchEmployees();
          } catch (error) {
            console.error(error);
            alert('Error al eliminar el empleado.');
          }
        }
      },
      editEmployee(employee) {
        this.$emit('edit-employee', employee);
      },
    },
  };
  </script>
  