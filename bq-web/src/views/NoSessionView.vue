<script setup lang="ts">
import { ref } from "vue"
import LoginForm from "../components/LoginForm.vue"
import RegisterForm from "../components/RegisterForm.vue"

enum Views {
  SELECT,
  LOGIN,
  REGISTER
}

const view = ref(Views.SELECT)

const setView = (newView: Views) => {
  view.value = newView;
}
</script>

<template>
  <div>
    <h1>bq</h1>
    <div class="form-container">
      <div v-if="view === Views.SELECT">
        <button @click="setView(Views.LOGIN)">Login</button>
        <button @click="setView(Views.REGISTER)">Register</button>
        <button disabled>Reset Password</button>
      </div>
      <div v-else-if="view === Views.LOGIN">
        <LoginForm />
        <button @click="setView(Views.SELECT)">Back</button>
      </div>
      <div v-else-if="view === Views.REGISTER">
        <RegisterForm />
        <button @click="setView(Views.SELECT)">Back</button>
      </div>
    </div>
  </div>
</template>