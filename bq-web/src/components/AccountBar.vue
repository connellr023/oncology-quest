<script setup lang="ts">
import { Ref, inject } from "vue"
import { UserSession } from "../models/user"
import useLogout from "../hooks/useLogout";

const sessionContext = inject<Ref<UserSession | null>>("session")!

const { logout } = useLogout()
</script>

<template>
  <div v-if="sessionContext" id="account-bar">
    <h1>{{ sessionContext.user.isAdmin ? "Admin Dashboard" : "Dashboard" }}</h1>
    <div>
      <span>
        Logged in as <b><i>{{ sessionContext.user.username }} ({{ sessionContext.user.name }})</i></b>
      </span>
      <button @click="logout">Log Out</button>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../main.scss";

div#account-bar {
  //background-color: $main-bg-color;
  background: linear-gradient(to bottom, $main-bg-color 15%, rgba(0, 0, 0, 0));
  position: fixed;
  top: 0;
  right: 0;
  width: 100lvw;
  padding-bottom: 160px;
  text-align: right;
  font-size: clamp(16px, 1.5lvw, 20px);
  display: flex;
  justify-self: space-between;
  align-items: center;
  pointer-events: none;

  $spacing: 17px;

  h1 {
    display: inline-block;
    text-align: left;
    padding-left: $spacing;
  }

  & > div {
    position: absolute;
    right: $spacing;
    pointer-events: initial;
  }

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