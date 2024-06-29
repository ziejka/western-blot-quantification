<script setup lang="ts">
import { useDocumentList } from '../useDocumentList';
import Button from './ui/Button.vue';
import { ref } from 'vue';

const docName = ref('');
const { addDocument, deleteDocument, docs } = useDocumentList()

function onFormSubmit() {
  addDocument(docName.value);
}
</script>

<template>
  <section class="flex gap-2 px-5 pb-7">
    <h1 class="text-2xl font-bold">List of documents</h1>
  </section>

  <section class="px-5 py-3 flex gap-4 items-center  bg-stone-50 border-t">
    <form class="flex gap-2 items-center" @submit.prevent="onFormSubmit">
      <label for="sampleName" class="text-nowrap">New Document:</label>
      <input class="border h-full px-4 py-2" v-model="docName" name="name" type="text">
      <Button text="Add" button-type="primary" type="submit" />
    </form>
  </section>

  <section class="flex flex-col align-middle justify-start px-5 py-3 border-t">
    <div v-for="doc in docs" class="flex gap-2 mt-2 w-1/3">
      <RouterLink class="w-full px-4 py-2 border hover:underline hover:bg-stone-100" :to="`/${doc}`">{{ doc }}
      </RouterLink>
      <Button text="❌" @click="deleteDocument(doc)" button-type="danger" />
    </div>
  </section>

</template>
