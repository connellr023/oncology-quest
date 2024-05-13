<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { UserSession } from "../models/user";
import useSaveEntries from "../hooks/useSaveEntries";

defineEmits(["click"])
const props = defineProps<{
  title: string,
  isActive: boolean,
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
  <div :class="`entry-heading-container ${isActive ? 'active' : ''}`">
    <div class="header" v-if="!inEditMode">
      <img src="/arrow.svg" alt="arrow" class="arrow" />
      <h3 @click="$emit('click')" class="dropdown">{{ title }}</h3>
    </div>
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
  </div>
</template>

<style scoped lang="scss">
@import "../main.scss";

div.header {
  display: flex;
  align-items: center;
}

img.arrow {
  opacity: 0.6;
  width: 16px;
  margin-right: 10px;
  display: inline-block;
  transform: rotate(90deg);
  transition: all 0.2s ease;
}

div.active {
  img.arrow {
    opacity: 1;
    transform: rotate(180deg);
  }
}

h3.dropdown {
  cursor: pointer;
  font-size: clamp(17px, 1.5lvw, 22px);
  font-weight: normal;
  color: $main-txt-color;
  display: inline-block;
}

button {
  margin-left: 10px;
}
</style>