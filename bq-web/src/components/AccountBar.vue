<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { UserSession } from "../models/user"
import { API_ENDPOINT } from "../utilities"

const sessionContext = inject<Ref<UserSession | null>>("session")!
const message = ref("")

const logout = async () => {
  message.value = "Logging out..."

  const response = await fetch(`${API_ENDPOINT}/api/user/logout`, {
    method: "POST",
    credentials: "include",
  })

  if (!response.ok) {
    message.value = "Failed to log out"
    return
  }

  sessionContext.value = null
  message.value = "Logged out"
}
</script>

<template>
  <div v-if="sessionContext" id="account-bar">
    <span>
      Logged in as {{ sessionContext.user.username }} ({{ sessionContext.user.name }})
      <template v-if="sessionContext.user.isAdmin">
        <span> (Admin)</span>
      </template>
    </span>
    <button @click="logout">Log out</button>
    <span>{{ message }}</span>
  </div>
</template>