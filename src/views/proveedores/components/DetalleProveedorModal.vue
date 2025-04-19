<template>
    <div class="modal-overlay">
      <div class="modal-container">
        <div class="modal-header">
          <h2>Detalles del Proveedor</h2>
          <button @click="$emit('close')" class="close-btn">
            <i class="fas fa-times"></i>
          </button>
        </div>
        
        <div class="modal-body">
          <div class="detail-section">
            <h3>Información Básica</h3>
            <div class="detail-grid">
              <div class="detail-item">
                <label>Nombre:</label>
                <span>{{ proveedor.nombre }}</span>
              </div>
              
              <div class="detail-item">
                <label>Estado:</label>
                <span :class="proveedor.estado === 'activo' ? 'activo' : 'inactivo'">
                  {{ proveedor.estado }}
                </span>
              </div>
              
              <div class="detail-item">
                <label>Contacto:</label>
                <span>{{ proveedor.contacto || 'No especificado' }}</span>
              </div>
              
              <div class="detail-item">
                <label>Contrato:</label>
                <span :class="proveedor.contrato_vigente ? 'vigente' : 'no-vigente'">
                  {{ proveedor.contrato_vigente ? 'Vigente' : 'No vigente' }}
                </span>
              </div>
            </div>
          </div>
          
          <div class="detail-section">
            <h3>Información de Contacto</h3>
            <div class="detail-grid">
              <div class="detail-item">
                <label>Teléfono:</label>
                <span v-if="proveedor.telefono">
                  <a :href="`tel:${proveedor.telefono}`">{{ proveedor.telefono }}</a>
                </span>
                <span v-else>No especificado</span>
              </div>
              
              <div class="detail-item">
                <label>Email:</label>
                <span v-if="proveedor.email">
                  <a :href="`mailto:${proveedor.email}`">{{ proveedor.email }}</a>
                </span>
                <span v-else>No especificado</span>
              </div>
            </div>
          </div>
          
          <div class="detail-section">
            <h3>Ubicación</h3>
            <div class="detail-grid">
              <div class="detail-item">
                <label>Dirección:</label>
                <span>{{ proveedor.direccion || 'No especificada' }}</span>
              </div>
              
              <div class="detail-item">
                <label>País:</label>
                <span>{{ proveedor.pais || 'No especificado' }}</span>
              </div>
            </div>
          </div>
          
          <div class="detail-section">
            <h3>Productos</h3>
            <ProductosProveedor :proveedorId="proveedor.id" />
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script>
  import ProductosProveedor from './ProductosProveedor.vue'
  
  export default {
    components: {
      ProductosProveedor
    },
    
    props: {
      proveedor: {
        type: Object,
        required: true
      }
    },
    
    emits: ['close']
  }
  </script>
  
  <style scoped>
  .detail-section {
    margin-bottom: 25px;
    padding-bottom: 15px;
    border-bottom: 1px solid #eee;
  }
  
  .detail-section:last-child {
    border-bottom: none;
  }
  
  .detail-section h3 {
    margin-top: 0;
    margin-bottom: 15px;
    color: #333;
  }
  
  .detail-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 15px;
  }
  
  .detail-item {
    display: flex;
    flex-direction: column;
  }
  
  .detail-item label {
    font-weight: 500;
    color: #666;
    font-size: 0.9em;
    margin-bottom: 3px;
  }
  
  .detail-item span {
    word-break: break-word;
  }
  
  .detail-item a {
    color: #2196F3;
    text-decoration: none;
  }
  
  .detail-item a:hover {
    text-decoration: underline;
  }
  
  .activo {
    color: #2e7d32;
    font-weight: 500;
  }
  
  .inactivo {
    color: #c62828;
    font-weight: 500;
  }
  
  .vigente {
    color: #2196F3;
    font-weight: 500;
  }
  
  .no-vigente {
    color: #f44336;
    font-weight: 500;
  }
  </style>