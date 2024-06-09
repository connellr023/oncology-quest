<script setup lang="ts">
import { Ref, inject, reactive, ref } from "vue"
import { User } from "../models/user"
import { Rotation } from "../models/rotation"
import { UserTask } from "../models/tasks"

import useLogout from "../hooks/useLogout"
import useValidateName from "../hooks/validation/useValidateName"
import useValidatePassword from "../hooks/validation/useValidatePassword"
import useRotations from "../hooks/useRotations"
import useEntries from "../hooks/useEntries"
import useDeleteUser from "../hooks/useDeleteUser"
import useExportProgress from "../hooks/useExportProgress"

import UserProfileIcon from "./UserProfileIcon.vue"
import LogoutIcon from "./vector/LogoutIcon.vue"
import InputModal from "./InputModal.vue"
import ConfirmationModal from "./ConfirmationModal.vue"
import NewRotationIcon from "./vector/NewRotationIcon.vue"
import DeleteIcon from "./vector/DeleteIcon.vue"
import CheckIcon from "./vector/CheckIcon.vue"
import Dropdown from "./Dropdown.vue"
import ExportIcon from "./vector/ExportIcon.vue"

const resetAll = inject<() => void>("resetAll")!
const session = inject<Ref<User>>("session")!
const rotations = inject<Ref<Record<number, Rotation>>>("rotations")!
const selectedRotation = inject<Ref<Rotation | null>>("selectedRotation")!
const tasks = inject<Ref<Record<number, UserTask>>>("tasks")!

const { logout } = useLogout()
const { name, nameError } = useValidateName()
const { password, passwordError } = useValidatePassword()
const { createRotation, deleteRotation } = useRotations()
const { fetchEntriesWithCaching } = useEntries()
const { deleteSelf } = useDeleteUser()
const { exportProgress } = useExportProgress()

const showProfileOptions = ref(false)
const showCreateRotationModal = ref(false)
const showDeleteAccountModal = ref(false)
const showDeleteRotationModal = ref(false)

const isCreateRotationLoading = ref(false)
const isDeleteAccountLoading = ref(false)

const deleteRotationError = ref("")

let focusedRotationId = -1
const visibleRotationDropdowns = reactive<boolean[]>([])

let scrollLeft = ref(0)
let timeoutId: number | null = null
const rotationsDiv = ref<HTMLElement | null>(null)

const rotationsScrollListener = () => {
  if (timeoutId) {
    return
  }

  const debounceTime = 300

  timeoutId = setTimeout(() => {
    scrollLeft.value = rotationsDiv.value!.scrollLeft;
    timeoutId = null;
  }, debounceTime);
}

const toggleRotationDropdown = (id: number) => {
  visibleRotationDropdowns[id] = !visibleRotationDropdowns[id]

  if (id !== focusedRotationId) {
    visibleRotationDropdowns[focusedRotationId] = false
  }

  if (visibleRotationDropdowns[id]) {
    focusedRotationId = id
  }
  else {
    focusedRotationId = -1
  }
}

const hideAllRotationDropdowns = () => {
  for (let i = 0; i < visibleRotationDropdowns.length; i++) {
    visibleRotationDropdowns[i] = false
  }
}

const toggleProfileOptions = () => {
  showProfileOptions.value = !showProfileOptions.value
}

const selectRotation = async (rotation: Rotation) => {
  if (!await fetchEntriesWithCaching(rotation.id)) {
    console.error("Failed to fetch entries.")
    return
  }

  selectedRotation.value = rotation
  visibleRotationDropdowns[focusedRotationId] = false
}

const confirmNewRotation = async () => {
  if (nameError.value || name.value.length === 0) {
    return
  }

  isCreateRotationLoading.value = true

  if (await createRotation(name.value)) {
    showCreateRotationModal.value = false
    name.value = ""
  }
  else {
    nameError.value = "Failed to create rotation."
  }

  isCreateRotationLoading.value = false
}

const confirmDeleteRotation = () => {
  if (!deleteRotation(focusedRotationId)) {
    deleteRotationError.value = "Failed to delete rotation."
  }
  else {
    showDeleteRotationModal.value = false
    visibleRotationDropdowns[focusedRotationId] = false
    selectedRotation.value = null
  }
}

const shouldAppearFocused = (id: number) => {
  return visibleRotationDropdowns[id] || selectedRotation.value?.id === id
}

const onLogoutClick = async () => {
  resetAll()
  await logout()
}

const onDeleteRotationClick = () => {
  visibleRotationDropdowns[focusedRotationId] = false
  showDeleteRotationModal.value = true
}

const onDeleteAccountClick = () => {
  showDeleteAccountModal.value = true
  showProfileOptions.value = false
}

const onExportProgressClick = () => {
  exportProgress(session.value.name, tasks.value)
  showProfileOptions.value = false
}

const deleteAccount = async () => {
  if (passwordError.value || password.value.length === 0) {
    return
  }

  isDeleteAccountLoading.value = true

  if (await deleteSelf(password.value)) {
    showDeleteAccountModal.value = false
    await onLogoutClick()
  }
  else {
    passwordError.value = "Failed to delete account. Check your password."
  }

  isDeleteAccountLoading.value = false
}
</script>

