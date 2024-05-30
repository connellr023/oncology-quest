<script setup lang="ts">
import { onMounted, provide, ref } from "vue"
import { UserWithTasks } from "./models/user"
import { Domain } from "./models/domain"
import { EntryStructure } from "./models/tasks"

import useSession from "./hooks/useSession"

import DashboardView from "./views/DashboardView.vue"
import NoSessionView from "./views/NoSessionView.vue"
import MainLogo from "./components/vector/MainLogo.vue"
import ManageUsersBar from "./components/ManageUsersBar.vue"
import TopBar from "./components/TopBar.vue"

const {
  fetchSession,
  session,
  tasks,
  domains,
  loading,
  connectionError
} = useSession()
provide("session", session)
provide("tasks", tasks)
provide("domains", domains)

onMounted(fetchSession)

const entries = ref<Record<number, EntryStructure>>({})
provide("entries", entries)

const selectedUser = ref<UserWithTasks | null>(null)
provide("selectedUser", selectedUser)

const selectedDomain = ref<Domain | null>(null)
provide("selectedDomain", selectedDomain)

const isEditing = ref(false)
provide("isEditing", isEditing)

const resetAll = () => {
  selectedUser.value = null
  selectedDomain.value = null
  isEditing.value = false
  entries.value = {}
}
</script>

<template>
  <main>
    <div class="flex-wrapper">
      <MainLogo :class="`${session ? 'fade-out' : ''} ${(!loading && !connectionError) ? 'fade-up' : ''}`" />
      <div v-if="connectionError" class="connect-error"><b><i>bq</i></b> is currently under maintenance.</div>
      <div v-else-if="!loading">
        <NoSessionView v-if="!session" />
        <div class="dash-container" v-else>
          <ManageUsersBar v-if="session.isAdmin" />
          <div class="dash">
            <TopBar :onLogout="resetAll" />
            <DashboardView />
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped lang="scss">
@import "main.scss";

div.dash-container {
  opacity: 0;
  animation: fade-in 1s forwards;
  animation-delay: 0.5s;
  display: flex;
  width: 100lvw;
  height: 100lvh;

  div.dash {
    flex-grow: 1;
    background-color: $main-bg-color;
    padding: 12px 25px 20px 35px;
  }
}

div.connect-error {
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