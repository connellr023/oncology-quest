<script setup lang="ts">
import { Ref, inject } from "vue"
import { User } from "../models/user"

import Entries from "../components/dash/Entries.vue"

const selectedUser = inject<Ref<User>>("selectedUser")!
</script>

<template>
  <Entries :key="selectedUser.username" :tasks="selectedUser.tasks" />
  <div class="note">
    <b><i></i>Note:</b> Popping entries does not change whether or not a user completed a task that was there before the entry was popped.
  </div>
</template>

<style scoped lang="scss">
div.note {
  opacity: 0.4;
  margin-top: 20px;
  text-align: center;
}

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