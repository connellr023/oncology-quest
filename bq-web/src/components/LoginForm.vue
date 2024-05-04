<script setup lang="ts">
import { Ref, inject, provide, ref } from "vue";
import { API_ENDPOINT } from "../utility";
import { UserSession } from "../models/user";
import useValidateUsername from "../hooks/useValidateUsername"
import useValidatePassword from "../hooks/useValidatePassword"

const { username, usernameError } = useValidateUsername()
const { password, passwordError } = useValidatePassword()

const loading = ref(false)
const serverError = ref("")
const sessionContext = inject<Ref<UserSession | null>>("session")

const login = async () => {
  loading.value = true

  try {
    const response = await fetch(`${API_ENDPOINT}/api/user/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        username: username.value,
        password: password.value
      })
    })

    if (response.ok) {
      try {
        const session: UserSession = await response.json()

        if (!sessionContext) {
          serverError.value = "Session context not found."
        }
        else {
          sessionContext.value = session
        }
      }
      catch (error) {
        serverError.value = "Failed to parse response."
      }
    }
    else if (response.status === 401) {
      serverError.value = "That username and password combination is incorrect."
    }
    else {
      serverError.value = `Received status code ${response.status}`
    }
  }
  catch (error) {
    serverError.value = "Server error. Please try again later."
  }

  loading.value = false
}

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
      <p v-if="serverError">{{ serverError }}</p>
      <p v-else-if="loading">Loading...</p>
    </div>
  </form>
</template>