<script setup lang="ts">
import { Ref, inject, reactive } from "vue"
import { User } from "../../models/user"
import { Task, UserTaskEntries } from "../../models/task"

import useTaskProgress from "../../hooks/useTaskProgress"

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

const computeKey = (...indicies: number[]) => indicies.join("-")

const { calculateTaskProgress, calculateSupertaskProgress } = useTaskProgress(props.tasks)
</script>

<template>
  <div id="entries-container">
    <div :class="`supertask focusable ${visibility[computeKey(superIndex)] ? 'focused': ''}`" v-for="(entry, superIndex) in entries" :key="computeKey(superIndex)">
      <ProgressableEntryHeading :progress="calculateSupertaskProgress(superIndex)" :isActive="visibility[computeKey(superIndex)] || false" :index="[superIndex]" :title="entry.title" @click="toggleVisibility(computeKey(superIndex))" />
      <ul v-show="visibility[computeKey(superIndex)]" :key="computeKey(superIndex, 0)">
        <li :class="`task focusable layer-2 ${visibility[computeKey(superIndex, index)] ? 'focused': ''}`" v-for="(subTask, index) in entry.tasks" :key="computeKey(superIndex, index)">
          <ProgressableEntryHeading :progress="calculateTaskProgress([superIndex, index])" :isActive="visibility[computeKey(superIndex, index)] || false" :index="[superIndex, index]" :title="subTask.title" @click="toggleVisibility(computeKey(superIndex, index))" />
          <ul v-show="visibility[computeKey(superIndex, index)]" :key="computeKey(superIndex, index, 0)">
            <li v-for="(task, subIndex) in subTask.tasks" :key="computeKey(superIndex, index, subIndex)">
              <EditTask
                :task="tasks[superIndex]?.[index]?.[subIndex] ?? null"
                :value="task"
                :index="[superIndex, index, subIndex]"
              />
            </li>
            <EditSubtaskEntry v-if="session.isAdmin" :index="[superIndex, index]" />
          </ul>
        </li>
        <EditTaskHeading v-if="session.isAdmin" :index="superIndex" />
      </ul>
    </div>
    <EditSupertaskHeading v-if="session.isAdmin" />
  </div>
</template>

<style scoped lang="scss">
@import "../../main.scss";

div#entries-container {
  display: flex;
  flex-direction: column;
  overflow: scroll;
  margin: auto;
  max-height: 100%;
}

.focusable {
  transition: background-color 0.1s ease;
  margin-bottom: 15px;
  padding-left: 15px;
  padding-right: 15px;
  border-radius: 10px;

  &.focused,
  &:hover {
    background-color: $secondary-bg-color;

    &.layer-2 {
      background-color: $tertiary-bg-color;
    }
  }
}

.task {
  &.focused {
    padding-bottom: 20px;
  }
}

ul {
  list-style-type: none;
  padding-top: 10px;
}
</style>