<template>
  <div>
    <span>&copy; <b><a target="_blank" href="https://github.com/connellr023">Connell Reffo</a></b> 2024</span>
  </div>
</template>

<style scoped lang="scss">
div {
  position: absolute;
  font-size: clamp(16px, 2lvw, 18px);
  bottom: 17px;
  animation: fade-in 1.5s ease-out;

  a {
    color: inherit;
    text-decoration: none;

    &:hover {
      text-decoration: underline;
    }
  }
}
</style>