<script setup lang="ts">
import EditEntryStructure from "./EditEntryStructure.vue"
import useEditEntryStructure from "../../hooks/useEditEntryStructure";

const { message, pushTaskHeading, popTaskHeading } = useEditEntryStructure()
</script>

<template>
    <EditEntryStructure :push="pushTaskHeading" :pop="popTaskHeading" entryType="Task Heading" />
    <div v-if="message">{{ message }}</div>
</template>