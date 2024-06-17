<script setup lang="ts">
import CheckIcon from "./vector/CheckIcon.vue"
import CancelIcon from "./vector/CancelIcon.vue"
import Spinner from "./vector/Spinner.vue"

const model = defineModel()

defineProps<{
  title: string,
  placeholder: string,
  loading: boolean,
  visible: boolean,
  error: string,
  isPassword: boolean,
  onConfirm: () => void,
  onCancel: () => void
}>()
</script>

<template>
  <div class="modal" v-if="visible">
    <div class="modal-content">
      <h3>{{ title }}</h3>
      <div class="input-container">
        <input @keyup.enter="onConfirm" :type="`${isPassword ? 'password' : 'text'}`" :class="`bubble ${error ? 'error' : ''}`" v-model="model" :placeholder="placeholder" />
        <Spinner class="spinner" v-show="loading" />
      </div>
      <p v-if="error" class="error">{{ error }}</p>
      <div class="modal-buttons"> 
        <button class="bubble green" @click="onConfirm" :disabled="error ? true : false">
          <CheckIcon />
          Confirm
        </button>
        <button class="bubble red" @click="onCancel">
          <CancelIcon />
          Cancel
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
div.input-container {
  position: relative;

  input {
    width: calc(100% - 50px);
  }

  svg.spinner {
    position: absolute;
    width: 22px;
    right: 25px;
    top: 20px;
    fill: rgba(255, 255, 255, 0.4);
  }
}
</style>