<script setup lang="ts">
import { computed, ref } from "vue"

import useRegister from "../../hooks/useRegister"

import LabeledFormInput from "./LabeledFormInput.vue"
import LoadingButton from "../LoadingButton.vue"

const props = defineProps<{
  onRegister: () => void,
  onBack: () => void
}>()

const {
  username,
  name,
  password,
  confirmedPassword,
  usernameError,
  nameError,
  passwordError,
  confirmedPasswordError,
  serverError,
  register,
  loading
} = useRegister()

const isStageOneErrors = computed(() => {
  return (usernameError.value || nameError.value) ? true : false
})

const isStageTwoErrors = computed(() => {
  return (passwordError.value || confirmedPasswordError.value) ? true : false
})

const canGotoStageTwo = computed(() => {
  return (!isStageOneErrors.value && username.value && name.value) ? true : false
})

const handleSubmit = async () => {
  if (!isStageTwoErrors.value) {
    await register()
  }

  if (!serverError.value) {
    props.onRegister()
  }
}

const inStageOne = ref(true)

const switchStage = () => {
  inStageOne.value = !inStageOne.value
}
</script>

<template>
  <h1>Register a <b><i>Oncology Quest</i></b> account below.</h1>
  <div class="stage-indicator-container">
    <div :class="`${inStageOne ? 'active' : ''}`" />
    <div :class="`${!inStageOne ? 'active' : ''}`" />
  </div>
  <form @submit.prevent="handleSubmit" @keydown.enter.prevent>
    <div class="stage-container" v-show="inStageOne">
      <LabeledFormInput
        title="Username"
        name="username"
        type="text"
        :error="usernameError"
        v-model="username"
      />
      <LabeledFormInput
        title="Name"
        name="name"
        type="text"
        :error="nameError"
        v-model="name"
      />
      <button :disabled="!canGotoStageTwo" class="form-button std" @click="switchStage">Next Step</button>
    </div>
    <div class="stage-container" v-show="!inStageOne">
      <LabeledFormInput
        title="Password"
        name="password"
        type="password"
        :error="passwordError"
        v-model="password"
      />
      <LabeledFormInput
        title="Confirm Password"
        name="password-confirm"
        type="password"
        :error="confirmedPasswordError"
        v-model="confirmedPassword"
      />
      <div>
        <div class="error-label" v-if="serverError">{{ serverError }}</div>
      </div>
      <LoadingButton :loading="loading" text="Register" />
      <button class="prev std" @click="switchStage">Previous Step</button>
    </div>
    <button class="back" @click="onBack">Back</button>
  </form>
</template>

<style scoped lang="scss">
div.stage-indicator-container {
  display: flex;
  justify-content: center;
  margin-top: 5px;
  margin-bottom: 15px;

  div {
    $size: 12px;

    width: $size;
    height: $size;
    border-radius: 50%;
    background-color: #ffffff;
    opacity: 0.7;
    margin: 0 5px;
    transition: all 0.3s ease;
  }

  div.active {
    opacity: 1;
    border-radius: 8px;
    width: 45px;
  }
}

button.prev {
  margin-top: -4px;
  margin-bottom: 7px;
}
</style>