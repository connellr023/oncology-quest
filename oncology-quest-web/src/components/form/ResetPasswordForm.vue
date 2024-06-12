<script setup lang="ts">
import { computed, ref } from "vue"

import useResetPassword from "../../hooks/useResetPassword"

import TwoStageForm from "./TwoStageForm.vue"
import LabeledFormInput from "./LabeledFormInput.vue"

const props = defineProps<{
  onReset: () => void,
  onBack: () => void
}>()

const {
  username,
  password,
  confirmedPassword,
  usernameError,
  passwordError,
  confirmedPasswordError,
  requestResetPassword,
  resetError,
  token,
  tokenError,
  loading
} = useResetPassword()

const inStageOne = ref(true)

const isStageOneError = computed(() => {
  return (usernameError.value || passwordError.value || confirmedPasswordError.value) ? true : false
})

const canGotoStageTwo = computed(() => {
  return (!isStageOneError.value && username.value && password.value && confirmedPassword.value) ? true : false
})

const handleSubmit = async () => {
  if (!tokenError.value) {
    await requestResetPassword()
  }

  if (!resetError.value) {
    props.onReset()
  }
}
</script>

<template>
  <TwoStageForm
    submitButtonText="Confirm"
    :error="resetError"
    :loading="loading"
    :onBack="onBack"
    :handleSubmit="handleSubmit"
    :inStageOne="inStageOne"
    :canGotoStageTwo="canGotoStageTwo"
    @update-stage="inStageOne = $event"
  >
    <template #title>Reset your <b><i>Oncology Quest</i></b> password below.</template>
    <template #stage-one>
      <LabeledFormInput
        title="Username"
        name="username"
        type="text"
        :error="usernameError"
        v-model="username"
      />
      <LabeledFormInput
        title="New Password"
        name="password"
        type="password"
        :error="passwordError"
        v-model="password"
      />
      <LabeledFormInput
        title="Confirm New Password"
        name="password-confirm"
        type="password"
        :error="confirmedPasswordError"
        v-model="confirmedPassword"
      />
    </template>
    <template #stage-two>
      <LabeledFormInput
        title="Token"
        name="password-reset-token"
        type="text"
        :error="tokenError"
        v-model="token"
      />
    </template>
  </TwoStageForm>
</template>