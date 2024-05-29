<script setup lang="ts">
import { Ref, VNodeRef, inject, onMounted, ref } from "vue"
import { UserTask } from "../../models/tasks"
import { User } from "../../models/user"

import useSaveTask from "../../hooks/useSaveTask"

import EntryHeading from "./EntryHeading.vue"

const props = defineProps<{
  saveTask: (title: string) => Promise<boolean>,
  deleteTask: () => Promise<boolean>,
  task?: UserTask,
  value: string
}>()

const user = inject<Ref<User>>("session")!.value

const {
  completed,
  comment,
  message,
  //loading,
} = useSaveTask()

onMounted(() => {
  if (props.task) {
    completed.value = props.task.isComplete
    comment.value = props.task.comment
  }
})

const toggleCompleted = async () => {
  completed.value = !completed.value

  // if (!await save(props.index)) {
  //   completed.value = !completed.value
  // }
}

const textArea = ref<VNodeRef | null>(null)

const adjustHeight = () => {
  textArea.value.style.height = "auto"
  textArea.value.style.height = textArea.value.scrollHeight + "px"
}

onMounted(adjustHeight)
</script>

<template>
  <div class="container">
    <div class="task-heading-container">
      <EntryHeading :saveHeading="saveTask" :deleteHeading="deleteTask" class="subtask-entry" :title="value"/>
      <button v-if="!user.isAdmin" class="minimal" @click="">Save</button>
      <div class="check-container" @click.stop="toggleCompleted">
        <div :class="`completed ${completed ? 'active' : ''}`" />
        <div :class="`${!completed ? 'active' : ''}`" />
      </div>
    </div>
    <textarea class="bubble" v-show="(user.isAdmin && comment) || !user.isAdmin" :disabled="user.isAdmin" @input="adjustHeight" ref="textArea" spellcheck="false" placeholder="Add a comment..." v-model="comment" :readonly="user.isAdmin"></textarea>
    <span v-if="message">{{ message }}</span>
  </div>
</template>

<style scoped lang="scss">
@import "../../main.scss";

div.subtask-entry {
  &::before {
    $size: 10px;

    content: " ";
    background-color: $theme-color-1;
    width: $size;
    height: $size;
    border-radius: 100px;
    display: inline-block;
    margin-right: 10px;
  }
}

div.task-heading-container {
  display: flex;
}

div.container {
  padding: 6px 0 6px 15px;
  margin-right: 13px;
  margin-top: 10px;
  border-radius: 8px;
  transition: background-color 0.1s ease;
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
  margin-top: 3px;
  margin-left: auto;
  opacity: 0.8;
  transition: opacity 0.3s ease;
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
</style>