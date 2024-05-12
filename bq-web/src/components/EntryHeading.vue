<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { UserSession } from "../models/user";
import useSaveEntries from "../hooks/useSaveEntries";

defineEmits(["click"])
const props = defineProps<{
  title: string,
  index: number[]
}>()

const sessionContext = inject<Ref<UserSession>>("session")!
const isEditing = inject<Ref<boolean>>("isEditing")!

const { title, titleError, message, save } = useSaveEntries()

title.value = props.title

const inEditMode = ref(false)
let savedTitle = ""

const toggleEditMode = () => {
  inEditMode.value = !inEditMode.value
  savedTitle = title.value

  if (inEditMode.value) {
    isEditing.value = true
  } else {
    isEditing.value = false
  }
}

const cancelEdit = () => {
  title.value = savedTitle
  toggleEditMode()
}

const saveEdit = async () => {
  if (await save(props.index)) {
    toggleEditMode()
  }
}
</script>

<template>
  <h3 v-if="!inEditMode" @click="$emit('click')" class="dropdown">{{ title }}</h3>
  <input v-else v-model="title" />
  <template v-if="sessionContext.user.isAdmin">
    <template v-if="inEditMode">
      <button @click="cancelEdit">Cancel</button>
      <button @click="saveEdit">Save</button>
      <span v-if="titleError">{{ titleError }}</span>
      <span v-if="message">{{ message }}</span>
    </template>
    <button v-else :disabled="isEditing" @click="toggleEditMode">Edit</button>
  </template>
</template>

<style scoped lang="scss">
@import "../main.scss";

h3 {
  font-size: clamp(24px, 1.5lvw, 27px);
  font-weight: normal;
  color: $main-txt-color;
}

.dropdown {
  cursor: pointer;
}

button {
  margin-left: 10px;
}
</style>