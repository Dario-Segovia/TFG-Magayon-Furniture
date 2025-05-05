<template>
  <div class="card fade-in">
    <div class="card-header">
      <h3>{{ compra.nombre_proveedor }}</h3>
    </div>

    <div class="card-content">
      <p class="proveedor"><strong>Proveedor:</strong> {{ compra.nombre_proveedor }}</p>

      <div v-if="compra.productos && compra.productos.length">
        <p><strong>Producto:</strong>
          <span>
            <span v-for="(producto, index) in compra.productos" :key="producto.id">
              {{ producto.nombre }}<span v-if="index !== compra.productos.length - 1">, </span>
            </span>
          </span>
        </p>
      </div>
      <p v-else><strong>Producto:</strong> No hay productos registrados</p>

      <div class="precios">
        <span><strong>Fecha:</strong> {{ formatearFecha(compra.fecha) }}</span>
        <span class="compra"><strong>Total:</strong> {{ compra.total }}€</span>
      </div>
    </div>

    <div class="card-actions">
      <button class="btn-action edit" @click="$emit('editar', compra)">
        ✏️
      </button>
      <button class="btn-action delete" @click="$emit('eliminar', compra)">
        🗑️
      </button>
    </div>
  </div>
</template>






<script setup>
defineProps({
  compra: {
    type: Object,
    default: () => ({ productos: [] }), // Asegura que productos sea un array vacío por defecto
  },
});

const formatearFecha = (fechaStr) => {
  const fecha = new Date(fechaStr);
  if (isNaN(fecha)) {
    return "Fecha inválida"; // Añadido para manejar fechas inválidas
  }
  return fecha.toLocaleString("es-ES", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
};
</script>

<style scoped>

.card {
  background: white;
  border-radius: 8px;
  padding: 18px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.10);
  display: flex;
  flex-direction: column;
  gap: 10px;
  position: relative;
  animation: fadeIn 0.5s;
}
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}
.card-header h3 {
  margin: 0;
  font-size: 1.15em;
  color: #333;
}
.cantidad {
  background: #e8f5e9;
  color: #388e3c;
  border-radius: 12px;
  padding: 3px 10px;
  font-size: 0.95em;
  font-weight: 600;
}
.card-content {
  font-size: 0.98em;
  color: #555;
  margin-bottom: 8px;
}
.precios {
  display: flex;
  gap: 15px;
  margin: 6px 0;
}
.compra {
  color: #2196f3;
}
.venta {
  color: #4caf50;
}
.proveedor {
  margin-top: 4px;
  color: #888;
  font-size: 0.95em;
}
.card-actions {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
}
.btn-action {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background 0.2s;
  background: #f1f1f1;
  color: #2196f3;
}
.btn-action.edit {
  color: #4caf50;
}
.btn-action.delete {
  color: #f44336;
}
.btn-action:hover {
  background: #e0e0e0;
}
.fade-in {
  animation: fadeIn 0.5s;
}
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}
</style>

