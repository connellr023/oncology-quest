<script setup lang="ts">
import { onMounted, ref } from "vue"
import LoginForm from "../components/LoginForm.vue"
import RegisterForm from "../components/RegisterForm.vue"
import CreditLabel from "../components/CreditLabel.vue"

enum Views {
  SELECT,
  LOGIN,
  REGISTER
}

const view = ref(Views.SELECT)
const shouldAnimate = ref(true)

const setView = (newView: Views) => {
  view.value = newView;
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
      <h1>Get started with <b><i>bq</i></b> below.</h1>
      <button @click="setView(Views.LOGIN)" class="glow gradient-button-0">Login</button>
      <button @click="setView(Views.REGISTER)" class="glow gradient-button-0">Register</button>
      <button disabled class="glow gradient-button-0">Reset Password</button>
    </div>
    <template v-else-if="view === Views.LOGIN">
      <LoginForm />
      <button @click="setView(Views.SELECT)" class="back glow gradient-button-1">Back</button>
    </template>
    <template v-else-if="view === Views.REGISTER">
      <RegisterForm />
      <button @click="setView(Views.SELECT)" class="back glow gradient-button-1">Back</button>
    </template>
    <CreditLabel />
  </div>
</template>

<style scoped lang="scss">
@import "../main.scss";

div.button-container {
  &.animate {
    @include fade-up-children(0.4s, 0.15s, 4, 0.5s);
  }

  display: flex;
  flex-direction: column;
  text-align: center;

  button {
    width: 85%;
    margin: auto;
    margin-bottom: 18px;
  }

  h1 {
    margin-bottom: 30px;
  }
}

div.container {
  height: auto;
  margin-top: max(25px, 2.6lvh);
  text-align: center;
}
</style>