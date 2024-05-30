<script setup lang="ts">
import { Ref, inject } from "vue"
import { UserTask } from "../models/tasks"
import { User, UserWithTasks } from "../models/user";

import Entries from "../components/dash/Entries.vue"

const session = inject<Ref<User>>("session")!
const tasks = inject<Ref<Record<number, UserTask>>>("tasks")!
const selectedUser = inject<Ref<UserWithTasks | null>>("selectedUser")!
</script>

<template>
  <Entries v-if="session.isAdmin" :key="selectedUser?.user.id" :tasks="selectedUser?.tasks || {}" />
  <Entries v-else :tasks="tasks" />
</template>