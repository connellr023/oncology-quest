<script setup lang="ts">
import { Ref, inject } from "vue"
import { UserSession } from "../models/user"
import useLogout from "../hooks/useLogout";

const sessionContext = inject<Ref<UserSession | null>>("session")!

const { logout, message } = useLogout()
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