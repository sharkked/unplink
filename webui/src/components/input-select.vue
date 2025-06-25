<script setup lang="ts">
import { RadioGroupItem, RadioGroupRoot } from "reka-ui";

defineProps<{
  options: {
    label: string;
    value: string | number;
  }[];
}>();
</script>

<template>
  <RadioGroupRoot class="input-select">
    <RadioGroupItem
      v-for="(option, index) in options"
      :key="index"
      :value="option.value"
    >
      {{ option.label }}
    </RadioGroupItem>
  </RadioGroupRoot>
</template>

<style>
.input-select {
  display: flex;
  gap: 1px;
  background-color: var(--color-on);
  border: 1px solid var(--color-on);
  border-radius: var(--radius);
  overflow: clip;

  button {
    flex: 1;
    height: 1.5rem;
    color: var(--color-on);
    background-color: var(--color-off);
    cursor: pointer;

    &[data-state="checked"] {
      color: var(--color-off);
      background-color: var(--color-on);
    }
  }
}
</style>