<template>
  <div class="topbar-container">
    <div class="profile-icon-container">
      <UserProfileIcon @click.stop="toggleProfileOptions" class="profile-icon" :initials="session.name.substring(0, 2)" />
      <Dropdown :isVisible="showProfileOptions" @change="showProfileOptions = $event">
        <span class="login-count"><b>{{ session.loginCount }}</b>Login(s)</span>
        <button class="bubble" :disabled="!selectedRotation" v-if="!session.isAdmin" @click="onExportProgressClick">
          <ExportIcon />
          Export Progress
        </button>
        <button class="bubble" @click="onLogoutClick">
          <LogoutIcon />
          Logout
        </button>
        <button class="bubble red" @click="onDeleteAccountClick">
          <DeleteIcon />
          Delete Account
        </button>
      </Dropdown>
    </div>
    <div class="name"><b>{{ session.name }}</b> ({{ session.username }})</div>
    <div class="rotation-select-container" :key="Object.keys(rotations).length">
      <template v-if="session.isAdmin">
        <div class="rotations" ref="rotationsDiv" @scroll="rotationsScrollListener" @mousedown="hideAllRotationDropdowns">
          <div v-for="rotation in rotations">
            <button @click.stop="toggleRotationDropdown(rotation.id)" :class="`bubble rotation-option ${shouldAppearFocused(rotation.id) ? 'focused' : ''}`" :key="rotation.id">{{ rotation.name }}</button>
            <Dropdown @mousedown.stop :style="`margin-left: -${scrollLeft}px`" class="rotation-option-dropdown" :isVisible="visibleRotationDropdowns[rotation.id]" @change="visibleRotationDropdowns[rotation.id] = $event">
              <button @click="selectRotation(rotation)" class="bubble green">
                <CheckIcon />
                Select
              </button>
              <button class="bubble red" @click="onDeleteRotationClick">
                <DeleteIcon />
                Delete
              </button>
            </Dropdown>
          </div>
        </div>
        <button @click="() => { showCreateRotationModal = true }" class="bubble highlight new-rotation">
          <NewRotationIcon />
          <span>New Rotation</span>
        </button>
      </template>
      <p v-else-if="rotations ? Object.keys(rotations).length === 0 : true">Currently no rotations to select.</p>
      <div v-else class="rotations">
        <button @click="selectRotation(rotation)" v-for="rotation in rotations" :class="`bubble rotation-option ${shouldAppearFocused(rotation.id) ? 'focused' : ''}`" :key="rotation.id">{{ rotation.name }}</button>
      </div>
    </div>
  </div>
  <template v-if="session.isAdmin">
    <InputModal
      v-model="name"
      title="New Rotation"
      placeholder="Enter rotation name..."
      :loading="isCreateRotationLoading"
      :error="nameError"
      :visible="showCreateRotationModal"
      :isPassword="false"
      :onConfirm="confirmNewRotation"
      :onCancel="() => { showCreateRotationModal = false }"
    />
    <ConfirmationModal
      title="Delete Rotation"
      description="Are you sure you want to delete this rotation?"
      :error="deleteRotationError"
      :visible="showDeleteRotationModal"
      :onConfirm="confirmDeleteRotation"
      :onCancel="() => { showDeleteRotationModal = false }"
    />
  </template>
  <InputModal
    v-model="password"
    title="Delete Account"
    placeholder="Enter password..."
    :loading="isDeleteAccountLoading"
    :error="passwordError"
    :visible="showDeleteAccountModal"
    :isPassword="true"
    :onConfirm="deleteAccount"
    :onCancel="() => { showDeleteAccountModal = false }"
  />
</template>

<style scoped lang="scss">
@import "../styles/variables.scss";

* {
  white-space: nowrap;
}

div.rotation-option-dropdown {
  top: 50px;
  margin-left: 2px;
}

button.new-rotation {
  margin-left: 10px;

  span {
    margin-left: 3px;
  }
}

button.rotation-option {
  $side-margin: 2px;

  margin-left: $side-margin;
  margin-right: $side-margin;
}

div.profile-icon {
  cursor: pointer;
}

div.topbar-container {
  background-color: $main-bg-color;
  padding-bottom: 12px;
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;
  height: auto;
}

div.name {
  margin-left: 10px;
  font-size: 1.1em;
  flex-grow: 1;
}

div.rotation-select-container {
  margin-left: 15px;
  display: flex;
  align-items: flex-end;

  div.rotations {
    overflow-x: auto;
    display: flex;
    align-items: flex-end;
    max-width: 40lvw;  
  }

  p {
    opacity: 0.7;
    text-align: right;
  }
}

@media (max-width: $mobile-breakpoint) {
  div.name {
    display: none;
  }

  div.profile-icon-container {
    flex-grow: 1;
  }

  button.new-rotation {
    svg {
      margin-right: 0;
    }

    span {
      display: none;
    }
  }

  div.rotation-select-container {
    div.rotations {
      max-width: 65lvw;
    }
  }
}
</style>