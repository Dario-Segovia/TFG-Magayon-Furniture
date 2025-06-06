<template>
  <div class="modal-overlay">
    <div class="modal-container">
      <div class="modal-header">
        <div class="header-content">
          <svg class="modal-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M19 13C19 15 19 18 14 18H13V6H14C19 6 19 9 19 11" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M5 13C5 15 5 18 10 18H11V6H10C5 6 5 9 5 11" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <h2>Importar Inventario desde XML</h2>
        </div>
        <button @click="$emit('close')" class="close-btn" aria-label="Cerrar">
          <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>
      
      <div class="modal-body">
        <p class="instructions">
          Selecciona un archivo XML con el siguiente formato:
        </p>
        
        <div class="code-container">
          <div class="code-header">
            <span class="code-language">XML</span>
            <button class="copy-btn" @click="copyExample">
              <svg viewBox="0 0 24 24" width="16" height="16" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
              </svg>
              Copiar
            </button>
          </div>
          <pre class="xml-example"><code>&lt;InventarioXml&gt;
  &lt;item&gt;
    &lt;nombre&gt;Producto Ejemplo&lt;/nombre&gt;
    &lt;descripcion&gt;Descripción del producto&lt;/descripcion&gt;
    &lt;cantidad&gt;100&lt;/cantidad&gt;
    &lt;precioUnitario&gt;19.99&lt;/precioUnitario&gt;
    &lt;categoria&gt;Categoría Ejemplo&lt;/categoria&gt;
  &lt;/item&gt;
  &lt;!-- Más items --&gt;
&lt;/InventarioXml&gt;</code></pre>
        </div>
        
        <div class="file-upload-area" @click="abrirExplorador" @dragover.prevent="dragover = true" @dragleave="dragover = false" @drop.prevent="handleDrop">
          <input ref="fileInput" type="file" accept=".xml" class="file-input" @change="onFileChange" />
          <div class="upload-content" :class="{ 'drag-active': dragover }">
            <svg class="upload-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M21 15V19C21 20.1046 20.1046 21 19 21H5C3.89543 21 3 20.1046 3 19V15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M17 8L12 3M12 3L7 8M12 3V15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <p class="upload-text">Arrastra tu archivo XML aquí o <span class="browse-link">busca en tu equipo</span></p>
            <p class="upload-hint">Solo se aceptan archivos .xml</p>
          </div>
        </div>
        
        <div v-if="nombreArchivo" class="file-preview">
          <svg class="file-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M13 2H6C5.46957 2 4.96086 2.21071 4.58579 2.58579C4.21071 2.96086 4 3.46957 4 4V20C4 20.5304 4.21071 21.0391 4.58579 21.4142C4.96086 21.7893 5.46957 22 6 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V9L13 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M13 2V9H20" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <div class="file-details">
            <p class="file-name">{{ nombreArchivo }}</p>
            <button class="remove-btn" @click.stop="removeFile">
              <svg viewBox="0 0 24 24" width="16" height="16" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"></line>
                <line x1="6" y1="6" x2="18" y2="18"></line>
              </svg>
            </button>
          </div>
        </div>
      </div>
      
      <div class="modal-footer" v-if="nombreArchivo">
        <button class="btn btn-outline" @click="$emit('close')">Cancelar</button>
        <button class="btn btn-primary" @click="confirmarImportacion">
          <svg v-if="loading" class="animate-spin" viewBox="0 0 24 24" width="18" height="18" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
          </svg>
          <span v-else>Importar Inventario</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const emit = defineEmits(['close', 'importar-xml']);
const fileInput = ref(null);
const archivo = ref(null);
const nombreArchivo = ref('');
const dragover = ref(false);
const loading = ref(false);

function abrirExplorador() {
  fileInput.value.click();
}

function handleDrop(e) {
  dragover.value = false;
  const files = e.dataTransfer.files;
  if (files && files[0] && files[0].name.endsWith('.xml')) {
    archivo.value = files[0];
    nombreArchivo.value = files[0].name;
  }
}

function onFileChange(e) {
  const files = e.target.files;
  if (files && files[0]) {
    archivo.value = files[0];
    nombreArchivo.value = files[0].name;
  }
}

function removeFile() {
  archivo.value = null;
  nombreArchivo.value = '';
  fileInput.value.value = '';
}

