<script setup lang="ts">
import { onMounted, provide, ref } from "vue"
import { User } from "./models/user"
import { Rotation } from "./models/rotation"
import { EntryStructure, UserTaskStructure } from "./models/tasks"

import useSession from "./hooks/useSession"

import DashboardView from "./views/DashboardView.vue"
import NoSessionView from "./views/NoSessionView.vue"
import MainLogo from "./components/vector/MainLogo.vue"

const {
  fetchSession,
  session,
  rotations,
  loading,
  connectionError
} = useSession()
provide("session", session)
provide("rotations", rotations)

onMounted(fetchSession)

const entries = ref<Record<number, EntryStructure>>({})
provide("entries", entries)

const tasks = ref<Record<number, UserTaskStructure>>({})
provide("tasks", tasks)

const selectedUser = ref<User | null>(null)
provide("selectedUser", selectedUser)

const selectedUserTasks = ref<UserTaskStructure | null>(null)
provide("selectedUserTasks", selectedUserTasks)

const selectedRotation = ref<Rotation | null>(null)
provide("selectedRotation", selectedRotation)

const isEditing = ref(false)
provide("isEditing", isEditing)

const resetAll = () => {
  selectedUser.value = null
  selectedUserTasks.value = null
  selectedRotation.value = null
  isEditing.value = false
  entries.value = {}
}
provide("resetAll", resetAll)
</script>

<template>
  <main>
    <div class="flex-wrapper">
      <div :class="`logo-container ${session ? 'fade-out' : ''} ${(!loading && !connectionError) ? 'fade-up' : ''}`">
        <MainLogo class="logo" />
      </div>
      <div v-if="connectionError" class="connect-error"><b><i>Oncology Quest</i></b> is currently under maintenance</div>
      <div v-else-if="!loading">
        <NoSessionView v-if="!session" />
        <DashboardView v-else />
      </div>
    </div>
  </main>
</template>

<style scoped lang="scss">
@import "./styles/variables.scss";

div.connect-error {
  margin-top: 20px;
  font-size: clamp(25px, 2.5lvw, 33px);
  color: #ffffff;
  text-align: center;
  animation: fade-in 1s;
}

$logo-vert-offset: 170px;

.fade-up {
  transform: translateY($logo-vert-offset);
  animation: move-up 0.8s;
}

.fade-out {
  animation: fade-out 0.5s;
  position: fixed;
  transform: translate(-50%, -50%);
  left: 50%;
  top: 50%;
}

.fade-up,
.fade-out {
  animation-fill-mode: forwards;
  animation-delay: 0.13s;
}

svg {
  width: 13lvh;
  min-width: 80px;
  max-width: 125px;

  &.logo {
    margin: 0 auto;
    display: block;
    fill: $theme-color-1;
  }

  &.logo-effect {
    fill: $theme-color-1;
    opacity: 0.2;
    animation: pulse 9s infinite ease-out;
    position: absolute;
    filter: blur(45px);
    z-index: 2;

    $start-scale: 1;
    $end-scale: 1.6;

    @keyframes pulse {
      0% {
        transform: scale($start-scale);
      }
      50% {
        transform: scale($end-scale);
      }
      100% {
        transform: scale($start-scale);
      }
    }
  }
}

div.logo-container {
  display: flex;
  align-items: center;
  justify-content: center;
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

@media (max-height: 400px) {
  svg.logo {
    display: none;
  }
}
</style>