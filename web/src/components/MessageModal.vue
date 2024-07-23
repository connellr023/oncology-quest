<script setup lang="ts">
import CheckIcon from "./vector/CheckIcon.vue"

const emits = defineEmits(["change"])
defineProps<{
  title: string,
  visible: boolean,
  error: string
}>()

const hide = () => {
  emits("change", false)
}
</script>

<template>
 <div class="modal" v-if="visible" @click.stop>
    <div class="modal-content">
      <h3>{{ title }}</h3>
      <p v-if="error" class="error">{{ error }}</p>
      <p v-else class="message">
        <slot name="message"></slot>
      </p>
      <div class="modal-buttons"> 
        <button class="bubble yellow" @click="hide">
          <span>
            <CheckIcon />
            <span>OK</span>
          </span>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
p.message {
  opacity: 0.7;
  padding: 10px;
}
</style>