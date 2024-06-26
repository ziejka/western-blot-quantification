<script lang="ts" setup>
import { emitter } from '../emitter'
import { onMounted, ref } from 'vue';

type Notification = {
  msg: string,
  type: 'error' | 'notify'
}
const list = ref<Notification[]>([]);

onMounted(() => {
  emitter.on('error', addError);
})

function addError(msg: string) {
  list.value.push({ msg, type: 'error' })
  console.error(msg)

  setTimeout(() => {
    const idx = list.value.length - 1;
    list.value.splice(idx)
  }, 3000)
}
</script>

<template>
  <div class="absolute top-5 right-5">
    <div class="shadow-md border border-red-500 bg-red-200 text-red-500 px-4 py-2 mb-2" v-for="n in list">
      <span class="font-bold">Error</span>
      <p>{{ n.msg }}</p>
    </div>
  </div>
</template>
