<template>
  <div class="space-y-4 select-none">
    
    <!-- Connection Header -->
    <div class="flex items-center justify-between">
      <div class="flex flex-col">
        <span class="text-[10px] font-bold text-gray-500 uppercase tracking-widest leading-none mb-1">
          Network Status
        </span>
        <span 
          class="text-xs font-bold transition-colors duration-500"
          :class="statusColor"
        >
          {{ statusLabel }}
        </span>
      </div>
      
      <!-- Status Indicator Dot -->
      <div class="relative flex h-3 w-3">
        <!-- Ping Effect (Synced only) -->
        <span 
          v-if="statusKey === 'synced'" 
          class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
        ></span>
        
        <!-- Pulse Effect (Loading/Syncing) -->
        <span 
          v-if="statusKey === 'syncing' || statusKey === 'loading' || statusKey === 'reindexing'" 
          class="animate-pulse absolute inline-flex h-full w-full rounded-full opacity-75"
          :class="dotColor"
        ></span>
        
        <!-- Solid Center -->
        <span 
          class="relative inline-flex rounded-full h-3 w-3 transition-colors duration-500 shadow-sm"
          :class="[dotColor, statusKey === 'synced' ? 'shadow-[0_0_10px_rgba(16,185,129,0.5)]' : '']"
        ></span>
      </div>
    </div>
    
    <!-- Data Display -->
    <div class="flex flex-col gap-2">
      
      <!-- Block Height Row -->
      <div class="flex items-center justify-between text-xs group cursor-default">
        <div class="flex items-center gap-1.5 text-gray-500 group-hover:text-gray-400 transition-colors">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/></svg>
          <span class="font-medium">Current Height</span>
        </div>
        
        <span class="font-mono text-gray-200 transition-all" :class="{ 'opacity-50': statusKey === 'disconnected' }">
          <span v-if="blocks > 0">{{ blocks.toLocaleString() }}</span>
          <span v-else class="text-gray-600">--</span>
        </span>
      </div>

      <!-- SYNCING STATE: Progress Bar -->
      <transition name="fade">
        <div v-if="statusKey === 'syncing'" class="space-y-1 pt-1">
          <div class="flex justify-between text-[9px] text-gray-500 font-mono uppercase tracking-tight">
            <span>Syncing Headers</span>
            <span class="text-amber-400">{{ syncPercentage }}%</span>
          </div>
          
          <div class="w-full bg-gray-800/50 h-1.5 rounded-full overflow-hidden border border-white/5 relative">
            <!-- Glowing Bar -->
            <div 
              class="h-full bg-gradient-to-r from-amber-600 to-amber-400 rounded-full shadow-[0_0_10px_rgba(251,191,36,0.3)] transition-all duration-700 ease-out"
              :style="{ width: `${syncPercentage}%` }"
            ></div>
          </div>
          
          <div class="text-right text-[9px] text-gray-600 font-mono">
            Target: {{ headers.toLocaleString() }}
          </div>
        </div>
      </transition>

      <!-- LOADING STATE: Info Message -->
      <transition name="fade">
        <div v-if="statusKey === 'loading' || statusKey === 'reindexing'" class="pt-1">
           <div class="bg-blue-500/10 border border-blue-500/20 rounded px-2 py-1.5">
             <p class="text-[9px] text-blue-300 leading-tight">
               <span class="animate-pulse">●</span> {{ rpcErrorMsg ? truncate(rpcErrorMsg) : 'Node is warming up...' }}
             </p>
           </div>
        </div>
      </transition>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useNodeStore } from '../stores/nodeStore';
import { invoke } from '@tauri-apps/api/core';

const node = useNodeStore();

// Data State
const blocks = ref(0);
const headers = ref(0);
const rpcErrorMsg = ref('');

// Computed Status Logic
const statusKey = computed(() => {
  if (!node.isConnected && !rpcErrorMsg.value) return 'disconnected';
  
  if (rpcErrorMsg.value) {
    const msg = rpcErrorMsg.value.toLowerCase();
    if (msg.includes('reindexing')) return 'reindexing';
    // Catch-all for other warmups like "Loading block index"
    return 'loading'; 
  }

  // Sync Logic
  if (headers.value > 0 && blocks.value < headers.value) {
    // If > 5 blocks behind, show sync UI
    if ((headers.value - blocks.value) > 5) return 'syncing';
  }

  return 'synced';
});

const statusLabel = computed(() => {
  switch (statusKey.value) {
    case 'disconnected': return 'Offline';
    case 'loading': return 'Initializing...';
    case 'reindexing': return 'Reindexing DB';
    case 'syncing': return 'Syncing Chain';
    case 'synced': return 'Fully Synced';
    default: return 'Unknown';
  }
});

const statusColor = computed(() => {
  switch (statusKey.value) {
    case 'disconnected': return 'text-rose-400';
    case 'loading': 
    case 'reindexing': return 'text-blue-400';
    case 'syncing': return 'text-amber-400';
    case 'synced': return 'text-emerald-400';
    default: return 'text-gray-400';
  }
});

const dotColor = computed(() => {
  switch (statusKey.value) {
    case 'disconnected': return 'bg-rose-500';
    case 'loading': 
    case 'reindexing': return 'bg-blue-500';
    case 'syncing': return 'bg-amber-500';
    case 'synced': return 'bg-emerald-500';
    default: return 'bg-gray-500';
  }
});

const syncPercentage = computed(() => {
  if (headers.value === 0) return 0;
  // Cap at 100% just in case of weird RPC returns
  const pct = (blocks.value / headers.value) * 100;
  return Math.min(pct, 100).toFixed(1);
});

function truncate(str: string) {
  if (str.length > 30) return str.substring(0, 28) + '...';
  return str;
}

async function checkStatus() {
  if (!node.rpcPort) {
    node.isConnected = false;
    return;
  }
  
  try {
    const info = await invoke<any>('get_blockchain_info', {
        host: node.rpcHost,
        port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass
    });
    
    // Success path
    rpcErrorMsg.value = '';
    node.isConnected = true;
    blocks.value = info.blocks;
    headers.value = info.headers;

  } catch (e: any) {
    const errStr = e.toString();
    
    // Detect Warmup States (Standard Zcash/Bitcoin RPC error code -28)
    if (errStr.includes('Loading') || errStr.includes('Verifying') || errStr.includes('Rewinding') || errStr.includes('Reindexing')) {
      // Clean up the error message for display
      // Often comes as "Rpc Error: ... message: Loading block index..."
      // We try to extract just the message part if possible, or use the whole string
      rpcErrorMsg.value = errStr.replace(/.*message:\s*/, '').replace(/[\"\}.]/g, '');
      node.isConnected = true; // Technically connected to the daemon process
    } else {
      node.isConnected = false;
      rpcErrorMsg.value = '';
    }
  }
}

let interval: any = null;

onMounted(() => {
  checkStatus();
  interval = setInterval(checkStatus, 2000);
});

onUnmounted(() => {
  if (interval) clearInterval(interval);
});
</script>

<style scoped>
/* Smooth fade for the progress bar area */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease, height 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
