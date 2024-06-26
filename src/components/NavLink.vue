<script lang="ts" setup>
defineProps({
  text: {
    type: String,
    required: true
  },
  to: {
    type: String,
    required: true
  },
})
</script>

<style scoped>
.router-link-active {
  @apply bg-stone-200
}
</style>

<template>
  <RouterLink class="py-2 px-2 hover:bg-stone-200 transition-colors border-t" :to="to">{{text}}</RouterLink>
</template>
