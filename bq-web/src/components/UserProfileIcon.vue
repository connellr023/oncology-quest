<script setup lang="ts">
defineProps<{ initials: string }>()
</script>

<template>
  <div class="icon">{{ initials }}</div>
</template>

<style scoped lang="scss">
@import "../main.scss";

div.icon {
  $size: 33px;

  user-select: none;
  width: $size;
  height: $size;
  background-color: $theme-color-1;
  border-radius: $size;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: $size / 2;
  border: 2px solid transparent;
  transition: border-color 0.15s ease;

  &:hover {
    border-color: rgba(255, 255, 255, 0.3);
  }
}
</style>