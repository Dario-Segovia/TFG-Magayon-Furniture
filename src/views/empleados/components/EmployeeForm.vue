<template>
    <transition name="modal">
      <div v-if="show" class="employee-form-modal">
        <div class="form-container">
          <div class="form-header">
            <h2>{{ isEditing ? 'Editar Empleado' : 'Nuevo Empleado' }}</h2>
            <button @click="close" class="close-btn">
              <i class="fas fa-times"></i>
            </button>
          </div>
  
          <form @submit.prevent="handleSubmit" class="employee-form">
            <div class="form-group" v-for="(value, key) in formData" :key="key">
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
              <button type="button" @click="close" class="cancel-btn">Cancelar</button>
              <button type="submit" class="submit-btn">{{ isEditing ? 'Actualizar' : 'Guardar' }}</button>
            </div>
          </form>
        </div>
      </div>
    </transition>

    <!-- Modal de confirmación -->
    <div v-if="showConfirm" class="confirm-modal-overlay">
      <div class="confirm-modal">
        <p>{{ confirmMessage }}</p>
        <div class="confirm-actions">
          <button @click="cancelAction" class="cancel-btn">No</button>
          <button @click="confirmAction" class="submit-btn">Sí</button>
          
        </div>
      </div>
    </div>
  </template>
  
  <script setup>
  import { watch, ref } from 'vue';
  
  const props = defineProps({
    show: Boolean,
    isEditing: Boolean,
    formData: Object,
    labels: Object,
    requiredFields: Array
  });
  
  const emit = defineEmits(['update:show', 'submit', 'update:formData', 'close']);
  
  const close = () => {
    emit('update:show', false);
    emit('close');
  };
  
  // Observar cambios en formData y emitirlos
  watch(() => props.formData, (newVal) => {
    emit('update:formData', newVal);
  }, { deep: true });
  
  const showConfirm = ref(false);
  const confirmMessage = ref('');
  let submitResolve = null;

  const handleSubmit = async () => {
    if (props.isEditing) {
      confirmMessage.value = '¿Estás seguro de que deseas actualizar este empleado?';
      showConfirm.value = true;
      // Esperar la respuesta del usuario solo si está editando
      await new Promise((resolve) => (submitResolve = resolve));
    } else {
      // Guardar directamente si es creación
      const newEmployee = {
        ...props.formData,
        id: props.formData.id || Date.now()
      };
      emit('submit', newEmployee);
      close();
    }
  };

  const confirmAction = () => {
    const newEmployee = {
      ...props.formData,
      id: props.formData.id || Date.now()
    };
    emit('submit', newEmployee);
    close();
    showConfirm.value = false;
    if (submitResolve) submitResolve();
  };

  const cancelAction = () => {
    showConfirm.value = false;
    if (submitResolve) submitResolve();
  };
  </script>
  
  <style scoped>
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
    background-color: #dc2626;
  }
  
  .submit-btn {
    background-color: var(--accent-color);
    border: none;
    
  }
  
  .submit-btn:hover {
    background-color: #3a5bd9;
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
  
  @media (max-width: 576px) {
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

  .confirm-modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0,0,0,0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
  }
  
  .confirm-modal {
    background: #fff;
    padding: 2rem;
    border-radius: 0.5rem;
    max-width: 350px;
    text-align: center;
    box-shadow: 0 0.5rem 1rem rgba(0,0,0,0.15);
  }
  
  .confirm-modal p {
    margin-bottom: 1.5rem;
    color: var(--text-color);
  }
  
  .confirm-actions {
    display: flex;
    justify-content: center;
    gap: 1rem;
    margin-top: 1.5rem;
  }
  
  .confirm-actions .submit-btn,
  .confirm-actions .cancel-btn {
    flex: 1;
    padding: 0.75rem;
    border-radius: 0.35rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  .confirm-actions .cancel-btn {
    background-color: #dc2626;
    border: none;
    color: white;
  }
  
  .confirm-actions .cancel-btn:hover {
    background-color: #b91c1c;
  }
  
  @media (max-width: 576px) {
    .confirm-modal {
      margin: 0 1rem;
    }
  }
  
  </style>