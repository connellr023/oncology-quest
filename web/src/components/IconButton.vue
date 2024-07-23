<script setup lang="ts">
defineProps<{
  isToggled?: boolean,
  firstText?: string,
  firstClass?: string,
  secondText?: string,
  secondClass?: string
}>()
</script>

<template>
  <button :class="`icon-button bubble solid ${!isToggled ? firstClass : secondClass}`">
    <span>
      <template v-if="!isToggled">
        <slot name="firstIcon"></slot>
        <span v-if="firstText">{{ firstText }}</span>
      </template>
      <template v-else>
        <slot name="secondIcon"></slot>
        <span v-if="secondText">{{ secondText }}</span>
      </template>
    </span>
  </button>
</template>
 
<style scoped lang="scss">
@import "../styles/variables.scss";

button.icon-button {
  background: none;
  height: min-content;
  padding: 7px;
  border-radius: 100px;

  $bg-opacity: 0.1;

  svg {
    opacity: 0.7;
  }

  &:hover {
    background-color: rgba($main-txt-color, $bg-opacity);

    svg {
      opacity: 1;
      fill: $main-txt-color;
    }
  }

  &.green {
    &:hover {
      background-color: rgba($theme-color-green, $bg-opacity);
    }
  }

  &.red {
    &:hover {
      background-color: rgba($theme-color-red, $bg-opacity);
    }
  }

  &.yellow {
    &:hover {
      background-color: rgba($theme-color-yellow, $bg-opacity);
    }
  }

  &:disabled {
    opacity: 0.25;
  }
}
</style>