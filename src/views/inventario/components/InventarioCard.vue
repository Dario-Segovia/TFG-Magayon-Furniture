<template>
  <div class="inventario-card fade-in">
    <div class="card-header">
      <h3>{{ item.nombre }}</h3>
      <span class="cantidad">x{{ item.cantidad }}</span>
    </div>
    <div class="card-content">
      <div class="desc">{{ item.descripcion || 'Sin descripción' }}</div>
      <div class="precio">
        <span>Precio unitario: ${{ Number(item.precio_unitario).toFixed(2) }}</span>
      </div>
      <div class="categoria" v-if="item.categoria">
        <i class="fas fa-tag"></i> {{ item.categoria }}
      </div>
    </div>
    <div class="card-actions">
      <button @click="$emit('edit', item)" class="btn-action edit" title="Editar">
        <i class="fas fa-edit"></i>
      </button>
      <button @click="$emit('delete', item)" class="btn-action delete" title="Eliminar">
        <i class="fas fa-trash"></i>
      </button>
    </div>
  </div>
</template>

<script setup>
defineProps({ item: Object });
</script>

<style scoped>
.inventario-card {
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