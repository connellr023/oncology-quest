<script setup lang="ts">
import { ref } from "vue";
import useUserSearch from "../hooks/useUserSearch"

const { search, results, loading, searchError } = useUserSearch()
const query = ref("")

const searchUser = () => {
  search(query.value)
}
</script>

<template>
  <h1>Admin Dashboard</h1>
  <div>
    <input v-model="query" placeholder="Search Username" />
    <button @click="searchUser">Search</button>
    <div>
      <ul v-if="results.length > 0">
        <li v-for="(user, index) in results" :key="index">
          {{ user.username }} ({{ user.name }})
        </li>
      </ul>
      <div v-else-if="loading">
        <h2>Loading...</h2>
      </div>
      <div v-else-if="searchError">
        <h2>Failed to fetch users</h2>
      </div>
      <div v-else>
        <h2>No results</h2>
      </div>
    </div>
  </div>
</template>