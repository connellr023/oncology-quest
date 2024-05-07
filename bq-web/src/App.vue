<script setup lang="ts">
import { provide, ref } from "vue";
import { UserSession } from "./models/user"

import useFetchSession from "./hooks/useFetchSession"

import NoSessionView from "./views/NoSessionView.vue"
import DashboardView from "./views/DashboardView.vue"
import AdminDashboardView from "./views/AdminDashboardView.vue"

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
      <NoSessionView v-if="!session" />
      <div v-else>
        <AccountBar />
        <AdminDashboardView v-if="session.user.isAdmin" />
        <DashboardView v-else />
      </div>
    </template>
    <CreditLabel />
  </main>
</template>

<style scoped>
</style>
