<template>
  <transition name="modal-fade">
    <div v-if="visible" class="modal-overlay" @keydown.esc="onClose" tabindex="0" @click.self="onClose">
      <div class="modal-content" ref="modalContent" @keydown.tab.prevent>
        <button class="modal-close" @click="onClose" aria-label="Cerrar modal">×</button>
        <slot />
      </div>
    </div>
  </transition>
</template>

<script setup>
import { onMounted, onUnmounted, ref, watch } from 'vue';

const props = defineProps({
  visible: Boolean,
  onClose: Function
});

const modalContent = ref(null);

function focusModal() {
  if (modalContent.value) {
    modalContent.value.focus();
  }
}

watch(() => props.visible, (val) => {
  if (val) {
    document.body.style.overflow = 'hidden';
    setTimeout(focusModal, 10);
  } else {
    document.body.style.overflow = '';
  }
});

onUnmounted(() => {
  document.body.style.overflow = '';
});
</script>

<style scoped>
.modal-fade-enter-active, .modal-fade-leave-active {
  transition: opacity 0.18s;
}
.modal-fade-enter-from, .modal-fade-leave-to {
  opacity: 0;
}
.modal-overlay {
  position: fixed;
  z-index: 1000;
  inset: 0;
  background: rgba(44, 62, 80, 0.35);
  display: flex;
  align-items: center;
  justify-content: center;
  outline: none;
}
.modal-content {
  background: #fff;
  border-radius: 14px;
  box-shadow: 0 8px 32px rgba(44,62,80,0.18);
  padding: 32px 28px 24px 28px;
  min-width: 340px;
  max-width: 95vw;
  max-height: 90vh;
  overflow-y: auto;
  position: relative;
  animation: modalIn 0.18s;
  outline: none;
}
@keyframes modalIn {
  from { transform: scale(0.95); opacity: 0; }
  to { transform: scale(1); opacity: 1; }
}
.modal-close {
  position: absolute;
  top: 12px;
  right: 18px;
  background: none;
  border: none;
  font-size: 2rem;
  color: #888;
  cursor: pointer;
  transition: color 0.2s;
  z-index: 2;
}
.modal-close:hover {
  color: #e74c3c;
}
</style>