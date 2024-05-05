<script setup lang="ts">
import { provide, ref } from "vue";
import { UserSession } from "./models/user"

import useFetchSession from "./hooks/useFetchSession"
import NoSession from "./components/NoSession.vue"
import Dashboard from "./components/Dashboard.vue"
import AdminDashboard from "./components/AdminDashboard.vue"
import AccountBar from "./components/AccountBar.vue"
import CreditLabel from "./components/CreditLabel.vue"

let session = ref<UserSession | null>(null);
provide("session", session);

const { loading, connectionError } = useFetchSession(session)
</script>

<template>
  <main>
    <div v-if="loading" id="loading-wrapper">
      <h1>Loading...</h1>
    </div>
    <div v-else-if="connectionError" id="connection-error-wrapper">
      <h1>Connection Error</h1>
    </div>
    <template v-else>
      <NoSession v-if="!session" />
      <div v-else>
        <AccountBar />
        <AdminDashboard v-if="session.user.isAdmin" />
        <Dashboard v-else />
      </div>
    </template>
    <CreditLabel />
  </main>
</template>

<style scoped>
</style>
