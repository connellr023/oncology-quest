<script setup lang="ts">
import { Component, defineProps, reactive } from "vue"
import { NamedTaskEntries } from "../models/task"

const props = defineProps<{
  entries: NamedTaskEntries,
  handlerComponent: Component
}>()
let visibility = reactive<Record<string, boolean>>({})

const toggleVisibility = (key: string) => {
  visibility[key] = !visibility[key]
}
</script>

<template>
  <div v-for="(entry, index) in Object.entries(props.entries)" :key="index">
    <h3 class="dropdown" @click="toggleVisibility(entry[0])">{{ entry[0] }}</h3>
    <ul v-show="visibility[entry[0]]">
      <li v-for="(innerEntry, innerIndex) in Object.entries(entry[1])" :key="innerIndex">
        <h4 class="dropdown" @click="toggleVisibility(entry[0] + innerEntry[0])">{{ innerEntry[0] }}:</h4>
        <ul v-show="visibility[entry[0] + innerEntry[0]]">
          <li v-for="(innerInnerValue, innerInnerIndex) in innerEntry[1]" :key="innerInnerIndex">
            <handlerComponent :value="innerInnerValue" :index="[index, innerIndex, innerInnerIndex]" />
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>

<style scoped lang="scss">
.dropdown {
  cursor: pointer;

  &:hover {
    text-decoration: underline;
  }
}
</style>