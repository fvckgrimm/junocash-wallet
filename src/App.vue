<template>
  <div class="flex h-screen w-screen bg-[var(--color-bg-deep)] text-gray-200 font-sans overflow-hidden relative selection:bg-indigo-500/30">
    
    <!-- Ambient Background Effects -->
    <div class="fixed inset-0 pointer-events-none z-0 overflow-hidden">
      <div class="absolute top-[-10%] left-[-10%] w-[500px] h-[500px] bg-indigo-900/20 rounded-full blur-[120px]"></div>
      <div class="absolute bottom-[-10%] right-[-10%] w-[600px] h-[600px] bg-purple-900/10 rounded-full blur-[120px]"></div>
    </div>

    <!-- Layout Container -->
    <div class="relative z-10 flex flex-col md:flex-row w-full h-full">
      
      <!-- MOBILE HEADER (Visible only on < md screens) -->
      <header class="md:hidden h-16 shrink-0 border-b border-white/5 bg-[#13161c]/80 backdrop-blur-md flex items-center justify-between px-4 sticky top-0 z-30">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center shadow-lg">
            <span class="text-white font-bold font-mono">J</span>
          </div>
          <span class="font-bold text-white tracking-tight">Juno<span class="text-indigo-400">Cash</span></span>
        </div>
        
        <!-- Hamburger Button -->
        <button 
          @click="isSidebarOpen = true"
          class="p-2 text-gray-400 hover:text-white bg-white/5 rounded-lg active:scale-95 transition-all"
        >
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 12h18M3 6h18M3 18h18"/></svg>
        </button>
      </header>

      <!-- Sidebar (Responsive) -->
      <Sidebar :is-open="isSidebarOpen" @close="isSidebarOpen = false" />

      <!-- Main Content Area -->
      <main class="flex-1 flex flex-col min-w-0 overflow-hidden relative h-[calc(100vh-64px)] md:h-full">
        <div class="flex-1 overflow-y-auto overflow-x-hidden custom-scrollbar p-4 md:p-8">
          <router-view v-slot="{ Component }">
            <transition name="page" mode="out-in">
              <component :is="Component" />
            </transition>
          </router-view>
        </div>
      </main>
    </div>

    <!-- GLOBAL NOTIFICATIONS WRAPPER -->
    <div class="fixed bottom-6 right-6 left-6 md:left-auto flex flex-col gap-3 pointer-events-none z-50 perspective-1000">
      <transition-group name="notification">
        <!-- (Existing notification code...) -->
        <div 
          v-for="note in visibleNotifications" 
          :key="note.id"
          class="pointer-events-auto w-full md:w-96 relative overflow-hidden rounded-xl bg-[#13161c]/90 backdrop-blur-xl border border-white/10 shadow-[0_8px_32px_rgba(0,0,0,0.5)] transform transition-all duration-300 hover:scale-[1.02]"
        >
          <!-- (Existing notification content...) -->
          <div 
            class="absolute left-0 top-0 bottom-0 w-1 shadow-[2px_0_12px_rgba(0,0,0,0.5)]"
            :class="note.status === 'success' ? 'bg-emerald-500 shadow-emerald-500/50' : 'bg-rose-500 shadow-rose-500/50'"
          ></div>

          <div class="p-4 pl-5">
            <div class="flex justify-between items-start mb-1">
              <div class="flex items-center gap-2">
                <div 
                  class="w-5 h-5 rounded-full flex items-center justify-center"
                  :class="note.status === 'success' ? 'bg-emerald-500/20 text-emerald-400' : 'bg-rose-500/20 text-rose-400'"
                >
                  <svg v-if="note.status === 'success'" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M20 6L9 17l-5-5"/></svg>
                  <svg v-else width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
                </div>
                <h4 class="font-bold text-sm text-white tracking-wide uppercase">
                  {{ note.status === 'success' ? 'Success' : 'Failed' }}
                </h4>
              </div>
              <button @click="dismiss(note.id)" class="text-gray-500 hover:text-white transition-colors p-1 -mr-2 -mt-2 rounded-lg hover:bg-white/5">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
              </button>
            </div>
            
            <div v-if="note.status === 'success'" class="space-y-2">
               <p class="text-xs text-gray-400">Transaction broadcast successfully.</p>
               <div class="bg-black/30 rounded border border-white/5 px-2 py-1.5 flex flex-col gap-1">
                 <span class="text-[10px] text-gray-500 font-bold uppercase tracking-wider">TxID</span>
                 <p class="font-mono text-[10px] text-emerald-400 break-all leading-tight select-all">
                   {{ note.result?.txid }}
                 </p>
               </div>
            </div>

            <div v-if="note.status === 'failed'" class="mt-1">
              <p class="text-xs text-rose-300 leading-relaxed font-medium">
                {{ note.error?.message || "Unknown error occurred during processing." }}
              </p>
            </div>
            <p class="text-[9px] text-gray-600 mt-3 font-mono text-right opacity-50">Event ID: {{ note.id.substring(0,8) }}</p>
          </div>
        </div>
      </transition-group>
    </div>

  </div>
</template>

<script setup lang="ts">
import Sidebar from './components/Sidebar.vue';
import { onMounted, computed, ref, watch } from 'vue';
import { useNodeStore } from './stores/nodeStore';
import { useWalletStore } from './stores/walletStore';

const node = useNodeStore();
const wallet = useWalletStore();

const dismissedIds = ref(new Set<string>());
const isSidebarOpen = ref(false); // Controls mobile sidebar

const visibleNotifications = computed(() => {
  return wallet.notifications
    .slice() 
    .reverse() 
    .filter(n => !dismissedIds.value.has(n.id))
    .slice(0, 3);
});

function dismiss(id: string) {
  dismissedIds.value.add(id);
}

watch(() => wallet.notifications, (newNotes) => {
  newNotes.forEach(n => {
    if (n.status === 'success' && !dismissedIds.value.has(n.id)) {
      setTimeout(() => dismiss(n.id), 8000);
    }
  });
}, { deep: true });

onMounted(async () => {
  await node.loadSettings();
  wallet.startPolling(10000); 
});
</script>

<style>
/* Page Routing Transitions */
.page-enter-active,
.page-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}
.page-enter-from {
  opacity: 0;
  transform: scale(0.98);
}
.page-leave-to {
  opacity: 0;
  transform: scale(1.02);
}

/* Notification Slide/Fade Animations */
.notification-enter-active,
.notification-leave-active {
  transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.notification-enter-from {
  opacity: 0;
  transform: translateX(50px) scale(0.9);
}

.notification-leave-to {
  opacity: 0;
  transform: translateX(50px) scale(0.9);
}

/* Ensure spacing works in flex container when items are removed */
.notification-leave-active {
  position: absolute; 
}
</style>
