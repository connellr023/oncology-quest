<script setup lang="ts">
import { Ref, inject, reactive } from "vue"
import { UserSession } from "../../models/user"
import { UserTaskEntries } from "../../models/task"

import EditTask from "./EditTask.vue"
import EditTaskHeadings from "./EditTaskHeadings.vue"
import EditSubTaskHeading from "./EditSubTaskHeading.vue"
import EditSubTaskEntry from "./EditSubTaskEntry.vue"
import ProgressableEntryHeading from "../ProgressableEntryHeading.vue"
import useTaskProgress from "../../hooks/useTaskProgress"

defineProps<{ tasks: UserTaskEntries }>()

const session = inject<Ref<UserSession>>("session")!
let visibility = reactive<Record<string, boolean>>({})

const toggleVisibility = (key: string) => {
  visibility[key] = !visibility[key]
}

const { calculateTaskProgress, calculateSubtaskProgress } = useTaskProgress()
</script>

<template>
  <div id="entries-container">
    <div :class="`entry ${visibility[entry.title] ? 'focused': ''}`" v-for="(entry, index) in session.entries">
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
            <EditSubTaskEntry v-if="session.user.isAdmin" :index="[index, subIndex]" />
          </ul>
        </li>
        <EditSubTaskHeading v-if="session.user.isAdmin" :index="index" />
      </ul>
    </div>
    <EditTaskHeadings v-if="session.user.isAdmin" />
  </div>
</template>

<style scoped lang="scss">
@import "../../main.scss";

div#entries-container {
  display: flex;
  flex-direction: column;
  overflow: scroll;
  height: 100lvh;
  padding-right: 25px;
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