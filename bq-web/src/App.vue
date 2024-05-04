<script setup lang="ts">
import useApiConnection from "./hooks/useApiConnection"
import NoSession from "./components/NoSession.vue"
import { provide, ref } from "vue";
import { UserSession } from "./models/user";

let session = ref<UserSession | null>(null);
provide("session", session);

const { loading, connectionError } = useApiConnection(session)
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
      <div v-else>
        <h1>hi</h1>
      </div>
    </div>
  </main>
</template>

<style scoped>
</style>
