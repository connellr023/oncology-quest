<script setup lang="ts">
import { provide, ref } from "vue";
import { UserSession } from "./models/user"

import useFetchSession from "./hooks/useFetchSession"

import NoSessionView from "./views/NoSessionView.vue"
import DashboardView from "./views/DashboardView.vue"
import AdminDashboardView from "./views/AdminDashboardView.vue"
import AccountBar from "./components/AccountBar.vue"

let session = ref<UserSession | null>(null)
provide("session", session)

const isEditing = ref(false)
provide("isEditing", isEditing)

const { loading, connectionError } = useFetchSession(session)
</script>

<template>
  <main>
    <div class="flex-wrapper">
      <img draggable="false" :class="((!loading && !connectionError) ? 'fade-up' : '')" src="/logo.svg" />
      <div v-if="connectionError" id="connect-error"><b><i>bq</i></b> is currently under maintenance.
      </div>
      <div v-if="!connectionError">
        <NoSessionView v-if="!session" />
        <div v-else>
          <AccountBar />
          <AdminDashboardView v-if="session.user.isAdmin" />
          <DashboardView v-else />
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped lang="scss">
@import "main.scss";

div#connect-error {
  margin-top: 20px;
  font-size: clamp(25px, 2.5lvw, 33px);
  color: #ffffff;
  text-align: center;
  animation: fade-in 1s;
}

$logo-vert-offset: 100px;

img {
  width: 15lvw;
  min-width: 125px;
  max-width: 150px;
  margin: 0 auto;
  display: block;
  animation: pulse 4s infinite ease-out;
  transform: translateY($logo-vert-offset);

  &.fade-up {
    animation: move-up 0.8s;
    animation-fill-mode: forwards;
    animation-delay: 0.3s;
  }
}

$max-opacity: 0.65;
$min-opacity: 0.45;

@keyframes pulse {
  0% {
    transform: scale(1);
    opacity: $min-opacity;
  }
  50% {
    transform: scale(1.07);
    opacity: $max-opacity;
  }
  100% {
    transform: scale(1);
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