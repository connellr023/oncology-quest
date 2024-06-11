<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { User } from "../models/user"
import { Rotation } from "../models/rotation"
import { UserTask } from "../models/tasks"

import useLogout from "../hooks/useLogout"
import useValidateName from "../hooks/validation/useValidateName"
import useValidatePassword from "../hooks/validation/useValidatePassword"
import useRotations from "../hooks/useRotations"
import useDeleteUser from "../hooks/useDeleteUser"
import useExportProgress from "../hooks/useExportProgress"

import UserProfileIcon from "./UserProfileIcon.vue"
import LogoutIcon from "./vector/LogoutIcon.vue"
import InputModal from "./InputModal.vue"
import NewRotationIcon from "./vector/NewRotationIcon.vue"
import DeleteIcon from "./vector/DeleteIcon.vue"
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
const { createRotation } = useRotations()
const { deleteSelf } = useDeleteUser()
const { exportProgress } = useExportProgress()

const showProfileOptions = ref(false)
const showCreateRotationModal = ref(false)
const showDeleteAccountModal = ref(false)

const isCreateRotationLoading = ref(false)
const isDeleteAccountLoading = ref(false)

const toggleProfileOptions = () => {
  showProfileOptions.value = !showProfileOptions.value
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

const onLogoutClick = async () => {
  resetAll()
  await logout()
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
    <div>
      <button v-if="session.isAdmin" @click="() => { showCreateRotationModal = true }" class="green bubble highlight new-rotation">
        <NewRotationIcon />
        New Rotation
      </button>
      <p v-else-if="rotations ? Object.keys(rotations).length === 0 : true">Currently no rotations to select.</p>
    </div>
  </div>
  <InputModal
    v-if="session.isAdmin"
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

button.new-rotation {
  margin-top: -2px;
}
</style>