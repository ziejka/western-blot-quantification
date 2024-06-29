<script lang="ts" setup>
import { emitter } from '../emitter'
import { onMounted, ref } from 'vue';

type Notification = {
  msg: string,
  type: 'error' | 'notify'
  id: number
}
const list = ref<Notification[]>([]);

onMounted(() => {
  emitter.on('error', addError);
})

function addError(msg: string) {
  list.value.push({ msg, type: 'error', id: Math.random() })
  console.error(msg)

  setTimeout(() => {
    const idx = list.value.length - 1;
    list.value.splice(idx)
  }, 3000)
}
</script>

<template>
  <ul class="absolute top-5 right-5">
    <TransitionGroup tag="ul" name="list">
      <li class="shadow-md border rounded-md border-red-500 bg-red-200 text-red-500 px-4 py-2 mb-2" v-for="n in list" :key="n.id">
        <span class="font-bold">Error</span>
        <p class="capitalize">{{ n.msg }}</p>
      </li>
    </TransitionGroup>
  </ul>
</template>
