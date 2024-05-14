<script setup lang="ts">
import EntryHeading from "./EntryHeading.vue"
import EntryProgress from "./editor/EntryProgress.vue"
import useTaskProgress from "../hooks/useTaskProgress";

defineEmits(["click"])
const props = defineProps<{
  isActive: boolean,
  index: number[],
  title: string
}>()

const { calculateTaskProgress, calculateSubtaskProgress } = useTaskProgress()

let progress = 0;

if (props.index.length === 1) {
  progress = calculateTaskProgress(props.index[0])
}
else {
  progress = calculateSubtaskProgress([props.index[0], props.index[1]])
}
</script>

<template>
  <div class="progressable-entry-container">
    <EntryHeading :is-active="isActive" :index="index" :title="title" @click="$emit('click')" />
    <EntryProgress :progress="progress" />
  </div>
</template>

<style scoped lang="scss">
div.progressable-entry-container {
  position: relative;
  display: flex;
}
</style>