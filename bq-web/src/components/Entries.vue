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
  <div v-for="(entry, index) in sessionContext.entries" :key="index">
    <h3 class="dropdown" @click="toggleVisibility(entry.title)">{{ entry.title }}</h3>
    <ul v-show="visibility[entry.title]">
      <li v-for="(subTask, subIndex) in entry.tasks" :key="subIndex">
        <h4 class="dropdown" @click="toggleVisibility(entry.title + subTask.title)">{{ subTask.title }}:</h4>
        <ul v-show="visibility[entry.title + subTask.title]">
          <li
            v-for="(task, taskIndex) in subTask.tasks"
            :key="taskIndex"
            :data-index="`${index},${subIndex},${taskIndex}`"
          >
            <handlerComponent
              :task="sessionContext.user.tasks[index]?.[subIndex]?.[taskIndex] ?? null"
              :value="task"
              :index="[index, subIndex, taskIndex]"
            />
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