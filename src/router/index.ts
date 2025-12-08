import { createRouter, createWebHistory } from 'vue-router';
import Dashboard from '../views/Dashboard.vue';
import Send from '../views/Send.vue';
import Receive from '../views/Receive.vue';
import Mining from '../views/Mining.vue';
import Settings from '../views/Settings.vue';
import History from '../views/History.vue';
import AddressBook from '../views/AddressBook.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Dashboard },
    { path: '/send', component: Send },
    { path: '/receive', component: Receive },
    { path: '/history', component: History },
    { path: '/address-book', component: AddressBook },
    { path: '/mining', component: Mining },
    { path: '/settings', component: Settings },
  ]
});

export default router;
