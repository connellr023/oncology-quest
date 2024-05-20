<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { User } from "../../models/user"
import useLogout from "../../hooks/useLogout";

const isAdminViewingUsers = inject<Ref<boolean>>("isAdminViewingUsers")!
const session = inject<Ref<User | null>>("session")!

const isCollapsed = ref(false)

const { logout } = useLogout()

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value
}

const manageUsers = () => {
  isAdminViewingUsers.value = true
}

const manageTasks = () => {
  isAdminViewingUsers.value = false
}
</script>

<template>

  <div v-if="session" id="account-bar-container">
    <div class="content-container" :class="`${isCollapsed ? 'collapsed' : ''}`">
      <h1>{{ session.isAdmin ? "Admin Dashboard" : "Dashboard" }}</h1>
      <div>
        <div>Logged in as</div>
        <div class="userinfo">
          <b><i>{{ session.username }} ({{ session.name }})</i></b>
        </div>
        <div class="admin-buttons" v-if="session.isAdmin">
          <button class="glow gradient-button-0" @click="manageTasks">Manage Tasks</button>
          <button class="glow gradient-button-0" @click="manageUsers">Manage Users</button>
        </div>
        <button class="logout glow gradient-button-2" @click="logout">Log Out</button>
      </div>
    </div>
    <div title="Toggle Panel" class="collapse-indicator-container" @click="toggleCollapse">
      <div :class="`${isCollapsed ? 'active' : ''}`" />
      <div :class="`${!isCollapsed ? 'active' : ''}`" />
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../main.scss";

div#account-bar-container {
  position: relative;
  height: 100lvh;
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

div.admin-buttons {
  margin-top: 30px;

  button {
    margin-top: 20px;
  }
}

div.content-container {
  $anim-speed: 0.3s ease;

  width: 30lvw;
  min-width: 180px;
  max-width: 210px;
  height: 100lvh;
  transition: min-width $anim-speed, max-width $anim-speed, width $anim-speed;
  text-align: center;
  overflow: hidden;

  img {
    margin-top: 55px;
    width: 50%;
    min-width: 110px;
    margin-bottom: 20px;
  }

  & > div {

    div.userinfo {
      margin-top: 5px;
    }

    h1 {
      margin: auto;
      margin-bottom: 25px;
      font-size: clamp(30px, 1.4lvw, 34px);
    }
  }

  button {
    $horizontal-pad: 13px;

    left: $horizontal-pad;
    width: 100%;
    width: calc(100% - ($horizontal-pad * 2));

    &.logout {
      bottom: -20px;
      position: absolute;
    }
  }

  &.collapsed {
    width: 0;
    min-width: 0;
    max-width: 0;
    padding: 0;

    * {
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }

    button {
      visibility: hidden;
    }
  }
}
</style>