<script setup lang="ts">
import { Ref, inject } from "vue"
import { UserSession } from "../models/user"
import Entries from "./Entries.vue"
import TaskEditor from "./TaskEditor.vue"

const sessionContext = inject<Ref<UserSession>>("session")
</script>

<template>
  <h1>Dashboard</h1>
  <Entries
    v-if="sessionContext"
    :entries="sessionContext.entries"
    :handlerComponent="TaskEditor"
  />
</template>

<style scoped lang="scss">
.dropdown {
  cursor: pointer;

  &:hover {
    text-decoration: underline;
  }
}
</style>