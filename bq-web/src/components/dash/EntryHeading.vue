<script setup lang="ts">
import { Ref, inject, ref } from "vue"
import { User } from "../../models/user"
import useValidateTitle from "../../hooks/validation/useValidateTitle";

defineEmits(["click"])
const props = defineProps<{
  save: (saveTitle: string) => Promise<boolean>,
  title: string
}>()

const session = inject<Ref<User>>("session")!
const isEditing = inject<Ref<boolean>>("isEditing")!

const { title, titleError } = useValidateTitle()

title.value = props.title

const inEditMode = ref(false)
let savedTitle = ""

const toggleEditMode = () => {
  inEditMode.value = !inEditMode.value
  savedTitle = title.value

  if (inEditMode.value) {
    isEditing.value = true
  } else {
    isEditing.value = false
  }
}

const cancelEdit = () => {
  title.value = savedTitle
  toggleEditMode()
}

const saveEdit = async () => {
  if (titleError.value) {
    return
  }

  if (await props.save(title.value)) {
    toggleEditMode()
  }
}
</script>

<template>
  <div class="entry-heading-container" @click="$emit('click')">
    <div class="header">
      <h3 v-if="!inEditMode" class="entry-heading">{{ title }}</h3>
      <input @click.stop class="bubble minimal" v-else v-model="title" />
    </div>
    <div class="edit-buttons-container" v-if="session.isAdmin">
      <template v-if="inEditMode">
        <button class="minimal" @click.stop="cancelEdit">Cancel</button>
        <button class="minimal" @click.stop="saveEdit">Save</button>
        <button class="minimal" @click.stop="">Delete</button>
        <span v-if="titleError">{{ titleError }}</span>
      </template>
      <button class="edit minimal" v-else :disabled="isEditing" @click.stop="toggleEditMode">Edit</button>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../main.scss";

h3.entry-heading {
  font-size: clamp(14px, 1.5lvw, 17px);
  font-weight: bold;
  color: $main-txt-color;
  display: inline-block;
  padding-right: 15px;
  margin: 0;
}

div.header {
  display: flex;
  align-items: center;
  padding: 8px;
  padding-left: 0;
}

div.entry-heading-container {
  display: flex;
  align-items: center;
}

div.edit-buttons-container {
  span {
    margin-left: 15px;
  }

  button {
    margin: auto;
    margin-left: 10px;

    &.edit {
      margin-right: 15px;
      margin-left: auto;
    }
  }
}
</style>