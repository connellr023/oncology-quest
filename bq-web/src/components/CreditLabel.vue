<template>
  <div>
    <span>&copy; <b><a target="_blank" href="https://github.com/connellr023">Connell Reffo</a></b> 2024</span>
  </div>
</template>

<style scoped lang="scss">
div {
  position: absolute;
  font-size: clamp(18px, 2lvw, 20px);
  bottom: 17px;

  a {
    color: inherit;
    text-decoration: none;

    &:hover {
      text-decoration: underline;
    }
  }
}
</style>