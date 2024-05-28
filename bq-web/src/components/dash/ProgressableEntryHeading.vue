<script setup lang="ts">
import EntryHeading from "./EntryHeading.vue"
import EntryProgress from "./EntryProgress.vue"
import Arrow from "../vector/Arrow.vue"

defineEmits(["click"])
defineProps<{
  save: (saveTitle: string) => Promise<boolean>,
  isActive: boolean,
  title: string,
  progress: number
}>()
</script>

<template>
  <div class="progressable-entry-container" @click="$emit('click')">
    <Arrow :class="`arrow ${isActive ? 'focused' : ''}`" />
    <EntryHeading :save="save" :is-active="isActive" :title="title" />
    <EntryProgress :progress="progress" />
  </div>
</template>

<style scoped lang="scss">
@import "../../main.scss";

div.progressable-entry-container {
  cursor: pointer;
  position: relative;
  display: flex;
}

svg.arrow {
  user-select: none;
  opacity: 0.6;
  width: 17px;
  margin-right: 10px;
  display: inline-block;
  transform: rotate(90deg);
  transition: all 0.2s ease;

  &.focused {
    fill: $theme-color-1;
    opacity: 1;
    transform: rotate(180deg);
  }
}
</style>