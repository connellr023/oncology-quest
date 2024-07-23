<script setup lang="ts">
import Spinner from "./vector/Spinner.vue"

defineEmits(["click"])
defineProps<{
  loading: boolean,
  text: string
}>()
</script>

<template>
  <button :disabled="loading" class="form-button std" type="submit" @click="$emit('click')">
    <span>
      <span :style="`opacity: ${loading ? 0 : 1};`">{{ text }}</span>
      <Spinner class="spinner" v-if="loading" />
    </span>
  </button>
</template>

<style scoped lang="scss">
button {
  & > span {
    display: flex;
    justify-content: center;
    align-items: center;

    & > span {
      flex-grow: 1;
    }

    & > svg {
      height: 22px;
      position: absolute;
    }
  }
}
</style>