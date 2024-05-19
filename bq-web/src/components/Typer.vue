<script setup lang="ts">
import { onMounted, ref } from "vue"

const props = defineProps<{
  step: number,
  value: string
}>()

const current = ref("")

onMounted(() => {
  let i = 1

  const interval = setInterval(() => {
    current.value = props.value.slice(0, i)
    i++

    if (props.value[i] === " ") {
      i++
    }
    else if (i > props.value.length) {
      clearInterval(interval)
    }
  }, props.step)
})
</script>

<template>
    <span>{{ current }}</span>
</template>