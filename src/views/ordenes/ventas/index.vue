<template>
  <div class="p-6 space-y-4">
    <h1 class="text-2xl font-bold">{{ ventaEditando ? 'Editar Venta' : 'Crear Venta' }}</h1>

    <div>
      <label class="block mb-1 font-medium">Cliente</label>
      <select v-model="venta.id_cliente" class="w-full border p-2 rounded">
        <option disabled value="">Seleccione un cliente</option>
        <option v-for="cliente in clientes" :key="cliente.id" :value="cliente.id">
          {{ cliente.nombre }}
        </option>
      </select>
    </div>

    <div>
      <label class="block mb-1 font-medium">Producto</label>
      <select v-model="productoSeleccionado" class="w-full border p-2 rounded">
        <option disabled value="">Seleccione un producto</option>
        <option v-for="producto in inventario" :key="producto.id" :value="producto.id">
          {{ producto.nombre }} ({{ producto.categoria }})
        </option>
      </select>
      <div class="mt-2">
        <label class="block mb-1">Cantidad</label>
        <input type="number" v-model.number="cantidad" class="w-full border p-2 rounded" />
      </div>
      <button @click="agregarProducto" class="mt-2 px-4 py-2 bg-blue-500 text-white rounded">
        Agregar Producto
      </button>
    </div>

    <div v-if="venta.detalles.length > 0">
      <h2 class="font-semibold text-lg mt-4">Productos Agregados</h2>
      <ul class="list-disc pl-5">
        <li v-for="(detalle, index) in venta.detalles" :key="index" class="flex items-center">
          {{ detalle.nombre }} - {{ detalle.cantidad }} x {{ detalle.precio_unitario }}€
          <button @click="eliminarProducto(index)" class="ml-2 text-red-500">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
            </svg>
          </button>
        </li>
      </ul>
    </div>

    <div class="flex space-x-4">
      <button 
        @click="ventaEditando ? actualizarVenta() : crearVenta()" 
        class="px-6 py-2 bg-green-600 text-white rounded"
        :disabled="venta.detalles.length === 0"
      >
        {{ ventaEditando ? 'Actualizar Venta' : 'Guardar Venta' }}
      </button>
      <button 
        v-if="ventaEditando"
        @click="cancelarEdicion" 
        class="px-6 py-2 bg-gray-500 text-white rounded"
      >
        Cancelar
      </button>
    </div>

    <!-- VENTAS REGISTRADAS -->
    <div class="mt-10">
      <h2 class="text-xl font-bold mb-2">Ventas Registradas</h2>
      <div v-for="v in ventas" :key="v.id" class="border rounded p-4 mb-4 shadow">
        <div class="flex justify-between items-start">
          <div>
            <h3 class="font-semibold">Venta #{{ v.id }} - {{ new Date(v.fecha).toLocaleString() }}</h3>
            <p><strong>Cliente:</strong> {{ v.nombre_cliente || 'Sin cliente' }}</p>
            <p><strong>Total:</strong> {{ v.total }} €</p>
          </div>
          <button 
            @click="editarVenta(v)" 
            class="px-3 py-1 bg-yellow-500 text-white rounded text-sm"
          >
            Editar
          </button>
        </div>

        <ul class="mt-2 list-disc pl-5">
          <li v-for="detalle in v.detalles" :key="detalle.id">
            Producto: {{ detalle.nombre_producto }} - Cantidad: {{ detalle.cantidad }}, Precio unitario: {{ detalle.precio_unitario }} €
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const clientes = ref([]);
const inventario = ref([]);
const productoSeleccionado = ref("");
const cantidad = ref(1);
const ventas = ref([]);
const ventaEditando = ref(null);

const venta = ref({
  id_cliente: '',
  detalles: []
});

const cargarVentas = async () => {
  console.log("Obteniendo datos de ventas desde la base de datos...");
  ventas.value = await invoke('get_ventas');
  console.log("Datos de ventas obtenidos:", JSON.stringify(ventas.value, null, 2));
};

