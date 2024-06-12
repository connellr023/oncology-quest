<script setup lang="ts">
const model = defineModel()

defineProps<{
  title: string,
  name: string,
  type: "text" | "password",
  error: string
}>()
</script>

<template>
  <label :for="name">
    {{ title }}
    <span class="error-label" v-if="error">{{ error }}</span>
  </label>
  <input :class="`${error ? 'error' : ''}`" :type="type" :id="name" :name="name" v-model="model" required>
</template>