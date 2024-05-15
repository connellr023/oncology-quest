<script setup lang="ts">
import EditEntryStructure from "./EditEntryStructure.vue"
import useEditEntryStructure from "../hooks/useEditEntryStructure";

const { message, pushTaskHeading, popTaskHeading } = useEditEntryStructure()
</script>

<template>
  <EditEntryStructure class="edit-task-heading-container" :push="() => pushTaskHeading('New Task Heading')" :pop="popTaskHeading" entryType="Task Heading" />
  <div v-if="message">{{ message }}</div>
</template>

<style scoped lang="scss">
div.edit-task-heading-container {
  margin-bottom: 30px;
}
</style>