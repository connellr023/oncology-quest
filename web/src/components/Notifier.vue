<script setup lang="ts">
import { notification } from "../hooks/useNotifications"
</script>

<template>
  <div :class="`notification ${notification.success ? 'success' : ''} ${!notification.isHidden ? 'visible' : ''}`">
    <div class="notification-content">
      <p>{{ notification.message }}</p>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../styles/variables.scss";

div.notification {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  background-color: $theme-color-red;
  color: $main-txt-color;
  padding: 2px;
  text-align: center;
  transform: translateY(100%);
  transition: transform 0.35s ease;
  font-size: clamp(15px, 1.3lvw, 17px);
  z-index: 1000;

  &.success {
    background-color: $theme-color-green;
  }

  &.visible {
    transform: translateY(0);
  }

  div.notification-content {
    max-width: 800px;
    margin: 0 auto;
  }
}
</style>