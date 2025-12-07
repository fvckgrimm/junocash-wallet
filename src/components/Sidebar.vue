<template>
  <aside class="w-64 bg-gray-800 flex flex-col h-screen text-white border-r border-gray-700">
    <!-- Logo Area -->
    <div class="p-6 border-b border-gray-700 flex items-center gap-2">
      <div class="w-8 h-8 bg-purple-500 rounded-full flex items-center justify-center font-bold">J</div>
      <h1 class="text-xl font-bold tracking-wide">JunoWallet</h1>
    </div>

    <!-- Navigation -->
    <nav class="flex-1 overflow-y-auto py-4">
      <ul class="space-y-1">
        <li v-for="link in links" :key="link.path">
          <router-link 
            :to="link.path"
            class="flex items-center gap-3 px-6 py-3 text-gray-300 hover:bg-gray-700 hover:text-white transition-colors"
            active-class="bg-gray-700 text-purple-400 border-r-4 border-purple-400"
          >
            <!-- Simple SVG Icons -->
            <component :is="link.icon" class="w-5 h-5" />
            <span>{{ link.name }}</span>
          </router-link>
        </li>
      </ul>
    </nav>

    <!-- Status Component -->
    <Status />
  </aside>
</template>

<script setup lang="ts">
import Status from './Status.vue';

// Simple Icon Components (You can replace these with Lucide-Vue or Heroicons)
const DashboardIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" /></svg>` };
const SendIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>` };
const ReceiveIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" /></svg>` };
const MiningIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>` };
const SettingsIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /></svg>` };

const links = [
  { name: 'Dashboard', path: '/', icon: DashboardIcon },
  { name: 'Send', path: '/send', icon: SendIcon },
  { name: 'Receive', path: '/receive', icon: ReceiveIcon },
  { name: 'Mining', path: '/mining', icon: MiningIcon },
  { name: 'Settings', path: '/settings', icon: SettingsIcon },
];
</script>
