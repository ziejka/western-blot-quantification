<script lang="ts" setup>
import { computed } from 'vue';
import { Sample } from '../types';

const props = defineProps<{
  samples: Sample[]
}>();

const range = computed(() => props.samples[0]?.area.map((_, i) => i) || [])
</script>

<template>
  <table class="table-auto border w-fit mb-4 last:mb-0" v-for="s in props.samples">
    <tr>
      <td class="border px-2 py-1 bg-stone-200 font-bold">{{ s.name }}</td>
    </tr>
    <tr class="bg-stone-100">
      <td class="border px-2 py-1">Area</td>
      <td class="border px-2 py-1">Mean OD</td>
      <td class="border px-2 py-1">Blank</td>
      <td class="border px-2 py-1">Nom by reference</td>
      <td class="border px-2 py-1">Normalized</td>
    </tr>

    <tr v-for="i in range">
      <td class="border px-2 py-1">{{ s.area[i]?.toFixed(3) }}</td>
      <td class="border px-2 py-1">{{ s.mean_od[i]?.toFixed(3) }}</td>
      <td class="border px-2 py-1">{{ s.blank[i]?.toFixed(3) }}</td>
      <td class="border px-2 py-1">{{ s.norm_by_reference[i]?.toFixed(3) }}</td>
      <td class="border px-2 py-1">{{ s.normalized[i]?.toFixed(3) }}</td>
    </tr>
  </table>
</template>
