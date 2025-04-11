<template>
    <transition name="modal">
      <div v-if="show" class="calendar-modal">
        <div class="modal-content">
          <div class="modal-header">
            <h2></h2>
            <button @click="close" class="close-btn">
              <i class="fas fa-times"></i>
            </button>
          </div>
          <div class="modal-body">
            <Calendar 
              :key="calendarKey"
              :empleado-id="employeeId" 
              @close="close" 
              @changes-made="setChangesMade" 
            />
          </div>
          <div class="modal-footer sticky-footer">
            <button @click="cancelChanges" class="cancel-btn">Cancelar</button>
            <button @click="saveChanges" class="submit-btn">Guardar</button>
          </div>
        </div>
      </div>
    </transition>
  </template>
  
  <script setup>
  import { ref, watch } from 'vue';
  import { invoke } from "@tauri-apps/api/core";
  import Calendar from './horarios.vue';
  
  const props = defineProps({
    show: Boolean,
    employeeId: [Number, String],
    employeeName: String
  });
  
  const emit = defineEmits(['update:show', 'close']);
  
  const calendarKey = ref(0);
  const changesMade = ref(false); // Estado para rastrear si se han realizado cambios
  const updatedData = ref(null); // Datos actualizados
  
  watch(() => props.employeeId, () => {
    calendarKey.value++;
  });
  
  // Método para cerrar el modal
  const close = () => {
    emit('update:show', false);
    emit('close');
  };
  
  // Método para manejar cambios realizados en el calendario
  const setChangesMade = (data) => {
    changesMade.value = true;
    updatedData.value = data; // Guardar los datos actualizados
  };
  
  // Método para cancelar cambios
  const cancelChanges = () => {
    if (changesMade.value) {
      // Aquí puedes agregar lógica para revertir los cambios si es necesario
      // Por ejemplo, puedes recargar el calendario o restablecer el estado
    }
    close();
  };
  
  const saveChanges = async () => {
  if (changesMade.value && updatedData.value) {
    try {
      // Asegúrate de que el ID sea un número entero
      const id = parseInt(updatedData.value.id, 10);

      // Llama a la API para guardar los cambios
      await invoke('update_horario', {
        id, // Ahora es un entero
        horario: {
          empleado_id: props.employeeId, // 👈 Agrega el empleado_id
          fecha: updatedData.value.start.split('T')[0],
          hora_inicio: updatedData.value.start.split('T')[1].substring(0, 5),
          hora_fin: updatedData.value.end.split('T')[1].substring(0, 5),
          tipo_turno: updatedData.value.extendedProps.tipo_turno,
          notas: updatedData.value.extendedProps.notas
        }
      });

      console.log('Cambios guardados exitosamente:', updatedData.value);
      changesMade.value = false;
    } catch (error) {
      console.error('Error al guardar los cambios:', error);
    }
  }
  close();
};
  </script>

<style scoped>
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
  display: flex;
  flex-direction: column;
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

.modal-body {
  padding: 1.5rem;
  flex: 1;
  overflow-y: auto;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  padding: 1rem;
  border-top: 1px solid var(--border-color);
  background-color: white;
}

.sticky-footer {
  position: sticky;
  bottom: 0;
  z-index: 10;
}

.cancel-btn, .submit-btn {
  padding: 0.75rem 1.5rem;
  border-radius: 0.35rem;
  font-weight: 600;
  cursor: pointer;
  margin-left: 1rem;
  transition: transform 0.2s ease, background-color 0.2s ease;
}

.cancel-btn {
  background-color: white;
  border: 1px solid var(--border-color);
  color: var(--text-color);
}

.cancel-btn:hover {
  background-color: #dc2626;
 
  transform: scale(1.05);
  
}

.submit-btn {
    background-color: #f8f9fa;
  border: none;
  
}

.submit-btn:hover {
  background-color: #3a5bd9;
  transform: scale(1.05);
}
</style>