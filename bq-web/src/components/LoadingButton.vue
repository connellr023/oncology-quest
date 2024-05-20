<script setup lang="ts">
import Spinner from "./vector/Spinner.vue"

defineEmits(["click"])
defineProps<{
  loading: boolean,
  text: string
}>()
</script>

<template>
  <button :disabled="loading" class="form-button std" type="submit" @click="$emit('click')">
    {{ text }}
    <Spinner v-if="loading" />
  </button>
</template>

<style scoped lang="scss">
svg {
  height: 20px;
  margin: auto;
  position: absolute;
  right: 12px;
  opacity: 0;
  top: 10px;
  animation: fade-in 0.4s forwards, spin 0.6s ease-in-out infinite;

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
}
</style>