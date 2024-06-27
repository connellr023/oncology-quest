<script setup lang="ts">
import { Ref, inject, onMounted, onUnmounted, ref } from "vue"
import { User } from "../models/user"
import { UserTaskStructure } from "../models/tasks"
import { Rotation } from "../models/rotation"

import useUserSearch from "../hooks/useUserSearch"
import useDeleteUser from "../hooks/useDeleteUser"
import useAllowResetPassword from "../hooks/useAllowResetPassword"
import useExportProgress from "../hooks/useExportProgress"

import Spinner from "../components/vector/Spinner.vue"
import UserProfileIcon from "./UserProfileIcon.vue"
import Dropdown from "./Dropdown.vue"
import SearchIcon from "../components/vector/SearchIcon.vue"
import DeleteIcon from "../components/vector/DeleteIcon.vue"
import UnlockIcon from "./vector/UnlockIcon.vue"
import ConfirmationModal from "./ConfirmationModal.vue"
import MessageModal from "./MessageModal.vue"
import ExportIcon from "./vector/ExportIcon.vue"

const selectedUser = inject<Ref<User | null>>("selectedUser")!
const selectedUserTasks = inject<Ref<UserTaskStructure | null>>("selectedUserTasks")!
const selectedRotation = inject<Ref<Rotation | null>>("selectedRotation")!
const session = inject<Ref<User | null>>("session")!
  
const { search, results, loading, searchError } = useUserSearch()
const { deleteUser } = useDeleteUser()
const { allowReset } = useAllowResetPassword()
const { exportProgress } = useExportProgress()

const query = ref("")
const deleteUserError = ref("")
const allowResetError = ref("")
const allowResetExpiryDate = ref("")
const resetToken = ref("")

const isCollapsed = ref(true)
const showUserOptions = ref(false)
const showConfirmationModal = ref(false)
const showAllowResetModal = ref(false)

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value
}
  
const searchUser = () => {
  if (query.value.length > 0 && !loading.value) {
    showUserOptions.value = false
    search(query.value)
  }
}

const setSelectedUser = (selection: User) => {
  selectedUser.value = selection
  showUserOptions.value = false
}

const toggleUserOptions = () => {
  showUserOptions.value = !showUserOptions.value
}

const onDeleteUserClicked = () => {
  deleteUserError.value = ""
  showUserOptions.value = false
  showConfirmationModal.value = true
}

const onExportProgressClicked = () => {
  exportProgress(selectedUser.value!.name, selectedUserTasks.value!)
  showUserOptions.value = false
}

const confirmDeleteUser = () => {
  const confirm = async () => {
    if (!selectedUser.value) return

    if (await deleteUser(selectedUser.value.id)) {
      delete results.value[selectedUser.value.id]
      selectedUser.value = null
      showConfirmationModal.value = false
    }
    else {
      deleteUserError.value = "Failed to delete user."
    }
  }

  confirm()
}

const onAllowResetClicked = async () => {
  const result = await allowReset(selectedUser.value!.id)

  allowResetError.value = ""
  showUserOptions.value = false
  showAllowResetModal.value = true

  if (!result) {
    allowResetError.value = "Failed to enable password reset."
  }
  else {
    allowResetExpiryDate.value = result.passwordResetTimestamp.toLocaleTimeString()
    resetToken.value = result.resetToken
  }
}

const clickOffListener = () => {
  isCollapsed.value = true
}

onMounted(() => {
  document.addEventListener("click", clickOffListener)
})

onUnmounted(() => {
  document.removeEventListener("click", clickOffListener)
})
</script>

