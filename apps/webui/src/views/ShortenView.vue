<script setup lang="ts">
import { ref } from 'vue'
import { nanoid } from 'nanoid'
const link = ref<string>('')
const shortLink = ref<string>('')

const randomCode = ref<string>('')

const randomizeText = () => {
  randomCode.value = nanoid(10)
  requestAnimationFrame(randomizeText)
}
randomizeText()

async function onSubmit(event: Event) {
  event.preventDefault()

  let url = link.value
  if (!/^https?:\/\//i.test(url)) {
    url = 'https://' + url
  }

  shortLink.value = await fetch('http://localhost:8000', {
    method: 'POST',
    headers: {
      'Access-Control-Allow-Header': '*',
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ url: url }),
  })
    .then((data) => data.json())
    .then((data) => data.url)
}

const onCleared = (event: Event) => {
  event.preventDefault()
  link.value = ''
}

const onCopy = (event: Event) => {
  event.preventDefault()
  if (!shortLink.value) return
  navigator.clipboard.writeText(shortLink.value)
}
</script>

<template>
  <main>
    <div class="plink-cat" />
    <form>
      <div class="input-container w-full">
        <div class="input-icon">
          <svg class="feather">
            <use href="../assets/feather-sprite.svg#link" />
          </svg>
        </div>
        <input
          name="link"
          placeholder="paste link here :3"
          v-model="link"
          @keydown.enter="onSubmit"
          @keydown.esc="onCleared"
        />
        <button id="clear-button" class="btn-sm" v-if="link.length > 0" @click="onCleared">
          <svg class="feather">
            <use href="../assets/feather-sprite.svg#x" />
          </svg>
        </button>
      </div>
      <div
        :class="
          'input-container w-min select-none ' + (shortLink.length <= 0 ? '' : 'cursor-pointer')
        "
        @click="onCopy"
      >
        <button class="btn-sm" :disabled="shortLink.length <= 0" @click="onCopy">
          <svg class="feather">
            <use href="../assets/feather-sprite.svg#copy" />
          </svg>
        </button>
        <output class="pr-1 text-nowrap">
          <template v-if="shortLink">{{ shortLink }}</template>
          <template v-else>https://<span class="rainbow">unpl.ink</span>/{{ randomCode }}</template>
        </output>
      </div>
    </form>
  </main>
</template>

<style scoped>
@reference "@/assets/styles.css";

main {
  @apply flex flex-col flex-1 items-center justify-center gap-4;
}

.feather {
  @apply w-4 h-4;
}

.plink-cat {
  @apply relative w-[318px] h-32 rounded-md bg-contain;

  &,
  &:after {
    background-size: contain;
    background-image: url('https://c.tenor.com/y1QFa-1vyKYAAAAd/tenor.gif');
  }

  &:after {
    @apply absolute -z-1 left-0 top-0 w-full h-full opacity-70;
    filter: blur(15px) saturate(1.5);
  }
}

form {
  @apply flex flex-col w-full max-w-[480px] gap-1 items-center;
}

button:not([disabled]):hover {
  cursor: pointer;
}

.input-container {
  @apply box-border flex justify-center items-center h-8 p-1 gap-2 text-nowrap bg-color border border-color rounded-md;

  @variant focus-within {
    @apply outline;
  }

  input {
    @apply flex-1 outline-none;

    &::placeholder {
      @apply text-color opacity-50;
    }
  }
}

.input-icon {
  @apply inline-block pl-1 opacity-50;
}

.btn-sm {
  @apply w-6 h-6 p-0.75 border border-color rounded-md opacity-50;
  transition:
    color 0.15s ease-in,
    border-color 0.2s ease-in,
    background-color 0.15s ease;

  @variant enabled {
    &:hover,
    &:focus {
      @apply opacity-100;
      background-color: color-mix(in srgb, var(--color-fg), transparent 75%);
    }
  }
}

#clear-button {
  &:hover,
  &:focus {
    @apply text-red-100 bg-red-500 border-red-200 border-t-red-100 border-b-red-300;
  }
}

.rainbow,
.rainbow::after {
  @apply relative z-2 inline-block text-transparent;
  background-clip: text;
  background-size: 800px 100%;
  background-image: linear-gradient(
    90deg,
    rgba(255, 0, 0, 1) 0%,
    rgba(255, 154, 0, 1) 10%,
    rgba(208, 222, 33, 1) 20%,
    rgba(79, 220, 74, 1) 30%,
    rgba(63, 218, 216, 1) 40%,
    rgba(47, 201, 226, 1) 50%,
    rgba(28, 127, 238, 1) 60%,
    rgba(95, 21, 242, 1) 70%,
    rgba(186, 12, 248, 1) 80%,
    rgba(251, 7, 217, 1) 90%,
    rgba(255, 0, 0, 1) 100%
  );
  background-repeat: repeat-x;
  animation: rgb 8s linear infinite;
}

.rainbow::after {
  content: 'unpl.ink';
  position: absolute;
  top: 0;
  left: 0;
  filter: blur(4px);
  z-index: 1;
  @variant not-dark {
    opacity: 0.8;
  }
}
</style>
