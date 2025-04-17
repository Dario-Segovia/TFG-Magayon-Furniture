<template>
  <form @submit.prevent="crear" class="clientes-form">
    <h2 class="clientes-form-title">Nuevo Cliente</h2>

    <div class="form-group">
      <label class="form-label">Nombre completo</label>
      <input 
        v-model="nombre" 
        class="clientes-input" 
        required 
      />
    </div>
    
    <div class="form-group">
      <label class="form-label">Correo electrónico</label>
      <input 
        v-model="email" 
        type="email"
        class="clientes-input" 
        required 
      />
    </div>
    
    <div class="form-group">
      <label class="form-label">Teléfono</label>
      <input 
        v-model="telefono" 
        type="tel"
        class="clientes-input" 
      />
    </div>

    <h3 class="section-title">Dirección</h3>
    <div class="clientes-direccion-grid">
      <div class="form-group">
        <label class="form-label">Calle/Avenida</label>
        <input 
          v-model="via" 
          class="clientes-input" 
        />
      </div>
      <div class="form-group">
        <label class="form-label">Número</label>
        <input 
          v-model="numero" 
          class="clientes-input" 
        />
      </div>
      <div class="form-group">
        <label class="form-label">Ciudad</label>
        <input 
          v-model="ciudad" 
          class="clientes-input" 
        />
      </div>
      <div class="form-group">
        <label class="form-label">Provincia</label>
        <input 
          v-model="provincia" 
          class="clientes-input" 
        />
      </div>
      <div class="form-group">
        <label class="form-label">País</label>
        <input 
          v-model="pais" 
          class="clientes-input" 
        />
      </div>
    </div>

    <div class="clientes-actions">
      <button type="button" @click="cancelar" class="clientes-cancel-btn">
        Cancelar
      </button>
      <button type="submit" class="clientes-submit-btn">
        <svg class="btn-icon" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
        </svg>
        Crear Cliente
      </button>
    </div>
  </form>
</template>
  
  <script setup>
  import { ref } from 'vue'
  import { invoke } from '@tauri-apps/api/core'
  
  const emit = defineEmits(['cliente-creado', 'cerrar'])
  
  const nombre = ref('')
  const email = ref('')
  const telefono = ref('')
  const via = ref('')
  const numero = ref('')
  const ciudad = ref('')
  const provincia = ref('')
  const pais = ref('')
  
  const crear = async () => {
    await invoke('crear_cliente', {
      nombre: nombre.value,
      email: email.value,
      telefono: telefono.value,
      via: via.value,
      numero: numero.value,
      ciudad: ciudad.value,
      provincia: provincia.value,
      pais: pais.value,
    })
    nombre.value = ''
    email.value = ''
    telefono.value = ''
    via.value = ''
    numero.value = ''
    ciudad.value = ''
    provincia.value = ''
    pais.value = ''
    emit('cliente-creado')
  }
  
  const cancelar = () => {
    nombre.value = ''
    email.value = ''
    telefono.value = ''
    via.value = ''
    numero.value = ''
    ciudad.value = ''
    provincia.value = ''
    pais.value = ''
    emit('cerrar')
  }
  </script>

  
<style scoped>
.clientes-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  max-width: 600px;
  margin: 0 auto;
  padding: 2.5rem;
  background-color: white;
  border-radius: 1rem;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.08);
}

.clientes-form-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #111827;
  text-align: center;
  margin-bottom: 0.5rem;
  position: relative;
  padding-bottom: 0.75rem;
}

.clientes-form-title::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 25%;
  right: 25%;
  height: 2px;
  background: linear-gradient(90deg, rgba(59,130,246,0) 0%, rgba(59,130,246,1) 50%, rgba(59,130,246,0) 100%);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: #4b5563;
}

.clientes-input {
  width: 100%;
  max-width: 90%; /* Evita que los campos excedan el contenedor */
  padding: 0.875rem;
  border: 1px solid #e5e7eb;
  border-radius: 0.5rem;
  font-size: 0.9375rem;
  background-color: #f9fafb;
  transition: all 0.2s ease;
}

.clientes-input:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  background-color: white;
}

.section-title {
  font-size: 1.125rem;
  font-weight: 500;
  color: #111827;
  margin: 0.5rem 0;
}

.clientes-direccion-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); /* Ajusta automáticamente */
  gap: 1rem;
}

.clientes-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 1.5rem;
}

.clientes-submit-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.875rem 1.75rem;
  background-color: #2563eb;
  color: white;
  border: none;
  border-radius: 0.5rem;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.clientes-submit-btn:hover {
  background-color: #1d4ed8;
  transform: translateY(-1px);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.clientes-submit-btn:active {
  transform: translateY(0);
}

.clientes-cancel-btn {
  padding: 0.875rem 1.75rem;
  background-color: white;
  color: #4b5563;
  border: 1px solid #e5e7eb;
  border-radius: 0.5rem;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.clientes-cancel-btn:hover {
  background-color: #f9fafb;
  border-color: #d1d5db;
}

.btn-icon {
  width: 1.25rem;
  height: 1.25rem;
}

@media (max-width: 640px) {
  .clientes-direccion-grid {
    grid-template-columns: 1fr; /* Una columna en pantallas pequeñas */
  }
  
  .clientes-form {
    padding: 2rem; /* Reduce el padding en pantallas pequeñas */
    gap: 1rem; /* Reduce el espacio entre los elementos */
  }

  .form-group {
    gap: 0.25rem; /* Reduce el espacio entre etiquetas y campos */
  }
}
</style>