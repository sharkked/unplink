<script setup lang="ts">
import { RouterView } from 'vue-router'
import GlobalNav from './components/global-nav.vue'
</script>

<template>
  <GlobalNav />
  <RouterView />
  <footer>
    made by
    <a href="https://bsky.app/profile/evvil.town">wren</a> (c)2025 @<a href="https://evvil.town"
      >evvil.town</a
    >
  </footer>
</template>

<style scoped>
footer {
  position: fixed;
  right: 0;
  bottom: 0;
  padding: 0.5rem 1rem;
  color: var(--color-half);
}

footer a:hover {
  color: #ff008a;
}
</style>
