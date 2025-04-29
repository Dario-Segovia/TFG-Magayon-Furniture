<template>
  <div class="modal">
    <h2>Agregar Compra</h2>
    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="proveedor">Proveedor:</label>
        <select
          id="proveedor"
          v-model.number="formData.id_proveedor"
          required
          class="form-control"
        >
          <option :value="null" disabled selected>Seleccione un proveedor</option>
          <option
            v-for="proveedor in proveedores"
            :key="proveedor.id"
            :value="proveedor.id"
          >
            {{ proveedor.nombre }} (ID: {{ proveedor.id }})
          </option>
        </select>
      </div>

      <h3>Productos</h3>
      <div
        v-for="(detalle, index) in formData.detalles"
        :key="index"
        class="detalle-producto"
      >
        <div class="form-group">
          <input
            v-model.number="detalle.id_producto"
            type="number"
            placeholder="ID Producto"
            required
            min="1"
            class="form-control"
          />
        </div>
        <div class="form-group">
          <input
            v-model.number="detalle.cantidad"
            type="number"
            min="1"
            placeholder="Cantidad"
            required
            class="form-control"
          />
        </div>
        <div class="form-group">
          <input
            v-model.number="detalle.precio_unitario"
            type="number"
            step="0.01"
            min="0.01"
            placeholder="Precio Unitario"
            required
            class="form-control"
          />
        </div>
        <button
          type="button"
          @click="removeDetalle(index)"
          class="btn btn-danger"
        >
          ×
        </button>
      </div>

      <button type="button" @click="addDetalle" class="btn btn-secondary">
        + Añadir Producto
      </button>

      <div class="form-actions">
        <button type="submit" class="btn btn-primary">
          Guardar Compra
        </button>
        <button type="button" @click="closeModal" class="btn btn-outline">
          Cancelar
        </button>
      </div>

      <div v-if="errorMessage" class="alert alert-error">
        {{ errorMessage }}
      </div>
    </form>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const emit = defineEmits(['close', 'refresh']);

const proveedores = ref([]);
const errorMessage = ref('');
const formData = ref({
  id_proveedor: null,
  detalles: [
    {
      id_producto: null,
      cantidad: null,
      precio_unitario: null
    }
  ]
});

onMounted(async () => {
  try {
    proveedores.value = await invoke('obtener_proveedores');
  } catch (err) {
    errorMessage.value = `Error al cargar proveedores: ${err}`;
  }
});

const addDetalle = () => {
  formData.value.detalles.push({
    id_producto: null,
    cantidad: null,
    precio_unitario: null
  });
};

const removeDetalle = (index) => {
  if (formData.value.detalles.length > 1) {
    formData.value.detalles.splice(index, 1);
  } else {
    errorMessage.value = 'Debe haber al menos un producto';
  }
};

const calcularTotal = () => {
  return formData.value.detalles.reduce((total, detalle) => {
    return total + (detalle.cantidad * detalle.precio_unitario);
  }, 0);
};

const handleSubmit = async () => {
  errorMessage.value = '';
  
  try {
    if (formData.value.id_proveedor === null) {
      throw new Error('Debe seleccionar un proveedor');
    }

    const detallesInvalidos = formData.value.detalles.some(d => 
      !d.id_producto || !d.cantidad || !d.precio_unitario ||
      d.cantidad <= 0 || d.precio_unitario <= 0
    );

    if (detallesInvalidos) {
      throw new Error('Complete todos los campos de producto con valores válidos');
    }

    const compraId = await invoke('crear_compra', {
      idProveedor: Number(formData.value.id_proveedor),  // Enviamos en camelCase
      total: parseFloat(calcularTotal().toFixed(2))
    });

    for (const detalle of formData.value.detalles) {
      await invoke('crear_detalle_compra', {
        id_compra: compraId,
        id_producto: Number(detalle.id_producto),
        cantidad: Number(detalle.cantidad),
        precio_unitario: Number(detalle.precio_unitario)
      });
    }

    resetForm();
    emit('refresh');
    closeModal();
    
  } catch (err) {
    errorMessage.value = `Error al guardar: ${err}`;
  }
};

const resetForm = () => {
  formData.value = {
    id_proveedor: null,
    detalles: [
      {
        id_producto: null,
        cantidad: null,
        precio_unitario: null
      }
    ]
  };
};

const closeModal = () => {
  emit('close');
};
</script>

<style scoped>
.modal {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  max-width: 800px;
  margin: 0 auto;
  box-shadow: 0 2px 20px rgba(0, 0, 0, 0.1);
}

.form-group {
  margin-bottom: 1rem;
}

.form-control {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
}

.detalle-producto {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr auto;
  gap: 1rem;
  align-items: center;
  margin-bottom: 1rem;
  padding: 1rem;
  background: #f8f9fa;
  border-radius: 4px;
}

.btn {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
}

.btn-primary {
  background: #4CAF50;
  color: white;
  border: none;
}

.btn-secondary {
  background: #2196F3;
  color: white;
  border: none;
  margin-bottom: 1rem;
}

.btn-danger {
  background: #f44336;
  color: white;
  border: none;
  padding: 0.5rem;
  width: 2rem;
  height: 2rem;
}

.btn-outline {
  background: transparent;
  border: 1px solid #ddd;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 2rem;
}

.alert {
  padding: 1rem;
  border-radius: 4px;
  margin-top: 1rem;
}

.alert-error {
  background: #ffebee;
  color: #f44336;
}

h2, h3 {
  color: #333;
  margin-bottom: 1.5rem;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
}
</style>