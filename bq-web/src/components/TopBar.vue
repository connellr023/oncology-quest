<script setup lang="ts">
import { Ref, inject, reactive, ref } from "vue"
import { User } from "../models/user"
import { Domain } from "../models/domain"

import useLogout from "../hooks/useLogout"
import useValidateName from "../hooks/validation/useValidateName"
import useDomains from "../hooks/useDomains"
import useEntries from "../hooks/useEntries"

import UserProfileIcon from "./UserProfileIcon.vue"
import LogoutIcon from "./vector/LogoutIcon.vue"
import InputModal from "./InputModal.vue"
import NewDomainIcon from "./vector/NewDomainIcon.vue"
import DeleteIcon from "./vector/DeleteIcon.vue"
import CheckIcon from "./vector/CheckIcon.vue"
import Dropdown from "./Dropdown.vue"

const props = defineProps<{ onLogout: () => void }>()

const session = inject<Ref<User>>("session")!
const domains = inject<Ref<Record<number, Domain>>>("domains")!
const selectedDomain = inject<Ref<Domain | null>>("selectedDomain")!

const { logout } = useLogout()
const { name, nameError } = useValidateName()
const { createDomain, deleteDomain } = useDomains()
const { fetchEntriesWithCaching } = useEntries()

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

const selectDomain = async (domain: Domain) => {
  if (!await fetchEntriesWithCaching(domain.id)) {
    console.error("Failed to fetch entries.")
    return
  }

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

const onLogoutClick = () => {
  props.onLogout()
  logout()
}
</script>

<template>
  <div class="topbar-container">
    <div>
      <UserProfileIcon @click.stop="toggleProfileOptions" class="profile-icon" :initials="session.name.substring(0, 2)" />
      <Dropdown :isVisible="showProfileOptions" @change="showProfileOptions = $event">
        <button class="logout bubble" @click="onLogoutClick">
          <LogoutIcon />
          Logout
        </button>
      </Dropdown>
    </div>
    <div class="name"><b>{{ session.name }}</b> ({{ session.username }})</div>
    <div class="domain-select-container" :key="Object.keys(domains).length">
      <template v-if="session.isAdmin">
        <div v-for="domain in domains">
          <button @click.stop="toggleDomainDropdown(domain.id)" :class="`bubble domain-option ${shouldAppearFocused(domain.id) ? 'focused' : ''}`" :key="domain.id">{{ domain.name }}</button>
          <Dropdown class="domain-option-dropdown" :isVisible="visibleDomainDropdowns[domain.id]" @change="visibleDomainDropdowns[domain.id] = $event">
            <button @click="selectDomain(domain)" class="bubble green">
              <CheckIcon />
              Select
            </button>
            <button class="bubble red" @click="confirmDeleteDomain">
              <DeleteIcon />
              Delete
            </button>
          </Dropdown>
        </div>
        <button @click="() => { showCreateDomainModal = true }" class="bubble highlight new-domain">
          <NewDomainIcon />
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

div.domain-option-dropdown {
  top: 50px;
  margin-left: 2px;
}

button.new-domain {
  margin-left: 10px;
}

button.domain-option {
  $side-margin: 2px;

  margin-left: $side-margin;
  margin-right: $side-margin;
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