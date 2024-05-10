<script setup lang="ts">
import useLogin from '../hooks/useLogin';

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
  <h1>Login to <b><i>bq</i></b> below.</h1>
  <form @submit.prevent="handleSubmit">
    <label for="username">Username</label>
    <input class="glow" type="text" id="username" name="username" v-model="username" required>
    <label for="password">Password</label>
    <input class="glow" type="password" id="password" name="password" v-model="password" required>
    <div>
      <p v-if="usernameError">{{ usernameError }}</p>
      <p v-if="passwordError">{{ passwordError }}</p>
      <p v-if="loginError">{{ loginError }}</p>
      <p v-else-if="loading">Loading...</p>
    </div>
    <button class="form-button glow gradient-button-0" type="submit">Login</button>
  </form>
</template>