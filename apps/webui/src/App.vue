<script setup lang="ts">
import { ref } from 'vue'
import { RouterView } from 'vue-router'

const currentTheme = ref(localStorage.theme)

const setTheme = (theme: 'light' | 'dark' | 'system') => {
  localStorage.theme = theme
  document.documentElement.dataset.theme = theme
  currentTheme.value = theme
}
</script>

<template>
  <nav>
    <div class="theme-selector">
      <button :class="currentTheme == 'system' && 'active'" @click="setTheme('system')">
        System
      </button>
      <button :class="currentTheme == 'light' && 'active'" @click="setTheme('light')">Light</button>
      <button :class="currentTheme == 'dark' && 'active'" @click="setTheme('dark')">Dark</button>
    </div>

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

<style scoped>
@reference "./assets/styles.css";

nav {
  @apply absolute flex flex-col p-2 gap-2;
}

.theme-selector {
  @apply flex border gap-px rounded-md overflow-clip;
  background-color: var(--color-fg);
  border-color: var(--color-fg);
}

.theme-selector button {
  @apply w-18 h-6 text-color bg-color;
}

.theme-selector button.active {
  @apply invert;
}

nav a {
  @apply opacity-50;
}

nav a.router-link-active {
  @apply opacity-100;
}

footer {
  @apply fixed right-0 bottom-0 px-4 py-2 text-color-disabled;
}

footer a:hover {
  color: #ff008a;
}
</style>
