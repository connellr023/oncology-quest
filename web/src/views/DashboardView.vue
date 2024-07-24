<script setup lang="ts">
import { Ref, inject, ref, watch } from "vue"
import { UserTaskStructure } from "../models/tasks"
import { User } from "../models/user";
import { Rotation } from "../models/rotation";

import ManageUsersBar from "../components/ManageUsersBar.vue"
import TopBar from "../components/TopBar.vue"
import Entries from "../components/dash/Entries.vue"
import RotationSelect from "../components/dash/RotationSelect.vue"
import SelectRotationGraphic from "../components/vector/SelectRotationGraphic.vue"

const session = inject<Ref<User>>("session")!
const tasks = inject<Ref<Record<number, UserTaskStructure>>>("tasks")!
const selectedUser = inject<Ref<User | null>>("selectedUser")!
const selectedUserTasks = inject<Ref<UserTaskStructure | null>>("selectedUserTasks")!
const selectedRotation = inject<Ref<Rotation | null>>("selectedRotation")!
const isEditing = inject<Ref<boolean>>("isEditing")!

const loadedTasksUserId = ref<number | undefined>(undefined)

if (session.value.isAdmin) {
  watch(() => [selectedUser.value?.id, selectedRotation.value?.id], () => {
    isEditing.value = false
  })

  watch(() => selectedUserTasks.value, (newVal) => {
    if (newVal) {
      loadedTasksUserId.value = selectedUser.value?.id
    }
    else {
      loadedTasksUserId.value = undefined
    }
  }, { deep: true })
}
</script>

<template>
  <div class="dash-container">
    <ManageUsersBar v-if="session.isAdmin" />
    <div :class="`dash ${session.isAdmin ? 'shifted' : ''}`">
      <TopBar />
      <div class="dash-content">
        <RotationSelect />
        <Entries :key="`${selectedRotation.id}.${loadedTasksUserId}`" :selectedUserName="selectedUser?.name" v-if="session.isAdmin && selectedRotation" :selectedRotation="selectedRotation" :tasks="selectedUserTasks || {}" />
        <Entries :key="selectedRotation.id" v-else-if="!session.isAdmin && selectedRotation" :tasks="tasks[selectedRotation.id]" :selectedRotation="selectedRotation" />
        <div v-else class="note">
          <SelectRotationGraphic class="graphic" />
          <p>Select a rotation from the above list to get started.</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../styles/variables.scss";

div.note {
  text-align: center;
  font-size: clamp(19px, 1.3lvw, 24px);
  text-wrap: wrap;
  text-align: center;
  margin-top: 10lvh;

  svg {
    width: 20lvw;
    min-width: 240px;
    max-width: 300px;
    fill: $tertiary-bg-color;
  }

  &.empty-rotation {
    padding: 30px;
  }

  &.empty-rotation,
  p {
    opacity: 0.8;
  }
}

div.dash-container {
  opacity: 0;
  animation: fade-in 1s forwards;
  animation-delay: 0.5s;
  display: flex;
  flex-direction: column;
  width: 100lvw;
  height: 100lvh;
  
  div.dash {
    $side-padding: 15px;
    $shifted-left-padding: 30px;

    padding: 13px $side-padding 0 $side-padding;
    height: 100%;

    &.shifted {
      padding-left: $shifted-left-padding;
    }

    div.dash-content {
      height: calc(100% - 60px);
    }
  }
}
</style>