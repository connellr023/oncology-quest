<script setup lang="ts">
import { computed } from "vue"

import useResetPassword from "../../hooks/useResetPassword"

import LabeledFormInput from "./LabeledFormInput.vue"
import LoadingButton from "../LoadingButton.vue"

const props = defineProps<{ onReset: () => void }>()

const {
  username,
  password,
  confirmedPassword,
  usernameError,
  passwordError,
  confirmedPasswordError,
  requestResetPassword,
  resetError,
  loading
} = useResetPassword()

const isFormError = computed(() => {
  return (usernameError.value || passwordError.value || confirmedPasswordError.value) ? true : false
})

const handleSubmit = async () => {
  if (isFormError.value) {
    return
  }

  await requestResetPassword()

  if (!resetError.value) {
    props.onReset()
  }
}
</script>

<template>
  <h1>Reset your <b><i>Oncology Quest</i></b> password below.</h1>
  <form @submit.prevent="handleSubmit">
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
    <div class="form-error error-label" v-if="resetError">{{ resetError }}</div>
    <LoadingButton :loading="loading" text="Confirm" />
  </form>
</template>