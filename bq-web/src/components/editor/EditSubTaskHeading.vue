<script setup lang="ts">
import EditEntryStructure from "./EditEntryStructure.vue"
import useEditEntryStructure from "../../hooks/useEditEntryStructure";

defineProps<{ index: number }>()

const { message, pushSubTaskHeading, popSubTaskHeading } = useEditEntryStructure()
</script>

<template>
    <EditEntryStructure :push="() => pushSubTaskHeading('New Sub Task Heading', index)" :pop="() => popSubTaskHeading(index)" entryType="Sub Task Heading" />
    <div v-if="message">{{ message }}</div>
</template>