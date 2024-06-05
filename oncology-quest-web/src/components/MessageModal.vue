<script setup lang="ts">
import CheckIcon from "./vector/CheckIcon.vue"

const emits = defineEmits(["change"])
defineProps<{
  title: string,
  visible: boolean,
  error: string,
  message: string
}>()

const hide = () => {
  emits("change", false)
}
</script>

<template>
 <div class="modal" v-if="visible">
    <div class="modal-content">
      <h3>{{ title }}</h3>
      <p v-if="error" class="error">{{ error }}</p>
      <p v-else class="message">{{ message }}</p>
      <div class="modal-buttons"> 
        <button class="bubble yellow" @click="hide">
          <CheckIcon />
          Ok
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
p.message {
  opacity: 0.7;
}
</style>