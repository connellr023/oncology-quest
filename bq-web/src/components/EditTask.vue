<script setup lang="ts">
import { Ref, inject, onMounted, ref } from "vue"
import { UserTask } from "../models/task";
import { User } from "../models/user";

import useSaveTask from "../hooks/useSaveTask";

import EntryHeading from "./EntryHeading.vue";
import LoadingButton from "./LoadingButton.vue";

const optionsVisible = ref(false)
const user = inject<Ref<User>>("session")!.value

const {
  completed,
  comment,
  message,
  loading,
  save
} = useSaveTask()

const props = defineProps<{
  task?: UserTask,
  value: string,
  index: [number, number, number]
}>()

onMounted(() => {
  if (props.task) {
    completed.value = props.task.completed
    comment.value = props.task.comment
  }
})

const toggleOptions = () => {
  optionsVisible.value = !optionsVisible.value
}

const toggleCompleted = async () => {
  completed.value = !completed.value

  if (!await save(props.index)) {
    completed.value = !completed.value
  }
}
</script>

<template>
  <div :class="`container ${optionsVisible ? 'focused' : ''}`" @click="toggleOptions">
    <div class="task-heading-container">
      <EntryHeading :is-active="optionsVisible" :index="index" :title="value"/>
      <div class="check-container" @click.stop="toggleCompleted">
        <div :class="`completed ${completed ? 'active' : ''}`" />
        <div :class="`${!completed ? 'active' : ''}`" />
      </div>
    </div>
    <div @click.stop class="options" v-show="optionsVisible">
      <textarea spellcheck="false" placeholder="Add a comment..." v-model="comment" :readonly="user.isAdmin"></textarea>
      <br />
      <div class="save-container" v-if="!user.isAdmin">
        <LoadingButton :loading="loading" @click="save(index)" text="Save" />
        <span v-if="message">{{ message }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../main.scss";

div.task-heading-container {
  display: flex;
}

div.container {
  cursor: pointer;
  padding: 6px 0 6px 15px;
  margin-right: 13px;
  margin-top: 10px;
  border-radius: 8px;
  transition: background-color 0.1s ease;
  
  &.focused,
  &:hover {
    background-color: $secondary-bg-color;
  }
}

div.save-container {
  width: 100%;
  display: flex;
  align-items: center;

  span {
    margin-left: 15px;
    font-size: clamp(17px, 1.5vw, 20px);
  }
}

div.check-container {
  display: flex;
  justify-content: center;
  margin-top: 13px;
  margin-left: auto;
  margin-right: 15px;
  opacity: 0.8;
  transition: opacity 0.3s ease;
  filter: drop-shadow(0px 0px 6px rgba(255, 255, 255, 0.06));
  cursor: pointer;

  &:hover {
    opacity: 1;
  }

  div {
    $size: 13px;

    width: $size;
    height: $size;
    border-radius: 50%;
    background-color: #ffffff;
    opacity: 0.7;
    margin: 0 4px;
    transition: all 0.2s ease;
  }

  div.active {
    opacity: 1;
    border-radius: 8px;
    width: 30px;
  }

  div.completed {
    background-color: $theme-color-green;
  }
}

div.options {
  margin-top: 10px;

  label {
    display: block;
  }

  textarea {
    $top-border-radius: 10px;

    border: none;
    border-top-left-radius: $top-border-radius;
    border-top-right-radius: $top-border-radius;
    border-bottom: 2px solid $main-txt-color;
    color: $main-txt-color;
    font-family: $main-font;
    font-size: clamp(17px, 1.5vw, 22px);
    height: 80px;
    width: calc(100% - 30px);
    resize: none;
    background-color: transparent;
    outline: none;
    padding: 12px;
    transition: background-color 0.1s ease;

    &:focus {
      background-color: $main-bg-color;
    }
  }

  button {
    font-size: clamp(15px, 1.5vw, 19px);
    margin-top: 10px;
    margin-bottom: 13px;
    width: 110px;
  }
}
</style>