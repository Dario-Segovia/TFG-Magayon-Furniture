<template>
  <div v-if="visible" class="modal-overlay">
    <div class="modal-container">
      <div class="modal-header">
        <h2>Modificar Compra</h2>
        <button class="close-btn" @click="cerrarModal">×</button>
      </div>
      <div class="modal-body">
        <form @submit.prevent="intentarGuardar" class="modal-form">
          <div class="form-group full-width">
            <label for="proveedor">Proveedor:</label>
            <select
              id="proveedor"
              v-model.number="compraParaEditar.id_proveedor"
              required
              class="form-select"
            >
              <option v-for="proveedor in compraParaEditar.proveedores" :key="proveedor.id" :value="proveedor.id">
                {{ proveedor.nombre }}
              </option>
            </select>
          </div>

          <!-- Fila para agregar producto -->
          <div class="agregar-producto-row">
            <select v-model="productoSeleccionado" class="form-select">
              <option disabled value="">Producto</option>
              <option v-for="producto in productosProveedor" :key="producto.id_producto || producto.id" :value="producto.id_producto || producto.id">
                {{ producto.nombre || producto.producto }}
              </option>
            </select>
            <input type="number" v-model.number="cantidad" min="1" class="form-input cantidad-input" placeholder="Cantidad" />
            <button @click.prevent="agregarProducto" class="btn btn-primary btn-agregar" :disabled="!productoSeleccionado || cantidad <= 0">
              <span>Agregar</span>
            </button>
          </div>

          <!-- Tabla de productos agregados -->
          <div v-if="compraParaEditar.productos.length > 0" class="productos-table-container">
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
                <tr v-for="(detalle, index) in compraParaEditar.productos" :key="index" class="fade-in-row">
                  <td>{{ detalle.nombre }}</td>
                  <td>
                    <div class="cantidad-control">
                      <button type="button" class="btn-cantidad" @click="modificarCantidad(index, -1)" :disabled="detalle.cantidad <= 1">−</button>
                      <input type="number" v-model.number="detalle.cantidad" min="1" class="form-input cantidad-input" style="width:60px;text-align:center;" />
                      <button type="button" class="btn-cantidad" @click="modificarCantidad(index, 1)">+</button>
                    </div>
                  </td>
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
            <button type="button" class="btn btn-primary" :disabled="compraParaEditar.productos.length === 0" @click="intentarGuardar">
              Guardar Cambios
            </button>
            <button type="button" class="btn btn-secondary" @click="cerrarModal">Cancelar</button>
          </div>
        </form>
      </div>
    </div>
  </div>

  <!-- Modal de confirmación -->
  <div v-if="mostrarConfirmacion" class="modal-overlay" style="z-index:2000;">
    <div class="modal-container" style="max-width:350px;">
      <div class="modal-header">
        <h2>Confirmar</h2>
      </div>
      <div class="modal-body">
        <p>¿Estás seguro de que deseas guardar los cambios en esta compra?</p>
        <div class="form-actions">
          <button class="btn btn-primary" @click="confirmarGuardar">Sí, guardar</button>
          <button class="btn btn-secondary" @click="cancelarGuardar">Cancelar</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  visible: Boolean,
  compraSeleccionada: Object
});
const emit = defineEmits(['cerrar', 'guardar', 'refresh']);

const compraParaEditar = ref({
  id: null,
  id_proveedor: null,
  proveedores: [],
  productos: [],
  fecha: '',
  total: 0
});

const productosProveedor = ref([]);
const productoSeleccionado = ref("");
const cantidad = ref(1);
const precioUnitario = ref(0);
const mostrarConfirmacion = ref(false);

watch(() => props.compraSeleccionada, (nuevaCompra) => {
  if (nuevaCompra) {
    compraParaEditar.value = {
      ...nuevaCompra,
      proveedores: nuevaCompra.proveedores || [
        { id: nuevaCompra.id_proveedor, nombre: nuevaCompra.nombre_proveedor }
      ],
      productos: nuevaCompra.productos?.map(producto => ({
        ...producto,
        id_producto: producto.id_producto || producto.id || null
      })) || [],
    };
    cargarProductosProveedor();
  }
}, { immediate: true });

const cargarProductosProveedor = async () => {
  if (!compraParaEditar.value.id_proveedor) return;
  try {
    productosProveedor.value = await invoke("obtener_productos_proveedor", {
      proveedorId: compraParaEditar.value.id_proveedor,
    });
  } catch (err) {
    productosProveedor.value = [];
  }
};

