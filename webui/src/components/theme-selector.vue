<script setup lang="ts">
import { ref } from 'vue'
import { ToggleGroupItem, ToggleGroupRoot } from 'reka-ui'

const currentTheme = ref(localStorage.theme)

const setTheme = (theme: string) => {
  localStorage.theme = theme
  document.documentElement.dataset.theme = theme
  currentTheme.value = theme
}
</script>

<template>
  <ToggleGroupRoot
    class="theme-selector"
    :model-value="currentTheme"
    @update:model-value="(value) => setTheme(value as string)"
  >
    <ToggleGroupItem value="system">System</ToggleGroupItem>
    <ToggleGroupItem value="light">Light</ToggleGroupItem>
    <ToggleGroupItem value="dark">Dark</ToggleGroupItem>
  </ToggleGroupRoot>
</template>

<style>
.theme-selector {
  display: flex;
  width: 18rem;
  gap: 1px;
  background-color: var(--color-fg);
  border: 1px solid var(--color-fg);
  border-radius: 6px;
  overflow: clip;

  button {
    flex: 1;
    height: 1.5rem;
    color: var(--color-fg);
    background-color: var(--color-bg);
    cursor: pointer;

    &[data-state='on'] {
      color: var(--color-bg);
      background-color: var(--color-fg);
    }
  }
}
</style>
