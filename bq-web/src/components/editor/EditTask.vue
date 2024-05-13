<script setup lang="ts">
import { Ref, inject, onMounted, ref } from "vue"
import { UserTask } from "../../models/task";
import { UserSession } from "../../models/user";

import EntryHeading from "../EntryHeading.vue";
import useSaveTask from "../../hooks/useSaveTask";

const optionsVisible = ref(false)
const user = inject<Ref<UserSession>>("session")!.value.user

const {
  completed,
  comment,
  message,
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

const toggleCompleted = () => {
  completed.value = !completed.value
}
</script>

<template>
  <div class="container">
    <EntryHeading :is-active="optionsVisible" @click="toggleOptions" :index="index" :title="value"/>
    <div class="check-container" @click="toggleCompleted">
      <div :class="`completed ${completed ? 'active' : ''}`" />
      <div :class="`${!completed ? 'active' : ''}`" />
    </div>
  </div>
  <div class="options" v-show="optionsVisible">
    <textarea v-model="comment" :readonly="user.isAdmin"></textarea>
    <template v-if="!user.isAdmin">
      <button @click="save($props.index)">Save</button>
      <div v-if="message">{{ message }}</div>
    </template>
  </div>
</template>

<style scoped lang="scss">
@import "../../main.scss";

div.container {
  display: flex;
}

div.check-container {
  display: flex;
  justify-content: center;
  margin-top: 25px;
  margin-left: auto;
  margin-right: 15px;
  opacity: 0.8;
  transition: opacity 0.2s ease;
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
    height: 50px;
  }
}
</style>