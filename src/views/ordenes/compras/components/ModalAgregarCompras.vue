<template>
  <div class="modal">
    <h2>Agregar Compra</h2>
    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="proveedor">Proveedor:</label>
        <select
          id="proveedor"
          v-model.number="formData.id_proveedor"
          @change="cargarProductosProveedor"
          required
          class="form-control"
        >
          <option :value="null" disabled selected>
            Seleccione un proveedor
          </option>
          <option
            v-for="proveedor in proveedores"
            :key="proveedor.id"
            :value="proveedor.id"
          >
            {{ proveedor.nombre }} (ID: {{ proveedor.id }})
          </option>
        </select>
      </div>

      <h3>Producto del Proveedor</h3>
      <div class="form-group">
        <select
          v-model.number="formData.detalles[0].id_producto"
          required
          class="form-control"
        >
          <option :value="null" disabled selected>
            Seleccione un producto
          </option>
          <option
            v-for="producto in productosProveedor"
            :key="producto.id"
            :value="producto.id"
          >
            {{ producto.producto }} (ID: {{ producto.id }})
          </option>
        </select>
      </div>

      <div class="form-group">
        <input
          v-model.number="formData.detalles[0].cantidad"
          type="number"
          min="1"
          placeholder="Cantidad"
          required
          class="form-control"
        />
      </div>
      <div class="form-group">
        <input
          v-model.number="formData.detalles[0].precio_unitario"
          type="number"
          step="0.01"
          min="0.01"
          placeholder="Precio Unitario"
          required
          class="form-control"
        />
      </div>

      <div class="form-actions">
        <button type="submit" class="btn btn-primary">Guardar Compra</button>
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
    if (
      formData.value.id_proveedor === null ||
      !formData.value.detalles[0].id_producto ||
      !formData.value.detalles[0].cantidad ||
      !formData.value.detalles[0].precio_unitario ||
      formData.value.detalles[0].cantidad <= 0 ||
      formData.value.detalles[0].precio_unitario <= 0
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

    for (const detalle of formData.value.detalles) {
      await invoke("crear_detalle_compra", {
        data: {
          id_compra: compraId,
          id_producto: Number(detalle.id_producto),
          cantidad: Number(detalle.cantidad),
          precio_unitario: Number(detalle.precio_unitario),
        },
      });
    }

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

.btn {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
}

.btn-primary {
  background: #4caf50;
  color: white;
  border: none;
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

h2,
h3 {
  color: #333;
  margin-bottom: 1.5rem;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
}
</style>
