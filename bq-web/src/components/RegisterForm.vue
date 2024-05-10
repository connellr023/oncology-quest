<script setup lang="ts">
import useRegister from '../hooks/useRegister'

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
</script>

<template>
  <h1>Register a <b><i>bq</i></b> account below.</h1>
  <form @submit.prevent="handleSubmit">
    <label for="username">Username</label>
    <input class="glow" type="text" id="username" name="username" v-model="username" required>
    <label for="name">Name</label>
    <input class="glow" type="text" id="name" name="name" v-model="name" required>
    <label for="email">Email</label>
    <input class="glow" type="email" id="email" name="email" v-model="email" required>
    <label for="password">Password</label>
    <input class="glow" type="password" id="password" name="password" v-model="password" required>
    <label for="password-confirm">Confirm Password</label>
    <input class="glow" type="password" id="password-confirm" name="password-confirm" v-model="confirmedPassword" required>
    <div>
      <p v-if="usernameError">{{ usernameError }}</p>
      <p v-if="nameError">{{ nameError }}</p>
      <p v-if="emailError">{{ emailError }}</p>
      <p v-if="passwordError">{{ passwordError }}</p>
      <p v-if="confirmedPasswordError">{{ confirmedPasswordError }}</p>
      <p v-if="serverError">{{ serverError }}</p>
      <p v-else-if="message">{{ message }}</p>
    </div>
    <button class="form-button glow gradient-button-0" type="submit">Register</button>
  </form>
</template>