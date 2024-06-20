<script setup lang="ts">
import { Ref, VNodeRef, inject, nextTick, onMounted, onUpdated, ref } from "vue"
import { UserTask } from "../../models/tasks"
import { User } from "../../models/user"
import { Rotation } from "../../models/rotation";

import useUserTasks from "../../hooks/useUserTasks";

import CheckIcon from "../vector/CheckIcon.vue"
import EntryHeading from "./EntryHeading.vue"
import useValidateComment from "../../hooks/validation/useValidateComment";

const props = defineProps<{
  saveHeading: (title: string) => Promise<boolean>,
  deleteHeading: () => Promise<boolean>,
  subtaskId: number,
  value: string,
  tasks: Record<number, UserTask>
}>()

const session = inject<Ref<User>>("session")!
const selectedRotation = inject<Ref<Rotation>>("selectedRotation")!

const isComplete = ref(false)
const isSaved = ref(true)

const { updateTask } = useUserTasks()
const { comment, commentError } = useValidateComment()

onMounted(() => {
  const task = props.tasks[props.subtaskId]

  if (task) {
    isComplete.value = task.isCompleted
    comment.value = task.comment
  }
})

const saveTask = async (): Promise<boolean> => {
  if (await updateTask(selectedRotation.value.id, props.subtaskId, isComplete.value, comment.value)) {
    isSaved.value = true
    return true
  }

  return false
}

const toggleCompleted = async () => {
  isComplete.value = !isComplete.value

  if (!await saveTask()) {
    isComplete.value = !isComplete.value
  }
}

const textArea = ref<VNodeRef | null>(null)

const adjustHeight = () => {
  textArea.value.style.height = "auto"
  textArea.value.style.height = textArea.value.scrollHeight + "px"
}

const onInput = () => {
  adjustHeight()
  isSaved.value = false
}

onUpdated(() => nextTick(adjustHeight))
</script>

<template>
  <li>
    <div class="container">
      <div class="task-heading-container">
        <EntryHeading :saveHeading="saveHeading" :deleteHeading="deleteHeading" class="subtask-entry" :title="value"/>
        <button :disabled="commentError ? true : false" v-if="!session.isAdmin" class="icon-button green" @click="saveTask">
          <CheckIcon />
          {{ isSaved ? "Saved" : "Not Saved" }}
        </button>
        <button @click.stop="toggleCompleted" :disabled="session.isAdmin" class="check" :class="`${isComplete ? 'active' : ''}`" />
      </div>
      <textarea :class="`bubble ${commentError ? 'error' : ''}`" v-show="(session.isAdmin && comment) || !session.isAdmin" :disabled="session.isAdmin" @input="onInput" ref="textArea" spellcheck="false" placeholder="Add a comment..." v-model="comment" :readonly="session.isAdmin"></textarea>
    </div>
  </li>
</template>

<style scoped lang="scss">
@import "../../styles/variables.scss";

li {
  margin-bottom: 5px;
}

button.check {
  $size: 13px;

  cursor: pointer;
  width: $size;
  height: $size;
  border: 3px solid $theme-color-green;
  border-radius: 100px;
  background-color: transparent;
  margin-left: auto;
  margin-top: 6px;
  transition: all 0.07s ease;
  opacity: 0.7;

  &.active,
  &:hover {
    opacity: 1;
  }

  &.active {
    background-color: $theme-color-green;
  }

  &:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }
}

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