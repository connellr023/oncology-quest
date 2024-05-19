<script setup lang="ts">
defineProps<{
  entryType: string,
  push: () => void,
  pop: () => void
}>()
</script>

<template>
  <div class="container">
    <button class="push minimal" @click="push">Push {{ entryType }}</button>
    <button class="pop minimal" @click="pop">Pop {{ entryType }}</button>
  </div>
</template>

<style scoped lang="scss">
div.container {
  display: flex;
  margin-top: 10px;
  margin-bottom: 7px;

  button {
    margin: 4px;
  }
}

</style>