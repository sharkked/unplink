<script setup lang="ts">
import ThemeSelector from './theme-selector.vue'
import FeatherIcon from './feather-icon.vue'
</script>

<template>
  <nav class="global-nav">
    <ThemeSelector />
    <RouterLink to="/">
      <FeatherIcon icon="home" />
    </RouterLink>
    <RouterLink to="/about">
      <FeatherIcon icon="info" />
    </RouterLink>
  </nav>
</template>

<style>
.global-nav {
  position: absolute;
  display: flex;
  flex-direction: column;
  padding: 0.5rem;
  gap: 0.5rem;

  a {
    color: var(--color-half);

    &.router-link-active {
      color: var(--color-fg);
    }
  }
}
</style>
