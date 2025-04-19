<template>
  <div class="proveedor-card" :class="{ 'contrato-vigente': proveedor.contrato_vigente, 'is-open': isOpen }">
    <div class="card-header">
      <h3>{{ proveedor.nombre }}</h3>
      <span class="estado-badge" :class="proveedor.estado === 'activo' ? 'activo' : 'inactivo'">
        {{ proveedor.estado }}
      </span>
    </div>

    <div class="card-content">
      <div v-if="proveedor.contacto" class="info-row">
        <i class="fas fa-user"></i>
        <span>{{ proveedor.contacto }}</span>
      </div>
      <div v-if="proveedor.telefono" class="info-row">
        <i class="fas fa-phone"></i>
        <a :href="`tel:${proveedor.telefono}`">{{ proveedor.telefono }}</a>
      </div>
      <div v-if="proveedor.email" class="info-row">
        <i class="fas fa-envelope"></i>
        <a :href="`mailto:${proveedor.email}`">{{ proveedor.email }}</a>
      </div>
      <div v-if="proveedor.direccion" class="info-row">
        <i class="fas fa-map-marker-alt"></i>
        <span>{{ proveedor.direccion }}</span>
      </div>
    </div>

    <div class="card-actions">
      <button 
        @click="$emit('toggle', proveedor.id)" 
        class="btn-action view" 
        title="Ver productos" 
        aria-label="Ver productos"
      >
        <i class="fas" :class="isOpen ? 'fa-chevron-up' : 'fa-chevron-down'"></i>
      </button>
      <button 
        @click="editarProveedor" 
        class="btn-action edit" 
        title="Editar" 
        aria-label="Editar proveedor"
      >
        <i class="fas fa-edit"></i>
      </button>
      <button 
        @click="$emit('delete', proveedor)" 
        class="btn-action delete" 
        title="Eliminar" 
        aria-label="Eliminar proveedor"
      >
        <i class="fas fa-trash"></i>
      </button>
    </div>

    <div v-if="isOpen" class="productos-container open">
      <ProductosProveedor :proveedorId="proveedor.id" />
    </div>
  </div>
</template>

<script>
import ProductosProveedor from './ProductosProveedor.vue';

export default {
  components: { ProductosProveedor },
  props: {
    proveedor: {
      type: Object,
      required: true,
    },
    isOpen: {
      type: Boolean,
      required: true,
    },
  },
  methods: {
    editarProveedor() {
      this.$emit('edit', this.proveedor);
    },
  },
};
</script>

<style scoped>
.proveedor-card {
  background: white;
  border-radius: 8px;
  padding: 15px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
  position: relative;
  overflow: hidden;
  transition: transform 0.2s, box-shadow 0.2s;
  border-left: 4px solid #4CAF50;
  display: flex;
  flex-direction: column;
  height: auto;
}

.proveedor-card.is-open {
  overflow: hidden;
  grid-row-end: span 2;
}

.proveedor-card.contrato-vigente {
  border-left-color: #2196F3;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  padding-bottom: 10px;
  border-bottom: 1px solid #eee;
}

.card-header h3 {
  margin: 0;
  font-size: 1.2em;
  color: #333;
}

.estado-badge {
  padding: 3px 8px;
  border-radius: 12px;
  font-size: 0.8em;
  font-weight: 500;
}

.estado-badge.activo {
  background-color: #e8f5e9;
  color: #2e7d32;
}

.estado-badge.inactivo {
  background-color: #ffebee;
  color: #c62828;
}

.card-content {
  margin-bottom: 15px;
}

.info-row {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 8px;
  font-size: 0.95em;
}

.info-row i {
  color: #666;
  width: 16px;
  text-align: center;
}

.info-row a {
  color: #2196F3;
  text-decoration: none;
}

.info-row a:hover {
  text-decoration: underline;
}

.card-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.btn-action {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-action.view {
  background: #e3f2fd;
  color: #2196F3;
}

.btn-action.edit {
  background: #e8f5e9;
  color: #4CAF50;
}

.btn-action.delete {
  background: #ffebee;
  color: #f44336;
}

.btn-action:hover {
  opacity: 0.8;
}

.contrato-tag {
  position: absolute;
  top: 10px;
  right: 10px;
  background: #e3f2fd;
  color: #2196F3;
  padding: 3px 8px;
  border-radius: 12px;
  font-size: 0.75em;
  display: flex;
  align-items: center;
  gap: 5px;
}

.productos-container {
  margin-top: 10px;
  padding: 10px;
  background: #f9f9f9;
  border-radius: 6px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
  transition: max-height 0.3s ease, opacity 0.3s ease;
  overflow: hidden;
  max-height: 0;
  opacity: 0;
  position: relative;
  z-index: 1;
}

.productos-container.open {
  max-height: 500px;
  opacity: 1;
}
</style>
