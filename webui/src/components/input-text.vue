<script setup lang="ts">
import { computed } from 'vue'

import FeatherIcon from './feather-icon.vue'

const props = defineProps<{
  type: 'text' | 'password' | 'email' | 'url'
  icon: string
  placeholder?: string
  clearable?: boolean
  readonly?: boolean
}>()

const inputText = defineModel<string>()

defineEmits(['submit'])

const showClear = computed(
  () =>
    props.clearable &&
    !props.readonly &&
    inputText.value != undefined &&
    inputText.value.length > 0,
)

const handleClear = () => {
  if (!props.clearable) return
  inputText.value = ''
}
</script>

<template>
  <label class="input-text" @keydown.enter="$emit('submit')">
    <FeatherIcon class="icon" :icon />
    <input v-model="inputText" :type :placeholder :readonly @keydown.esc="handleClear" />
    <button :disabled="!showClear" @click="handleClear">
      <FeatherIcon class="icon" icon="x" />
    </button>
  </label>
</template>

<style scoped>
.input-text {
  box-sizing: border-box;
  display: inline-flex;
  height: 2rem;
  text-wrap: nowrap;
  background-color: var(--color-bg);
  border: 1px solid var(--color-half);
  border-radius: 6px;

  &:focus-within {
    outline: 1px solid var(--color-fg);
  }

  & .icon {
    margin: auto 0;
    color: var(--color-half);
    padding: 0 0.5rem;
  }

  input {
    flex: 1;
    color: var(--color-fg);
    outline: none;
  }

  button {
    height: 100%;

    & > * {
      display: block;
      margin: auto;
    }

    &:hover {
      cursor: pointer;
    }

    &:disabled {
      display: none;
    }
  }
}
</style>
