<template>
    <div>
      <h2>{{ isEdit ? 'Actualizar Empleado' : 'Crear Empleado' }}</h2>
      <form @submit.prevent="submitForm">
        <div>
          <label for="nombre">Nombre:</label>
          <input type="text" v-model="employee.nombre" required />
        </div>
        <div>
          <label for="apellido">Apellido:</label>
          <input type="text" v-model="employee.apellido" required />
        </div>
        <div>
          <label for="email">Email:</label>
          <input type="email" v-model="employee.email" required />
        </div>
        <div>
          <label for="telefono">Teléfono:</label>
          <input type="text" v-model="employee.telefono" />
        </div>
        <div>
          <label for="puesto">Puesto:</label>
          <input type="text" v-model="employee.puesto" required />
        </div>
        <div>
          <label for="salario">Salario:</label>
          <input type="number" v-model="employee.salario" />
        </div>
        <div>
          <label for="fecha_contratacion">Fecha de Contratación:</label>
          <input type="date" v-model="employee.fecha_contratacion" required />
        </div>
        <button type="submit">{{ isEdit ? 'Actualizar' : 'Crear' }}</button>
      </form>
    </div>
  </template>
  
  <script>
  import { invoke } from "@tauri-apps/api/core";
  
  export default {
    props: {
      employeeData: {
        type: Object,
        default: null,
      },
    },
    data() {
      return {
        employee: {
          id: this.employeeData?.id || null,
          nombre: this.employeeData?.nombre || '',
          apellido: this.employeeData?.apellido || '',
          email: this.employeeData?.email || '',
          telefono: this.employeeData?.telefono || '',
          puesto: this.employeeData?.puesto || '',
          salario: this.employeeData?.salario || '',
          fecha_contratacion: this.employeeData?.fecha_contratacion || '',
        },
      };
    },
    computed: {
      isEdit() {
        return this.employee.id !== null;
      },
    },
    methods: {
      async submitForm() {
        if (!this.employee.fecha_contratacion) {
          alert("La fecha de contratación es obligatoria.");
          return;
        }
  
        if (this.isEdit) {
          await this.updateEmployee();
        } else {
          await this.createEmployee();
        }
      },
      
      async createEmployee() {
        try {
          const response = await invoke('create_employee', {
            nombre: this.employee.nombre,
            apellido: this.employee.apellido,
            email: this.employee.email,
            telefono: this.employee.telefono,
            puesto: this.employee.puesto,
            salario: this.employee.salario,
            fechaContratacion: this.employee.fecha_contratacion,
          });
          alert(response);
          this.$emit('employee-updated');
        } catch (error) {
          console.error(error);
          alert('Error al crear el empleado.');
        }
      },
      
      async updateEmployee() {
        try {
          const response = await invoke('update_employee', {
            id: this.employee.id,
            nombre: this.employee.nombre,
            apellido: this.employee.apellido,
            email: this.employee.email,
            telefono: this.employee.telefono,
            puesto: this.employee.puesto,
            salario: this.employee.salario,
            fechaContratacion: this.employee.fecha_contratacion,
          });
          alert(response);
          this.$emit('employee-updated');
        } catch (error) {
          console.error(error);
          alert('Error al actualizar el empleado.');
        }
      },
    },
    watch: {
      employeeData: {
        immediate: true,
        handler(newVal) {
          if (newVal) {
            this.employee = { ...newVal };
          }
        },
      },
    },
  };
  </script>