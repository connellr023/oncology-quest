<script setup lang="ts">
import { onMounted, onUnmounted } from "vue"

const emits = defineEmits(["change"])
defineProps<{ isVisible?: boolean }>()

const hide = () => {
  emits("change", false)
}

onMounted(() => {
  window.addEventListener("click", hide)
});

onUnmounted(() => {
  window.removeEventListener("click", hide)
});
</script>

<template>
  <div v-show="isVisible" class="dropdown-container" @click.stop>
    <slot></slot>
  </div>
</template>

<style scoped lang="scss">
@import "../styles/variables.scss";

div.dropdown-container {
  position: absolute;
  top: 55px;
  background-color: $main-bg-color;
  display: flex;
  flex-direction: column;
  align-items: center;
  z-index: 1;
  background-color: $tertiary-bg-color;
  border-radius: $ui-border-radius;
  box-shadow: 0 0 15px rgba(0, 0, 0, 0.8);
  left: 2px;
  overflow: hidden;

  & > * {
    flex-direction: row;
    flex-grow: 1;
    width: 100%;
    text-align: left;
  }
}
</style>