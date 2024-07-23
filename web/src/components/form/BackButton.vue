<script setup lang="ts">
import IconButton from "../IconButton.vue"
import ExportIcon from "../vector/ExportIcon.vue"

defineProps<{
  onBack: () => void
}>()
</script>

<template>
  <button class="main-back" @click="onBack">Back</button>
  <IconButton
    firstClass="secondary-back"
    firstText="Back"
    @click="onBack"
  >
    <template #firstIcon>
      <ExportIcon />
    </template>
  </IconButton>
</template>

<style scoped lang="scss">
button.main-back {
  display: block;
}

button.secondary-back {
  display: none;
  position: fixed;
  top: 10px;
  left: 10px;
  width: min-content;
}

@media (max-height: 800px) {
  button.secondary-back {
    display: block;
  }

  button.main-back {
    display: none;
  }
}
</style>