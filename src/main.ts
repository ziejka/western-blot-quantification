import { createApp } from 'vue';
import './index.css';
import App from './App.vue';

import { createMemoryHistory, createRouter } from 'vue-router';
import Document from './components/Document.vue';
import Documents from './components/Documents.vue';

const routes = [
  { path: '/', component: Documents },
  { path: '/:id', component: Document },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

createApp(App).use(router).mount('#app');
