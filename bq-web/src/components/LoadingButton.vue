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
    <Spinner class="spinner" v-if="loading" />
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
}
</style>