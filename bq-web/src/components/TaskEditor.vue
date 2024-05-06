<script setup lang="ts">
import { Ref, defineProps, inject, onMounted, ref } from "vue"
import { UserTask } from "../models/task";
import { UserSession } from "../models/user";

import EntryHeading from "./EntryHeading.vue";
import useSaveTask from "../hooks/useSaveTask";

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
</script>

<template>
  <EntryHeading @click="toggleOptions" :index="index" :title="value"/>
  <div class="options" v-show="optionsVisible">
    <label>Completed</label>
    <input type="checkbox" v-model="completed" :disabled="user.isAdmin" />
    <br />
    <textarea v-model="comment" :readonly="user.isAdmin"></textarea>
    <br />
    <template v-if="!user.isAdmin">
      <button @click="save($props.index)">Save</button>
      <div v-if="message">{{ message }}</div>
    </template>
  </div>
</template>

<style scoped lang="scss">
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