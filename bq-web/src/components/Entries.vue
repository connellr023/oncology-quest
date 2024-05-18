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
    <div :class="`entry ${visibility[entry.title] ? 'focused': ''}`" v-for="(entry, superIndex) in entries">
      <ProgressableEntryHeading :progress="calculateTaskProgress(superIndex)" :isActive="visibility[entry.title] || false" :index="[superIndex]" :title="entry.title" @click="toggleVisibility(entry.title)" />
      <ul v-show="visibility[entry.title]" :key="superIndex">
        <li v-for="(subTask, index) in entry.tasks">
          <ProgressableEntryHeading :progress="calculateSubtaskProgress([superIndex, index])" :is-active="visibility[entry.title + subTask.title] || false" :index="[superIndex, index]" :title="subTask.title" @click="toggleVisibility(entry.title + subTask.title)" />
          <ul v-show="visibility[entry.title + subTask.title]" :key="`${superIndex},${index}`">
            <li v-for="(task, subIndex) in subTask.tasks">
              <EditTask
                :task="tasks[superIndex]?.[index]?.[subIndex] ?? null"
                :value="task"
                :index="[superIndex, index, subIndex]"
              />
            </li>
            <EditSubtaskEntry v-if="session.isAdmin" :index="[0, superIndex, index]" />
          </ul>
        </li>
        <EditTaskHeading v-if="session.isAdmin" :index="[0, superIndex]" />
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