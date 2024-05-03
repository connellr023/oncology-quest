<script setup lang="ts">
import useApiConnection from "./hooks/useApiConnection"
import NoSession from "./components/NoSession.vue"

const { session, loading, connectionError } = useApiConnection()
</script>

<template>
  <main>
    <div v-if="loading" id="loading-wrapper">
      <h1>Loading...</h1>
    </div>
    <div v-else-if="connectionError" id="connection-error-wrapper">
      <h1>Connection Error</h1>
    </div>
    <div v-else>
      <NoSession v-if="!session" />
    </div>
  </main>
</template>

<style scoped>
</style>
