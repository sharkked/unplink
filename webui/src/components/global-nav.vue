<script setup lang="ts">
import FeatherIcon from "./feather-icon.vue";
</script>

<template>
  <nav class="global-nav">
    <h1>unpl.ink</h1>
    <RouterLink to="/">
      <FeatherIcon icon="link" /><span>shorten</span>
    </RouterLink>
    <RouterLink to="/history">
      <FeatherIcon icon="clock" /><span>history</span>
    </RouterLink>
    <div class="spacer"></div>
    <RouterLink to="/about">
      <FeatherIcon icon="info" /><span>about</span>
    </RouterLink>
    <RouterLink to="/config">
      <FeatherIcon icon="settings" /><span>config</span>
    </RouterLink>
  </nav>
</template>

<style>
.global-nav {
  display: flex;
  width: 8rem;
  flex-direction: column;
  padding: 0.25rem;
  color: var(--color-half);
  border-right: 1px solid var(--color-half);

  h1 {
    margin: 0.25rem 0.5rem;
  }

  a {
    display: block;
    padding: 0.5rem;
    border-radius: var(--radius);
    white-space: nowrap;
    transition: scale 0.1s ease;

    &.router-link-active {
      color: var(--color-off);
      background: var(--color-on);
    }

    & > svg {
      line-height: 1;
      vertical-align: bottom;
    }

    & span {
      padding-left: 0.5rem;
    }

    &:active {
      color: var(--color-off);
      background-color: var(--color-half);
      scale: 0.98;
    }
  }

  .spacer {
    flex: 1;
  }
}
</style>
