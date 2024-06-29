import { invoke } from '@tauri-apps/api/tauri';
import { ref } from 'vue';
import { emitter } from './emitter';
import { Router, useRouter } from 'vue-router';

const docs = ref<string[]>([]);
const isInitialized = ref(false);
const router = ref<Router>();

async function getDocuments() {
  try {
    docs.value = await invoke('get_documents_names');
    docs.value.sort();
  } catch (e) {
    emitter.emit('error', String(e));
  }
}

async function addDocument(name: string) {
  try {
    await invoke('add_document', { name });
    getDocuments();

    router.value?.push(`/${name}`);
  } catch (e) {
    emitter.emit('error', String(e));
  }
}

async function deleteDocument(name: string) {
  try {
    await invoke('delete_document', { title: name });
    getDocuments();
  } catch (e) {
    emitter.emit('error', String(e));
  }
}

async function updateDocumentName(oldTitle: string, newTitle: string) {
  try {
    await invoke('update_document_name', { oldTitle, newTitle });
    getDocuments();

    router.value?.push(`/${newTitle}`);
  } catch (e) {
    emitter.emit('error', String(e));
    throw e
  }
}

export const useDocumentList = () => {
  router.value = useRouter();

  if (!isInitialized.value) {
    isInitialized.value = true;
    getDocuments();
  }
  return {
    docs,
    addDocument,
    deleteDocument,
    updateDocumentName,
  };
};
