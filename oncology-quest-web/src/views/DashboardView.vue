<script setup lang="ts">
import { Ref, inject, watch } from "vue"
import { UserTask } from "../models/tasks"
import { User, UserWithTasks } from "../models/user";
import { Rotation } from "../models/rotation";

import ManageUsersBar from "../components/ManageUsersBar.vue"
import TopBar from "../components/TopBar.vue"
import Entries from "../components/dash/Entries.vue"

const session = inject<Ref<User>>("session")!
const tasks = inject<Ref<Record<number, UserTask>>>("tasks")!
const selectedUser = inject<Ref<UserWithTasks | null>>("selectedUser")!
const selectedRotation = inject<Ref<Rotation | null>>("selectedRotation")!
const isEditing = inject<Ref<boolean>>("isEditing")!

watch(() => [selectedUser.value?.user.id, selectedRotation.value?.id], () => {
  isEditing.value = false
})
</script>

<template>
  <div class="dash-container">
    <ManageUsersBar v-if="session.isAdmin" />
    <div class="dash">
      <TopBar />
      <Entries v-if="session.isAdmin" :selectedRotation="selectedRotation" :key="`${selectedUser?.user.id}.${selectedRotation?.id}`" :tasks="selectedUser?.tasks || {}" />
      <Entries v-else :tasks="tasks" :selectedRotation="selectedRotation" />
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../styles/variables.scss";

div.dash-container {
  opacity: 0;
  animation: fade-in 1s forwards;
  animation-delay: 0.5s;
  display: flex;
  width: 100lvw;
  height: 100lvh;

  div.dash {
    flex-grow: 1;
    background-color: $main-bg-color;
    padding: 12px 25px 20px 35px;
  }
}
</style>