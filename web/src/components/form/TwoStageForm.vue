<script setup lang="ts">
import LoadingButton from "../LoadingButton.vue"
import BackButton from "./BackButton.vue"

const emits = defineEmits(["update-stage"])

const props = defineProps<{
  onBack: () => void,
  handleSubmit: () => void,
  submitButtonText: string,
  inStageOne: boolean,
  canGotoStageTwo: boolean,
  error: string,
  loading: boolean
}>()

const switchStage = () => {
  emits("update-stage", !props.inStageOne)
}
</script>

<template>
  <h1>
    <slot name="title"></slot>
  </h1>
  <div class="stage-indicator-container">
    <div :class="`${inStageOne ? 'active' : ''}`" />
    <div :class="`${!inStageOne ? 'active' : ''}`" />
  </div>
  <form @submit.prevent="handleSubmit" @keydown.enter.prevent>
    <div class="stage-container" v-if="inStageOne">
      <slot name="stage-one"></slot>
      <button :disabled="!canGotoStageTwo" class="form-button std" @click="switchStage">Next Step</button>
    </div>
    <div class="stage-container" v-else>
      <slot name="stage-two"></slot>
      <div>
        <div class="error-label" v-if="error">{{ error }}</div>
      </div>
      <LoadingButton :loading="loading" :text="submitButtonText" />
      <button class="prev std" @click="switchStage">Previous Step</button>
    </div>
    <BackButton :onBack="onBack" />
  </form>
</template>

<style scoped lang="scss">
div.stage-indicator-container {
  display: flex;
  justify-content: center;
  margin-top: 5px;
  margin-bottom: 15px;

  div {
    $size: 12px;

    width: $size;
    height: $size;
    border-radius: 50%;
    background-color: #ffffff;
    opacity: 0.7;
    margin: 0 5px;
    transition: all 0.3s ease;
  }

  div.active {
    opacity: 1;
    border-radius: 8px;
    width: 45px;
  }
}

button.prev {
  margin-top: -4px;
  margin-bottom: 7px;
}
</style>