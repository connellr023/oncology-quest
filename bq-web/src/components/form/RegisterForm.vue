<script setup lang="ts">
import { ref } from "vue";
import useRegister from "../../hooks/useRegister"

import LabeledFormInput from "./LabeledFormInput.vue"
import LoadingButton from "../LoadingButton.vue"

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
  register,
  loading
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
  <div class="stage-indicator-container">
    <div :class="`${inStageOne ? 'active' : ''}`" />
    <div :class="`${!inStageOne ? 'active' : ''}`" />
  </div>
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
        <div class="error-label" v-if="serverError">{{ serverError }}</div>
        <div class="success-label" v-else-if="message">{{ message }}</div>
      </div>
      <LoadingButton :loading="loading" text="Register" />
      <button class="prev glow gradient-button-0" @click="switchStage">Previous Step</button>
    </template>
  </form>
</template>

<style scoped lang="scss">
div.stage-indicator-container {
  display: flex;
  justify-content: center;
  margin-top: 5px;
  margin-bottom: 15px;
  filter: drop-shadow(0px 0px 6px rgba(255, 255, 255, 0.06));

  div {
    $size: 13px;

    width: $size;
    height: $size;
    border-radius: 50%;
    background-color: #ffffff;
    opacity: 0.7;
    margin: 0 5px;
    transition: all 0.3s ease-in-out;
  }

  div.active {
    opacity: 1;
    border-radius: 8px;
    width: 30px;
  }
}

button.prev {
  margin-top: 8px;
}
</style>