async function confirmarImportacion() {
  if (archivo.value) {
    loading.value = true;
    try {
      const text = await archivo.value.text();
      emit('importar-xml', text);
    } finally {
      loading.value = false;
    }
  }
}

function copyExample() {
  const example = `<InventarioXml>
  <item>
    <nombre>Producto Ejemplo</nombre>
    <descripcion>Descripción del producto</descripcion>
    <cantidad>100</cantidad>
    <precioUnitario>19.99</precioUnitario>
    <categoria>Categoría Ejemplo</categoria>
  </item>
</InventarioXml>`;
  navigator.clipboard.writeText(example);
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
  animation: fadeIn 0.3s ease-out;
}

.modal-container {
  background: white;
  border-radius: 12px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  width: 95%;
  max-width: 580px;
  overflow: hidden;
  transform: translateY(0);
  opacity: 1;
  transition: all 0.3s ease;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  background: #f8fafc;
  border-bottom: 1px solid #e2e8f0;
}

.header-content {
  display: flex;
  align-items: center;
  gap: 12px;
}

.modal-icon {
  width: 24px;
  height: 24px;
  color: #3b82f6;
}

h2 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: #1e293b;
}

.close-btn {
  background: none;
  border: none;
  padding: 4px;
  border-radius: 6px;
  cursor: pointer;
  color: #64748b;
  transition: all 0.2s;
}

.close-btn:hover {
  color: #475569;
  background: #e2e8f0;
}

.modal-body {
  padding: 24px;
}

.instructions {
  margin: 0 0 16px 0;
  color: #475569;
  font-size: 0.95rem;
}

.code-container {
  background: #1e293b;
  border-radius: 8px;
  margin-bottom: 24px;
  overflow: hidden;
}

.code-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: #0f172a;
}

.code-language {
  color: #94a3b8;
  font-size: 0.8rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.copy-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: #e2e8f0;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.75rem;
  cursor: pointer;
  transition: all 0.2s;
}

.copy-btn:hover {
  background: rgba(255, 255, 255, 0.2);
}

.xml-example {
  margin: 0;
  padding: 16px;
  color: #e2e8f0;
  font-family: 'SF Mono', 'Roboto Mono', monospace;
  font-size: 0.9rem;
  line-height: 1.5;
  overflow-x: auto;
}

.xml-example code {
  display: block;
}

.file-upload-area {
  margin-bottom: 16px;
  cursor: pointer;
}

.file-input {
  display: none;
}

.upload-content {
  border: 2px dashed #cbd5e1;
  border-radius: 8px;
  padding: 32px 24px;
  text-align: center;
  transition: all 0.3s;
}

.upload-content.drag-active {
  border-color: #3b82f6;
  background: #f0f9ff;
}

.upload-icon {
  width: 48px;
  height: 48px;
  margin: 0 auto 12px;
  color: #94a3b8;
}

.drag-active .upload-icon {
  color: #3b82f6;
}

.upload-text {
  margin: 0 0 4px 0;
  color: #334155;
  font-size: 1rem;
}

.browse-link {
  color: #3b82f6;
  font-weight: 500;
}

.upload-hint {
  margin: 0;
  color: #94a3b8;
  font-size: 0.85rem;
}

.file-preview {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: #f8fafc;
  border-radius: 8px;
  border: 1px solid #e2e8f0;
}

.file-icon {
  width: 24px;
  height: 24px;
  color: #64748b;
  flex-shrink: 0;
}

.file-details {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  min-width: 0;
}

.file-name {
  margin: 0;
  font-size: 0.95rem;
  color: #334155;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.remove-btn {
  background: none;
  border: none;
  padding: 4px;
  border-radius: 4px;
  color: #94a3b8;
  cursor: pointer;
  transition: all 0.2s;
}

.remove-btn:hover {
  color: #ef4444;
  background: #fee2e2;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  background: #f8fafc;
  border-top: 1px solid #e2e8f0;
}

.btn {
  padding: 8px 16px;
  border-radius: 6px;
  font-weight: 500;
  font-size: 0.95rem;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
}

.btn-outline {
  background: white;
  border: 1px solid #cbd5e1;
  color: #334155;
}

.btn-outline:hover {
  background: #f1f5f9;
  border-color: #94a3b8;
}

.btn-primary {
  background: #3b82f6;
  border: 1px solid #3b82f6;
  color: white;
}

.btn-primary:hover {
  background: #2563eb;
  border-color: #2563eb;
}

.animate-spin {
  animation: spin 1s linear infinite;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>