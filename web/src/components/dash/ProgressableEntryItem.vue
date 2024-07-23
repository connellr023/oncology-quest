<script setup lang="ts">
import { ref } from "vue"

import EntryHeading from "./EntryHeading.vue"
import EntryProgress from "./EntryProgress.vue"
import Arrow from "../vector/Arrow.vue"

const childrenVisible = ref(false)

defineProps<{
  saveHeading: (saveTitle: string) => Promise<boolean>,
  deleteHeading: () => Promise<boolean>,
  title: string,
  progress: number
}>()

const toggleChildrenVisibility = () => {
  childrenVisible.value = !childrenVisible.value
}
</script>

<template>
  <li :class="`${childrenVisible ? 'focused' : ''}`">
    <div class="progressable-entry-container" @click="toggleChildrenVisibility">
      <Arrow :class="`arrow ${childrenVisible ? 'focused' : ''}`" />
      <EntryHeading :saveHeading="saveHeading" :deleteHeading="deleteHeading" :is-active="childrenVisible" :title="title" />
      <EntryProgress :progress="progress" />
    </div>
    <ul v-show="childrenVisible">
      <slot></slot>
    </ul>
  </li>
</template>

<style scoped lang="scss">
@import "../../styles/variables.scss";

li {
  $side-padding: 15px;

  padding: 5px;
  padding-left: $side-padding;
  padding-right: $side-padding;
  margin-top: 10px;

  & > ul {
    margin-left: calc(10px + 2%);
    margin-bottom: 10px;
  }

  transition: background-color 0.07s ease;
  margin-bottom: 15px;
  padding-left: 15px;
  padding-right: 15px;
  border-radius: 15px;

  &.focused,
  &:hover {
    background-color: $secondary-bg-color;

    &.layer-2 {
      background-color: $tertiary-bg-color;
    }
  }
}

div.progressable-entry-container {
  cursor: pointer;
  position: relative;
  display: flex;
  align-items: center;
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
