<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue"

const props = defineProps<{ progress: number }>();

enum ColorClasses {
  GREEN = "green",
  YELLOW = "yellow",
  RED = "red",
}

const computeColorClass = () => {
  if (props.progress < 50) {
    barColorClass.value = ColorClasses.RED;
  } else if (props.progress < 80) {
    barColorClass.value = ColorClasses.YELLOW;
  } else {
    barColorClass.value = ColorClasses.GREEN;
  }
}

const barColorClass = ref(ColorClasses.GREEN)
const progress = ref(props.progress)
const roundedProgress = computed(() => Math.round(props.progress))

watch(() => props.progress, (newProgress) => {
  computeColorClass()
  progress.value = newProgress
})

onMounted(computeColorClass)
</script>

<template>
  <div class="container">
    <div class="percentage">{{ roundedProgress }}%</div>
    <div class="progress-container">
      <div :class="barColorClass" class="progress-bar" :style="{ width: `${roundedProgress}%` }" />
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/variables.scss";

div.container {
  flex-grow: 1;
  margin-left: 30px;
  margin-left: auto;
  margin-top: 9px;
  max-width: 300px;
}

div.percentage {
  user-select: none;
  font-size: 16px;
  font-style: italic;
  color: $main-txt-color;
  text-align: center;
  margin-top: -3px;
  margin-bottom: 5px;
}

div.progress-container {
  height: 6px;
  background-color: rgba(255, 255, 255, 0.04);
  border-radius: 5px;
  overflow: hidden;
  margin-bottom: 10px;

  div.progress-bar {
    height: 100%;
    width: 100%;
    border-radius: 5px;
    background-color: #ffffff;
    transition: all 0.55s ease;

    &.green {
      background-color: $theme-color-green;
    }

    &.yellow {
      background-color: $theme-color-yellow;
    }

    &.red {
      background-color: $theme-color-red;
    }
  }
}
</style>