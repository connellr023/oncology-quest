<script setup lang="ts">
import { defineProps } from "vue"

defineProps<{
  value: string,
  index: [number, number, number]
}>()
</script>

<template>
  <div>{{ value }}</div>
</template>