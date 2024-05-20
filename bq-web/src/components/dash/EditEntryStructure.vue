<script setup lang="ts">
import PushStackIcon from "../vector/PushStackIcon.vue"
import PopStackIcon from "../vector/PopStackIcon.vue"

defineProps<{
  entryType: string,
  push: () => void,
  pop: () => void
}>()
</script>

<template>
  <div class="container">
    <button class="push bubble" @click="push">
      <PushStackIcon />
      Push {{ entryType }}
    </button>
    <button class="pop bubble" @click="pop">
      <PopStackIcon />
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
    margin-left: 0;
  }
}

</style>