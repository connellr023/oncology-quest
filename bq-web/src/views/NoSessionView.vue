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
  <div class="container">
    <div>
      <div v-if="view === Views.SELECT">
        <h1>Get started with <b><i>bq</i></b> below...</h1>
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

<style scoped lang="scss">
div.container {
  margin-top: max(25px, 2.6lvh);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

h1 {
  font-weight: normal;
}
</style>