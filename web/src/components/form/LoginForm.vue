<script setup lang="ts">
import useLogin from "../../hooks/useLogin"
import useNotifications from "../../hooks/useNotifications"

import LabeledFormInput from "./LabeledFormInput.vue"
import LoadingButton from "../LoadingButton.vue"
import BackButton from "./BackButton.vue"

defineProps<{ onBack: () => void }>()

const { pushNotification } = useNotifications()
const {
  loading,
  login,
  loginError,
  password,
  passwordError,
  username,
  usernameError
} = useLogin()

const handleSubmit = async (_: Event) => {
  const isError = (usernameError.value || passwordError.value)

  if (!isError) {
    await login()

    if (loginError.value.length === 0) {
      pushNotification("Logged in successfully.", true)
    }
    else {
      pushNotification(loginError.value)
    }
  }
}
</script>

<template>
  <h1>Login to <b><i>Oncology Quest</i></b> below.</h1>
  <form @submit.prevent="handleSubmit">
    <LabeledFormInput
      title="Username"
      name="username"
      type="text"
      :error="usernameError"
      v-model="username"
    />
    <LabeledFormInput
      title="Password"
      name="password"
      type="password"
      :error="passwordError"
      v-model="password"
    />
    <LoadingButton :loading="loading" text="Login" />
    <BackButton :onBack="onBack" />
  </form>
</template>