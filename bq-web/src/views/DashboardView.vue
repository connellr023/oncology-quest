<script setup lang="ts">
import { Ref, inject } from "vue"
import { User } from "../models/user"

import Entries from "../components/Entries.vue"

const tasks = inject<Ref<User>>("session")!.value.tasks
</script>

<template>
  <Entries :tasks="tasks" />
</template>