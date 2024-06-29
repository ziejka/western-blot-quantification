<script setup lang="ts">
import { ref } from 'vue';
import { useDocumentList } from '../useDocumentList';
import NavLink from './NavLink.vue';
import Button from './ui/Button.vue';

const isAddModalVisible = ref(false);
const docName = ref('');
const { addDocument, docs } = useDocumentList()

function onFormSubmit() {
  addDocument(docName.value);
  closeModal();
}

function onAddClick() {
  isAddModalVisible.value = true;
}

function closeModal() {
  isAddModalVisible.value = false;
}
</script>

<template>
  <button @click="onAddClick" class="py-2 px-2 bg-sky-100 hover:bg-sky-200 transition-colors border-t text-left">Add new doc</button>
  <NavLink to="/" text="All docs" class="font-bold" />
  <NavLink v-for="doc in docs" :key="doc" :to="`/${doc}`" :text="doc" />

  <section v-if="isAddModalVisible" class="absolute w-screen h-screen z-10">
    <div class="absolute left-0 bg-stone-900 opacity-20 w-screen h-screen z-0" />

    <form
      class="flex gap-2 items-center mx-auto absolute top-10 left-1/2 -translate-x-1/2 bg-slate-50 px-10 py-12 shadow-lg shadow-stone-400 border-stone-600"
      @submit.prevent="onFormSubmit">

      <label for="sampleName" class="text-nowrap">New Document:</label>
      <input class="border h-full px-4 py-2" v-model="docName" name="name" type="text">
      <Button text="Add" button-type="primary" type="submit" />
      <Button text="Close" button-type="secondary" @click="closeModal" />
    </form>
  </section>
</template>