<template>
  <div @click.stop v-if="session" class="account-bar-container" :class="`${isCollapsed ? 'collapsed' : ''}`">
    <div class="content-container">
      <h3>Manage Users</h3>
      <div class="search-container">
        <input @keyup.enter="searchUser" v-model="query" type="text" placeholder="Search users..." class="bubble" />
        <button class="icon-button" @click="searchUser">
          <Spinner class="spinner" v-if="loading" />
          <SearchIcon v-else />
        </button>
      </div>
      <div class="results-container" @click="() => { showUserOptions = false }">
        <div v-if="searchError" class="status">An error occurred while searching for users.</div>
        <div v-else-if="Object.keys(results).length === 0" class="status">No results found.</div>
        <div v-else>
          <div v-for="result in results" :key="result.id" :class="`user-option ${selectedUser?.id === result.id ? 'selected' : ''}`" @click="setSelectedUser(result)">
            <UserProfileIcon :initials="result.name.substring(0, 2)" @click.stop="() => { if (selectedUser?.id === result.id) { toggleUserOptions() } else { setSelectedUser(result) } }" />
            <Dropdown :isVisible="showUserOptions && selectedUser?.id === result.id" @change="showUserOptions = $event">
              <span class="login-count"><b>{{ result.loginCount }}</b>Login(s)</span>
              <button class="bubble" @click="onExportProgressClicked" :disabled="selectedUserTasks === null || selectedRotation === null">
                <ExportIcon />
                Export Progress
              </button>
              <button class="bubble" @click="onAllowResetClicked">
                <UnlockIcon />
                Enable Password Reset
              </button>
              <button class="bubble red" @click="onDeleteUserClicked">
                <DeleteIcon />
                Delete User
              </button>
            </Dropdown>
            <div class="user-info">
              <span class="name"><b>{{ result.name }}</b> ({{ result.username }})</span>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div title="Toggle Panel" class="collapse-indicator-container" @click="toggleCollapse">
      <div :class="`${isCollapsed ? 'active' : ''}`" />
      <div :class="`${!isCollapsed ? 'active' : ''}`" />
    </div>
  </div>
  <ConfirmationModal
    title="Delete User"
    description="Are you sure you want to delete this user?"
    :error="deleteUserError"
    :visible="showConfirmationModal"
    :onConfirm="confirmDeleteUser"
    :onCancel="() => { showConfirmationModal = false }"
  />
  <MessageModal
    title="Allow Password Reset"
    :error="allowResetError"
    :visible="showAllowResetModal"
    @change="showAllowResetModal = $event"
  >
    <template #message><b>{{ selectedUser?.name }}</b> has until <b>{{ allowResetExpiryDate }}</b> to reset their password with the following token: <b>{{ resetToken }}</b>. Make sure to provide this information to the user.</template>
  </MessageModal>
</template>

<style scoped lang="scss">
@import "../styles/variables.scss";

div.account-bar-container {
  position: relative;
  background-color: $secondary-bg-color;
}

div.collapse-indicator-container {
  cursor: pointer;
  position: absolute;
  top: 50%;
  left: calc(100% + 4px);
  transform: translateY(-50%);
  opacity: 0.5;
  transition: opacity 0.3s ease;

  &:hover {
    opacity: 1;
  }

  div {
    $size: 9px;

    width: $size;
    height: $size;
    border-radius: 50%;
    background-color: #ffffff;
    opacity: 0.7;
    margin: 0 5px;
    transition: all 0.17s ease-in-out;
  }

  div.active {
    $margin: 7px;

    margin-top: $margin;
    margin-bottom: $margin;
    opacity: 1;
    border-radius: 8px;
    height: 40px;
  }
}

div.results-container {
  margin-top: 25px;
  height: fit-content;
  overflow-x: hidden;
  overflow-y: auto;
  height: calc(100% - 155px);

  div.status {
    text-align: center;
    font-size: 1.1rem;
  }

  span.name {
    margin-right: 25px;
  }

  div.user-info {
    margin-left: 12px;
  }

  div.user-option {
    cursor: pointer;
    position: relative;
    padding: 10px;
    border-radius: 10px;
    background-color: transparent;
    margin-bottom: 12px;
    transition: background-color 0.15s ease;
    text-align: left;
    display: flex;
    align-items: center;

    &.selected,
    &:hover {
      background-color: $main-bg-color;
    }

    &.selected {
      span.name {
        cursor: auto;
      }
    }
  }
}

div.search-container {
  display: flex;
  width: 100%;
  position: relative;

  input {
    width: calc(100% - 15px);
    font-size: clamp(16px, 1.2lvw, 18px);
    padding-right: 38px;
  }

  button {
    position: absolute;
    min-width: 17px;
    width: 8%;
    right: 13px;
    top: 8px;
    transition: all 0.12s ease;
  }
}

div.account-bar-container {
  position: absolute;
  z-index: 2;
  left: 0;
  height: 100%;
  width: 40lvw;
  min-width: 300px;
  max-width: 350px;
  transition: all 0.2s ease;
  box-shadow: 0 0 13px rgba(0, 0, 0, 0.7);

  &.collapsed {
    box-shadow: none;
    transform: translateX(-100%);
  }

  div.content-container {
    text-align: center;
    padding: 15px;
    position: relative;
    height: 100%;

    h3 {
      margin-top: 7px;
      margin-bottom: 25px;
    }
  }
}
</style>