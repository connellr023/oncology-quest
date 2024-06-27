<script setup lang="ts">
import { Ref, computed, nextTick, reactive, ref, watch } from "vue"

import useResetPassword from "../../hooks/useResetPassword"

import TwoStageForm from "./TwoStageForm.vue"
import LabeledFormInput from "./LabeledFormInput.vue"

const props = defineProps<{
  onReset: () => void,
  onBack: () => void
}>()

const {
  username,
  password,
  confirmedPassword,
  usernameError,
  passwordError,
  confirmedPasswordError,
  requestResetPassword,
  resetError,
  token,
  tokenError,
  loading
} = useResetPassword()

const inStageOne = ref(true)
const tokenInputs = reactive(["", "", "", ""])
const tokenInputFields = reactive<Ref<[HTMLInputElement] | null>[]>([ref(null), ref(null), ref(null), ref(null)])

const isStageOneError = computed(() => {
  return (usernameError.value || passwordError.value || confirmedPasswordError.value) ? true : false
})

const canGotoStageTwo = computed(() => {
  return (!isStageOneError.value && username.value && password.value && confirmedPassword.value) ? true : false
})

const joinedTokenInputs = computed(() => {
  return tokenInputs.join("")
})

watch(() => joinedTokenInputs.value, (newVal) => {
  token.value = newVal
})

const handleSubmit = async () => {
  if (!tokenError.value) {
    await requestResetPassword()
  }

  if (!resetError.value) {
    props.onReset()
  }
}

const gotoNextInputField = (index: number) => {
  if (index < tokenInputFields.length - 1) {
    nextTick(() => {
      const field = tokenInputFields[index + 1].value

      if (field) {
        field[0].focus()
      }
    })
  }
}
</script>

<template>
  <TwoStageForm
    submitButtonText="Confirm"
    :error="resetError"
    :loading="loading"
    :onBack="onBack"
    :handleSubmit="handleSubmit"
    :inStageOne="inStageOne"
    :canGotoStageTwo="canGotoStageTwo"
    @update-stage="inStageOne = $event"
  >
    <template #title>Reset your <b><i>Oncology Quest</i></b> password below.</template>
    <template #stage-one>
      <LabeledFormInput
        title="Username"
        name="username"
        type="text"
        :error="usernameError"
        v-model="username"
      />
      <LabeledFormInput
        title="New Password"
        name="password"
        type="password"
        :error="passwordError"
        v-model="password"
      />
      <LabeledFormInput
        title="Confirm New Password"
        name="password-confirm"
        type="password"
        :error="confirmedPasswordError"
        v-model="confirmedPassword"
      />
    </template>
    <template #stage-two>
      <div class="token-input-container">
        <input
          v-for="(field, index) in tokenInputFields"
          :class="`${tokenError ? 'error' : ''}`"
          :key="index"
          :ref="field"
          v-model="tokenInputs[index]"
          maxlength="1"
          type="text"
          @input="gotoNextInputField(index)"
        />
      </div>
      <span v-if="tokenError" class="error-label">{{ tokenError }}</span>
    </template>
  </TwoStageForm>
</template>

<style scoped lang="scss">
div.token-input-container {
  display: flex;
  justify-content: center;
  margin-top: 25px;
  gap: 15px;

  input {
    $size: 30px;

    border-radius: 12px;
    width: $size;
    height: $size;
    font-size: $size;
    text-align: center;
  }
}
</style>