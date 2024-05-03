<script setup lang="ts">
import useValidateUsername from "../hooks/useValidateUsername"

const handleSubmit = (_: Event) => {
  console.log("form submitted")
}

const { username, usernameError } = useValidateUsername()
</script>

<template>
  <h3>Register</h3>
  <form @submit.prevent="handleSubmit">
    <label for="username">Username</label>
    <input type="text" id="username" name="username" v-model="username" required>
    <label for="name">Name</label>
    <input type="text" id="name" name="name" required>
    <label for="email">Email</label>
    <input type="email" id="email" name="email" required>
    <label for="password">Password</label>
    <input type="password" id="password" name="password" required>
    <label for="password-confirm">Confirm Password</label>
    <input type="password" id="password-confirm" name="password-confirm" required>
    <button type="submit">Register</button>
  </form>
  <div>
    <p v-if="usernameError">{{ usernameError }}</p>
  </div>
</template>