<script setup lang="ts">
import EntryHeading from "./EntryHeading.vue"
import EntryProgress from "./editor/EntryProgress.vue"

defineEmits(["click"])
defineProps<{
  isActive: boolean,
  index: number[],
  title: string
}>()
</script>

<template>
  <div class="progressable-entry-container">
    <EntryHeading :is-active="isActive" :index="index" :title="title" @click="$emit('click')" />
    <EntryProgress />
  </div>
</template>

<style scoped lang="scss">
div.progressable-entry-container {
  position: relative;
  display: flex;
}
</style>