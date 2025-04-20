<template>
  <div class="modal-overlay">
    <div class="modal-container fade-in">
      <div class="modal-header">
        <h2>Editar Ítem</h2>
        <button @click="$emit('close')" class="close-btn" aria-label="Cerrar">✕</button>
      </div>
      <div class="modal-body">
        <form @submit.prevent="guardar" class="modal-form">
          <div class="form-group">
            <label>Nombre*</label>
            <input v-model="nombre" required class="form-input" />
          </div>
          <div class="form-group">
            <label>Descripción</label>
            <input v-model="descripcion" class="form-input" />
          </div>
          <div class="form-group">
            <label>Cantidad*</label>
            <input v-model.number="cantidad" type="number" min="0" required class="form-input" />
          </div>
          <div class="form-group">
            <label>Precio unitario*</label>
            <input v-model.number="precio_unitario" type="number" min="0" step="0.01" required class="form-input" />
          </div>
          <div class="form-group">
            <label>Categoría</label>
            <input v-model="categoria" class="form-input" />
          </div>
          <div class="form-actions">
            <button type="button" @click="$emit('close')" class="btn btn-secondary">Cancelar</button>
            <button type="submit" class="btn btn-primary">Guardar</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';
const emit = defineEmits(['close', 'save']);
const props = defineProps({ item: Object, proveedores: Array });

const nombre = ref(props.item.nombre);
const descripcion = ref(props.item.descripcion);
const cantidad = ref(props.item.cantidad);
const precio_unitario = ref(props.item.precio_unitario);
const categoria = ref(props.item.categoria);

watch(() => props.item, (val) => {
  nombre.value = val.nombre;
  descripcion.value = val.descripcion;
  cantidad.value = val.cantidad;
  precio_unitario.value = val.precio_unitario;
  categoria.value = val.categoria;
});

const guardar = () => {
  emit('save', {
    id: props.item.id,  // Asegúrate de incluir el ID
    nombre: nombre.value,
    descripcion: descripcion.value,
    cantidad: cantidad.value,
    precio_unitario: precio_unitario.value,
    categoria: categoria.value,
  });
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  backdrop-filter: blur(3px);
}

.modal-container {
  background-color: white;
  border-radius: 12px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
  width: 90%;
  max-width: 700px;
  max-height: 90vh;
  overflow-y: auto;
  animation: modalFadeIn 0.3s ease-out;
}

@keyframes modalFadeIn {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 25px;
  border-bottom: 1px solid #f0f0f0;
}

.modal-header h2 {
  margin: 0;
  font-size: 1.5rem;
  color: #2c3e50;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.2rem;
  color: #7f8c8d;
  cursor: pointer;
  transition: color 0.2s;
  padding: 5px;
}

.close-btn:hover {
  color: #e74c3c;
}

.modal-body {
  padding: 25px;
}

.modal-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 20px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-group.full-width {
  grid-column: 1 / -1;
}

.form-group label {
  font-size: 0.9rem;
  color: #34495e;
  font-weight: 500;
}

.form-input, .form-select {
  padding: 12px 15px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 0.95rem;
  transition: border-color 0.2s, box-shadow 0.2s;
}

.form-input:focus, .form-select:focus {
  outline: none;
  border-color: #3498db;
  box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1);
}

.checkbox-group {
  flex-direction: row;
  align-items: center;
  margin-top: 10px;
}

.checkbox-container {
  display: flex;
  align-items: center;
  cursor: pointer;
  position: relative;
  user-select: none;
}

.checkbox-container input {
  position: absolute;
  opacity: 0;
  cursor: pointer;
  height: 0;
  width: 0;
}

.checkmark {
  height: 20px;
  width: 20px;
  background-color: white;
  border: 1px solid #ddd;
  border-radius: 4px;
  margin-right: 10px;
  transition: background-color 0.2s;
}

.checkbox-container:hover input ~ .checkmark {
  background-color: #f8f9fa;
}

.checkbox-container input:checked ~ .checkmark {
  background-color: #3498db;
  border-color: #3498db;
}

.checkmark:after {
  content: "";
  position: absolute;
  display: none;
}

.checkbox-container input:checked ~ .checkmark:after {
  display: block;
}

.checkbox-container .checkmark:after {
  left: 7px;
  top: 3px;
  width: 5px;
  height: 10px;
  border: solid white;
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 15px;
  padding-top: 20px;
  border-top: 1px solid #f0f0f0;
  margin-top: 10px;
}

.btn {
  padding: 12px 20px;
  border-radius: 8px;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.2s;
  border: none;
}

.btn-primary {
  background-color: #3498db;
  color: white;
}

.btn-primary:hover {
  background-color: #2980b9;
  transform: translateY(-1px);
}

.btn-secondary {
  background-color: #f8f9fa;
  color: #34495e;
}

.btn-secondary:hover {
  background-color: #e9ecef;
}

@media (max-width: 600px) {
  .form-grid {
    grid-template-columns: 1fr;
  }
  
  .form-actions {
    flex-direction: column-reverse;
  }
  
  .btn {
    width: 100%;
    justify-content: center;
  }
}
</style>