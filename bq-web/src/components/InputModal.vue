<script setup lang="ts">
import CheckIcon from "./vector/CheckIcon.vue"
import CancelIcon from "./vector/CancelIcon.vue"

const model = defineModel()

defineProps<{
  title: string,
  placeholder: string,
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
      <input :type="`${isPassword ? 'password' : 'text'}`" :class="`bubble ${error ? 'error' : ''}`" v-model="model" :placeholder="placeholder" />
      <p v-if="error" class="error">{{ error }}</p>
      <div class="modal-buttons"> 
        <button class="bubble green" @click="onConfirm">
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
@import "../main.scss";

div.modal {
  position: fixed;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.8);
  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
  z-index: 1;

  div.modal-content {
    padding: 10px;
    border-radius: 14px;
    background-color: $secondary-bg-color;
    width: 40lvw;
    min-width: 290px;
    max-width: 400px;

    h3 {
      font-weight: normal;
    }

    input {
      font-size: 17px;
      margin: 8px;
    }

    p.error {
      $side-padding: 10px;

      padding-left: $side-padding;
      padding-right: $side-padding;
      color: $theme-color-red;
      margin: auto;
    }

    div.modal-buttons {
      display: flex;
      justify-content: center;
      margin-top: 10px;

      button {
        margin: 0 5px;
      }
    }
  }
}
</style>