<script setup lang="ts">
import { ref } from "vue";

import InputText from "@/components/input-text.vue";
import InputSelect from "@/components/input-select.vue";

const themeColor = ref(localStorage.theme);
const apiKey = ref<string>("");

const username = ref<string>("");
const password = ref<string>("");

const themeOptions = [
  { value: "system", label: "System" },
  { value: "light", label: "Light" },
  { value: "dark", label: "Dark" },
];

const setTheme = (theme: string) => {
  localStorage.theme = theme;
  document.documentElement.dataset.theme = theme;
  themeColor.value = theme;
};

const handleLogin = () => {
  console.log(username.value, password.value);
};
</script>

<template>
  <main>
    <h1>theme</h1>
    <InputSelect
      :options="themeOptions"
      :model-value="themeColor"
      @update:model-value="(value) => setTheme(value as string)"
    />
    <h1>authorization</h1>
    <InputText
      v-model="apiKey"
      type="password"
      icon="lock"
      placeholder="api key"
    />
    <form @submit.prevent="handleLogin">
      <InputText
        v-model="username"
        type="email"
        placeholder="email"
        icon="user"
      />
      <InputText
        v-model="password"
        type="password"
        placeholder="password"
        icon="lock"
      />
    </form>
  </main>
</template>
