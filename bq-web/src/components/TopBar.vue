<script setup lang="ts">
import { Ref, inject, onMounted, onUnmounted, ref } from "vue";
import { User } from "../models/user"

import useLogout from "../hooks/useLogout"

import UserProfileIcon from "./UserProfileIcon.vue"
import LogoutIcon from "./vector/LogoutIcon.vue"
import { Domain } from "../models/domain";

const session = inject<Ref<User>>("session")!.value
const domains = inject<Ref<Domain[]>>("domains")!.value

const { logout } = useLogout()

const showProfileOptions = ref(false)

const toggleProfileOptions = () => {
  showProfileOptions.value = !showProfileOptions.value
}

const hideProfileOptions = () => {
  if (showProfileOptions.value) {
    showProfileOptions.value = false
  }
}

onMounted(() => {
  window.addEventListener("click", hideProfileOptions)
});

onUnmounted(() => {
  window.removeEventListener("click", hideProfileOptions)
});
</script>

<template>
  <div class="topbar-container">
    <div class="profile-container">
      <UserProfileIcon @click.stop="toggleProfileOptions" class="profile-icon" :initials="session.name.substring(0, 2)" />
      <div v-show="showProfileOptions" class="profile-options" @click.stop>
        <button class="logout bubble highlight" @click="logout">
          <LogoutIcon />
          Logout
        </button>
      </div>
    </div>
    <div class="name"><b>{{ session.name }}</b> ({{ session.username }})</div>
    <div class="domain-select-container">
      <p v-if="domains.length === 0">Currently no domains to select.</p>
      <button v-else v-for="domain in domains" class="bubble highlight" :key="domain.id">{{ domain.name }}</button>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../main.scss";

div.profile-options {
  position: absolute;
  top: 55px;
  background-color: $main-bg-color;
  display: flex;
  flex-direction: column;
  align-items: center;
  z-index: 1;
  background-color: $tertiary-bg-color;
  border-radius: 8px;
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
}

div.domain-select-container {
  margin-left: 15px;
  display: flex;
  flex-grow: 1;
  justify-content: flex-end;
  overflow-x: auto;

  p {
    opacity: 0.7;
    text-align: right;
  }
}
</style>