<script setup lang="ts">
import { provide, ref } from "vue"

import useFetchSession from "./hooks/useFetchSession"

import NoSessionView from "./views/NoSessionView.vue"
import DashboardView from "./views/DashboardView.vue"
import AdminDashboardView from "./views/AdminDashboardView.vue"
import MainLogo from "./components/vector/MainLogo.vue"
import AccountBar from "./components/dash/AccountBar.vue"
import TopBar from "./components/TopBar.vue"

const { session, entries, loading, connectionError } = useFetchSession()
provide("session", session)
provide("entries", entries)

const isEditing = ref(false)
provide("isEditing", isEditing)

const isAdminViewingUsers = ref(false)
provide("isAdminViewingUsers", isAdminViewingUsers)
</script>

<template>
  <main>
    <div class="flex-wrapper">
      <MainLogo :class="`${session ? 'fade-out' : ''} ${(!loading && !connectionError) ? 'fade-up' : ''}`" />
      <div v-if="connectionError" id="connect-error"><b><i>bq</i></b> is currently under maintenance.
      </div>
      <div v-if="!connectionError && !loading">
        <NoSessionView v-if="!session" />
        <div id="dash-container" v-else>
          <AccountBar />
          <div id="dash-content-container">
            <TopBar />
            <AdminDashboardView v-if="session.isAdmin" />
            <DashboardView v-else />
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped lang="scss">
@import "main.scss";

div#dash-container {
  opacity: 0;
  animation: fade-in 1s forwards;
  animation-delay: 0.5s;
  display: flex;
  width: 100lvw;

  div#dash-content-container {
    flex-grow: 1;
    background-color: $main-bg-color;
    padding: 20px 25px 20px 40px;
    margin-top: 30px;
  }
}

div#connect-error {
  margin-top: 20px;
  font-size: clamp(25px, 2.5lvw, 33px);
  color: #ffffff;
  text-align: center;
  animation: fade-in 1s;
}

$logo-vert-offset: 170px;

svg {
  width: 15lvw;
  min-width: 50px;
  max-width: 80px;
  margin: 0 auto;
  display: block;
  animation: pulse 4s infinite ease-out;
  
  &.fade-up {
    transform: translateY($logo-vert-offset);
    animation: move-up 0.8s;
  }

  &.fade-out {
    animation: fade-out 0.5s;
    position: fixed;
    transform: translate(-50%, -50%);
    left: 50%;
    top: 50%;
  }

  &.fade-up,
  &.fade-out {
    animation-fill-mode: forwards;
    animation-delay: 0.13s;
  }
}

$max-opacity: 0.65;
$min-opacity: 0.45;

@keyframes pulse {
  0% {
    opacity: $min-opacity;
  }
  50% {
    opacity: $max-opacity;
  }
  100% {
    opacity: $min-opacity;
  }
}

@keyframes move-up {
  0% {
    transform: translateY($logo-vert-offset);
  }
  100% {
    transform: translateY(0) scale(0.8);
    opacity: 0.9;
  }
}
</style>