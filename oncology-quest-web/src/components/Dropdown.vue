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
    <div>
      <slot></slot>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../styles/variables.scss";

div.dropdown-container {
  $offset: 2px;

  position: absolute;
  top: 55px;
  
  & > div {
    background-color: $main-bg-color;
    display: flex;
    flex-direction: column;
    align-items: center;
    z-index: 1;
    background-color: $tertiary-bg-color;
    border-radius: 8px;
    box-shadow: 0 0 15px rgba(0, 0, 0, 0.3);
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    width: 100%;
    height: 100%;
    left: $offset;
    
    & > * {
      text-align: left;
      width: 100%;
    }
  }

  &.right {
    right: $offset;
  }
}
</style>