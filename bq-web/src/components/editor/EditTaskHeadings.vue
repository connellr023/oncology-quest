<script setup lang="ts">
import EditEntryStructure from "./EditEntryStructure.vue"
import useEditEntryStructure from "../../hooks/useEditEntryStructure";

const { message, pushTaskHeading, popTaskHeading } = useEditEntryStructure()
</script>

<template>
    <EditEntryStructure :push="() => pushTaskHeading('New Task Heading')" :pop="popTaskHeading" entryType="Task Heading" />
    <div v-if="message">{{ message }}</div>
</template>