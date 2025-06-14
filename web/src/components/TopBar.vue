<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { User } from "../models/user"
import { Rotation } from "../models/rotation"
import { UserTaskStructure } from "../models/tasks"

import useLogout from "../hooks/useLogout"
import useValidatePassword from "../hooks/validation/useValidatePassword"
import useDeleteUser from "../hooks/useDeleteUser"
import useExportProgress from "../hooks/useExportProgress"
import useNotifications from "../hooks/useNotifications"

import UserProfileIcon from "./UserProfileIcon.vue"
import LogoutIcon from "./vector/LogoutIcon.vue"
import InputModal from "./InputModal.vue"
import DeleteIcon from "./vector/DeleteIcon.vue"
import Dropdown from "./Dropdown.vue"
import ExportIcon from "./vector/ExportIcon.vue"

const resetAll = inject<() => void>("resetAll")!
const session = inject<Ref<User>>("session")!
const selectedRotation = inject<Ref<Rotation | null>>("selectedRotation")!
const tasks = inject<Ref<Record<number, UserTaskStructure>>>("tasks")!

const { logout } = useLogout()
const { password, passwordError } = useValidatePassword()
const { deleteSelf } = useDeleteUser()
const { exportProgress } = useExportProgress()
const { pushNotification } = useNotifications()

const showProfileOptions = ref(false)
const showDeleteAccountModal = ref(false)
const isDeleteAccountLoading = ref(false)

const toggleProfileOptions = () => {
  showProfileOptions.value = !showProfileOptions.value
}

const onLogoutClick = async () => {
  resetAll()
  await logout()

  pushNotification("Logged out successfully.", true)
}

const onDeleteAccountClick = () => {
  showDeleteAccountModal.value = true
  showProfileOptions.value = false
}

const onExportProgressClick = () => {
  exportProgress(session.value.name, tasks.value[selectedRotation.value!.id])
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

    pushNotification("Account deleted successfully.", true)
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
        <button class="left crisp bubble" :disabled="!selectedRotation" v-if="!session.isAdmin" @click="onExportProgressClick">
          <span>
            <ExportIcon />
            <span>Export Progress</span>
          </span>
        </button>
        <button class="left crisp bubble" @click="onLogoutClick">
          <span>
            <LogoutIcon />
            <span>Logout</span>
          </span>
        </button>
        <button class="left crisp bubble red" @click="onDeleteAccountClick">
          <span>
            <DeleteIcon />
            <span>Delete Account</span>
          </span>
        </button>
      </Dropdown>
    </div>
    <div class="name"><b>{{ session.name }}</b> ({{ session.username }})</div>
  </div>
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
  $vertical-padding: 10px;

  padding-top: $vertical-padding;
  padding-bottom: $vertical-padding;
  background-color: $main-bg-color;
  top: 0;
  position: fixed;
  display: flex;
  align-items: center;
  width: 100%;
  height: auto;
  z-index: 100;
}

div.name {
  margin-left: 10px;
  font-size: clamp(15px, 1.2lvw, 18px);
  flex-grow: 1;
}
</style>
