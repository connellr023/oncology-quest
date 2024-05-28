<script setup lang="ts">
import { Ref, inject, reactive, ref } from "vue"
import { User } from "../../models/user"
import { EntryStructure, UserTask } from "../../models/tasks"
import { Domain } from "../../models/domain"

// import useTaskProgress from "../../hooks/useTaskProgress"

import EditTask from "./EditTask.vue"
import ProgressableEntryHeading from "./ProgressableEntryHeading.vue"
import PushStackIcon from "../vector/PushStackIcon.vue"
import InputModal from "../InputModal.vue"
import useValidateTitle from "../../hooks/validation/useValidateTitle"
import useEntries from "../../hooks/useEntries"
// import EditTaskHeading from "./EditTaskHeading.vue"
// import EditSubtaskEntry from "./EditSubtaskEntry.vue"

defineProps<{ tasks: Record<number, UserTask> }>()

const session = inject<Ref<User>>("session")!
const selectedDomain = inject<Ref<Domain | null>>("selectedDomain")!
const entries = inject<Ref<Record<number, EntryStructure>>>("entries")!

const { title, titleError } = useValidateTitle()

let createEntryCallback = ref<() => any>(() => {})
const isCreateEntryModalVisible = ref(false)

let visibility = reactive<Record<string, boolean>>({})

const toggleVisibility = (key: string) => {
  visibility[key] = !visibility[key]
}

const computeKey = (...values: number[]) => values.join("-")

const showCreateEntryModal = (onConfirm: (confirmTitle: string) => Promise<Boolean>) => {
  isCreateEntryModalVisible.value = true
  createEntryCallback.value = async () => {
    if (await onConfirm(title.value)) {
      isCreateEntryModalVisible.value = false
      title.value = ""
      titleError.value = ""
    }
  }
}

const {
  createSupertask,
  createTask,
  createSubtask
} = useEntries()
</script>

<template>
  <InputModal
    v-if="session.isAdmin"
    v-model="title"
    :error="titleError"
    :visible="isCreateEntryModalVisible"
    :title="'Create New Entry'"
    :placeholder="'Entry entry title...'"
    :onConfirm="createEntryCallback"
    :onCancel="() => { isCreateEntryModalVisible = false }"
  />
  <div id="entries-container" v-if="selectedDomain">
    <div :class="`supertask focusable ${visibility[computeKey(supertaskIndex)] ? 'focused': ''}`" v-for="(supertask, supertaskIndex) in entries[selectedDomain.id]" :key="computeKey(supertaskIndex)">
      <ProgressableEntryHeading :progress="0" :isActive="visibility[computeKey(supertaskIndex)] || false" :index="[-1]" :title="supertask.entry.title" @click="toggleVisibility(computeKey(supertaskIndex))" />
      <ul v-show="visibility[computeKey(supertaskIndex)]" :key="computeKey(supertaskIndex, -1)">
        <li :class="`task focusable layer-2 ${visibility[computeKey(supertaskIndex, taskIndex)] ? 'focused': ''}`" v-for="(task, taskIndex) in supertask.children" :key="computeKey(supertaskIndex, taskIndex)">
          <ProgressableEntryHeading :progress="0" :isActive="visibility[computeKey(supertaskIndex, taskIndex)] || false" :index="[-1, -1]" :title="task.entry.title" @click="toggleVisibility(computeKey(supertaskIndex, taskIndex))" />
          <ul v-show="visibility[computeKey(supertaskIndex, taskIndex)]" :key="computeKey(supertaskIndex, taskIndex, -1)">
            <li v-for="(subtask, subtaskIndex) in task.children" :key="computeKey(supertaskIndex, taskIndex, subtaskIndex)">
              <EditTask
                :task="tasks[subtask.id] ?? null"
                :value="subtask.title"
                :index="[-1, -1, -1]"
              />
            </li>
            <button class="bubble push highlight" v-if="session.isAdmin" @click="showCreateEntryModal((confirmTitle: string) => createSubtask(confirmTitle, selectedDomain!.id, task.entry.id, supertaskIndex, taskIndex))">
              <PushStackIcon />
              Push New Subtask
            </button>
          </ul>
        </li>
        <button class="bubble push highlight" v-if="session.isAdmin" @click="showCreateEntryModal((confirmTitle: string) => createTask(confirmTitle, selectedDomain!.id, supertask.entry.id, supertaskIndex))">
          <PushStackIcon />
          Push New Task
        </button>
      </ul>
    </div>
    <p class="empty-domain note" v-if="entries[selectedDomain.id].length === 0">This domain is looking sparse.</p>
    <button @click="showCreateEntryModal((confirmTitle: string) => createSupertask(confirmTitle, selectedDomain!.id))" class="bubble push highlight" v-if="session.isAdmin">
      <PushStackIcon />
      Push New Supertask
    </button>
  </div>
  <p class="note" v-else>Select a domain from the list in the top right corner to get started.</p>
</template>

<style scoped lang="scss">
@import "../../main.scss";

button.push {
  width: 100%;
}

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
    padding-bottom: 15px;
  }
}

ul {
  list-style-type: none;
  padding-top: 10px;
  padding-left: 0;
}
</style>