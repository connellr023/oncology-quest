<script setup lang="ts">
import { Ref, inject } from "vue"
import { Rotation } from "../../models/rotation"

import useEntries from "../../hooks/useEntries"

import CheckIcon from "../vector/CheckIcon.vue"

const rotations = inject<Ref<Record<number, Rotation>>>("rotations")!
const selectedRotation = inject<Ref<Rotation | null>>("selectedRotation")!

const { fetchEntriesWithCaching } = useEntries()

const selectRotation = async (rotation: Rotation) => {
  if (!await fetchEntriesWithCaching(rotation.id)) {
    console.error("Failed to fetch entries.")
    return
  }

  selectedRotation.value = rotation
}

const unselectRotation = () => {
  selectedRotation.value = null
}
</script>

<template>
  <div class="rotation-select-container">
    <h1 class="section-heading">Rotations</h1>
    <div class="rotations" v-if="Object.keys(rotations).length > 0">
      <button
        v-for="rotation in rotations"
        :class="`rotation bubble ${selectedRotation?.id === rotation.id ? 'focused' : ''}`"
        :key="rotation.id"
        @click="() => { if (selectedRotation?.id === rotation.id) { unselectRotation() } else { selectRotation(rotation) } }"
      >
        <CheckIcon v-show="selectedRotation?.id === rotation.id" />
        {{ rotation.name }}
      </button>
    </div>
    <p class="no-rotations" v-else>No Rotations Available.</p>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/variables.scss";

p.no-rotations {
  text-align: center;
  font-size: clamp(18px, 1.3lvw, 23px);
  opacity: 0.7;
  padding: 8px;
}

div.rotations {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  gap: 10px;

  button.rotation {
    background-color: $secondary-bg-color;
    flex: 1 0 auto;
    min-width: 130px;
    height: 50px;

    &:hover {
      background-color: $tertiary-bg-color;
    }

    &.focused {
      background-color: $tertiary-bg-color;
      color: $theme-color-green;
      
      svg {
        fill: $theme-color-green;
      }
    }
  }
}
</style>