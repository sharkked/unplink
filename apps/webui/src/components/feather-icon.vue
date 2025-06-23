<script setup lang="ts">
import { computed } from 'vue'

const { icon } = defineProps<{ icon: string }>()

const href = computed(() => `/src/assets/feather-sprite.svg#${icon}`)
</script>

<template>
  <svg class="icon">
    <use :href />
  </svg>
</template>

<style>
.icon {
  display: inline-block;
  height: 1rem;
  width: 1rem;
  fill: none;
  stroke: currentColor;
  stroke-width: 2px;
  stroke-linecap: round;
  stroke-linejoin: round;
}
</style>
