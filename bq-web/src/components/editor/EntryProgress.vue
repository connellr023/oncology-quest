<script setup lang="ts">
import { onMounted, ref } from "vue"

const props = defineProps<{
  progress: number;
}>();

enum ColorClasses {
  GREEN = "green",
  YELLOW = "yellow",
  RED = "red",
}

const barColorClass = ref(ColorClasses.GREEN);

onMounted(() => {
  if (props.progress < 50) {
    barColorClass.value = ColorClasses.RED;
  } else if (props.progress < 80) {
    barColorClass.value = ColorClasses.YELLOW;
  } else {
    barColorClass.value = ColorClasses.GREEN;
  }
})
</script>

<template>
  <div class="container">
    <div class="percentage">{{ Math.round(progress) }}%</div>
    <div class="progress-container">
      <div :class="`progress-bar ${barColorClass}`" :style="`width: ${progress}%`" />
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../main.scss";

div.container {
  flex-grow: 1;
  margin-left: 30px;
  margin-right: 15px;
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
}

div.progress-container {
  height: 8px;
  background-color: rgba(255, 255, 255, 0.04);
  border-radius: 5px;
  overflow: hidden;
  margin-top: 7px;
  filter: drop-shadow(0px 0px 4px rgba(255, 255, 255, 0.03));

  div.progress-bar {
    height: 100%;
    border-radius: 5px;

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