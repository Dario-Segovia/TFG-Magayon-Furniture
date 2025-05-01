<template>
  <div class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-container">
      <div class="modal-header">
        <h3>Nuevo Producto</h3>
        <button @click="$emit('close')" class="close-btn" aria-label="Cerrar">✕</button>
      </div>

      <div class="modal-body">
        <form @submit.prevent="guardarProducto" class="modal-form">
          <div class="form-group">
            <label for="nombreProducto">Nombre del Producto</label>
            <input
              id="nombreProducto"
              v-model="producto"
              required
              class="form-input"
              placeholder="Escribe el nombre..."
            />
          </div>

          <div class="form-group">
            <label for="descripcion">Descripción</label>
            <textarea
              id="descripcion"
              v-model="descripcion"
              class="form-input"
              placeholder="Describe el producto..."
            ></textarea>
          </div>

          <div class="form-group">
            <label for="precio">Precio Unitario (€)</label>
            <input
  id="precio"
  type="number"
  min="0"
  step="0.01"
  v-model.number="precio_unitario"
  required
  class="form-input"
  placeholder="0.00"
  @input="precio_unitario = $event.target.valueAsNumber || 0"
/>
          </div>

          <div class="form-group">
            <label for="categoria">Categoría</label>
            <input
              id="categoria"
              v-model="categoria"
              class="form-input"
              placeholder="Ej. Tela, Cuero, Repuesto..."
            />
          </div>

          <div class="form-actions">
            <button type="button" @click="$emit('close')" class="btn btn-secondary">
              Cancelar
            </button>
            <button type="submit" class="btn btn-primary">
              Agregar
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const props = defineProps({
  proveedorId: { type: Number, required: true }
});

const emit = defineEmits(['close', 'save']);

const producto = ref('');
const descripcion = ref('');
const precio_unitario = ref(0.00);
const categoria = ref('');

const guardarProducto = () => {
  const precio = parseFloat(precio_unitario.value);
  if (isNaN(precio)) {
    alert('Ingrese un precio válido');
    return;
  }

  const nuevoProducto = {
    proveedor_id: props.proveedorId,
    producto: producto.value.trim(),
    descripcion: descripcion.value.trim() || null,
    precio_unitario: precio.toFixed(2), // Envía como string con 2 decimales
    categoria: categoria.value.trim() || null
  };

  emit('save', nuevoProducto);
};
</script>


<style scoped>
.modal-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 50;
  animation: fadeIn 0.3s ease-out; /* Animación de fondo */
}

.modal-container {
  background: #fff;
  border-radius: 8px;
  padding: 1rem;
  width: 90%;
  max-width: 320px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transform: translateY(-30px); /* Inicia desde arriba */
  animation: slideUp 0.5s ease-out forwards; /* Animación de entrada */
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.modal-header h3 {
  margin: 0;
  font-size: 1.1rem;
  color: #333;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.3rem;
  color: #777;
  cursor: pointer;
}

.close-btn:hover {
  color: #333;
}

.modal-body {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.modal-form {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.form-group {
  display: flex;
  flex-direction: column;
}

.form-group label {
  font-size: 0.9rem;
  color: #555;
  margin-bottom: 4px;
}

.form-input {
  width: 100%;
  box-sizing: border-box;
  padding: 0.6rem 0.8rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 0.9rem;
  outline: none;
}

.form-input:focus {
  border-color: #4CAF50;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
  margin-top: 1rem;
}

.btn {
  padding: 0.4rem 0.8rem;
  font-size: 0.85rem;
  border-radius: 4px;
  font-weight: 500;
  cursor: pointer;
  border: none;
}

.btn-primary {
  background: #4CAF50;
  color: white;
}

.btn-primary:hover {
  background: #45A049;
}

.btn-secondary {
  background: #f1f1f1;
  color: #333;
}

.btn-secondary:hover {
  background: #e2e2e2;
}

/* Animación de desvanecimiento del fondo */
@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

/* Animación de deslizamiento del modal */
@keyframes slideUp {
  from {
    transform: translateY(-30px);
  }
  to {
    transform: translateY(0);
  }
}
</style>
