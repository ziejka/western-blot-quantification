<script lang="ts" setup>
defineProps<{ text: string, buttonType: 'primary' | 'secondary' | 'danger' }>();

</script>

<template>
  <button class="w-fit px-4 py-2 rounded transition-all shadow-sm hover:shadow-md" type="button" :class="buttonType === 'primary'
    ? 'bg-violet-300 hover:bg-violet-400'
    : buttonType === 'danger'
      ? 'bg-red-100 hover:bg-red-200'
      : 'bg-stone-300 hover:bg-stone-400'">
    {{ text }}
  </button>
</template>
