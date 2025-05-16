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
                {{ proveedor.nombre }}
              </option>
            </select>
          </div>

          <!-- Fila para agregar producto -->
          <div class="agregar-producto-row">
            <select v-model="productoSeleccionado" class="form-select">
              <option disabled value="">Producto</option>
              <option v-for="producto in productosProveedor" :key="producto.id" :value="producto.id">
                {{ producto.producto }}
              </option>
            </select>
            <input type="number" v-model.number="cantidad" min="1" class="form-input cantidad-input" placeholder="Cantidad" />
            <button @click.prevent="agregarProducto" class="btn btn-primary btn-agregar" :disabled="!productoSeleccionado || cantidad <= 0">
              <span>Agregar</span>
            </button>
          </div>

          <!-- Tabla de productos agregados -->
          <div v-if="formData.detalles.length > 0" class="productos-table-container">
            <table class="productos-table">
              <thead>
                <tr>
                  <th>Producto</th>
                  <th>Cant.</th>
                  <th>Precio unit.</th>
                  <th>Subtotal</th>
                  <th></th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(detalle, index) in formData.detalles" :key="index" class="fade-in-row">
                  <td>{{ detalle.nombre }}</td>
                  <td>{{ detalle.cantidad }}</td>
                  <td>{{ detalle.precio_unitario }} €</td>
                  <td>{{ (detalle.cantidad * detalle.precio_unitario).toFixed(2) }} €</td>
                  <td>
                    <button @click="eliminarProducto(index)" class="btn btn-secondary btn-eliminar" title="Eliminar">
                      🗑️
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
            <div class="total-row">
              <span>Total:</span>
              <span class="total-valor">{{ totalCompra }} €</span>
            </div>
          </div>

          <div class="form-actions">
            <button type="submit" class="btn btn-primary" :disabled="formData.detalles.length === 0">Guardar Compra</button>
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
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits(["close", "refresh"]);

const proveedores = ref([]);
const productosProveedor = ref([]);
const errorMessage = ref("");
const formData = ref({
  id_proveedor: null,
  detalles: [],
});

const productoSeleccionado = ref("");
const cantidad = ref(1);

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
    productoSeleccionado.value = "";
  } catch (err) {
    errorMessage.value = `Error al cargar productos del proveedor: ${err}`;
  }
};

const agregarProducto = () => {
  if (!productoSeleccionado.value || cantidad.value <= 0) return;
  const producto = productosProveedor.value.find(p => p.id === productoSeleccionado.value);
  if (!producto) return;
  const indexExistente = formData.value.detalles.findIndex(
    d => d.id_producto === producto.id
  );
  if (indexExistente >= 0) {
    formData.value.detalles[indexExistente].cantidad += Number(cantidad.value);
  } else {
    formData.value.detalles.push({
      id_producto: producto.id,
      nombre: producto.producto,
      cantidad: Number(cantidad.value),
      precio_unitario: Number(producto.precio_unitario), // ← asegúrate que es número
    });
  }
  productoSeleccionado.value = "";
  cantidad.value = 1;
};

const eliminarProducto = (index) => {
  formData.value.detalles.splice(index, 1);
};

const totalCompra = computed(() =>
  formData.value.detalles.reduce((acc, d) => acc + d.cantidad * d.precio_unitario, 0).toFixed(2)
);

const handleSubmit = async () => {
  errorMessage.value = "";
  try {
    if (
      formData.value.id_proveedor === null ||
      formData.value.detalles.length === 0
    ) {
      throw new Error(
        "Complete todos los campos con valores válidos y seleccione un proveedor y al menos un producto"
      );
    }
    // Crea la compra
    const compraId = await invoke("crear_compra", {
      data: {
        id_proveedor: Number(formData.value.id_proveedor),
        total: parseFloat(totalCompra.value),
      },
    });
    // Crea los detalles
    for (const detalle of formData.value.detalles) {
      await invoke("crear_detalle_compra", {
        data: {
          id_compra: compraId,
          id_producto: Number(detalle.id_producto),
          nombre: detalle.nombre, // ← CAMBIA ESTO
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
    detalles: [],
  };
  productosProveedor.value = [];
  productoSeleccionado.value = "";
  cantidad.value = 1;
};

const closeModal = () => {
  emit("close");
};
</script>

<style scoped>
/* Puedes copiar los estilos de ModalAgregarVentas.vue para mantener coherencia visual */
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
  from { opacity: 0; transform: translateY(-20px);}
  to { opacity: 1; transform: translateY(0);}
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
.close-btn:hover { color: #e74c3c; }
.modal-body { padding: 25px; }
.modal-form { display: flex; flex-direction: column; gap: 20px; }
.form-group { display: flex; flex-direction: column; gap: 8px; }
.form-group.full-width { grid-column: 1 / -1; }
.form-group label { font-size: 0.9rem; color: #34495e; font-weight: 500; }
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
.agregar-producto-row {
  display: flex;
  gap: 12px;
  align-items: flex-end;
  margin-bottom: 12px;
}
.cantidad-input { width: 80px; }
.btn-agregar {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 1rem;
  padding: 10px 18px;
}
.productos-table-container { margin-top: 18px; }
.productos-table {
  width: 100%;
  border-collapse: collapse;
  margin-bottom: 8px;
  background: #f8f9fa;
  border-radius: 8px;
  overflow: hidden;
}
.productos-table th, .productos-table td {
  padding: 8px 10px;
  text-align: left;
}
.productos-table th {
  background: #e0e7ff;
  color: #405890;
  font-weight: 600;
}
.productos-table td { background: #fff; }
.btn-eliminar {
  color: #f44336;
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  border-radius: 50%;
  transition: background 0.2s;
}
.btn-eliminar:hover { background: #ffeaea; }
.total-row {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  gap: 12px;
  font-size: 1.1em;
  font-weight: 600;
  color: #405890;
  margin-top: 6px;
}
.total-valor {
  color: #2196f3;
  font-size: 1.2em;
}
.fade-in-row {
  animation: fadeIn 0.3s;
}
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-10px);}
  to { opacity: 1; transform: translateY(0);}
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
.alert-error {
  color: #e74c3c;
  background: #ffeaea;
  padding: 10px;
  border-radius: 6px;
  margin-top: 10px;
}
</style>