<script setup lang="ts">
defineProps<{ initials: string }>()
</script>

<template>
  <div class="icon">{{ initials }}</div>
</template>

<style scoped lang="scss">
@import "../styles/variables.scss";

div.icon {
  $size: clamp(32px, 3lvw, 37px);

  user-select: none;
  width: $size;
  min-width: $size;
  max-width: $size;
  height: $size;
  min-height: $size;
  max-height: $size;
  background-color: $theme-color-1;
  border-radius: $size;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: calc($size / 2);
  border: 2px solid transparent;
  transition: border-color 0.15s ease;

  &:hover {
    border-color: rgba(255, 255, 255, 0.3);
  }
}
</style>