<script setup lang="ts">
defineProps<{
  loading: boolean,
  text: string
}>()
</script>

<template>
  <button :disabled="loading" class="form-button glow gradient-button-0" type="submit">
    {{ text }}<img v-if="loading" src="/spinner.svg" alt="loading" />
  </button>
</template>

<style scoped lang="scss">
img {
  height: 20px;
  margin: auto;
  position: absolute;
  right: 12px;
  opacity: 0;
  top: 9px;
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