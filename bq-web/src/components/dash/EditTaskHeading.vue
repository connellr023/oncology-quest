<script setup lang="ts">
import EditEntryStructure from "./EditEntryStructure.vue"
import useEditEntryStructure from "../../hooks/useEditEntryStructure";

defineProps<{ index: number }>()

const { message, pushTaskHeading, popTaskHeading } = useEditEntryStructure()
</script>

<template>
    <EditEntryStructure :push="() => pushTaskHeading('New Task Heading', index)" :pop="() => popTaskHeading(index)" entryType="Task Heading" />
    <div v-if="message">{{ message }}</div>
</template>