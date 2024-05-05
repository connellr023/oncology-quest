<script setup lang="ts">
import { defineProps } from "vue"

defineProps<{
  title: string,
  index: number[]
}>()
</script>

<template>
  <h3 class="dropdown">{{ title }}</h3>
</template>

<style scoped lang="scss">
.dropdown {
  cursor: pointer;

  &:hover {
    text-decoration: underline;
  }
}
</style>