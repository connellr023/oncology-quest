<script setup lang="ts">
import { Ref, inject } from "vue"
import { UserTask } from "../models/tasks"

import Entries from "../components/dash/Entries.vue"

const tasks = inject<Ref<Record<number, UserTask>>>("tasks")!
</script>

<template>
  <Entries :tasks="tasks" />
</template>