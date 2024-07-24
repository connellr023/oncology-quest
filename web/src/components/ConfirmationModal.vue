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
 <div class="modal" v-if="visible" @click.stop>
    <div class="modal-content">
      <h3>{{ title }}</h3>
      <p>{{ description }}</p>
      <p v-if="error" class="error">{{ error }}</p>
      <div class="modal-buttons"> 
        <button class="bubble green" @click="onConfirm">
          <span>
            <CheckIcon />
            <span>Confirm</span>
          </span>
        </button>
        <button class="bubble red" @click="onCancel">
          <span>
            <CancelIcon />
            <span>Cancel</span>
          </span>
        </button>
      </div>
    </div>
  </div>
</template>