<script setup lang="ts">
import { Ref, inject } from "vue"
import { UserSession } from "../models/user"
import useLogout from "../hooks/useLogout";

const sessionContext = inject<Ref<UserSession | null>>("session")!

const { logout, message } = useLogout()
</script>

<template>
  <div v-if="sessionContext" id="account-bar">
    <span>
      Logged in as <b><i>{{ sessionContext.user.username }} ({{ sessionContext.user.name }})</i></b>
      <template v-if="sessionContext.user.isAdmin">
        <span> (Admin)</span>
      </template>
    </span>
    <button @click="logout">Log out</button>
    <span>{{ message }}</span>
  </div>
</template>

<style scoped lang="scss">
@import "../main.scss";

div#account-bar {
  // background-color: invert($main-bg-color);
  // color: invert($main-txt-color);
  position: fixed;
  top: 0;
  right: 0;
  padding: 14px;
  width: 100lvw;
  text-align: right;
  font-size: clamp(16px, 1.5lvw, 20px);

  button {
    // color: invert($main-txt-color);
    // border-color: invert($main-txt-color);
    font-weight: 500;
    width: 13lvw;
    min-width: 120px;
    max-width: 200px;
    margin-left: 15px;
    padding: 4px;
  }
}
</style>