<template>
  <div class="modal-overlay">
    <div class="modal-container">
      <div class="modal-header">
        <h2>Agregar Compra</h2>
        <button class="close-btn" @click="closeModal">×</button>
      </div>

      <div class="modal-body">
        <form @submit.prevent="handleSubmit" class="modal-form">
          <div class="form-group full-width">
            <label for="proveedor">Proveedor:</label>
            <select
              id="proveedor"
              v-model.number="formData.id_proveedor"
              @change="cargarProductosProveedor"
              required
              class="form-select"
            >
              <option :value="null" disabled selected>Seleccione un proveedor</option>
              <option v-for="proveedor in proveedores" :key="proveedor.id" :value="proveedor.id">
                {{ proveedor.nombre }} (ID: {{ proveedor.id }})
              </option>
            </select>
          </div>

          <div class="form-group full-width">
            <label>Producto del Proveedor:</label>
            <select
              v-model.number="formData.detalles[0].id_producto"
              required
              class="form-select"
            >
              <option :value="null" disabled selected>Seleccione un producto</option>
              <option
                v-for="producto in productosProveedor"
                :key="producto.id"
                :value="producto.id"
              >
                {{ producto.producto }} (ID: {{ producto.id }})
              </option>
            </select>
          </div>

          <div class="form-grid">
            <div class="form-group">
              <label>Cantidad:</label>
              <input
                v-model.number="formData.detalles[0].cantidad"
                type="number"
                min="1"
                required
                class="form-input"
                placeholder="Cantidad"
              />
            </div>
            <div class="form-group">
              <label>Precio Unitario:</label>
              <input
                v-model.number="formData.detalles[0].precio_unitario"
                type="number"
                step="0.01"
                min="0.01"
                required
                class="form-input"
                placeholder="Precio Unitario"
              />
            </div>
          </div>

          <div class="form-actions">
            <button type="submit" class="btn btn-primary">Guardar Compra</button>
            <button type="button" @click="closeModal" class="btn btn-secondary">Cancelar</button>
          </div>

          <div v-if="errorMessage" class="alert alert-error">
            {{ errorMessage }}
          </div>
        </form>
      </div>
    </div>
  </div>
</template>



<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits(["close", "refresh"]);

const proveedores = ref([]);
const productosProveedor = ref([]);
const errorMessage = ref("");
const formData = ref({


  id_proveedor: null,
  detalles: [
    {
      id_producto: null,
      cantidad: null,
      precio_unitario: null,
    },
  ],
});

onMounted(async () => {
  try {
    proveedores.value = await invoke("obtener_proveedores");
  } catch (err) {
    errorMessage.value = `Error al cargar proveedores: ${err}`;
  }
});

const cargarProductosProveedor = async () => {
  try {
    productosProveedor.value = await invoke("obtener_productos_proveedor", {
      proveedorId: formData.value.id_proveedor,
    });
    formData.value.detalles[0].id_producto = null;
  } catch (err) {
    errorMessage.value = `Error al cargar productos del proveedor: ${err}`;
  }
};

const calcularTotal = () => {
  return formData.value.detalles.reduce((total, detalle) => {
    return total + detalle.cantidad * detalle.precio_unitario;
  }, 0);
};

const handleSubmit = async () => {
  errorMessage.value = "";

  try {
    const detalle = formData.value.detalles[0];

    if (
      formData.value.id_proveedor === null ||
      !detalle.id_producto ||
      !detalle.cantidad ||
      !detalle.precio_unitario ||
      detalle.cantidad <= 0 ||
      detalle.precio_unitario <= 0
    ) {
      throw new Error(
        "Complete todos los campos con valores válidos y seleccione un proveedor y un producto"
      );
    }

    const compraId = await invoke("crear_compra", {
      data: {
        id_proveedor: Number(formData.value.id_proveedor),
        total: parseFloat(calcularTotal().toFixed(2)),
      },
    });

    await invoke("crear_detalle_compra", {
  data: {
    id_compra: compraId,
    cantidad: detalle.cantidad,
    precio_unitario: detalle.precio_unitario,
    nombre: productosProveedor.value.find(p => p.id === detalle.id_producto)?.producto || "Producto desconocido",
    descripcion: productosProveedor.value.find(p => p.id === detalle.id_producto)?.descripcion || "",
    categoria: productosProveedor.value.find(p => p.id === detalle.id_producto)?.categoria || "",
  },
});


    resetForm();
    emit("refresh");
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
        precio_unitario: null,
      },
    ],
  };
  productosProveedor.value = [];
};

const closeModal = () => {
  emit("close");
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