<script setup lang="ts">
import { computed, ref } from "vue"

import useRegister from "../../hooks/useRegister"
import useNotifications from "../../hooks/useNotifications"

import LabeledFormInput from "./LabeledFormInput.vue"
import TwoStageForm from "./TwoStageForm.vue"

const props = defineProps<{
  onRegister: () => void,
  onBack: () => void
}>()

const { pushNotification } = useNotifications()
const {
  username,
  name,
  password,
  confirmedPassword,
  usernameError,
  nameError,
  passwordError,
  confirmedPasswordError,
  serverError,
  register,
  loading
} = useRegister()

const isStageOneErrors = computed(() => {
  return (usernameError.value || nameError.value) ? true : false
})

const isStageTwoErrors = computed(() => {
  return (passwordError.value || confirmedPasswordError.value) ? true : false
})

const canGotoStageTwo = computed(() => {
  return (!isStageOneErrors.value && username.value && name.value) ? true : false
})

const handleSubmit = async () => {
  if (!isStageTwoErrors.value) {
    await register()
  }

  if (serverError.value.length === 0) {
    props.onRegister()
    pushNotification("Registered successfully.", true)
  }
  else {
    pushNotification(serverError.value)
  }
}

const inStageOne = ref(true)
</script>

<template>
  <TwoStageForm
    submitButtonText="Register"
    :disableSubmit="isStageTwoErrors"
    :loading="loading"
    :onBack="onBack"
    :handleSubmit="handleSubmit"
    :inStageOne="inStageOne"
    :canGotoStageTwo="canGotoStageTwo"
    @update-stage="inStageOne = $event"
  >
    <template #title>Register a <b><i>Oncology Quest</i></b> account below.</template>
    <template #stage-one>
      <LabeledFormInput
        title="Username"
        name="username"
        type="text"
        :error="usernameError"
        v-model="username"
      />
      <LabeledFormInput
        title="Name"
        name="name"
        type="text"
        :error="nameError"
        v-model="name"
      />
    </template>
    <template #stage-two>
      <LabeledFormInput
        title="Password"
        name="password"
        type="password"
        :error="passwordError"
        v-model="password"
      />
      <LabeledFormInput
        title="Confirm Password"
        name="password-confirm"
        type="password"
        :error="confirmedPasswordError"
        v-model="confirmedPassword"
      />
    </template>
  </TwoStageForm>
</template>