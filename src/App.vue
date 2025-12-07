<template>
  <div class="flex h-screen w-screen bg-gray-50 text-gray-900 font-sans overflow-hidden">
    <!-- Left Sidebar -->
    <Sidebar />

    <!-- Main Content Area -->
    <main class="flex-1 overflow-y-auto">
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
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

  // Start polling wallet data
  // Note: This will only succeed if the node is running.
  // The 'Status' component in the Sidebar handles the visualization of connection state.
  wallet.startPolling(10000); // Poll every 10s
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
