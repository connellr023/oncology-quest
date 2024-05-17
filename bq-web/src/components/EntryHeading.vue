<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { User } from "../models/user"

import useSaveEntryTitle from "../hooks/useSaveEntryTitle"

defineEmits(["click"])
const props = defineProps<{
  title: string,
  isActive: boolean,
  index: number[]
}>()

const session = inject<Ref<User>>("session")!
const isEditing = inject<Ref<boolean>>("isEditing")!

const { title, titleError, message, save } = useSaveEntryTitle()

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
  <div :class="`entry-heading-container ${isActive ? 'active' : ''}`" @click="$emit('click')">
    <div class="header">
      <img draggable="false" src="/arrow.svg" alt="arrow" class="arrow" />
      <h3 v-if="!inEditMode" class="dropdown">{{ title }}</h3>
      <input @click.stop class="minimal" v-else v-model="title" />
    </div>
    <div class="edit-buttons-container" v-if="session.isAdmin">
      <template v-if="inEditMode">
        <button class="minimal" @click.stop="cancelEdit">Cancel</button>
        <button class="minimal" @click.stop="saveEdit">Save</button>
        <span v-if="titleError">{{ titleError }}</span>
        <span v-if="message">{{ message }}</span>
      </template>
      <button class="edit minimal" v-else :disabled="isEditing" @click.stop="toggleEditMode">Edit</button>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../main.scss";

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

img.arrow {
  user-select: none;
  opacity: 0.6;
  width: 16px;
  margin-right: 10px;
  display: inline-block;
  transform: rotate(90deg);
  transition: all 0.2s ease;
  filter: drop-shadow(0px 0px 4px rgba(255, 255, 255, 0.03));
}

div.active {
  img.arrow {
    opacity: 1;
    transform: rotate(180deg);
  }
}

h3.dropdown {
  font-size: clamp(16px, 1.5lvw, 19px);
  font-weight: normal;
  color: $main-txt-color;
  display: inline-block;
  padding-right: 15px;
  margin: auto;
}

div.edit-buttons-container {
  span {
    margin-left: 15px;
  }

  button {
    margin: auto;
    margin-left: 10px;

    &.edit {
      margin-right: 15px;
      margin-left: auto;
    }
  }
}
</style>