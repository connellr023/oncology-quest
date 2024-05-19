<script setup lang="ts">
defineProps<{
  entryType: string,
  push: () => void,
  pop: () => void
}>()
</script>

<template>
  <div class="container">
    <button class="push bubble" @click="push">
      <img src="/push-stack.svg" alt="push" />
      Push {{ entryType }}
    </button>
    <button class="pop bubble" @click="pop">
      <img src="/pop-stack.svg" alt="pop" />
      Pop {{ entryType }}
    </button>
  </div>
</template>

<style scoped lang="scss">
div.container {
  display: flex;
  margin-top: 5px;
  margin-bottom: 7px;

  button {
    margin: 4px;
  }
}

</style>