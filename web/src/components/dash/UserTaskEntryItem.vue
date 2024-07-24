<script setup lang="ts">
import { Ref, VNodeRef, inject, nextTick, onMounted, onUpdated, ref } from "vue"
import { UserTask } from "../../models/tasks"
import { User } from "../../models/user"
import { Rotation } from "../../models/rotation"

import useUserTasks from "../../hooks/useUserTasks"
import useValidateComment from "../../hooks/validation/useValidateComment"

import CheckIcon from "../vector/CheckIcon.vue"
import CancelIcon from "../vector/CancelIcon.vue"
import EntryHeading from "./EntryHeading.vue"
import IconButton from "../IconButton.vue"

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
      <div class="task-entry-container">
        <div class="spacer">
          <EntryHeading :saveHeading="saveHeading" :deleteHeading="deleteHeading" class="subtask-entry" :title="value"/>
        </div>
        <IconButton
          v-if="!session.isAdmin"
          :disabled="isSaved || commentError.length > 0"
          :isToggled="!isSaved"
          firstClass="green"
          firstText="Saved"
          secondClass="yellow"
          secondText="Not Saved"
          @click.stop="saveTask"
        >
          <template #firstIcon>
            <CheckIcon />
          </template>
          <template #secondIcon>
            <CheckIcon />
          </template>
        </IconButton>
        <IconButton
          class="check"
          :disabled="session.isAdmin || commentError.length > 0"
          :isToggled="!isComplete"
          firstClass="green"
          firstText="Complete"
          secondClass="red"
          secondText="Working"
          @click.stop="toggleCompleted"
        >
          <template #firstIcon>
            <CheckIcon />
          </template>
          <template #secondIcon>
            <CancelIcon />
          </template>
        </IconButton>
      </div>
      <div class="comment-container">
        <textarea :class="`bubble ${commentError ? 'error' : ''}`" v-show="(session.isAdmin && comment) || !session.isAdmin" :disabled="session.isAdmin" @input="onInput" ref="textArea" spellcheck="false" placeholder="Add a comment..." v-model="comment" :readonly="session.isAdmin"></textarea>
      </div>
    </div>
  </li>
</template>

<style scoped lang="scss">
@import "../../styles/variables.scss";

li {
  margin-bottom: 5px;
}

div.comment-container {
  display: flex;
  align-items: center;
  margin-top: 10px;
  width: 100%;

  textarea {
    width: 100%;
  }
}


div.subtask-entry {
  position: relative;

  &::before {
    $size: 10px;

    content: " ";
    background-color: $theme-color-1;
    width: $size;
    height: $size;
    border-radius: 100px;
    display: inline-block;
    left: -20px;
    position: absolute;
  }
}

div.container {
  padding: 6px 15px 6px 15px;
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

div.task-entry-container {
  display: flex;
  flex-direction: row;
  margin-left: 25px;

  button.check {
    margin-left: 5px;
  }
}
</style>