watch(() => compraParaEditar.value.id_proveedor, () => {
  cargarProductosProveedor();
});

const agregarProducto = async () => {
  if (!productoSeleccionado.value || cantidad.value <= 0) return;
  const productoProveedor = productosProveedor.value.find(
    p => (p.id_producto || p.id) === productoSeleccionado.value
  );
  if (!productoProveedor) return;

  // Buscar el producto en inventario por nombre y categoría
  const inventario = await invoke('buscar_producto_inventario', {
    nombre: productoProveedor.producto || productoProveedor.nombre,
    categoria: productoProveedor.categoria
  });
  if (!inventario || !inventario.id) {
    errorMessage.value = "El producto no existe en inventario.";
    return;
  }

  const indexExistente = compraParaEditar.value.productos.findIndex(
    d => Number(d.id_producto) === Number(inventario.id)
  );
  if (indexExistente >= 0) {
    compraParaEditar.value.productos[indexExistente].cantidad += cantidad.value;
  } else {
    compraParaEditar.value.productos.push({
      id_producto: inventario.id, // Usar el id de inventario
      nombre: productoProveedor.producto || productoProveedor.nombre,
      categoria: productoProveedor.categoria,
      cantidad: cantidad.value,
      precio_unitario: productoProveedor.precio_unitario,
    });
  }
  productoSeleccionado.value = "";
  cantidad.value = 1;
};

const eliminarProducto = (index) => {
  compraParaEditar.value.productos.splice(index, 1);
};

const modificarCantidad = (index, delta) => {
  const detalle = compraParaEditar.value.productos[index];
  if (!detalle) return;
  const nuevaCantidad = detalle.cantidad + delta;
  if (nuevaCantidad >= 1) {
    detalle.cantidad = nuevaCantidad;
  }
};

const totalCompra = computed(() =>
  compraParaEditar.value.productos.reduce((acc, d) => acc + d.cantidad * d.precio_unitario, 0).toFixed(2)
);

function formatearFecha(fechaStr) {
  const fecha = typeof fechaStr === 'string' ? new Date(fechaStr) : fechaStr;
  const yyyy = fecha.getFullYear();
  const mm = String(fecha.getMonth() + 1).padStart(2, '0');
  const dd = String(fecha.getDate()).padStart(2, '0');
  const hh = String(fecha.getHours()).padStart(2, '0');
  const min = String(fecha.getMinutes()).padStart(2, '0');
  const ss = String(fecha.getSeconds()).padStart(2, '0');
  return `${yyyy}-${mm}-${dd} ${hh}:${min}:${ss}`;
}

const guardarCambios = async () => {
  try {
    const fechaFormateada = formatearFecha(compraParaEditar.value.fecha);
    await invoke('actualizar_compra', {
      detalles: compraParaEditar.value.productos.map(detalle => ({
        id_compra: compraParaEditar.value.id,
        fecha: formatearFecha(compraParaEditar.value.fecha),
        id_proveedor: compraParaEditar.value.id_proveedor,
        total: parseFloat(totalCompra.value),
        id_producto: Number(detalle.id_producto),
        nombre_producto: detalle.nombre,
        cantidad: Number(detalle.cantidad),
        precio_unitario: Number(detalle.precio_unitario),
      }))
    });
    emit('guardar', {
      ...compraParaEditar.value,
      total: parseFloat(totalCompra.value),
      productos: [...compraParaEditar.value.productos]
    });
    cerrarModal();
    emit('refresh');
  } catch (error) {
    console.error('Error guardando los cambios:', error);
  }
};

const intentarGuardar = () => {
  mostrarConfirmacion.value = true;
};

const confirmarGuardar = async () => {
  mostrarConfirmacion.value = false;
  await guardarCambios();
};

const cancelarGuardar = () => {
  mostrarConfirmacion.value = false;
};

const cerrarModal = () => {
  emit('cerrar');
};

</script>

<style scoped>
/* Usa los mismos estilos que ModalAgregarCompras.vue para coherencia */
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
.cantidad-control {
  display: flex;
  align-items: center;
  gap: 8px;
}
.btn-cantidad {
  background-color: #e0e7ff;
  color: #405890;
  border: none;
  border-radius: 4px;
  padding: 6px 10px;
  cursor: pointer;
  transition: background 0.2s;
}
.btn-cantidad:disabled {
  background-color: #f0f0f0;
  color: #b0b0b0;
  cursor: not-allowed;
}
.btn-cantidad:hover:not(:disabled) {
  background-color: #d1e7ff;
}
</style>