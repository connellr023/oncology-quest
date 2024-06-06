<script setup lang="ts">
import { onMounted, ref } from "vue"

import LoginForm from "../components/form/LoginForm.vue"
import RegisterForm from "../components/form/RegisterForm.vue"
import ResetPasswordForm from "../components/form/ResetPasswordForm.vue"
import CreditLabel from "../components/CreditLabel.vue"

enum Views {
  SELECT,
  LOGIN,
  REGISTER,
  RESET_PASSWORD
}

const view = ref(Views.SELECT)
const shouldAnimate = ref(true)

const setView = (newView: Views) => {
  view.value = newView;
}

const gotoLogin = () => {
  setView(Views.LOGIN)
}

const gotoSelect = () => {
  setView(Views.SELECT)
}

onMounted(() => {
  setTimeout(() => {
    shouldAnimate.value = false;
  }, 1700);
})
</script>

<template>
  <div class="container flex-wrapper">
    <div v-if="view === Views.SELECT" :class="`button-container ${shouldAnimate ? 'animate' : ''}`">
      <h1>Get started with <b><i>Oncology Quest</i></b> below.</h1>
      <button @click="gotoLogin" class="std">Login</button>
      <button @click="setView(Views.REGISTER)" class="std">Register</button>
      <button @click="setView(Views.RESET_PASSWORD)" class="std">Reset Password</button>
    </div>
    <LoginForm :onBack="gotoSelect" v-else-if="view === Views.LOGIN" />
    <RegisterForm :onBack="gotoSelect" :onRegister="gotoLogin" v-else-if="view === Views.REGISTER" />
    <ResetPasswordForm :onBack="gotoSelect" :onReset="gotoLogin" v-else-if="view === Views.RESET_PASSWORD" />
    <CreditLabel />
  </div>
</template>

<style scoped lang="scss">
@import "../styles/utilities.scss";
@import "../styles/variables.scss";

div.button-container {
  &.animate {
    @include fade-up-children(0.4s, 0.15s, 4, 0.5s);
  }

  width: 100lvw;
  display: flex;
  flex-direction: column;
  text-align: center;

  button {
    width: 35%;
    min-width: 250px;
    max-width: 290px;
    margin: auto;
    margin-bottom: 18px;
    padding: 15px
  }

  h1 {
    margin-bottom: 30px;
  }
}

div.container {
  height: auto;
  margin-top: 5px;
  text-align: center;
}
</style>