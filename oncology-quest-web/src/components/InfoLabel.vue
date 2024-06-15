<template>
  <div>
    <span>&copy; <b><a target="_blank" href="https://github.com/connellr023">Connell Reffo</a></b> 2024</span>
    <span class="divider">&verbar;</span>
    <span>Found an issue? Report it to the project repository <b><a href="https://github.com/connellr023/oncology-quest" target="_blank">here</a></b></span>
  </div>
</template>

<style scoped lang="scss">
div {
  position: absolute;
  font-size: clamp(16px, 2lvw, 18px);
  bottom: 17px;
  animation: fade-in 1.5s ease-out;
  opacity: 0.7;

  span.divider {
    $side-margin: 10px;

    font-weight: bold;
    margin-left: $side-margin;
    margin-right: $side-margin;
  }

  a {
    color: inherit;
    text-decoration: none;

    &:hover {
      text-decoration: underline;
    }
  }
}

@media (max-height: 750px) {
  div {
    display: none;
  }
}
</style>