<template>
  <div class="productos-proveedor">
    <div class="header">
      <h4>Productos suministrados</h4>
      <button @click="showAgregarProducto = true" class="btn-small">
        <i class="fas fa-plus"></i> Agregar
      </button>
    </div>
    <div v-if="loading" class="loading-state">
      <p>Cargando productos...</p>
    </div>
    <div v-else-if="productos.length === 0" class="empty-state">
      <p>No se han registrado productos para este proveedor</p>
    </div>
    <ul v-else class="productos-list">
      <li v-for="producto in productos" :key="producto.id" class="producto-item">
        <div class="producto-info">
          <span class="producto-nombre">{{ producto.producto }}</span>
          <span class="producto-precio">${{ formatPrecio(producto.precio_unitario) }}</span>
        </div>
        <button @click="eliminarProducto(producto.id)" class="btn-icon danger">
          <i class="fas fa-trash"></i>
        </button>
      </li>
    </ul>

    <!-- Modal overlay -->
    <div v-if="showAgregarProducto" class="modal-overlay">
      <div class="modal-content">
        <ModalAgregarProducto
          :proveedorId="proveedorId"
          @close="showAgregarProducto = false"
          @save="agregarProducto"
        />
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ModalAgregarProducto from "./ModalAgregarProducto.vue";

export default {
  components: {
    ModalAgregarProducto,
  },

  props: {
    proveedorId: {
      type: Number,
      required: true,
    },
  },

  setup(props) {
    const productos = ref([]);
    const showAgregarProducto = ref(false);
    const loading = ref(true);
    const error = ref(null);

    const formatPrecio = (precio) => {
      return Number(precio).toFixed(2);
    };

    const cargarProductos = async () => {
  try {
    loading.value = true;
    error.value = null;
    const result = await invoke("obtener_productos_proveedor", {
      proveedorId: props.proveedorId,
    });
    
    productos.value = result.map(producto => ({
      ...producto,
      // Ensure numeric values are properly converted
      precio_unitario: parseFloat(producto.precio_unitario) || 0
    }));
  } catch (err) {
    console.error("Error al cargar productos:", err);
    error.value = `Error al cargar productos: ${err.message || err}`;
  } finally {
    loading.value = false;
  }
};

const agregarProducto = async (productoData) => {
  try {
    const productoParaEnviar = {
      ...productoData,
      proveedor_id: props.proveedorId,
      precio_unitario: parseFloat(productoData.precio_unitario) || 0
    };
    
    await invoke("agregar_producto_proveedor", {
      producto: productoParaEnviar
    });
    
    await cargarProductos();
    showAgregarProducto.value = false;
  } catch (err) {
    console.error("Error al agregar producto:", err);
    error.value = `Error al agregar producto: ${err.message || err}`;
  }
};

    const eliminarProducto = async (id) => {
      if (!confirm("¿Está seguro de eliminar este producto?")) return;
      
      try {
        await invoke("eliminar_producto_proveedor", { id });
        await cargarProductos();
      } catch (err) {
        console.error("Error al eliminar producto:", err);
        alert(`Error al eliminar producto: ${err.message || err}`);
      }
    };

    onMounted(() => {
      cargarProductos();
    });

    return {
      productos,
      showAgregarProducto,
      loading,
      error,
      agregarProducto,
      eliminarProducto,
      formatPrecio
    };
  },
};
</script>

<style scoped>
.productos-proveedor {
  background: #f9f9f9;
  border-radius: 6px;
  padding: 15px;
  position: relative;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}

.header h4 {
  margin: 0;
  font-size: 1.1em;
  color: #333;
}

.btn-small {
  padding: 5px 10px;
  font-size: 0.85em;
  background: #4caf50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 5px;
  transition: background-color 0.2s;
}

.btn-small:hover {
  background-color: #3d8b40;
}

.loading-state,
.empty-state {
  text-align: center;
  padding: 15px;
  color: #777;
}

.productos-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.producto-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px;
  border-bottom: 1px solid #eee;
  transition: background-color 0.2s;
}

.producto-item:hover {
  background-color: #f0f0f0;
}

.producto-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.producto-nombre {
  font-weight: 500;
}

.producto-precio {
  font-size: 0.9em;
  color: #4caf50;
}

.btn-icon {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  background: transparent;
  transition: background-color 0.2s;
}

.btn-icon.danger {
  color: #f44336;
}

.btn-icon.danger:hover {
  background: #ffebee;
}

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
}

.modal-content {
  background: white;
  padding: 20px;
  border-radius: 8px;
  max-width: 500px;
  width: 90%;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  animation: modal-fade 0.3s ease-out;
}

@keyframes modal-fade {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>