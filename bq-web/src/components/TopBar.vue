<script setup lang="ts">
import { Ref, inject } from "vue";
import { User } from "../models/user"

import useLogout from "../hooks/useLogout"

import UserProfileIcon from "./UserProfileIcon.vue"
import LogoutIcon from "./vector/LogoutIcon.vue"

const session = inject<Ref<User>>("session")!.value

const { logout } = useLogout()
</script>

<template>
  <div class="topbar-container">
    <UserProfileIcon :initials="session.name.substring(0, 2)" />
    <div class="name"><b>{{ session.name }}</b> ({{ session.username }})</div>
    <button class="logout bubble highlight" @click="logout">
      <LogoutIcon />
      Logout
    </button>
  </div>
</template>

<style scoped lang="scss">
@import "../main.scss";

div.topbar-container {
  background-color: $main-bg-color;
  margin-top: 35px;
  padding-bottom: 10px;
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

button.logout {
  $side-padding: 13px;

  padding: 7px;
  padding-left: $side-padding;
  padding-right: $side-padding;
  position: absolute;
  right: 0;

  svg {
    width: 19px;
  }
}
</style>