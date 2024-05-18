<script setup lang="ts">
import EditEntryStructure from "./EditEntryStructure.vue"
import useEditEntryStructure from "../hooks/useEditEntryStructure";

defineProps<{ index: [number, number] }>()

const { message, pushSubtaskEntry, popSubtaskEntry } = useEditEntryStructure()
</script>

<template>
    <EditEntryStructure :push="() => pushSubtaskEntry('New Sub Task Entry', index)" :pop="() => popSubtaskEntry(index)" entryType="Sub Task Entry" />
    <div v-if="message">{{ message }}</div>
</template>