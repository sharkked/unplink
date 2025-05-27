<script setup lang="ts">
import { RouterView } from 'vue-router'
</script>

<template>
  <nav>
    <RouterLink to="/">
      <svg class="feather">
        <use href="./assets/feather-sprite.svg#home" />
      </svg>
    </RouterLink>
    <RouterLink to="/about">
      <svg class="feather">
        <use href="./assets/feather-sprite.svg#info" />
      </svg>
    </RouterLink>
  </nav>
  <RouterView />
  <footer>
    made by
    <a href="https://bsky.app/profile/evvil.town">wren</a> (c)2025 @<a href="https://evvil.town"
      >evvil.town</a
    >
  </footer>
</template>

<style>
@reference "./assets/styles.css";

nav {
  @apply absolute flex flex-col p-2 gap-2;
}

nav a {
  @apply opacity-50;
}

nav a.router-link-active {
  @apply opacity-100;
}

footer {
  @apply fixed right-0 bottom-0 px-4 py-2;
  color: color-mix(in srgb, var(--color-fg), transparent 50%);
}

footer a:hover {
  color: #ff008a;
}
</style>
