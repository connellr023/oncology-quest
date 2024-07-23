<script setup lang="ts">
import { ref } from "vue"

import EntryHeading from "./EntryHeading.vue"
import EntryProgress from "./EntryProgress.vue"
import Arrow from "../vector/Arrow.vue"

const isExpanded = ref(false)

defineProps<{
  saveHeading: (saveTitle: string) => Promise<boolean>,
  deleteHeading: () => Promise<boolean>,
  title: string,
  progress: number
}>()

const toggleChildrenVisibility = () => {
  isExpanded.value = !isExpanded.value
}
</script>

<template>
  <li :class="`${isExpanded ? 'focused' : ''}`">
    <div class="progressable-entry-container" @click="toggleChildrenVisibility">
      <Arrow :class="`arrow ${isExpanded ? 'focused' : ''}`" />
      <EntryHeading :saveHeading="saveHeading" :deleteHeading="deleteHeading" :is-active="isExpanded" :title="title" />
      <EntryProgress :progress="progress" />
    </div>
    <div class="expand-container">
      <ul :class="`${isExpanded ? 'expanded' : 'collapsed'}`">
        <slot></slot>
      </ul>
    </div>
  </li>
</template>

<style scoped lang="scss">
@import "../../styles/variables.scss";

div.progressable-entry-container {
  $vertical-padding: 5px;

  cursor: pointer;
  padding-top: $vertical-padding;
  padding-bottom: $vertical-padding;
  position: relative;
  display: flex;
  align-items: center;
  margin-right: 15px;
}

svg.arrow {
  user-select: none;
  opacity: 0.6;
  width: 17px;
  margin-left: 15px;
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

li {
  transition: background-color 0.07s ease;

  &.focused,
  &:hover {
    background-color: $secondary-bg-color;

    &.layer-2 {
      background-color: $tertiary-bg-color;
    }
  }
}

div.expand-container {
  overflow: hidden;

  & > ul {
    &.expanded {
      transition: margin-top 0.17s ease;
      margin-top: 0;
    }

    &.collapsed {
      transition: margin-top 0.4s ease-in;
      margin-top: -200%;
    }
  }
}
</style>
