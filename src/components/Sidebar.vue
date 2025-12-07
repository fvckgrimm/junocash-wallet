<template>
  <aside class="w-72 flex flex-col h-screen glass border-r border-white/5 relative z-20">
    
    <!-- Logo Area -->
    <div class="h-24 flex items-center px-8">
      <div class="flex items-center gap-3 group cursor-default">
        <div class="relative w-10 h-10 flex items-center justify-center rounded-xl bg-gradient-to-br from-indigo-500 to-purple-600 shadow-lg shadow-indigo-500/20 group-hover:shadow-indigo-500/40 transition-all duration-500 ease-out">
          <!-- Juno Symbol -->
          <span class="text-white font-bold text-xl leading-none font-mono">J</span>
          <!-- Shine effect -->
          <div class="absolute inset-0 rounded-xl bg-white/20 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
        </div>
        <div>
          <h1 class="text-lg font-bold tracking-tight text-white">Juno<span class="text-indigo-400">Cash</span></h1>
          <p class="text-[10px] uppercase tracking-widest text-gray-500 font-medium">Wallet</p>
        </div>
      </div>
    </div>

    <!-- Navigation -->
    <nav class="flex-1 px-4 py-6 overflow-y-auto">
      <ul class="space-y-1">
        <li v-for="(link, index) in links" :key="link.path" 
            class="animate-slide-right" 
            :style="{ animationDelay: `${index * 50}ms` }">
          
          <router-link 
            :to="link.path"
            class="group relative flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-300 ease-[cubic-bezier(0.25,1,0.5,1)]"
            active-class="bg-white/5 shadow-inner"
            v-slot="{ isActive }"
          >
            <!-- Hover/Active Glow Background -->
            <div 
              class="absolute inset-0 rounded-xl opacity-0 transition-opacity duration-300"
              :class="isActive ? 'opacity-100 bg-gradient-to-r from-indigo-500/10 to-transparent' : 'group-hover:opacity-100 bg-white/[0.03]'"
            ></div>

            <!-- Active Indicator Bar -->
            <div 
              class="absolute left-0 h-6 w-1 rounded-r-full bg-indigo-500 transition-all duration-300 transform origin-left"
              :class="isActive ? 'scale-x-100 opacity-100' : 'scale-x-0 opacity-0'"
            ></div>

            <!-- Icon -->
            <component 
              :is="link.icon" 
              class="w-5 h-5 transition-colors duration-300 relative z-10"
              :class="isActive ? 'text-indigo-400 drop-shadow-[0_0_8px_rgba(99,102,241,0.5)]' : 'text-gray-400 group-hover:text-gray-200'" 
            />
            
            <!-- Label -->
            <span 
              class="font-medium text-sm transition-colors duration-300 relative z-10"
              :class="isActive ? 'text-white' : 'text-gray-400 group-hover:text-gray-200'"
            >
              {{ link.name }}
            </span>

          </router-link>
        </li>
      </ul>
    </nav>

    <!-- Status Area (Pinned to bottom) -->
    <div class="p-4 border-t border-white/5 bg-black/20">
      <Status />
    </div>
  </aside>
</template>

<script setup lang="ts">
import Status from './Status.vue';

// Refined Icons (Lucide Style - Crisp 2px stroke)
const DashboardIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="7" height="7" rx="1" /><rect x="14" y="3" width="7" height="7" rx="1" /><rect x="14" y="14" width="7" height="7" rx="1" /><rect x="3" y="14" width="7" height="7" rx="1" /></svg>` };
const SendIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><line x1="22" y1="2" x2="11" y2="13" /><polygon points="22 2 15 22 11 13 2 9 22 2" /></svg>` };
const ReceiveIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M12 3v12"/><path d="m8 11 4 4 4-4"/><path d="M8 21h8"/></svg>` };
const MiningIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M6 3h12l4 6-10 13L2 9z"/><path d="M12 22V9"/><path d="m2 9 20 0"/><path d="m14.5 9-5-6"/><path d="m9.5 9 5-6"/></svg>` };
const SettingsIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.47a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>` };

const links = [
  { name: 'Dashboard', path: '/', icon: DashboardIcon },
  { name: 'Send', path: '/send', icon: SendIcon },
  { name: 'Receive', path: '/receive', icon: ReceiveIcon },
  { name: 'Mining', path: '/mining', icon: MiningIcon },
  { name: 'Settings', path: '/settings', icon: SettingsIcon },
];
</script>
