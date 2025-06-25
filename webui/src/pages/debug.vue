<script setup lang="ts">
import { useGlobalState } from "@/hooks/useGlobalState";

const state = useGlobalState();
</script>

<template>
  <main>
    <h1>state</h1>
    <code>{{ state }}</code>
  </main>
</template>

<style>
code {
  display: block;
  padding: 0.5rem 1rem;
  white-space: pre-wrap;
  border: 1px solid var(--color-half);
  border-radius: var(--radius);
}
</style>
