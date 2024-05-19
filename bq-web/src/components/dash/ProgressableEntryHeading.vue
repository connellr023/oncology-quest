<script setup lang="ts">
import EntryHeading from "./EntryHeading.vue"
import EntryProgress from "./EntryProgress.vue"

defineEmits(["click"])
defineProps<{
  isActive: boolean,
  index: number[],
  title: string,
  progress: number
}>()
</script>

<template>
  <div class="progressable-entry-container" @click="$emit('click')">
    <EntryHeading :is-active="isActive" :index="index" :title="title" />
    <EntryProgress :progress="progress" />
  </div>
</template>

<style scoped lang="scss">
div.progressable-entry-container {
  cursor: pointer;
  position: relative;
  display: flex;
}
</style>