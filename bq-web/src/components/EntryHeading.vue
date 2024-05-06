<script setup lang="ts">
import { Ref, defineProps, inject, ref } from "vue"
import { UserSession } from "../models/user";

const props = defineProps<{
  title: string,
  index: number[]
}>()

const emit = defineEmits(["click"])
const sessionContext = inject<Ref<UserSession>>("session")!

const inEditMode = ref(false)
const title = ref(props.title)
let savedTitle = ""

const toggleEditMode = () => {
  inEditMode.value = !inEditMode.value
  savedTitle = title.value
}

const cancelEdit = () => {
  title.value = savedTitle
  toggleEditMode()
}

const saveEdit = () => {
  toggleEditMode()
}
</script>

<template>
  <h3 v-if="!inEditMode" @click="$emit('click')" class="dropdown">{{ title }}</h3>
  <input v-else v-model="title" />
  <template v-if="sessionContext.user.isAdmin">
    <template v-if="inEditMode">
      <button @click="cancelEdit">Cancel</button>
      <button @click="saveEdit">Save</button>
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