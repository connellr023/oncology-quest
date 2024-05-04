<script setup lang="ts">
import { Component, Ref, defineProps, inject, reactive } from "vue"
import { UserSession } from "../models/user"

defineProps<{ handlerComponent: Component }>()

const sessionContext = inject<Ref<UserSession>>("session")!
let visibility = reactive<Record<string, boolean>>({})

const toggleVisibility = (key: string) => {
  visibility[key] = !visibility[key]
}
</script>

<template>
  <div v-for="(entry, index) in Object.entries(sessionContext.entries).sort((a, b) => a[0].localeCompare(b[0])).reverse()" :key="index">
    <h3 class="dropdown" @click="toggleVisibility(entry[0])">{{ entry[0] }}</h3>
    <ul v-show="visibility[entry[0]]">
      <li v-for="(innerEntry, innerIndex) in Object.entries(entry[1]).sort((a, b) => a[0].localeCompare(b[0])).reverse()" :key="innerIndex">
        <h4 class="dropdown" @click="toggleVisibility(entry[0] + innerEntry[0])">{{ innerEntry[0] }}:</h4>
        <ul v-show="visibility[entry[0] + innerEntry[0]]">
          <li
            v-for="(innerInnerValue, innerInnerIndex) in innerEntry[1]"
            :key="innerInnerIndex"
            :data-index="`${index},${innerIndex},${innerInnerIndex}`"
          >
            <handlerComponent
              :task="sessionContext.user.tasks[index]?.[innerIndex]?.[innerInnerIndex] ?? null"
              :value="innerInnerValue"
              :index="[index, innerIndex, innerInnerIndex]"
            />
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>
<!-- <template>
  <div v-for="(entry, index) in sessionContext.entries" :key="index">
    <h3 class="dropdown" @click="toggleVisibility(entry.key)">{{ entry.key }}</h3>
    <ul v-show="visibility[entry.key]">
      <li v-for="(innerEntry, innerIndex) in Object.entries(entry.value)" :key="innerIndex">
        <h4 class="dropdown" @click="toggleVisibility(entry.key + innerEntry[0])">{{ innerEntry[0] }}:</h4>
        <ul v-show="visibility[entry.key + innerEntry[0]]">
          <li
            v-for="(innerInnerValue, innerInnerIndex) in innerEntry[1]"
            :key="innerInnerIndex"
            :data-index="`${index},${innerIndex},${innerInnerIndex}`"
          >
            <handlerComponent
              :task="sessionContext.user.tasks[index]?.[innerIndex]?.[innerInnerIndex] ?? null"
              :value="innerInnerValue"
              :index="[index, innerIndex, innerInnerIndex]"
            />
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template> -->

<style scoped lang="scss">
.dropdown {
  cursor: pointer;

  &:hover {
    text-decoration: underline;
  }
}
</style>