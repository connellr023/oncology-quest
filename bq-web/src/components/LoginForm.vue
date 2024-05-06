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
  <h3>Login</h3>
  <form @submit.prevent="handleSubmit">
    <label for="username">Username</label>
    <input type="text" id="username" name="username" v-model="username" required>
    <label for="password">Password</label>
    <input type="password" id="password" name="password" v-model="password" required>
    <button type="submit">Login</button>
    <div>
      <p v-if="usernameError">{{ usernameError }}</p>
      <p v-if="passwordError">{{ passwordError }}</p>
    </div>
    <div>
      <p v-if="loginError">{{ loginError }}</p>
      <p v-else-if="loading">Loading...</p>
    </div>
  </form>
</template>