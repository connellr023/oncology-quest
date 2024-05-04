<script setup lang="ts">
import Entries from "./Entries.vue"
import TaskEditor from "./TaskEditor.vue"
</script>

<template>
  <h1>Dashboard</h1>
  <Entries :handlerComponent="TaskEditor" />
</template>

<style scoped lang="scss">
.dropdown {
  cursor: pointer;

  &:hover {
    text-decoration: underline;
  }
}
</style>