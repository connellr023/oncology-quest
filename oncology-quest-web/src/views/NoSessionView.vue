<script setup lang="ts">
import { onMounted, ref } from "vue"

import LoginForm from "../components/form/LoginForm.vue"
import RegisterForm from "../components/form/RegisterForm.vue"
import ResetPasswordForm from "../components/form/ResetPasswordForm.vue"
import InfoLabel from "../components/InfoLabel.vue"

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
    <div v-if="view === Views.SELECT" :class="`content-container ${shouldAnimate ? 'animate' : ''}`">
      <div class="greeting-container">
        <h1>Welcome to <b><i>Oncology Quest</i></b></h1>
        <p>an aid for Medical Oncology Trainees</p>
      </div>
      <button @click="gotoLogin" class="std">Login</button>
      <button @click="setView(Views.REGISTER)" class="std">Register</button>
      <button @click="setView(Views.RESET_PASSWORD)" class="std">Reset Password</button>
    </div>
    <LoginForm :onBack="gotoSelect" v-else-if="view === Views.LOGIN" />
    <RegisterForm :onBack="gotoSelect" :onRegister="gotoLogin" v-else-if="view === Views.REGISTER" />
    <ResetPasswordForm :onBack="gotoSelect" :onReset="gotoLogin" v-else-if="view === Views.RESET_PASSWORD" />
    <InfoLabel />
  </div>
</template>

<style scoped lang="scss">
@import "../styles/utilities.scss";
@import "../styles/variables.scss";

div.content-container {
  &.animate {
    @include fade-up-children(0.4s, 0.15s, 4, 0.5s);
  }

  width: 100%;
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

  div.greeting-container {
    margin-bottom: 17px;
    margin-top: -15px;

    p {
      $side-padding: 20px;

      padding-left: $side-padding;
      padding-right: $side-padding;
      margin-top: -5px;
      font-size: clamp(18px, 1.3lvw, 23px);
      white-space: pre-wrap;
    }
  }
}

div.container {
  height: auto;
  margin-top: 5px;
  text-align: center;
}
</style>