<script setup lang="ts">
import { Ref, inject } from "vue";
import { UserSession } from "../models/user";

import Entries from "../components/Entries.vue"

const tasks = inject<Ref<UserSession>>("session")!.value.user.tasks
</script>

<template>
  <Entries :tasks="tasks" />
</template>