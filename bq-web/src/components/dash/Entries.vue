<script setup lang="ts">
import { Ref, inject, reactive } from "vue"
import { User } from "../../models/user"
import { EntryStructure, UserTask } from "../../models/tasks"
import { Domain } from "../../models/domain"

// import useTaskProgress from "../../hooks/useTaskProgress"

import EditTask from "./EditTask.vue"
import EditSupertaskHeading from "./EditSupertaskHeading.vue"
import ProgressableEntryHeading from "./ProgressableEntryHeading.vue"
// import EditTaskHeading from "./EditTaskHeading.vue"
// import EditSubtaskEntry from "./EditSubtaskEntry.vue"

defineProps<{ tasks: Record<number, UserTask> }>()

const session = inject<Ref<User>>("session")!
const selectedDomain = inject<Ref<Domain | null>>("selectedDomain")!
const entries = inject<Ref<Record<number, EntryStructure>>>("entries")!

let visibility = reactive<Record<string, boolean>>({})

const toggleVisibility = (key: string) => {
  visibility[key] = !visibility[key]
}

const computeKey = (...values: number[]) => values.join("-")

// const { calculateTaskProgress, calculateSupertaskProgress } = useTaskProgress(props.tasks)
</script>

<template>
  <div id="entries-container" v-if="selectedDomain">
    <div :class="`supertask focusable ${visibility[computeKey(domainId)] ? 'focused': ''}`" v-for="(supertask, domainId) in entries[selectedDomain.id]" :key="computeKey(domainId)">
      <ProgressableEntryHeading :progress="0" :isActive="visibility[computeKey(domainId)] || false" :index="[-1]" :title="supertask.entry.title" @click="toggleVisibility(computeKey(domainId))" />
      <ul v-show="visibility[computeKey(domainId)]" :key="computeKey(domainId, -1)">
        <li :class="`task focusable layer-2 ${visibility[computeKey(domainId, taskIndex)] ? 'focused': ''}`" v-for="(task, taskIndex) in supertask.children" :key="computeKey(domainId, taskIndex)">
          <ProgressableEntryHeading :progress="0" :isActive="visibility[computeKey(domainId, taskIndex)] || false" :index="[-1, -1]" :title="task.entry.title" @click="toggleVisibility(computeKey(domainId, taskIndex))" />
          <ul v-show="visibility[computeKey(domainId, taskIndex)]" :key="computeKey(domainId, taskIndex, -1)">
            <li v-for="(subtask, subtaskIndex) in task.children" :key="computeKey(domainId, taskIndex, subtaskIndex)">
              <EditTask
                :task="tasks[subtask.id] ?? null"
                :value="subtask.title"
                :index="[-1, -1, -1]"
              />
            </li>
            <!-- <EditSubtaskEntry v-if="session.isAdmin" :index="[superIndex, index]" /> -->
          </ul>
        </li>
        <!-- <EditTaskHeading v-if="session.isAdmin" :index="superIndex" /> -->
      </ul>
    </div>
    <p class="empty-domainn note" v-if="entries[selectedDomain.id] ? entries[selectedDomain.id].length === 0 : true">This domain is looking sparse.</p>
    <EditSupertaskHeading v-if="session.isAdmin" />
  </div>
  <p class="note" v-else>Select a domain from the list in the top right corner to get started.</p>
</template>

<style scoped lang="scss">
@import "../../main.scss";

p.note {
  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
  font-size: clamp(19px, 1.3lvw, 24px);
  text-wrap: wrap;
  opacity: 0.8;
  height: 100%;
}

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