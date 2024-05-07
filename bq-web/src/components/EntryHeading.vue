<script setup lang="ts">
import { Ref, defineProps, inject, ref } from "vue"
import { UserSession } from "../models/user";
import useSaveEntries from "../hooks/useSaveEntries";

const props = defineProps<{
  title: string,
  index: number[]
}>()

const emit = defineEmits(["click"])
const sessionContext = inject<Ref<UserSession>>("session")!

const { title, titleError, message, save } = useSaveEntries()

title.value = props.title

const inEditMode = ref(false)
let savedTitle = ""

const toggleEditMode = () => {
  inEditMode.value = !inEditMode.value
  savedTitle = title.value
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
    <button v-else @click="toggleEditMode">Edit</button>
  </template>
</template>

<style scoped lang="scss">
.dropdown {
  cursor: pointer;
  display: inline-block;

  &:hover {
    text-decoration: underline;
  }
}

button {
  margin-left: 10px;
}
</style>