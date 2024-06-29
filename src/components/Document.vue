<script lang="ts" setup>
import { save } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { emitter } from '../emitter';
import { Sample } from '../types';
import AddSampleForm from './AddSampleForm.vue';
import Preview from './Preview.vue';
import Button from './ui/Button.vue';

const route = useRoute()
const title = ref(route.params.id as string || '');
const isNewSample = ref(false);
const isPreview = ref(false);
const samples = ref<Sample[]>([]);

onMounted(() => getSamples());
watch(() => route.params.id, (newId) => {
  title.value = newId as string;
  getSamples();
})

function showNewSampleForm() {
  isNewSample.value = true;
}

async function getSamplesOrThrow() {
  samples.value = await invoke('get_samples_by_title', { title: title.value });
  samples.value.sort(s => s.is_reference ? 1 : -1)
}

async function getSamples() {
  try {
    getSamplesOrThrow()
  } catch (e) {
    emitter.emit('error', String(e))
  }
}

function onSampleAdded() {
  getSamples();
  isNewSample.value = false;
}

async function calculateOrThrow() {
  await invoke('calculate', { title: title.value })
}

async function togglePreview() {
  try {
    await calculateOrThrow();
    await getSamplesOrThrow();
    isPreview.value = !isPreview.value;
  } catch (e) {
    emitter.emit('error', String(e))
  }
}

function onAddSampleClose() {
  isNewSample.value = false;
}

async function saveCSV() {
  try {
    await calculateOrThrow();

    const savePath = await save();

    if (!savePath) return;
    await invoke("save_csv", { title: title.value, pathStr: savePath })
  } catch (e) {
    emitter.emit('error', String(e))
  }
}

async function deleteSample(sampleName: string) {
  try {
    await invoke('delete_sample', { title: title.value, name: sampleName });
    getSamples();
  } catch (e) {
    emitter.emit('error', String(e))
  }
}
</script>

<template>
  <section class="flex gap-2 px-5 pb-4">
    <h1 class="text-2xl font-bold mr-4" type="text">📄 {{ title }}</h1>
    <Button v-if="!isPreview" text="Show Preview" button-type="primary" @click="togglePreview" />
    <Button v-if="isPreview" text="Close Preview" button-type="secondary" @click="togglePreview" />
    <Button text="Save as CSV" button-type="primary" @click="saveCSV" />
  </section>

  <section class="px-5 py-3 flex gap-4 items-center bg-stone-50 border-t">
    <p>Added samples:</p>
    <div class="flex border rounded items-center relative" v-for="sample in samples">
      <p class="px-2">{{ sample.name.toString() }}</p>
      <span v-if="sample.is_reference"
        class="absolute -top-2.5 -left-3 text-sky-600 font-bold border border-sky-300 bg-sky-100 px-1.5 text-sm rounded-full">R</span>
      <Button text="❌" button-type="danger" @click="deleteSample(sample.name.toString())" />
    </div>
    <Button text="➕" button-type="primary" @click="showNewSampleForm" />
  </section>


  <section v-if="isNewSample" class="px-5 py-4 border-t">
    <AddSampleForm :membrane_title="title" @sample-added="onSampleAdded" @close="onAddSampleClose" />
  </section>

  <section v-if="isPreview" class="px-5 py-4 bg-stone-50 border-t">
    <Preview :samples="samples" />
  </section>
</template>
