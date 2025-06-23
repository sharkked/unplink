<script setup lang="ts">
import { ref } from 'vue'
import { nanoid } from 'nanoid'

import InputText from '@/components/input-text.vue'
import InputButton from '@/components/input-button.vue'

const link = ref<string>('')
const token = ref<string>('')
const shortLink = ref<string>('')

const randomCode = ref<string>('')

const randomizeText = () => {
  randomCode.value = nanoid(10)
  requestAnimationFrame(randomizeText)
}
randomizeText()

async function handleSubmit() {
  let url = link.value
  if (!/^https?:\/\//i.test(url)) {
    url = 'https://' + url
  }

  const shortLink = await fetch('http://localhost:8000', {
    method: 'POST',
    headers: {
      Authorization: token.value,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ url: url }),
  })
  const data = await shortLink.json()
  return data.url
}

const handleCopy = () => {
  if (!shortLink.value) return
  navigator.clipboard.writeText(shortLink.value)
}
</script>

<template>
  <main>
    <div class="plink-cat" />
    <form @submit.prevent="handleSubmit">
      <InputText
        v-model="token"
        type="password"
        icon="lock"
        placeholder="api key"
        :clearable="true"
      />
      <InputText v-model="link" type="url" icon="link" placeholder="paste link here :3" />
      <InputButton class="copy-button" icon="copy" @click.prevent="handleCopy">
        <template v-if="shortLink">{{ shortLink }}</template>
        <template v-else>
          <span class="text-color-disabled"
            >https://<span class="rainbow">unpl.ink</span>/{{ randomCode }}</span
          >
        </template>
      </InputButton>
    </form>
  </main>
</template>

<style>
.plink-cat {
  position: relative;
  margin: auto;
  margin-bottom: 1rem;

  &,
  &:after {
    width: 318px;
    height: 128px;
    background-image: url('https://c.tenor.com/y1QFa-1vyKYAAAAd/tenor.gif');
    background-size: contain;
    border-radius: 6px;
  }

  &:after {
    content: '';
    position: absolute;
    inset: 0;
    z-index: -1;
    opacity: 40%;
    filter: blur(15px) saturate(1.5);
  }
}

form {
  display: flex;
  flex-direction: column;
  max-width: 480px;
  gap: 0.25rem;

  &,
  & > * {
    width: 100%;
    margin: 0 auto;
  }

  .copy-button {
    width: min-content;
  }
}
</style>
