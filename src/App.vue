<template>
  <div class="flex h-screen w-screen bg-[var(--color-bg-deep)] text-gray-200 font-sans overflow-hidden relative selection:bg-indigo-500/30">
    
    <!-- Ambient Background Effects (The "Exodus" Vibe) -->
    <div class="fixed inset-0 pointer-events-none z-0 overflow-hidden">
      <div class="absolute top-[-10%] left-[-10%] w-[500px] h-[500px] bg-indigo-900/20 rounded-full blur-[120px]"></div>
      <div class="absolute bottom-[-10%] right-[-10%] w-[600px] h-[600px] bg-purple-900/10 rounded-full blur-[120px]"></div>
    </div>

    <!-- Layout Container -->
    <div class="relative z-10 flex w-full h-full">
      <!-- Left Sidebar -->
      <Sidebar />

      <!-- Main Content Area -->
      <main class="flex-1 flex flex-col min-w-0 overflow-hidden relative">
        <!-- Scrollable Content Wrapper -->
        <div class="flex-1 overflow-y-auto overflow-x-hidden custom-scrollbar p-8">
          <router-view v-slot="{ Component }">
            <transition name="page" mode="out-in">
              <component :is="Component" />
            </transition>
          </router-view>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import Sidebar from './components/Sidebar.vue';
import { onMounted } from 'vue';
import { useNodeStore } from './stores/nodeStore';
import { useWalletStore } from './stores/walletStore';

// Initialize stores
const node = useNodeStore();
const wallet = useWalletStore();

onMounted(async () => {
  // Load saved node settings (paths)
  await node.loadSettings();
  
  // Start polling
  wallet.startPolling(10000); 
});
</script>

<style>
/* Simple Fade Transition for routing */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
