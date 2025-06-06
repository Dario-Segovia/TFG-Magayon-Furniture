<template>
  <div class="search-container">
    <div class="search-wrapper">
      <div class="search-box">
        <i class="fas fa-search search-icon"></i>
        <input 
          type="text" 
          :value="modelValue" 
          @input="$emit('update:modelValue', $event.target.value)"
          placeholder="Buscar empleados por nombre, puesto..."
          class="search-input"
        >
        <button
          class="sort-btn"
          :title="asc ? 'Ordenar Z-A' : 'Ordenar A-Z'"
          @click="$emit('toggle-sort')"
        >
          <i :class="asc ? 'fas fa-sort-alpha-down' : 'fas fa-sort-alpha-up-alt'"></i>
        </button>
      </div>
      <div class="total-employees">
        <span>{{ count }}</span> empleados
      </div>
    </div>
  </div>
</template>

<script setup>
defineProps({
  modelValue: String,
  count: Number,
  asc: Boolean // Nuevo prop para saber el orden
});
defineEmits(['update:modelValue', 'toggle-sort']);
</script>

<style scoped>
.search-container {
  background-color: var(--card-bg);
  padding: 1rem 2rem;
  box-shadow: 0 0.15rem 1.75rem 0 rgba(58, 59, 69, 0.1);
}

.search-wrapper {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1.5rem;
}

.search-box {
  flex: 1;
  position: relative;
  display: flex;
  align-items: center;
}

.search-input {
  width: 100%;
  padding: 0.75rem 1rem 0.75rem 2.5rem;
  border: 1px solid var(--border-color);
  border-radius: 0.35rem;
  font-size: 0.9rem;
  transition: all 0.3s;
}

.search-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 0.2rem rgba(78, 115, 223, 0.25);
}

.search-icon {
  position: absolute;
  left: 1rem;
  color: var(--inactive-color);
}

.total-employees {
  min-width: 120px;
  text-align: right;
  padding: 0.5rem 1rem;
  border-radius: 0.35rem;
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--primary-color);
  background: none;
}

.total-employees span {
  font-size: 1.1rem;
  margin-right: 0.25rem;
}

.sort-btn {
  background: none;
  border: none;
  color: var(--primary-color, #667eea);
  font-size: 1.2rem;
  margin-left: 0.5rem;
  cursor: pointer;
  transition: color 0.2s;
  display: flex;
  align-items: center;
}

.sort-btn:hover {
  color: var(--accent-color, #764ba2);
}

@media (max-width: 768px) {
  .search-container {
    padding: 1rem;
  }
  .search-wrapper {
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
  }
  .total-employees {
    margin-left: 0;
    text-align: center;
  }
}
</style>