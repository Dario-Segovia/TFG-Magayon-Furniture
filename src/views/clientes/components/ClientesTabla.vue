<template>
    <div class="clientes-container">
      <div class="clientes-header">
        <input 
          v-model="filtro" 
          class="clientes-search" 
          placeholder="Buscar cliente..."
        />
        <button class="clientes-btn-primary" @click="abrirModalCrear">
          <svg class="clientes-icon" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
          </svg>
          Nuevo cliente
        </button>
      </div>
  
      <div class="clientes-table-container">
        <table class="clientes-table">
          <thead class="clientes-table-head">
            <tr>
              <th class="clientes-table-header">Nombre</th>
              <th class="clientes-table-header">Email</th>
              <th class="clientes-table-header">Teléfono</th>
              <th class="clientes-table-header">Dirección</th>
              <th class="clientes-table-header clientes-table-header-actions">Acciones</th>
            </tr>
          </thead>
          <tbody class="clientes-table-body">
            <tr 
              v-for="cliente in clientesFiltrados" 
              :key="cliente.id"
              class="clientes-table-row"
            >
              <td class="clientes-table-cell">{{ cliente.nombre }}</td>
              <td class="clientes-table-cell">{{ cliente.email }}</td>
              <td class="clientes-table-cell">{{ cliente.telefono }}</td>
              <td class="clientes-table-cell">{{ cliente.direccion }}</td>
              <td class="clientes-table-cell clientes-table-cell-actions">
                <div class="clientes-actions">
                  <button 
                    class="clientes-btn-icon clientes-btn-edit" 
                    @click="abrirModalEditar(cliente)"
                    title="Editar"
                  >
                    <svg class="clientes-icon" viewBox="0 0 20 20" fill="currentColor">
                      <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z" />
                    </svg>
                  </button>
                  <button 
                    class="clientes-btn-icon clientes-btn-delete" 
                    @click="confirmarEliminar(cliente)"
                    title="Eliminar"
                  >
                    <svg class="clientes-icon" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
                    </svg>
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
  
      <!-- Modal Crear Cliente -->
      <transition name="fade">
        <div v-if="mostrarCrear" class="modal-overlay" @click.self="cerrarModalCrear">
          <div class="modal-content">
            <CrearCliente @cerrar="cerrarModalCrear" @creado="clienteCreado" />
          </div>
        </div>
      </transition>
  
      <!-- Modal Editar Cliente -->
      <transition name="fade">
        <div v-if="clienteEditando" class="modal-overlay" @click.self="cerrarModalEditar">
          <div class="modal-content">
            <EditarCliente 
              :cliente="clienteEditando" 
              @cerrar="cerrarModalEditar" 
              @actualizado="clienteActualizado" 
            />
          </div>
        </div>
      </transition>
    </div>
  </template>
  
  <script setup>
  import { ref, computed, onMounted } from 'vue'
  import { invoke } from '@tauri-apps/api/core'
  import CrearCliente from './CrearCliente.vue'
  import EditarCliente from './EditarCliente.vue'
  
  const clientes = ref([])
  const filtro = ref('')
  const mostrarCrear = ref(false)
  const clienteEditando = ref(null)
  
  const fetchClientes = async () => {
    clientes.value = await invoke('obtener_clientes')
  }
  
  const confirmarEliminar = async (cliente) => {
    if (confirm(`¿Estás seguro de eliminar a ${cliente.nombre}?`)) {
      await invoke('eliminar_cliente', { id: cliente.id })
      fetchClientes()
    }
  }
  
  const abrirModalCrear = () => {
    mostrarCrear.value = true
    document.body.style.overflow = 'hidden'
  }
  
  const cerrarModalCrear = () => {
    mostrarCrear.value = false
    document.body.style.overflow = 'auto'
  }
  
  const abrirModalEditar = (cliente) => {
    clienteEditando.value = cliente
    document.body.style.overflow = 'hidden'
  }
  
  const cerrarModalEditar = () => {
    clienteEditando.value = null
    document.body.style.overflow = 'auto'
  }
  
  const clienteCreado = () => {
    cerrarModalCrear()
    fetchClientes()
  }
  
  const clienteActualizado = () => {
    cerrarModalEditar()
    fetchClientes()
  }
  
  const clientesFiltrados = computed(() =>
    clientes.value.filter(c =>
      c.nombre.toLowerCase().includes(filtro.value.toLowerCase()) ||
      c.email.toLowerCase().includes(filtro.value.toLowerCase()) ||
      c.telefono?.toLowerCase().includes(filtro.value.toLowerCase())
    )
  )
  
  onMounted(fetchClientes)
  </script>
  
  <style scoped>
  .clientes-container {
    padding: 1.5rem;
    background-color: white;
    border-radius: 0.5rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }
  
  .clientes-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }
  
  .clientes-search {
    width: 100%;
    max-width: 20rem;
    padding: 0.5rem 1rem;
    border: 1px solid #d1d5db;
    border-radius: 0.5rem;
    outline: none;
    transition: all 0.2s;
  }
  
  .clientes-search:focus {
    border-color: #3b82f6;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
  }
  
  .clientes-btn-primary {
    display: flex;
    align-items: center;
    padding: 0.5rem 1rem;
    background-color: #2563eb;
    color: white;
    border: none;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: background-color 0.2s;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }
  
  .clientes-btn-primary:hover {
    background-color: #1d4ed8;
  }
  
  .clientes-icon {
    width: 1.25rem;
    height: 1.25rem;
    margin-right: 0.5rem;
  }
  
  .clientes-table-container {
    overflow-x: auto;
  }
  
  .clientes-table {
    width: 100%;
    border-collapse: collapse;
  }
  
  .clientes-table-head {
    background-color: #f9fafb;
  }
  
  .clientes-table-header {
    padding: 0.75rem 1rem;
    text-align: left;
    font-size: 0.75rem;
    font-weight: 500;
    color: #6b7280;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  
  .clientes-table-header-actions {
    text-align: center;
  }
  
  .clientes-table-body {
    font-size: 0.875rem;
  }
  
  .clientes-table-row {
    border-top: 1px solid #e5e7eb;
    transition: background-color 0.2s;
  }
  
  .clientes-table-row:hover {
    background-color: #f9fafb;
  }
  
  .clientes-table-cell {
    padding: 1rem;
    vertical-align: top;
    color: #374151;
  }
  
  .clientes-table-cell-actions {
    text-align: center;
  }
  
  .clientes-actions {
    display: flex;
    justify-content: center;
    gap: 0.75rem;
  }
  
  .clientes-btn-icon {
    padding: 0.5rem;
    border: none;
    background: none;
    cursor: pointer;
    border-radius: 50%;
    transition: background-color 0.2s;
  }
  
  .clientes-btn-icon:hover {
    background-color: #f3f4f6;
  }
  
  .clientes-btn-edit {
    color: #2563eb;
  }
  
  .clientes-btn-edit:hover {
    color: #1d4ed8;
  }
  
  .clientes-btn-delete {
    color: #dc2626;
  }
  
  .clientes-btn-delete:hover {
    color: #b91c1c;
  }
  
 /* Estilos mejorados para los modales */
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
  padding: 1rem; /* Espacio para móviles */
}

.modal-content {
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 2px 20px rgba(0, 0, 0, 0.25);
  width: 100%;
  max-width: 600px; /* Ancho máximo para escritorio */
  max-height: 90vh; /* Altura máxima */
  overflow-y: auto;
  margin: auto;
  position: relative;
  animation: modal-appear 0.3s ease-out;
}

/* Animación de aparición suave */
@keyframes modal-appear {
  from {
    transform: translateY(20px);
    opacity: 0.8;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

/* Estilos para el contenido interno del modal */
.modal-body {
  padding: 2rem;
}



/* Transiciones para el efecto fade */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* Efecto para el contenido del modal */
.fade-enter-active .modal-content,
.fade-leave-active .modal-content {
  transition: all 0.3s ease;
}

.fade-enter-from .modal-content,
.fade-leave-to .modal-content {
  transform: translateY(20px);
  opacity: 0.8;
}
  </style>