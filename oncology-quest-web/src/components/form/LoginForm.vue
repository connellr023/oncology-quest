<script setup lang="ts">
import useLogin from "../../hooks/useLogin";

import LabeledFormInput from "./LabeledFormInput.vue"
import LoadingButton from "../LoadingButton.vue"

defineProps<{ onBack: () => void }>()

const {
  loading,
  login,
  loginError,
  password,
  passwordError,
  username,
  usernameError
} = useLogin()

const handleSubmit = (_: Event) => {
  const isError = (usernameError.value || passwordError.value)

  if (!isError) {
    login()
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
    <div class="form-error error-label" v-if="loginError">{{ loginError }}</div>
    <LoadingButton :loading="loading" text="Login" />
    <button type="button" class="back" @click="onBack">Back</button>
  </form>
</template>