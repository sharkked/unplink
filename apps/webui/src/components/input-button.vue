<script setup lang="ts">
import FeatherIcon from './feather-icon.vue'

defineProps<{ icon: string }>()
</script>

<template>
  <button class="input-button">
    <FeatherIcon :icon />
    <span>
      <slot></slot>
    </span>
  </button>
</template>

<style scoped>
.input-button {
  box-sizing: border-box;
  display: inline-flex;
  height: 2rem;
  padding: 0 0.5rem;
  gap: 0.5rem;
  text-wrap: nowrap;
  color: var(--color-fg);
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

  button {
    &:disabled {
      visibility: hidden;
    }

    &:not([disabled]):hover {
      cursor: pointer;
    }
  }
}
</style>
