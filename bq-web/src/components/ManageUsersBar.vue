<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { UserWithTasks, User } from "../models/user"

import useUserSearch from "../hooks/useUserSearch"
import useDeleteUser from "../hooks/useDeleteUser"

import Spinner from "../components/vector/Spinner.vue"
import UserProfileIcon from "./UserProfileIcon.vue"
import Dropdown from "./Dropdown.vue"
import SearchIcon from "../components/vector/SearchIcon.vue"
import DeleteIcon from "../components/vector/DeleteIcon.vue"
import UnlockIcon from "./vector/UnlockIcon.vue"

const selectedUser = inject<Ref<UserWithTasks | null>>("selectedUser")!
const session = inject<Ref<User | null>>("session")!
  
const { search, results, loading, searchError } = useUserSearch()
const { deleteUser } = useDeleteUser()

const query = ref("")
const isCollapsed = ref(true)
const userOptionsVisible = ref(false)

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value
}
  
const searchUser = () => {
  if (query.value.length > 0 && !loading.value) {
    userOptionsVisible.value = false
    search(query.value)
  }
}

const setSelectedUser = (selection: UserWithTasks) => {
  selectedUser.value = selection
  userOptionsVisible.value = false
}

const toggleUserOptions = () => {
  userOptionsVisible.value = !userOptionsVisible.value
}

const onDeleteUser = async (userId: number) => {
  if (await deleteUser(userId)) {
    delete results.value[userId]
    selectedUser.value = null
  }
}
</script>

<template>
  <div v-if="session" class="account-bar-container">
    <div class="content-container" :class="`${isCollapsed ? 'collapsed' : ''}`">
      <h3>Manage Users</h3>
      <div class="search-container">
        <input @keyup.enter="searchUser" v-model="query" type="text" placeholder="Search users..." class="bubble" />
        <button class="icon-button" @click="searchUser">
          <Spinner class="spinner" v-if="loading" />
          <SearchIcon v-else />
        </button>
      </div>
      <div class="results-container">
        <div v-if="searchError" class="status">An error occurred while searching for users.</div>
        <div v-else-if="Object.keys(results).length === 0" class="status">No results found.</div>
        <div v-else>
          <div v-for="result in results" :key="result.user.id" :class="`user-option ${selectedUser?.user.id === result.user.id ? 'selected' : ''}`" @click.stop="() => { if (selectedUser?.user.id === result.user.id) { toggleUserOptions() } else { setSelectedUser(result) } }">
            <UserProfileIcon :initials="result.user.name.substring(0, 2)" />
            <Dropdown :isVisible="userOptionsVisible && selectedUser?.user.id === result.user.id" @change="userOptionsVisible = $event">
              <span class="login-count"><b>{{ result.user.loginCount }}</b>Login(s)</span>
              <button class="bubble">
                <UnlockIcon />
                Enable Password Reset
              </button>
              <button class="bubble red" @click="onDeleteUser(result.user.id)">
                <DeleteIcon />
                Delete User
              </button>
            </Dropdown>
            <div class="user-info">
              <span class="name"><b>{{ result.user.name }}</b> ({{ result.user.username }})</span>
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
</template>

<style scoped lang="scss">
@import "../main.scss";

span.login-count {
  cursor: auto;
  padding: 10px;
  padding-top: 15px;
  margin-left: 20px;

  b {
    margin-right: 10px;
  }
}

div.account-bar-container {
  position: relative;
  background-color: $secondary-bg-color;
}

div.collapse-indicator-container {
  cursor: pointer;
  position: absolute;
  top: 50%;
  left: calc(100% + 8px);
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

div.content-container {
  width: 30lvw;
  min-width: 180px;
  max-width: 325px;
  height: 100%;
  transition: all 0.25s ease;
  text-align: center;
  overflow: hidden;
  padding: 15px;
  position: relative;

  h3 {
    margin-top: 7px;
    margin-bottom: 25px;
  }

  &.collapsed {
    width: 0;
    min-width: 0;
    padding: 0;
    overflow: hidden;

    * {
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }
  }
}
</style>