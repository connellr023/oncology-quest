<script setup lang="ts">
import { Ref, inject, reactive, ref } from "vue"
import { User } from "../../models/user"
import { EntryStructure, UserTask } from "../../models/tasks"
import { Rotation } from "../../models/rotation"

import useProgress from "../../hooks/useProgress"

import useValidateTitle from "../../hooks/validation/useValidateTitle"
import useEntries from "../../hooks/useEntries"
import UserTaskEntry from "./UserTaskEntry.vue"
import ProgressableEntryHeading from "./ProgressableEntryHeading.vue"
import PushStackIcon from "../vector/PushStackIcon.vue"
import InputModal from "../InputModal.vue"
import SelectRotationGraphic from "../vector/SelectRotationGraphic.vue"

const props = defineProps<{
  tasks: Record<number, UserTask>,
  selectedRotation: Rotation | null
  externalKey?: string | number,
}>()

const session = inject<Ref<User>>("session")!
const entries = inject<Ref<Record<number, EntryStructure>>>("entries")!

const { title, titleError } = useValidateTitle()

const createEntryCallback = ref<() => any>(() => {})
const isCreateEntryModalVisible = ref(false)
const isCreateEntryLoading = ref(false)

let visibility = reactive<Record<string, boolean>>({})

const toggleVisibility = (key: string) => {
  visibility[key] = !visibility[key]
}

const computeKey = (...values: number[]) => values.join("-")

const showCreateEntryModal = (onConfirm: (confirmTitle: string) => Promise<Boolean>) => {
  isCreateEntryModalVisible.value = true
  createEntryCallback.value = async () => {
    isCreateEntryLoading.value = true

    if (await onConfirm(title.value)) {
      isCreateEntryModalVisible.value = false
      title.value = ""
      titleError.value = ""
    }

    isCreateEntryLoading.value = false
  }
}

const {
  createSupertask,
  updateSupertask,
  deleteSupertask,
  createTask,
  updateTask,
  deleteTask,
  createSubtask,
  updateSubtask,
  deleteSubtask
} = useEntries()

const {
  calculateSupertaskProgress,
  calculateTaskProgress
} = useProgress(props.tasks)
</script>

<template>
  <InputModal
    v-if="session.isAdmin"
    v-model="title"
    :isPassword="false"
    :error="titleError"
    :loading="isCreateEntryLoading"
    :visible="isCreateEntryModalVisible"
    :title="'Create New Entry'"
    :placeholder="'Entry entry title...'"
    :onConfirm="createEntryCallback"
    :onCancel="() => { isCreateEntryModalVisible = false }"
  />
  <div class="entries-container" v-if="selectedRotation" :key="selectedRotation.id">
    <h1 class="section-heading">Tasks</h1>
    <div :class="`supertask focusable ${visibility[computeKey(supertaskIndex)] ? 'focused': ''}`" v-for="(supertask, supertaskIndex) in entries[selectedRotation.id]" :key="computeKey(supertaskIndex)">
      <ProgressableEntryHeading
        :progress="calculateSupertaskProgress(selectedRotation!.id, supertaskIndex)"
        :isActive="visibility[computeKey(supertaskIndex)] || false"
        :title="supertask.entry.title"
        :saveHeading="(saveTitle: string) => updateSupertask(selectedRotation!.id, supertaskIndex, supertask.entry.id, saveTitle)"
        :deleteHeading="() => deleteSupertask(selectedRotation!.id, supertaskIndex, supertask.entry.id)"
        @click="toggleVisibility(computeKey(supertaskIndex))"
      />
      <ul v-show="visibility[computeKey(supertaskIndex)]" :key="computeKey(supertaskIndex, -1)">
        <li :class="`task focusable layer-2 ${visibility[computeKey(supertaskIndex, taskIndex)] ? 'focused': ''}`" v-for="(task, taskIndex) in supertask.children" :key="computeKey(supertaskIndex, taskIndex)">
          <ProgressableEntryHeading
            :progress="calculateTaskProgress(selectedRotation!.id, supertaskIndex, taskIndex)"
            :isActive="visibility[computeKey(supertaskIndex, taskIndex)] || false"
            :title="task.entry.title"
            :saveHeading="(saveTitle: string) => updateTask(selectedRotation!.id, supertaskIndex, taskIndex, task.entry.id, saveTitle)"
            :deleteHeading="() => deleteTask(selectedRotation!.id, supertaskIndex, taskIndex, task.entry.id)"
            @click="toggleVisibility(computeKey(supertaskIndex, taskIndex))"
          />
          <ul v-show="visibility[computeKey(supertaskIndex, taskIndex)]" :key="computeKey(supertaskIndex, taskIndex, -1)">
            <li v-for="(subtask, subtaskIndex) in task.children" :key="computeKey(supertaskIndex, taskIndex, subtaskIndex)">
              <UserTaskEntry
                :tasks="props.tasks"
                :subtaskId="subtask.id"
                :value="subtask.title"
                :saveHeading="(saveTitle: string) => updateSubtask(selectedRotation!.id, supertaskIndex, taskIndex, subtaskIndex, subtask.id, saveTitle)"
                :deleteHeading="() => deleteSubtask(selectedRotation!.id, supertaskIndex, taskIndex, subtaskIndex, subtask.id)"
              />
            </li>
            <button class="bubble push highlight" v-if="session.isAdmin" @click="showCreateEntryModal((confirmTitle: string) => createSubtask(confirmTitle, selectedRotation!.id, task.entry.id, supertaskIndex, taskIndex))">
              <PushStackIcon />
              Push New Clinical Experience
            </button>
          </ul>
        </li>
        <button class="bubble push highlight" v-if="session.isAdmin" @click="showCreateEntryModal((confirmTitle: string) => createTask(confirmTitle, selectedRotation!.id, supertask.entry.id, supertaskIndex))">
          <PushStackIcon />
          Push New EPA
        </button>
      </ul>
    </div>
    <div class="empty-rotation note" v-if="entries[selectedRotation.id].length === 0">This rotation is looking sparse.</div>
    <button @click="showCreateEntryModal((confirmTitle: string) => createSupertask(confirmTitle, selectedRotation!.id))" class="bubble push highlight" v-if="session.isAdmin">
      <PushStackIcon />
      Push New CBD Phase
    </button>
  </div>
  <div v-else class="note">
    <SelectRotationGraphic class="graphic" />
    <p>Select a rotation from the above list to get started.</p>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/variables.scss";

button.push {
  width: 100%;
}

div.supertask {
  background-color: rgba($secondary-bg-color, 0.65);

  &:hover,
  &.focused {
    background-color: $secondary-bg-color;
  }
}

div.note {
  text-align: center;
  font-size: clamp(19px, 1.3lvw, 24px);
  text-wrap: wrap;
  text-align: center;
  margin-top: 10lvh;

  svg {
    width: 20lvw;
    min-width: 240px;
    max-width: 300px;
    fill: $tertiary-bg-color;
  }

  &.empty-rotation {
    padding: 30px;
  }

  &.empty-rotation,
  p {
    opacity: 0.8;
  }
}

div.entries-container {
  display: flex;
  flex-direction: column;
  margin: auto;
  margin-top: 15px;
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