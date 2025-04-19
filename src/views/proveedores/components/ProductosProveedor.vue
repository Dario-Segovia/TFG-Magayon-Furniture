<template>
  <div class="productos-proveedor">
    <div class="header">
      <h4>Productos suministrados</h4>
      <button @click="showAgregarProducto = true" class="btn-small">
        <i class="fas fa-plus"></i> Agregar
      </button>
    </div>
    <div v-if="productos.length === 0" class="empty-state">
      <p>No se han registrado productos para este proveedor</p>
    </div>
    <ul v-else class="productos-list">
      <li v-for="producto in productos" :key="producto.id" class="producto-item">
        <span>{{ producto.producto }}</span>
        <button @click="eliminarProducto(producto.id)" class="btn-icon danger">
          <i class="fas fa-trash"></i>
        </button>
      </li>
    </ul>
    <!-- Modal dentro del card -->
    <ModalAgregarProducto
      v-if="showAgregarProducto"
      :proveedorId="proveedorId"
      @close="showAgregarProducto = false"
      @save="agregarProducto"
    />
  </div>
</template>
  
<script>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import ModalAgregarProducto from './ModalAgregarProducto.vue'

export default {
  components: {
    ModalAgregarProducto
  },
  
  props: {
    proveedorId: { // Cambiado a camelCase
      type: Number,
      required: true
    }
  },
  
  setup(props) {
    const productos = ref([]);
    const showAgregarProducto = ref(false);

    const cargarProductos = async () => {
      try {
          productos.value = await invoke('obtener_productos_proveedor', {
proveedorId: props.proveedorId, // aquí ahora es camelCase

        });
      } catch (error) {
        console.error('Error al cargar productos:', error);
      }
    };

    const agregarProducto = async (nuevoProducto) => {
      try {
        await invoke('agregar_producto_proveedor', {
          producto: {
            proveedor_id: props.proveedorId, // Mapeado correctamente al backend
            producto: nuevoProducto, // Solo envía los campos necesarios
          },
        });
        await cargarProductos();
        showAgregarProducto.value = false;
      } catch (error) {
        console.error('Error al agregar producto:', error);
      }
    };

    const eliminarProducto = async (id) => {
      try {
        await invoke('eliminar_producto_proveedor', { id });
        await cargarProductos();
      } catch (error) {
        console.error('Error al eliminar producto:', error);
      }
    };

    onMounted(() => {
      cargarProductos();
    });

    return {
      productos,
      showAgregarProducto,
      agregarProducto,
      eliminarProducto
    };
  }
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
  background: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 5px;
}

.btn-small:hover {
  opacity: 0.9;
}

.empty-state {
  text-align: center;
  padding: 15px;
  color: #777;
  font-style: italic;
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
  padding: 8px 10px;
  border-bottom: 1px solid #eee;
}

.producto-item:last-child {
  border-bottom: none;
}

.btn-icon {
  width: 25px;
  height: 25px;
  border-radius: 50%;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  background: transparent;
}

.btn-icon.danger {
  color: #f44336;
}

.btn-icon.danger:hover {
  background: #ffebee;
}
</style>