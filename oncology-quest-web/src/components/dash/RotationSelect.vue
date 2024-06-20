<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { Rotation } from "../../models/rotation"
import { User } from "../../models/user"

import useEntries from "../../hooks/useEntries"
import useRotations from "../../hooks/useRotations"
import useUserTasks from "../../hooks/useUserTasks"

import CheckIcon from "../vector/CheckIcon.vue"
import EditIcon from "../vector/EditIcon.vue"
import CancelIcon from "../vector/CancelIcon.vue"
import DeleteIcon from "../vector/DeleteIcon.vue"

const session = inject<Ref<User>>("session")!
const rotations = inject<Ref<Record<number, Rotation>>>("rotations")!
const selectedRotation = inject<Ref<Rotation | null>>("selectedRotation")!
const selectedUser = inject<Ref<User | null>>("selectedUser")!

const { fetchEntriesWithCaching } = useEntries()
const { fetchOwnTasksWithCaching, fetchUserTasks } = useUserTasks()
const { deleteRotation } = useRotations()

const isEditing = ref(false)

const seenRotations = new Set<number>()

const selectRotation = async (rotation: Rotation) => {
  if (session.value.isAdmin) {
    if (selectedUser.value && !await fetchUserTasks(rotation.id, selectedUser.value.id)) {
      console.error("Failed to fetch user tasks.")
      return
    }
  }

  if (seenRotations.has(rotation.id)) {
    selectedRotation.value = rotation
    return
  }

  if (!await fetchEntriesWithCaching(rotation.id)) {
    console.error("Failed to fetch entries.")
    return
  }

  if (!session.value.isAdmin) {
    if (!await fetchOwnTasksWithCaching(rotation.id)) {
      console.error("Failed to fetch owned tasks.")
      return
    }
  }

  seenRotations.add(rotation.id)
  selectedRotation.value = rotation
}

const unselectRotation = () => {
  selectedRotation.value = null
}

const toggleIsEditing = () => {
  unselectRotation()
  isEditing.value = !isEditing.value
}

const onRotationClick = (rotation: Rotation) => {
  if (isEditing.value) {
    deleteRotation(rotation.id)
  }
  else {
    if (selectedRotation.value?.id === rotation.id) {
      unselectRotation()
    }
    else {
      selectRotation(rotation)
    }
  }
}
</script>

<template>
  <div class="rotation-select-container">
    <div class="heading-container">
      <h1 class="section-heading">Rotations</h1>
      <button :class="`icon-button ${isEditing ? 'red' : 'highlight'}`" v-if="session.isAdmin" @click="toggleIsEditing">
        <template v-if="isEditing">
          <CancelIcon  />
          Cancel
        </template>
        <template v-else>
          <EditIcon />
          Edit
        </template>
      </button>
    </div>
    <div class="rotations" v-if="Object.keys(rotations).length > 0">
      <button
        v-for="rotation in rotations"
        :class="`rotation bubble ${isEditing ? 'red' : ''} ${selectedRotation?.id === rotation.id ? 'focused' : ''}`"
        :key="rotation.id"
        @click="onRotationClick(rotation)"
      >
        <CheckIcon v-show="selectedRotation?.id === rotation.id" />
        <DeleteIcon v-if="isEditing" />
        {{ rotation.name }}
      </button>
    </div>
    <p class="no-rotations" v-else>No Rotations Available.</p>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/variables.scss";

p.no-rotations {
  text-align: center;
  font-size: clamp(18px, 1.3lvw, 23px);
  opacity: 0.7;
  padding: 8px;
}

div.heading-container {
  display: flex;

  h1 {
    margin-right: 20px;
  }
}

div.rotations {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  gap: 10px;

  button.rotation {
    background-color: $secondary-bg-color;
    flex: 1 0 auto;
    min-width: 130px;
    height: 50px;

    &:hover {
      background-color: $tertiary-bg-color;
    }

    &.focused {
      background-color: $tertiary-bg-color;
      color: $theme-color-green;
      
      svg {
        fill: $theme-color-green;
      }
    }
  }
}
</style>