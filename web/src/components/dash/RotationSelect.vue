<script setup lang="ts">
import { Ref, inject, ref, watch } from "vue"
import { Rotation } from "../../models/rotation"
import { User } from "../../models/user"

import useEntries from "../../hooks/useEntries"
import useRotations from "../../hooks/useRotations"
import useUserTasks from "../../hooks/useUserTasks"
import useValidateName from "../../hooks/validation/useValidateName"
import useNotifications from "../../hooks/useNotifications"

import CheckIcon from "../vector/CheckIcon.vue"
import DeleteIcon from "../vector/DeleteIcon.vue"
import NewRotationIcon from "../vector/NewRotationIcon.vue"
import InputModal from "../InputModal.vue"
import IconButton from "../IconButton.vue"

const session = inject<Ref<User>>("session")!
const rotations = inject<Ref<Record<number, Rotation>>>("rotations")!
const selectedRotation = inject<Ref<Rotation | null>>("selectedRotation")!
const selectedUser = inject<Ref<User | null>>("selectedUser")!

const { fetchEntries } = useEntries()
const { fetchOwnTasks, fetchUserTasks } = useUserTasks()
const { deleteRotation, createRotation } = useRotations()
const { name, nameError } = useValidateName()
const { pushNotification } = useNotifications()

const isDeleting = ref(false)
const isCreateRotationLoading = ref(false)
const isCreateRotationModalVisible = ref(false)

const handleFetchUserTasks = async (rotation: Rotation) => {
  if (selectedUser.value && !await fetchUserTasks(rotation.id, selectedUser.value.id)) {
    pushNotification("Failed to fetch user tasks. Please try again later.")
    return
  }
}

if (session.value.isAdmin) {
  watch(() => selectedUser.value, () => {
    if (selectedRotation.value) {
      handleFetchUserTasks(selectedRotation.value)
    }
  })
}

const confirmNewRotation = async () => {
  if (nameError.value || name.value.length === 0) {
    return
  }

  isCreateRotationLoading.value = true

  if (await createRotation(name.value)) {
    isCreateRotationModalVisible.value = false
    name.value = ""

    pushNotification("Rotation created successfully.", true)
  }
  else {
    nameError.value = "Failed to create rotation."
  }

  isCreateRotationLoading.value = false
}

const selectRotation = async (rotation: Rotation) => {
  if (session.value.isAdmin) {
    await handleFetchUserTasks(rotation)
  }
  else if (!await fetchOwnTasks(rotation.id)) {
    pushNotification("Failed to fetch own tasks. Please try again later.")
    return
  }

  if (!await fetchEntries(rotation.id)) {
    pushNotification("Failed to fetch entries. Please try again later.")
    return
  }

  selectedRotation.value = rotation
}

const unselectRotation = () => {
  selectedRotation.value = null
}

const toggleIsDeleting = () => {
  unselectRotation()
  isDeleting.value = !isDeleting.value
}

const onRotationClick = async (rotation: Rotation) => {
  if (isDeleting.value) {
    if (await deleteRotation(rotation.id)) {
      pushNotification("Rotation deleted successfully.", true)
    }
    else {
      pushNotification("Failed to delete rotation.")
    }
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
  <InputModal
    v-if="session.isAdmin"
    v-model="name"
    title="New Rotation"
    placeholder="Enter rotation name..."
    :loading="isCreateRotationLoading"
    :error="nameError"
    :visible="isCreateRotationModalVisible"
    :isPassword="false"
    :onConfirm="confirmNewRotation"
    :onCancel="() => { isCreateRotationModalVisible = false }"
  />
  <div class="rotation-select-container">
    <div class="heading-container">
      <div class="spacer">
        <h1 class="section-heading">Rotations</h1>
      </div>
      <template v-if="session.isAdmin">
        <IconButton
          class="select-rotation-button"
          firstClass="green"
          firstText="New"
          @click="() => { isCreateRotationModalVisible = true }"
        >
          <template #firstIcon>
            <NewRotationIcon />
          </template>
        </IconButton>
        <IconButton
          :isToggled="isDeleting"
          firstText="Delete"
          firstClass="red"
          secondText="Done"
          secondClass="green"
          @click="toggleIsDeleting"
        >
          <template #firstIcon>
            <DeleteIcon />
          </template>
          <template #secondIcon>
            <CheckIcon />
          </template>
        </IconButton>
      </template>
    </div>
    <div class="rotations" v-if="Object.keys(rotations).length > 0">
      <button
        v-for="rotation in rotations"
        :class="`ripple rotation bubble ${isDeleting ? 'red' : ''} ${selectedRotation?.id === rotation.id ? 'focused' : ''}`"
        :key="rotation.id"
        @click="onRotationClick(rotation)"
      >
        <span>
          <CheckIcon v-show="selectedRotation?.id === rotation.id" />
          <DeleteIcon v-if="isDeleting" />
          <span>{{ rotation.name }}</span>
        </span>
      </button>
    </div>
    <p class="no-rotations" v-else>No Rotations Available.</p>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/variables.scss";

button.select-rotation-button {
  margin-right: 5px;
}

div.rotation-select-container {
  margin-top: 40px;
  margin-bottom: 40px;
}

p.no-rotations {
  text-align: center;
  font-size: clamp(18px, 1.3lvw, 23px);
  opacity: 0.7;
  padding: 8px;
}

div.heading-container {
  display: flex;
  align-items: center;
  flex-direction: row;

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
    border-radius: $ui-border-radius;
    border: none;
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
