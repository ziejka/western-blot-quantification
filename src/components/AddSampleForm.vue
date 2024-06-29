<script lang="ts" setup>
import { computed, onBeforeMount, onBeforeUnmount, ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";
import { emitter } from '../emitter'
import Button from './ui/Button.vue'

const props = defineProps({
  membrane_title: {
    type: String,
    required: true,
  }
});

const samplesName = ref<string[]>([]);
const isHintVisible = ref(false);
const sampleName = ref(null);

const filteredSamplesName = computed(() =>
  samplesName.value.filter(s => s.toLocaleLowerCase().includes(sampleData.value.name.toLocaleLowerCase()))
)

const emit = defineEmits(['sample-added', 'close']);

onBeforeMount(() => {
  getSamplesNames()
  document.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
})

const result: any = ref(null)
const sampleData = ref({
  membrane_title: props.membrane_title,
  name: '',
  values: '',
  is_reference: false,
})

function handleClickOutside(event: MouseEvent) {
  if (event.target != sampleName.value) {
    isHintVisible.value = false;
  }
}

function setSampleName(name: string) {
  sampleData.value.name = name;
  isHintVisible.value = false;
}

function showHints() {
  isHintVisible.value = true;
}

async function addExperimentData() {
  try {
    await invoke("add_sample_data", { sampleData: sampleData.value });
    emit('sample-added');
  } catch (e) {
    emitter.emit('error', String(e))
  }
}

async function getSamplesNames() {
  try {
    samplesName.value = await invoke("get_samples_names");
  } catch (e) {
    // noop
  }
}
</script>

<template>
  <h1 class="text-xl font-bold mb-2">Add Sample Data</h1>
  <form class="grid grid-cols-[1fr_3fr] gap-4 w-1/2" @submit.prevent="addExperimentData">
    <label for="sampleName">Sample Name</label>
    <div class="relative">
      <input ref="sampleName" class="border" v-model="sampleData.name" name="name" type="text" @focusin="showHints">
      <Transition>
        <ul v-if="isHintVisible && filteredSamplesName.length > 0"
          class="absolute bg-white top-8 left-0 border rounded-md shadow w-fit">
          <li class="py-2 pl-3 pr-9 rounded hover:bg-stone-200 transition-colors w-full cursor-pointer"
            v-for="name in filteredSamplesName" :value="name" @click="setSampleName(name)">{{ name }}</li>
        </ul>
      </Transition>
    </div>

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
