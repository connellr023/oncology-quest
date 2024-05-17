<script setup lang="ts">
import { Ref, inject, reactive } from "vue"
import { User } from "../models/user"
import { Task, UserTaskEntries } from "../models/task"

import useTaskProgress from "../hooks/useTaskProgress"

import EditTask from "./EditTask.vue"
import EditSupertaskHeading from "./EditSupertaskHeading.vue"
import EditTaskHeading from "./EditTaskHeading.vue"
import EditSubtaskEntry from "./EditSubtaskEntry.vue"
import ProgressableEntryHeading from "./ProgressableEntryHeading.vue"

const props = defineProps<{ tasks: UserTaskEntries }>()

const session = inject<Ref<User>>("session")!
const entries = inject<Ref<Task[]>>("entries")!

let visibility = reactive<Record<string, boolean>>({})

const toggleVisibility = (key: string) => {
  visibility[key] = !visibility[key]
}

const { calculateTaskProgress, calculateSubtaskProgress } = useTaskProgress(props.tasks)
</script>

<template>
  <div id="entries-container">
    <div :class="`entry ${visibility[entry.title] ? 'focused': ''}`" v-for="(entry, index) in entries">
      <ProgressableEntryHeading :progress="calculateTaskProgress(index)" :isActive="visibility[entry.title] || false" :index="[index]" :title="entry.title" @click="toggleVisibility(entry.title)" />
      <ul v-show="visibility[entry.title]" :key="index">
        <li v-for="(subTask, subIndex) in entry.tasks">
          <ProgressableEntryHeading :progress="calculateSubtaskProgress([index, subIndex])" :is-active="visibility[entry.title + subTask.title] || false" :index="[index, subIndex]" :title="subTask.title" @click="toggleVisibility(entry.title + subTask.title)" />
          <ul v-show="visibility[entry.title + subTask.title]" :key="`${index},${subIndex}`">
            <li v-for="(task, taskIndex) in subTask.tasks">
              <EditTask
                :task="tasks[index]?.[subIndex]?.[taskIndex] ?? null"
                :value="task"
                :index="[index, subIndex, taskIndex]"
              />
            </li>
            <EditSubtaskEntry v-if="session.isAdmin" :index="[index, subIndex]" />
          </ul>
        </li>
        <EditTaskHeading v-if="session.isAdmin" :index="index" />
      </ul>
    </div>
    <EditSupertaskHeading v-if="session.isAdmin" />
  </div>
</template>

<style scoped lang="scss">
@import "../main.scss";

div#entries-container {
  display: flex;
  flex-direction: column;
  overflow: scroll;
  height: 100lvh;
  margin: auto;
}

.entry {
  border-radius: 10px;
  transition: background-color 0.1s ease;
  padding-left: 15px;
  margin-bottom: 15px;
  
  &.focused,
  &:hover {
    background-color: $tertiary-bg-color;
  }
}

ul {
  list-style-type: none;
  padding-top: 10px;
}
</style>