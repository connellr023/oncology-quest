<script setup lang="ts">
import { Ref, inject } from "vue";
import { UserSession } from "../models/user";

import Entries from "../components/editor/Entries.vue"

const tasks = inject<Ref<UserSession>>("session")!.value.user.tasks
</script>

<template>
  <h1>Dashboard</h1>
  <Entries :tasks="tasks" />
</template>

<style scoped lang="scss">
.dropdown {
  cursor: pointer;

  &:hover {
    text-decoration: underline;
  }
}
</style>