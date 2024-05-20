<script setup lang="ts">
import EditEntryStructure from "./EditEntryStructure.vue"
import useEditEntryStructure from "../../hooks/useEditEntryStructure"

const { message, pushSupertaskHeading, popSupertaskHeading } = useEditEntryStructure()
</script>

<template>
  <EditEntryStructure class="edit-task-heading-container" :push="() => pushSupertaskHeading('New Supertask Heading')" :pop="popSupertaskHeading" entryType="Supertask Heading" />
  <div v-if="message">{{ message }}</div>
</template>

<style scoped lang="scss">
div.edit-task-heading-container {
  margin-bottom: 30px;
}
</style>