<script setup lang="ts">
import { ref } from "vue"
import LoginForm from "../components/LoginForm.vue"
import RegisterForm from "../components/RegisterForm.vue"
import CreditLabel from "../components/CreditLabel.vue"

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
  <div class="container flex-wrapper">
    <div>
      <div v-if="view === Views.SELECT" class="button-container">
        <h1>Get started with <b><i>bq</i></b> below...</h1>
        <button @click="setView(Views.LOGIN)" class="glow gradient-button-0">Login</button>
        <button @click="setView(Views.REGISTER)" class="glow gradient-button-0">Register</button>
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
    <CreditLabel />
  </div>
</template>

<style scoped lang="scss">
@import "../main.scss";

div.button-container {
  display: flex;
  flex-direction: column;
  text-align: center;

  button {
    width: 85%;
    margin: auto;
    margin-bottom: 18px;
  }
}

div.container {
  height: auto;
  margin-top: max(25px, 2.6lvh);
  text-align: center;
}

h1 {
  font-weight: normal;
  font-size: clamp(25px, 2.4lvw, 32px);
}
</style>