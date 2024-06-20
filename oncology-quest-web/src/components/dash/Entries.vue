<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { User } from "../../models/user"
import { EntryStructure, UserTaskStructure } from "../../models/tasks"
import { Rotation } from "../../models/rotation"

import useProgress from "../../hooks/useProgress"

import useValidateTitle from "../../hooks/validation/useValidateTitle"
import useEntries from "../../hooks/useEntries"
import UserTaskEntryItem from "./UserTaskEntryItem.vue"
import ProgressableEntryItem from "./ProgressableEntryItem.vue"
import PushStackIcon from "../vector/PushStackIcon.vue"
import InputModal from "../InputModal.vue"

const props = defineProps<{
  tasks: UserTaskStructure,
  selectedRotation: Rotation,
  externalKey?: string | number,
}>()

const session = inject<Ref<User>>("session")!
const entries = inject<Ref<Record<number, EntryStructure>>>("entries")!

const { title, titleError } = useValidateTitle()

const createEntryCallback = ref<() => any>(() => {})
const createEntryModalTitle = ref("")

const isCreateEntryModalVisible = ref(false)
const isCreateEntryLoading = ref(false)

const showCreateEntryModal = (modalTitle: string, onConfirm: (confirmTitle: string) => Promise<Boolean>) => {
  isCreateEntryModalVisible.value = true

  createEntryModalTitle.value = modalTitle
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
    :title="createEntryModalTitle"
    :placeholder="'Entry entry title...'"
    :onConfirm="createEntryCallback"
    :onCancel="() => { isCreateEntryModalVisible = false }"
  />
  <div class="entries-container">
    <h1 class="section-heading">Tasks</h1>
    <ul>
      <ProgressableEntryItem
        v-for="(supertask, supertaskIndex) in entries[selectedRotation.id]"
        class="supertask"
        :key="`supertask.${supertask.entry.id}`"
        :progress="calculateSupertaskProgress(selectedRotation!.id, supertaskIndex)"
        :title="supertask.entry.title"
        :saveHeading="(saveTitle: string) => updateSupertask(selectedRotation!.id, supertaskIndex, supertask.entry.id, saveTitle)"
        :deleteHeading="() => deleteSupertask(selectedRotation!.id, supertaskIndex, supertask.entry.id)"
      >
        <ProgressableEntryItem
          v-for="(task, taskIndex) in supertask.children"
          class="layer-2"
          :key="`task.${task.entry.id}`"
          :progress="calculateTaskProgress(selectedRotation!.id, supertaskIndex, taskIndex)"
          :title="task.entry.title"
          :saveHeading="(saveTitle: string) => updateTask(selectedRotation!.id, supertaskIndex, taskIndex, task.entry.id, saveTitle)"
          :deleteHeading="() => deleteTask(selectedRotation!.id, supertaskIndex, taskIndex, task.entry.id)"
        >
          <UserTaskEntryItem
            v-for="(subtask, subtaskIndex) in task.children"
            :key="`subtask.${subtask.id}`"
            :tasks="props.tasks"
            :subtaskId="subtask.id"
            :value="subtask.title"
            :saveHeading="(saveTitle: string) => updateSubtask(selectedRotation!.id, supertaskIndex, taskIndex, subtaskIndex, subtask.id, saveTitle)"
            :deleteHeading="() => deleteSubtask(selectedRotation!.id, supertaskIndex, taskIndex, subtaskIndex, subtask.id)"
          />
          <button class="bubble push highlight" v-if="session.isAdmin" @click="showCreateEntryModal('Create Clinical Experience Entry', (confirmTitle: string) => createSubtask(confirmTitle, selectedRotation!.id, task.entry.id, supertaskIndex, taskIndex))">
            <PushStackIcon />
            Push New Clinical Experience
          </button>
        </ProgressableEntryItem>
        <button class="bubble push highlight" v-if="session.isAdmin" @click="showCreateEntryModal('Create EPA Entry', (confirmTitle: string) => createTask(confirmTitle, selectedRotation!.id, supertask.entry.id, supertaskIndex))">
          <PushStackIcon />
          Push New EPA
        </button>
      </ProgressableEntryItem>
    </ul>
    <div class="empty-rotation note" v-if="entries[selectedRotation.id].length === 0">This rotation is looking sparse.</div>
    <button @click="showCreateEntryModal('Create CBD Phase Entry', (confirmTitle: string) => createSupertask(confirmTitle, selectedRotation!.id))" class="bubble push highlight" v-if="session.isAdmin">
      <PushStackIcon />
      Push New CBD Phase
    </button>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/variables.scss";

button.push {
  width: 100%;
}

li.supertask {
  background-color: rgba($secondary-bg-color, 0.65);

  &:hover,
  &.focused {
    background-color: $secondary-bg-color;
  }
}
</style>