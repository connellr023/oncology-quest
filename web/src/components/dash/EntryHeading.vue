<script setup lang="ts">
import { Ref, inject, onUnmounted, ref } from "vue"
import { User } from "../../models/user"

import useValidateTitle from "../../hooks/validation/useValidateTitle"

import EditIcon from "../vector/EditIcon.vue"
import CheckIcon from "../vector/CheckIcon.vue"
import CancelIcon from "../vector/CancelIcon.vue"
import DeleteIcon from "../vector/DeleteIcon.vue"
import IconButton from "../IconButton.vue"

defineEmits(["click"])

const props = defineProps<{
  saveHeading: (saveTitle: string) => Promise<boolean>,
  deleteHeading: () => Promise<boolean>,
  title: string
}>()

const session = inject<Ref<User>>("session")!
const isEditing = inject<Ref<boolean>>("isEditing")!

const { title, titleError } = useValidateTitle()

title.value = props.title

const inEditMode = ref(false)
let savedTitle = ""

const toggleEditMode = () => {
  if (isEditing.value && !inEditMode.value) {
    return
  }

  inEditMode.value = !inEditMode.value
  savedTitle = title.value

  if (inEditMode.value) {
    window.addEventListener("click", outsideClickListener)
    isEditing.value = true
  } else {
    window.removeEventListener("click", outsideClickListener)
    isEditing.value = false
  }
}

const outsideClickListener = () => {
  if (inEditMode.value) {
    cancelEdit()
  }
}

const cancelEdit = () => {
  title.value = savedTitle
  toggleEditMode()
}

const saveEdit = async () => {
  if (titleError.value) {
    return
  }

  if (await props.saveHeading(title.value)) {
    toggleEditMode()
  }
}

const deleteTaskHeading = async () => {
  if (!await props.deleteHeading()) {
    console.error("Failed to delete task")
  }

  toggleEditMode()
}

onUnmounted(() => {
  window.removeEventListener("click", outsideClickListener)
})
</script>

<template>
  <div class="entry-heading-container" @click="$emit('click')">
    <div class="header">
      <h3 v-if="!inEditMode" class="entry-heading">{{ title }}</h3>
      <input @click.stop :class="`bubble minimal ${titleError ? 'error' : ''}`" v-else v-model="title" />
    </div>
    <div class="edit-buttons-container" v-if="session.isAdmin">
      <template v-if="inEditMode">
        <IconButton
          firstClass="green"
          @click.stop="saveEdit"
        >
          <template #firstIcon>
            <CheckIcon />
          </template>
        </IconButton>
        <IconButton
          firstClass="red"
          @click.stop="cancelEdit"
        >
          <template #firstIcon>
            <CancelIcon />
          </template>
        </IconButton>
        <IconButton
          firstClass="red"
          @click.stop="deleteTaskHeading"
        >
          <template #firstIcon>
            <DeleteIcon />
          </template>
        </IconButton>
      </template>
      <IconButton
        v-else
        :disabled="isEditing"
        @click.stop="toggleEditMode"
      >
        <template #firstIcon>
          <EditIcon />
        </template>
      </IconButton>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/variables.scss";

h3.entry-heading {
  word-break: break-word;
  font-size: clamp(14px, 1.5lvw, 17px);
  font-weight: normal;
  color: $main-txt-color;
  display: inline-block;
  padding-right: 15px;
  margin: 0;
}

div.header {
  display: flex;
  align-items: center;
  padding: 8px;
  padding-left: 0;
}

div.entry-heading-container {
  display: flex;
  align-items: center;
}

div.edit-buttons-container {
  margin-right: 15px;

  span {
    margin-left: 15px;
  }

  button {
    margin: auto;
    margin-left: 2px;

    &.edit {
      margin-left: auto;
    }
  }
}
</style>
