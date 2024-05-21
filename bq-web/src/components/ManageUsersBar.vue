<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { User } from "../models/user"

const session = inject<Ref<User | null>>("session")!
const isCollapsed = ref(true)

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value
}
</script>

<template>
  <div v-if="session" class="account-bar-container">
    <div class="content-container" :class="`${isCollapsed ? 'collapsed' : ''}`">
      <h3>Manage Users</h3>
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

div.content-container {
  $anim-speed: 0.3s ease;

  width: 30lvw;
  min-width: 180px;
  max-width: 265px;
  transition: min-width $anim-speed, max-width $anim-speed, width $anim-speed;
  text-align: center;
  overflow: hidden;
  padding: 15px;

  h3 {
    margin: 0;
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