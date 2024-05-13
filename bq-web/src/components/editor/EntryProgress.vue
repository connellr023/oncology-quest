<script setup lang="ts">

</script>

<template>
  <div class="container">
    <div class="percentage">50%</div>
    <div class="progress-container">
      <div class="progress-bar" />
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
    width: 50%;
    height: 100%;
    background-color: $theme-color-green;
    border-radius: 5px;
  }
}
</style>