<template>
  <h1 class="text-xl font-bold mb-2">Add Sample Data</h1>
  <form class="grid grid-cols-[1fr_3fr] gap-4 w-1/2" @submit.prevent="addExperimentData">
    <label for="sampleName">Sample Name</label>
    <input class="border" v-model="sampleData.name" name="name" type="text">

    <label for="experimentData">Experiment data</label>
    <textarea class="border h-48" v-model="sampleData.values" name="experimentData" />

    <label for="isReference">Is reference</label>
    <input class="w-8 h-8" v-model="sampleData.is_reference" name="isReference" type="checkbox">

    <div class="flex gap-2">
      <Button text="Add" button-type="primary" type="submit" />
      <Button text="Cancel" button-type="secondary" type="submit" @click="$emit('close')" />
    </div>
  </form>
  <p class="mt-6">
    {{ result }}
  </p>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";
import { emitter } from '../emitter'
import Button from './ui/Button.vue'

const props = defineProps({
  membrane_title: {
    type: String,
    required: true,
  }
});

const emit = defineEmits(['sample-added', 'close'])

const result: any = ref(null)
const sampleData = ref({
  membrane_title: props.membrane_title,
  name: '',
  values: '',
  is_reference: false,
})

async function addExperimentData() {
  try {
    await invoke("add_sample_data", { sampleData: sampleData.value });
    emit('sample-added');
  } catch (e) {
    emitter.emit('error', String(e))
  }
}
</script>
