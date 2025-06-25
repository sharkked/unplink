<script setup lang="ts">
import { ref } from "vue";
import { nanoid } from "nanoid";

import { useLinks } from "@/hooks/useLinks";
import InputText from "@/components/input-text.vue";
import InputButton from "@/components/input-button.vue";

const { shorten, lastResult } = useLinks();

const inputLink = ref<string>("");
const randomCode = ref<string>("");

const randomizeText = () => {
  randomCode.value = nanoid(24);
  requestAnimationFrame(randomizeText);
};
randomizeText();

async function handleSubmit() {
  let url = inputLink.value;
  if (!/^https?:\/\//i.test(url)) {
    url = "https://" + url;
  }

  shorten(url);
}

const handleCopy = () => {
  if (!lastResult.value) return;
  navigator.clipboard.writeText(lastResult.value);
};
</script>

<template>
  <form>
    <h1>short linkener</h1>
    <InputText
      v-model="inputLink"
      type="url"
      icon="link"
      placeholder="paste link here :3"
      :clearable="true"
      @submit="handleSubmit"
    />
    <InputButton
      class="copy-button"
      icon="copy"
      :data-result="lastResult"
      :disabled="lastResult == ''"
      @click.prevent="handleCopy"
    >
      <template v-if="lastResult">{{ lastResult }}</template>
      <template v-else> https://{{ randomCode }} </template>
    </InputButton>
  </form>
</template>

<style>
.copy-button {
  width: min-content;
  margin: 0 auto;
}
</style>