onMounted(async () => {
  clientes.value = await invoke('obtener_clientes');
  inventario.value = await invoke('get_inventory');
  await cargarVentas();
});

const agregarProducto = async () => {
  if (!productoSeleccionado.value || cantidad.value <= 0) return;
  
  const producto = await invoke('get_inventory_item', { id: productoSeleccionado.value });
  
  // Verificar si el producto ya está en los detalles
  const indexExistente = venta.value.detalles.findIndex(
    d => d.id_producto === producto.id
  );
  
  if (indexExistente >= 0) {
    // Si ya existe, actualizar la cantidad
    venta.value.detalles[indexExistente].cantidad += cantidad.value;
  } else {
    // Si no existe, agregarlo
    venta.value.detalles.push({
      id_producto: producto.id,
      cantidad: cantidad.value,
      precio_unitario: producto.precio_unitario,
      nombre: producto.nombre
    });
  }
  
  productoSeleccionado.value = "";
  cantidad.value = 1;
};

const eliminarProducto = (index) => {
  venta.value.detalles.splice(index, 1);
};

const crearVenta = async () => {
  const payload = {
    id_cliente: venta.value.id_cliente ? parseInt(venta.value.id_cliente) : null,
    detalles: venta.value.detalles.map(d => ({
      id_producto: d.id_producto,
      cantidad: d.cantidad,
      precio_unitario: d.precio_unitario
    }))
  };

  try {
    const id = await invoke('crear_venta', { data: payload });
    alert(`Venta creada con ID: ${id}`);
    resetForm();
    await cargarVentas();
  } catch (e) {
    alert('Error al crear la venta: ' + e);
  }
};

const editarVenta = (v) => {
  ventaEditando.value = v.id;
  venta.value = {
    id_cliente: v.id_cliente,
    detalles: [] // Limpiamos los detalles para agregarlos uno por uno
  };
  
  // Mostrar el primer producto en los campos de edición
  if (v.detalles.length > 0) {
    const primerDetalle = v.detalles[0];
    productoSeleccionado.value = primerDetalle.id_producto;
    cantidad.value = primerDetalle.cantidad;
    
    // Agregar los demás productos a la lista de detalles
    if (v.detalles.length > 1) {
      venta.value.detalles = v.detalles.slice(1).map(d => ({
        id_producto: d.id_producto,
        cantidad: d.cantidad,
        precio_unitario: d.precio_unitario,
        nombre: d.nombre_producto
      }));
    }
  }
  
  window.scrollTo({ top: 0, behavior: 'smooth' });
};

const actualizarVenta = async () => {
  // Asegurarnos de incluir el producto que está en los campos de edición
  if (productoSeleccionado.value && cantidad.value > 0) {
    const producto = await invoke('get_inventory_item', { id: productoSeleccionado.value });
    venta.value.detalles.unshift({
      id_producto: producto.id,
      cantidad: cantidad.value,
      precio_unitario: producto.precio_unitario,
      nombre: producto.nombre
    });
  }

  const payload = {
    id_cliente: venta.value.id_cliente ? parseInt(venta.value.id_cliente) : null,
    detalles: venta.value.detalles.map(d => ({
      id_producto: d.id_producto,
      cantidad: d.cantidad,
      precio_unitario: d.precio_unitario
    }))
  };

  try {
    await invoke('update_venta', {
  params: {
    idVenta: ventaEditando.value,
    data: payload
  }
});

    alert('Venta actualizada correctamente');
    resetForm();
    await cargarVentas();
  } catch (e) {
    alert('Error al actualizar la venta: ' + e);
  }
};

const cancelarEdicion = () => {
  resetForm();
};

const resetForm = () => {
  venta.value = {
    id_cliente: '',
    detalles: []
  };
  ventaEditando.value = null;
  productoSeleccionado.value = "";
  cantidad.value = 1;
};
</script>

<style scoped>
input, select {
  outline: none;
}
button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>