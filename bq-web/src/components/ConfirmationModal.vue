<script setup lang="ts">
import CheckIcon from "./vector/CheckIcon.vue"
import CancelIcon from "./vector/CancelIcon.vue"

defineProps<{
  title: string,
  visible: boolean,
  description: string,
  error: string,
  onConfirm: () => void,
  onCancel: () => void
}>()
</script>

<template>
 <div class="modal" v-if="visible">
    <div class="modal-content">
      <h3>{{ title }}</h3>
      <p>{{ description }}</p>
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