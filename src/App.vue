<script setup lang="ts">
import DocNavigation from './components/DocNavigation.vue';
import Notification from './components/Notification.vue'
</script>

<template>
  <div class="grid grid-cols-[1fr_6fr] min-h-screen">
    <nav class="bg-stone-50 flex flex-col border-r">
      <img src="./assets/logo.png" class="w-16 p-2 block absolute top-0 right-0" />
      <DocNavigation />
    </nav>
    <main class="mx-auto py-5 w-full">
      <Notification />
      <RouterView />
    </main>
  </div>
</template>

<style scoped></style>
