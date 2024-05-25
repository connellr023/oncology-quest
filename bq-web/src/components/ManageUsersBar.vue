<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { User } from "../models/user"

import useUserSearch from "../hooks/useUserSearch"

import Spinner from "../components/vector/Spinner.vue"
import SearchIcon from "../components/vector/SearchIcon.vue"
import UserProfileIcon from "./UserProfileIcon.vue"

const selectedUser = inject<Ref<User>>("selectedUser")!
const session = inject<Ref<User | null>>("session")!
  
const { search, results, loading, searchError } = useUserSearch()
const query = ref("")
const isCollapsed = ref(true)

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value
}
  
const searchUser = () => {
  if (query.value.length > 0 && !loading) {
    search(query.value)
  }
}

const setSelectedUser = (user: User) => {
  selectedUser.value = user
}
</script>

<template>
  <div v-if="session" class="account-bar-container">
    <div class="content-container" :class="`${isCollapsed ? 'collapsed' : ''}`">
      <h3>Manage Users</h3>
      <div class="search-container">
        <input @keyup.enter="searchUser" v-model="query" type="text" placeholder="Search users..." class="bubble" />
        <Spinner class="spinner" v-if="loading" />
        <SearchIcon class="search" v-else @click="searchUser" />
      </div>
      <div class="results-container">
        <div v-if="searchError" class="status">An error occurred while searching for users.</div>
        <div v-else-if="results.length === 0" class="status">No results found.</div>
        <div v-else>
          <div v-for="user in results" :key="user.username" class="user-option" :class="`${selectedUser.name === user.name ? 'selected' : ''}`" @click="setSelectedUser(user)">
            <UserProfileIcon :initials="user.name.substring(0, 2)" />
            <span><b>{{ user.name }}</b> ({{ user.username }})</span>
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

  div.user-option {
    cursor: pointer;
    padding: 10px;
    border-radius: 10px;
    background-color: transparent;
    margin-bottom: 12px;
    transition: background-color 0.15s ease;
    text-align: left;
    display: flex;
    align-items: center;

    span {
      margin-left: 10px;
    }

    &.selected,
    &:hover {
      background-color: $tertiary-bg-color;
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

  svg {
    position: absolute;
    min-width: 17px;
    width: 8%;
    right: 10px;
    top: 10px;
    opacity: 0.6;
    transition: all 0.12s ease;
    
    &.search {
      cursor: pointer;

      &:hover {
        opacity: 1;
        fill: $theme-color-1;
      }
    }
  }
}

div.content-container {
  width: 30lvw;
  min-width: 180px;
  max-width: 265px;
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