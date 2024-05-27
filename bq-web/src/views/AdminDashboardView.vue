<script setup lang="ts">
import { Ref, inject } from "vue"
import { UserWithTasks } from "../models/user"

import Entries from "../components/dash/Entries.vue"

const selectedUser = inject<Ref<UserWithTasks | null>>("selectedUser")!
</script>

<template>
  <Entries :key="selectedUser?.user.id" :tasks="selectedUser?.tasks || {}" />
</template>

<style scoped lang="scss">
div.user-option {
  cursor: pointer;
  margin: 5px;
}

div.user-search-container {
  display: flex;
  flex-direction: column;
  margin: auto;
  height: 100lvh;
}
</style>