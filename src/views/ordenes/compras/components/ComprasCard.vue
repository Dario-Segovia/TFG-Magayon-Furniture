<template>
  <div class="card">
    <p><strong>Proveedor:</strong> {{ compra.nombre_proveedor }}</p>
    
    <div v-if="compra.productos && compra.productos.length">
      <p><strong>Producto: </strong> <!-- Espacio añadido después de "Producto:" -->
        <span>
          <span v-for="(producto, index) in compra.productos" :key="producto.id">
            {{ producto.nombre }}<span v-if="index !== compra.productos.length - 1">, </span>
          </span>
        </span>
      </p>
    </div>
    <p v-else><strong>Producto:</strong> No hay productos registrados</p>
    
    <p><strong>Fecha:</strong> {{ formatearFecha(compra.fecha) }}</p>
    <p><strong>Total:</strong> {{ compra.total }}€</p>

    <button @click="$emit('editar', compra)">Editar</button>
    <button @click="$emit('eliminar', compra)">Eliminar</button>
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
  border: 1px solid #ddd;
  padding: 15px;
  margin-bottom: 15px;
  border-radius: 8px;
}

ul {
  padding-left: 20px;
  margin: 5px 0;
}

li {
  margin-bottom: 5px;
}

ul li::before {
  content: none; /* Asegura que no haya ningún guion añadido antes del producto */
}


p span {
  display: inline; /* Asegura que los productos estén en la misma línea */
}
</style>
