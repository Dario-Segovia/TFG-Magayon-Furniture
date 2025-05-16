<template>
  <div class="card fade-in">
    <div class="card-header">
      <div>
        <h3>
          <span class="venta-id">#{{ venta.id }}</span>
          <span class="venta-fecha">{{ new Date(venta.fecha).toLocaleString() }}</span>
        </h3>
        <p class="cliente">
          <i class="fa fa-user"></i>
          {{ venta.nombre_cliente || 'Sin cliente' }}
        </p>
      </div>
      <div class="card-actions">
        <button 
          @click="$emit('editar-venta', venta)" 
          class="btn-action edit" 
          title="Editar"
        >✏️</button>
        <button 
          @click="$emit('eliminar-venta', venta.id)" 
          class="btn-action delete" 
          title="Eliminar"
        >🗑️</button>
      </div>
    </div>

    <div class="productos-lista">
      <table>
        <thead>
          <tr>
            <th>Producto</th>
            <th>Cant.</th>
            <th>Precio unit.</th>
            <th>Subtotal</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="detalle in venta.detalles" :key="detalle.id">
            <td>{{ detalle.nombre_producto }}</td>
            <td class="cantidad">{{ detalle.cantidad }}</td>
            <td class="venta">{{ detalle.precio_unitario }} €</td>
            <td class="subtotal">{{ (detalle.cantidad * detalle.precio_unitario).toFixed(2) }} €</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="card-footer">
      <span class="total-label">Total:</span>
      <span class="total-valor">{{ venta.total }} €</span>
      <span class="productos-count">({{ venta.detalles.length }} productos)</span>
    </div>
  </div>
</template>

<script setup>
defineProps({
  venta: {
    type: Object,
    required: true
  }
});
defineEmits(['editar-venta', 'eliminar-venta']);
</script>

<style scoped>
.card {
  background: white;
  border-radius: 10px;
  padding: 20px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.10);
  display: flex;
  flex-direction: column;
  gap: 16px;
  position: relative;
  animation: fadeIn 0.5s;
}
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 8px;
}
.card-header h3 {
  margin: 0;
  font-size: 1.1em;
  color: #405890;
  font-weight: 700;
  display: flex;
  gap: 10px;
  align-items: center;
}
.venta-id {
  background: #e0e7ff;
  color: #405890;
  border-radius: 6px;
  padding: 2px 8px;
  font-size: 0.95em;
}
.venta-fecha {
  color: #888;
  font-size: 0.95em;
}
.cliente {
  margin: 0;
  color: #4caf50;
  font-weight: 500;
  font-size: 1em;
  display: flex;
  align-items: center;
  gap: 6px;
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
.btn-action.edit { color: #4caf50; }
.btn-action.delete { color: #f44336; }
.btn-action:hover { background: #e0e0e0; }

.productos-lista {
  margin: 10px 0;
}
.productos-lista table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.98em;
}
.productos-lista th, .productos-lista td {
  padding: 6px 8px;
  text-align: left;
}
.productos-lista th {
  background: #f8f9fa;
  color: #405890;
  font-weight: 600;
}
.cantidad {
  background: #e8f5e9;
  color: #388e3c;
  border-radius: 8px;
  padding: 2px 8px;
  font-weight: 600;
  text-align: center;
}
.venta {
  color: #4caf50;
  font-weight: 600;
}
.subtotal {
  color: #405890;
  font-weight: 500;
}
.card-footer {
  display: flex;
  align-items: center;
  gap: 10px;
  justify-content: flex-end;
  margin-top: 10px;
}
.total-label {
  font-weight: 600;
  color: #405890;
}
.total-valor {
  font-size: 1.15em;
  color: #4caf50;
  font-weight: 700;
}
.productos-count {
  color: #888;
  font-size: 0.95em;
}
.fade-in {
  animation: fadeIn 0.5s;
}
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}
</style>

