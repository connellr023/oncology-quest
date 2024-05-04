<script setup lang="ts">
import { Ref, inject } from "vue"
import { UserSession } from "../models/user"

const sessionContext = inject<Ref<UserSession>>("session")
</script>

<template>
  <div v-if="sessionContext" id="account-bar">
    <span>Logged in as {{ sessionContext.user.username }} ({{ sessionContext.user.name }})</span>
    <button>Log out</button>
  </div>
</template>