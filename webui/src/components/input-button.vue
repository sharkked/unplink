<script setup lang="ts">
import FeatherIcon from "./feather-icon.vue";

defineProps<{ icon: string }>();
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
  display: flex;
  height: 2rem;
  padding: 0 0.5rem;
  gap: 0.5rem;
  text-wrap: nowrap;
  color: var(--color-on);
  background-color: var(--color-off);
  border: 1px solid var(--color-on);
  border-radius: var(--radius);

  &:focus-within {
    outline: 1px solid var(--color-on);
  }

  & > * {
    margin: auto 0;
  }

  &:disabled {
    color: var(--color-half);
    border-color: var(--color-half);
  }

  &:not(:disabled):hover {
    cursor: pointer;
  }
}
</style>
