<template>
  <div class="space-y-3 select-none">
    
    <!-- Connection State -->
    <div class="flex items-center justify-between">
      <span class="text-[10px] font-bold text-gray-500 uppercase tracking-widest">Network Status</span>
      
      <div class="flex items-center gap-2">
        <span 
          class="text-xs font-medium transition-colors duration-300"
          :class="node.isConnected ? 'text-emerald-400' : 'text-rose-400'"
        >
          {{ node.isConnected ? 'Synced' : 'Disconnected' }}
        </span>
        
        <!-- Animated Indicator -->
        <div class="relative flex h-2.5 w-2.5">
          <span 
            v-if="node.isConnected" 
            class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
          ></span>
          <span 
            class="relative inline-flex rounded-full h-2.5 w-2.5 transition-colors duration-300"
            :class="node.isConnected ? 'bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.6)]' : 'bg-rose-500'"
          ></span>
        </div>
      </div>
    </div>
    
    <!-- Block Height (The "Heartbeat" metric) -->
    <div class="flex items-center justify-between group cursor-default">
      <div class="flex items-center gap-1.5 text-gray-500 group-hover:text-gray-300 transition-colors">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/></svg>
        <span class="text-xs font-medium">Height</span>
      </div>
      
      <span class="font-mono text-xs text-indigo-300 tracking-wide transition-all" :class="{ 'opacity-50': !node.isConnected }">
        <span v-if="blockHeight > 0">#{{ blockHeight.toLocaleString() }}</span>
        <span v-else class="text-gray-600">--</span>
      </span>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useNodeStore } from '../stores/nodeStore';
import { invoke } from '@tauri-apps/api/core';

const node = useNodeStore();
const blockHeight = ref(0);
let interval: any = null;

async function checkStatus() {
  if (!node.rpcPort) {
    node.isConnected = false;
    return;
  }
  
  try {
    // Attempt to fetch block count. 
    // This acts as both a "Are we online?" check and a data fetch.
    const count = await invoke<number>('get_block_count', { 
        port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass 
    });
    
    // Simple logic: if the call succeeds, we are connected.
    blockHeight.value = count;
    node.isConnected = true;
  } catch (e) {
    node.isConnected = false;
  }
}

onMounted(() => {
  // Immediate check
  checkStatus();
  // Poll every 5 seconds
  interval = setInterval(checkStatus, 5000); 
});

onUnmounted(() => {
  if (interval) clearInterval(interval);
});
</script>
