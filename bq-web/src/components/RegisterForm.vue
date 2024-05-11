<script setup lang="ts">
import { ref } from "vue";
import useRegister from "../hooks/useRegister"
import LabeledFormInput from "./LabeledFormInput.vue"

const {
  username,
  name,
  email,
  password,
  confirmedPassword,
  usernameError,
  nameError,
  emailError,
  passwordError,
  confirmedPasswordError,
  serverError,
  message,
  register
} = useRegister()

const handleSubmit = (_: Event) => {
  const isError = (usernameError.value || nameError.value || emailError.value || passwordError.value || confirmedPasswordError.value)

  if (!isError) {
    register()
  }
}

const inStageOne = ref(true)

const switchStage = () => {
  inStageOne.value = !inStageOne.value
}
</script>

<template>
  <h1>Register a <b><i>bq</i></b> account below.</h1>
  <form @submit.prevent="handleSubmit">
    <template v-if="inStageOne">
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
      <LabeledFormInput
        title="Email"
        name="email"
        type="email"
        :error="emailError"
        v-model="email"
      />
      <button class="form-button glow gradient-button-0" @click="switchStage">Next Step</button>
    </template>
    <template v-else>
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
      <div>
        <div class="form-error error-label" v-if="serverError">{{ serverError }}</div>
        <p v-else-if="message">{{ message }}</p>
      </div>
      <button class="form-button glow gradient-button-0" type="submit">Register</button>
      <button class="prev glow gradient-button-0" @click="switchStage">Previous Step</button>
    </template>
  </form>
</template>

<style scoped lang="scss">
button.prev {
  margin-top: 8px;
}
</style>