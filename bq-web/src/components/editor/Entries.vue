<script setup lang="ts">
import { Ref, inject, reactive } from "vue"
import { UserSession } from "../../models/user"
import { UserTaskEntries } from "../../models/task"

import EntryHeading from "../EntryHeading.vue"
import EditTask from "./EditTask.vue"
import EditTaskHeadings from "./EditTaskHeadings.vue"
import EditSubTaskHeading from "./EditSubTaskHeading.vue"
import EditSubTaskEntry from "./EditSubTaskEntry.vue"
import EntryProgress from "./EntryProgress.vue"

defineProps<{ tasks: UserTaskEntries }>()

const sessionContext = inject<Ref<UserSession>>("session")!
let visibility = reactive<Record<string, boolean>>({})

const toggleVisibility = (key: string) => {
  visibility[key] = !visibility[key]
}
</script>

<template>
  <div id="entries-container">
    <div :class="`entry ${visibility[entry.title] ? 'focused': ''}`" v-for="(entry, index) in sessionContext.entries">
      <div class="progressable-entry-container">
        <EntryHeading :index="[index]" :title="entry.title" @click="toggleVisibility(entry.title)" />
        <EntryProgress />
      </div>
      <ul v-show="visibility[entry.title]">
        <li v-for="(subTask, subIndex) in entry.tasks">
          <EntryHeading :index="[index, subIndex]" :title="subTask.title" @click="toggleVisibility(entry.title + subTask.title)" />
          <ul v-show="visibility[entry.title + subTask.title]">
            <li
              v-for="(task, taskIndex) in subTask.tasks"
              :key="taskIndex"
              :data-index="`${index},${subIndex},${taskIndex}`"
            >
              <EditTask
                :task="tasks[index]?.[subIndex]?.[taskIndex] ?? null"
                :value="task"
                :index="[index, subIndex, taskIndex]"
              />
            </li>
            <EditSubTaskEntry v-if="sessionContext.user.isAdmin" :index="[index, subIndex]" />
          </ul>
        </li>
        <EditSubTaskHeading v-if="sessionContext.user.isAdmin" :index="index" />
      </ul>
    </div>
    <EditTaskHeadings v-if="sessionContext.user.isAdmin" />
  </div>
</template>

<style scoped lang="scss">
@import "../../main.scss";

div#entries-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}

div.progressable-entry-container {
  position: relative;
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
}
</style>