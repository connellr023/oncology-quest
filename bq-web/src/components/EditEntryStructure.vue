<script setup lang="ts">
defineProps<{
  entryType: string,
  push: () => void,
  pop: () => void
}>()
</script>

<template>
  <button @click="push">Push {{ entryType }}</button>
  <button @click="pop">Pop {{ entryType }}</button>
</template>