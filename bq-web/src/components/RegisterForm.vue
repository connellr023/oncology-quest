<script setup lang="ts">
import { ref } from "vue";
import { API_ENDPOINT } from "../utilities";
import useValidateUsername from "../hooks/useValidateUsername"
import useValidateName from "../hooks/useValidateName";
import useValidateEmail from "../hooks/useValidateEmail";
import useValidateConfirmedPassword from "../hooks/useValidateConfirmedPassword";

const { username, usernameError } = useValidateUsername()
const { name, nameError } = useValidateName()
const { email, emailError } = useValidateEmail()
const {
  password,
  passwordError,
  confirmedPassword,
  confirmedPasswordError 
} = useValidateConfirmedPassword()

const message = ref("")
const serverError = ref("")

const createUser = async () => {
  try {
    const response = await fetch(`${API_ENDPOINT}/api/user/create`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        username: username.value,
        name: name.value,
        email: email.value,
        password: password.value
      })
    })

    if (response.status === 201) {
      message.value = "User created successfully."
    }
    else {
      serverError.value = `Received status code ${response.status}`
    }
  }
  catch (error) {
    serverError.value = "Server error. Please try again later."
  }
}

const handleSubmit = (_: Event) => {
  const isError = (usernameError.value || nameError.value || emailError.value || passwordError.value || confirmedPasswordError.value)

  if (!isError) {
    createUser()
  }
}
</script>

<template>
  <h3>Register</h3>
  <form @submit.prevent="handleSubmit">
    <label for="username">Username</label>
    <input type="text" id="username" name="username" v-model="username" required>
    <label for="name">Name</label>
    <input type="text" id="name" name="name" v-model="name" required>
    <label for="email">Email</label>
    <input type="email" id="email" name="email" v-model="email" required>
    <label for="password">Password</label>
    <input type="password" id="password" name="password" v-model="password" required>
    <label for="password-confirm">Confirm Password</label>
    <input type="password" id="password-confirm" name="password-confirm" v-model="confirmedPassword" required>
    <button type="submit">Register</button>
  </form>
  <div>
    <p v-if="usernameError">{{ usernameError }}</p>
    <p v-if="nameError">{{ nameError }}</p>
    <p v-if="emailError">{{ emailError }}</p>
    <p v-if="passwordError">{{ passwordError }}</p>
    <p v-if="confirmedPasswordError">{{ confirmedPasswordError }}</p>
  </div>
  <div>
    <p v-if="serverError">{{ serverError }}</p>
    <p v-else-if="message">{{ message }}</p>
  </div>
</template>