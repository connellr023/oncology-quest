<script setup lang="ts">
import { defineProps, onMounted, ref } from "vue"
import { UserTask } from "../models/task";
import { API_ENDPOINT } from "../utilities";

const optionsVisible = ref(false)
const completed = ref(false)
const comment = ref("")
const message = ref("")

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

const save = async () => {
  message.value = "Saving..."

  try {
    const task: UserTask = {
      completed: completed.value,
      comment: comment.value
    }

    const response = await fetch(`${API_ENDPOINT}/api/tasks/update`, {
      method: "PATCH",
      credentials: "include",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        task,
        index: props.index
      })
    })

    if (response.ok) {
      message.value = "Saved!"
    }
    else {
      message.value = `Server responded with ${response.status}.`
    }
  }
  catch (error) {
    message.value = "Failed to save task."
  }
}
</script>

<template>
  <div class="task-name" @click="toggleOptions">{{ value }}</div>
  <div class="options" v-show="optionsVisible">
    <label>Completed</label>
    <input type="checkbox" v-model="completed" />
    <br />
    <textarea v-model="comment"></textarea>
    <br />
    <button @click="save">Save</button>
    <div v-if="message">{{ message }}</div>
  </div>
</template>

<style scoped lang="scss">
div.task-name {
  cursor: pointer;

  &:hover {
    text-decoration: underline;
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