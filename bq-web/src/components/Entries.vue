<script setup lang="ts">
import { Ref, defineProps, inject, reactive } from "vue"
import { UserSession } from "../models/user"
import { UserTaskEntries } from "../models/task"

import EntryHeading from "./EntryHeading.vue"
import TaskEditor from "./TaskEditor.vue"
import EditTaskHeadings from "./EditTaskHeadings.vue"

defineProps<{ tasks: UserTaskEntries }>()

const sessionContext = inject<Ref<UserSession>>("session")!
let visibility = reactive<Record<string, boolean>>({})

const toggleVisibility = (key: string) => {
  visibility[key] = !visibility[key]
}
</script>

<template>
  <div v-for="(entry, index) in sessionContext.entries">
    <EntryHeading :index="[index]" :title="entry.title" @click="toggleVisibility(entry.title)" />
    <ul v-show="visibility[entry.title]">
      <li v-for="(subTask, subIndex) in entry.tasks">
        <EntryHeading :index="[index, subIndex]" :title="subTask.title" @click="toggleVisibility(entry.title + subTask.title)" />
        <ul v-show="visibility[entry.title + subTask.title]">
          <li
            v-for="(task, taskIndex) in subTask.tasks"
            :key="taskIndex"
            :data-index="`${index},${subIndex},${taskIndex}`"
          >
            <TaskEditor
              :task="tasks[index]?.[subIndex]?.[taskIndex] ?? null"
              :value="task"
              :index="[index, subIndex, taskIndex]"
            />
          </li>
          <!-- <EditEntryStructure v-if="sessionContext.user.isAdmin" :entryType="EntryType.SUB_TASK_ENTRY" /> -->
        </ul>
      </li>
      <!-- <EditEntryStructure v-if="sessionContext.user.isAdmin" :entryType="EntryType.SUB_TASK_HEADING" /> -->
    </ul>
  </div>
  <!-- <EditEntryStructure v-if="sessionContext.user.isAdmin" :entryType="EntryType.TASK_HEADING" /> -->
  <EditTaskHeadings v-if="sessionContext.user.isAdmin" />
</template>