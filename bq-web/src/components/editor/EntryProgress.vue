<script setup lang="ts">

</script>

<template>
  <div class="container">
    <span class="percentage">50%</span>
    <div class="progress-container">
      <div class="progress-bar" />
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../main.scss";

div.container {
  font-style: italic;
  position: absolute;
  right: 20px;
  top: -5px;
  width: 40%;
  max-width: 600px;
  min-width: 110px;
}

div.progress-container {
  height: 10px;
  background-color: rgba(255, 255, 255, 0.04);
  border-radius: 5px;
  overflow: hidden;
  margin-top: 10px;
  filter: drop-shadow(0px 0px 4px rgba(255, 255, 255, 0.03));

  div.progress-bar {
    width: 50%;
    height: 100%;
    background-color: $theme-color-green;
    border-radius: 5px;
  }
}
</style>