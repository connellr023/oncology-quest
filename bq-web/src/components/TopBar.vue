<script setup lang="ts">
import { Ref, inject, onMounted, onUnmounted, reactive, ref } from "vue";
import { User } from "../models/user"
import { Domain } from "../models/domain";

import useLogout from "../hooks/useLogout"
import useValidateName from "../hooks/validation/useValidateName";
import useDomains from "../hooks/useDomains";

import UserProfileIcon from "./UserProfileIcon.vue"
import LogoutIcon from "./vector/LogoutIcon.vue"
import PushStackIcon from "./vector/PushStackIcon.vue"
import InputModal from "./InputModal.vue"

const session = inject<Ref<User>>("session")!
const domains = inject<Ref<Record<number, Domain>>>("domains")!
const selectedDomain = inject<Ref<Domain | null>>("selectedDomain")!

const { logout } = useLogout()
const { name, nameError } = useValidateName()
const { createDomain, deleteDomain } = useDomains()

const showProfileOptions = ref(false)
const showCreateDomainModal = ref(false)

let focusedDomainId = -1
const visibleDomainDropdowns = reactive<boolean[]>([])

const toggleDomainDropdown = (id: number) => {
  visibleDomainDropdowns[id] = !visibleDomainDropdowns[id]

  if (id !== focusedDomainId) {
    visibleDomainDropdowns[focusedDomainId] = false
  }

  if (visibleDomainDropdowns[id]) {
    focusedDomainId = id
  }
  else {
    focusedDomainId = -1
  }
}

const toggleProfileOptions = () => {
  showProfileOptions.value = !showProfileOptions.value
}

const hideDropdowns = () => {
  showProfileOptions.value = false
  visibleDomainDropdowns[focusedDomainId] = false
}

const selectDomain = (domain: Domain) => {
  selectedDomain.value = domain
  visibleDomainDropdowns[focusedDomainId] = false
}

const confirmNewDomain = () => {
  if (nameError.value || name.value.length === 0) {
    return
  }

  if (!createDomain(name.value)) {
    nameError.value = "Failed to create domain."
    return
  }

  showCreateDomainModal.value = false
}

const confirmDeleteDomain = () => {
  if (!deleteDomain(focusedDomainId)) {
    console.error("Failed to delete domain.")
  }
  else {
    visibleDomainDropdowns[focusedDomainId] = false
  }
}

const shouldAppearFocused = (id: number) => {
  return visibleDomainDropdowns[id] || selectedDomain.value?.id === id
}

onMounted(() => {
  window.addEventListener("click", hideDropdowns)
});

onUnmounted(() => {
  window.removeEventListener("click", hideDropdowns)
});
</script>

<template>
  <div class="topbar-container">
    <div class="profile-container">
      <UserProfileIcon @click.stop="toggleProfileOptions" class="profile-icon" :initials="session.name.substring(0, 2)" />
      <div v-show="showProfileOptions" class="dropdown-container profile-options" @click.stop>
        <button class="logout bubble highlight" @click="logout">
          <LogoutIcon />
          Logout
        </button>
      </div>
    </div>
    <div class="name"><b>{{ session.name }}</b> ({{ session.username }})</div>
    <div class="domain-select-container" :key="Object.keys(domains).length">
      <template v-if="session.isAdmin">
        <div v-for="domain in domains">
          <button @click.stop="toggleDomainDropdown(domain.id)" :class="`bubble domain-option ${shouldAppearFocused(domain.id) ? 'focused' : ''}`" :key="domain.id">{{ domain.name }}</button>
          <div class="dropdown-container" v-show="visibleDomainDropdowns[domain.id]" @click.stop>
            <button @click="selectDomain(domain)" class="bubble green">Select</button>
            <button class="bubble red" @click="confirmDeleteDomain">Delete</button>
          </div>
        </div>
        <button @click="() => { showCreateDomainModal = true }" class="bubble highlight new-domain">
          <PushStackIcon />
          New Domain
        </button>
      </template>
      <p v-else-if="Object.keys(domains).length === 0">Currently no domains to select.</p>
      <button v-else @click="selectDomain(domain)" v-for="domain in domains" :class="`bubble domain-option ${shouldAppearFocused(domain.id) ? 'focused' : ''}`" :key="domain.id">{{ domain.name }}</button>
    </div>
  </div>
  <InputModal
    v-if="session.isAdmin"
    v-model="name"
    title="New Domain"
    placeholder="Enter domain name..."
    :error="nameError"
    :visible="showCreateDomainModal"
    :onConfirm="confirmNewDomain"
    :onCancel="() => { showCreateDomainModal = false }"
  />
</template>

<style scoped lang="scss">
@import "../main.scss";

button.new-domain {
  margin-left: 10px;
}

button.domain-option {
  $side-margin: 2px;

  margin-left: $side-margin;
  margin-right: $side-margin;
}

div.dropdown-container {
  position: absolute;
  top: 50px;
  background-color: $main-bg-color;
  display: flex;
  flex-direction: column;
  align-items: center;
  z-index: 1;
  background-color: $tertiary-bg-color;
  border-radius: 8px;

  &.profile-options {
    top: 55px;
  }
}

div.profile-icon {
  cursor: pointer;
}

div.topbar-container {
  background-color: $main-bg-color;
  padding-bottom: 12px;
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;
  height: auto;
}

div.name {
  margin-left: 10px;
  font-size: 1.1em;
}

div.domain-select-container {
  margin-left: 15px;
  display: flex;
  flex-grow: 1;
  justify-content: flex-end;
  overflow-x: auto;

  p {
    opacity: 0.7;
    text-align: right;
  }
}
</style>