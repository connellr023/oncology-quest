<script setup lang="ts">
import EditEntryStructure from "./EditEntryStructure.vue"
import useEditEntryStructure from "../../hooks/useEditEntryStructure";

defineProps<{ index: number[] }>()

const { message, pushSubTaskEntry, popSubTaskEntry } = useEditEntryStructure()
</script>

<template>
    <EditEntryStructure :push="() => pushSubTaskEntry('New Sub Task Entry', index)" :pop="() => popSubTaskEntry(index)" entryType="Sub Task Entry" />
    <div v-if="message">{{ message }}</div>
</template>