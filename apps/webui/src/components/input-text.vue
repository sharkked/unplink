<script setup lang="ts">
import { computed } from 'vue'

import FeatherIcon from './feather-icon.vue'

const props = defineProps<{
  type: 'text' | 'password' | 'email' | 'url'
  icon: string
  placeholder?: string
  clearable?: boolean
}>()

const modelValue = defineModel<string>()

const showClear = computed(
  () => !props.clearable && modelValue.value && modelValue.value.length > 0,
)
</script>

<template>
  <label class="input-text">
    <FeatherIcon class="icon" :icon />
    <input v-model="modelValue" :type :placeholder @keydown.esc="modelValue = ''" />
    <button :disabled="!showClear" @click="modelValue = ''">
      <FeatherIcon class="icon" icon="x" />
    </button>
  </label>
</template>

<style scoped>
.input-text {
  box-sizing: border-box;
  display: inline-flex;
  height: 2rem;
  padding: 0 0.5rem;
  gap: 0.5rem;
  text-wrap: nowrap;
  background-color: var(--color-bg);
  border: 1px solid var(--color-half);
  border-radius: 6px;

  &:focus-within {
    outline: 1px solid var(--color-fg);
  }

  & > * {
    margin: auto 0;
  }

  input {
    flex: 1;
    outline: none;

    &::placeholder {
      color: var(--color-half);
    }
  }

  .icon {
    color: var(--color-half);
  }

  button:disabled {
    visibility: hidden;
  }
}
</style>
