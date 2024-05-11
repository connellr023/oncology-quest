<script setup lang="ts">
import { ref } from "vue";
import { User } from "../models/user";

import Entries from "../components/editor/Entries.vue"

import useUserSearch from "../hooks/useUserSearch"
import useValidateUsername from "../hooks/useValidateUsername";

const { search, results, loading, searchError } = useUserSearch()
const { username, usernameError } = useValidateUsername()

const selectedUser = ref<User>({
  username: "",
  name: "",
  email: "",
  isAdmin: false,
  tasks: {}
})

const searchUser = () => {
  if (!usernameError.value && username.value.length > 0) {
    search(username.value)
  }
}

const setSelectedUser = (user: User) => {
  selectedUser.value = user
}
</script>

<template>
  <div v-if="selectedUser">
    <h4>Selected User</h4>
    <div>
      <div>Username: {{ selectedUser.username }}</div>
      <div>Name: {{ selectedUser.name }}</div>
      <div>Email: {{ selectedUser.email }}</div>
    </div>
  </div>
  <div>
    <input v-model="username" placeholder="Search Username" />
    <button @click="searchUser">Search</button>
    <div>
      <span v-if="usernameError">{{ usernameError }}</span>
      <ul v-if="results.length > 0">
        <li v-for="(user, index) in results" :key="index">
          <div class="user-option" @click="setSelectedUser(user)">
            {{ user.username }} ({{ user.name }})
          </div>
          <button>Allow Password Reset</button>
          <button>Remove Account</button>
        </li>
      </ul>
      <div v-else-if="loading">
        <h2>Loading...</h2>
      </div>
      <div v-else-if="searchError">
        <h2>Failed to fetch users</h2>
      </div>
      <div v-else>
        <h2>No results</h2>
      </div>
    </div>
  </div>
  <h3>Task Manager</h3>
  <Entries :key="selectedUser.username" :tasks="selectedUser.tasks" />
</template>

<style scoped lang="scss">
input {
  margin-top: 25px;
}

div.user-option {
  cursor: pointer;
  margin: 5px;
}
</style>