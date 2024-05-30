<script setup lang="ts">
import useValidateConfirmedPassword from "../../hooks/validation/useValidateConfirmedPassword"
import useValidateUsername from "../../hooks/validation/useValidateUsername";

import LabeledFormInput from "./LabeledFormInput.vue"
import LoadingButton from "../LoadingButton.vue"

const { username, usernameError } = useValidateUsername()

const {
  password,
  confirmedPassword,
  passwordError,
  confirmedPasswordError
} = useValidateConfirmedPassword()

const handleSubmit = () => {
  console.log("Submitted")
}
</script>

<template>
  <h1>Reset your <b><i>bq</i></b> password below.</h1>
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
    <!-- <div class="form-error error-label" v-if="loginError">{{ loginError }}</div> -->
    <LoadingButton :loading="false" text="Confirm" />
  </form>
</template>