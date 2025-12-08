<template>
  <div class="max-w-6xl mx-auto min-h-full flex flex-col p-2 pb-20 relative">
    
    <!-- Header -->
    <div class="mb-8 px-2 animate-fade-in">
      <h1 class="text-2xl font-bold text-white tracking-tight">Network Status</h1>
      <p class="text-gray-400 text-sm">Real-time connection metrics and peer topology.</p>
    </div>

    <!-- 1. KEY METRICS GRID -->
    <div v-if="node.networkInfo" class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8 animate-slide-up">
      
      <!-- Connection Count -->
      <div class="glass p-5 rounded-2xl border border-white/5 relative overflow-hidden group">
        <div class="absolute top-0 right-0 p-3 opacity-10 group-hover:opacity-20 transition-opacity">
          <svg width="60" height="60" fill="currentColor" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/></svg>
        </div>
        <div class="relative z-10">
          <p class="text-xs font-bold text-gray-500 uppercase tracking-wider">Active Peers</p>
          <div class="flex items-baseline gap-2 mt-2">
            <span class="text-3xl font-mono font-bold text-white">{{ node.networkInfo.connections }}</span>
            <span class="text-xs font-medium text-emerald-400">Nodes</span>
          </div>
        </div>
      </div>

      <!-- Protocol Version -->
      <div class="glass p-5 rounded-2xl border border-white/5 relative overflow-hidden group">
        <div class="relative z-10">
          <p class="text-xs font-bold text-gray-500 uppercase tracking-wider">Protocol Ver</p>
          <div class="mt-2">
            <span class="text-2xl font-mono font-bold text-white">{{ node.networkInfo.protocolversion }}</span>
          </div>
          <p class="text-[10px] text-gray-400 mt-1 truncate">{{ node.networkInfo.subversion }}</p>
        </div>
      </div>

      <!-- Relay Fee -->
      <div class="glass p-5 rounded-2xl border border-white/5 relative overflow-hidden group">
        <div class="relative z-10">
          <p class="text-xs font-bold text-gray-500 uppercase tracking-wider">Relay Fee</p>
          <div class="mt-2">
            <span class="text-2xl font-mono font-bold text-indigo-300">{{ node.networkInfo.relayfee }}</span>
            <span class="text-xs text-indigo-500/70 ml-1">JUNO/kB</span>
          </div>
        </div>
      </div>

      <!-- Traffic Summary (Calculated from peers) -->
      <div class="glass p-5 rounded-2xl border border-white/5 relative overflow-hidden group">
        <div class="relative z-10">
          <p class="text-xs font-bold text-gray-500 uppercase tracking-wider">Session Traffic</p>
          <div class="mt-2 flex flex-col gap-1">
             <div class="flex justify-between text-xs">
               <span class="text-gray-400">Down:</span>
               <span class="text-emerald-400 font-mono">{{ totalRecv }}</span>
             </div>
             <div class="flex justify-between text-xs">
               <span class="text-gray-400">Up:</span>
               <span class="text-rose-400 font-mono">{{ totalSent }}</span>
             </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 2. PEER LIST TABLE -->
    <div class="glass rounded-3xl border border-white/5 overflow-hidden animate-slide-up animation-delay-100 flex flex-col h-[600px]">
      <div class="p-6 border-b border-white/5 bg-white/[0.02] flex justify-between items-center">
        <h3 class="font-bold text-white">Connected Nodes</h3>
        <button @click="refresh" class="p-2 hover:bg-white/5 rounded-lg text-gray-400 hover:text-white transition-colors">
           <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M23 4v6h-6"/><path d="M1 20v-6h6"/><path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/></svg>
        </button>
      </div>

      <!-- Table Header -->
      <div class="grid grid-cols-12 gap-4 px-6 py-3 bg-black/20 text-[10px] uppercase font-bold text-gray-500 tracking-wider">
        <div class="col-span-4">Address / User Agent</div>
        <div class="col-span-2 text-right">Ping</div>
        <div class="col-span-2 text-right">Height</div>
        <div class="col-span-2 text-center">Direction</div>
        <div class="col-span-2 text-right">Traffic</div>
      </div>

      <!-- Table Body -->
      <div class="flex-1 overflow-y-auto custom-scrollbar">
        <div v-for="peer in sortedPeers" :key="peer.id" 
             class="grid grid-cols-12 gap-4 px-6 py-4 border-b border-white/5 items-center hover:bg-white/[0.02] transition-colors group">
          
          <!-- Address & Agent -->
          <div class="col-span-4 min-w-0">
             <div class="flex items-center gap-2">
               <div class="w-2 h-2 rounded-full shrink-0" :class="peer.inbound ? 'bg-indigo-500' : 'bg-purple-500'"></div>
               <p class="font-mono text-xs text-white truncate" :title="peer.addr">{{ peer.addr }}</p>
             </div>
             <p class="text-[10px] text-gray-500 truncate mt-1 pl-4">{{ cleanSubver(peer.subver) }}</p>
          </div>

          <!-- Ping -->
          <div class="col-span-2 text-right">
             <span class="font-mono text-xs font-bold" :class="getPingColor(peer.pingtime)">
               {{ (peer.pingtime * 1000).toFixed(0) }} ms
             </span>
          </div>

          <!-- Height -->
          <div class="col-span-2 text-right">
             <span class="font-mono text-xs text-gray-300">{{ peer.synced_blocks }}</span>
          </div>

          <!-- Direction -->
          <div class="col-span-2 flex justify-center">
             <span class="px-2 py-0.5 rounded text-[10px] font-bold uppercase"
                   :class="peer.inbound ? 'bg-indigo-500/10 text-indigo-400' : 'bg-purple-500/10 text-purple-400'">
               {{ peer.inbound ? 'Inbound' : 'Outbound' }}
             </span>
          </div>

          <!-- Traffic -->
          <div class="col-span-2 text-right">
             <p class="text-[10px] text-gray-400">↓ {{ formatBytes(peer.bytesrecv) }}</p>
             <p class="text-[10px] text-gray-400">↑ {{ formatBytes(peer.bytessent) }}</p>
          </div>

        </div>
        
        <div v-if="node.peers.length === 0" class="p-12 text-center text-gray-500">
          No peers connected.
        </div>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, computed } from 'vue';
import { useNodeStore } from '../stores/nodeStore';

const node = useNodeStore();
let pollInterval: any;

onMounted(() => {
  refresh();
  pollInterval = setInterval(refresh, 5000);
});

onUnmounted(() => {
  if (pollInterval) clearInterval(pollInterval);
});

function refresh() {
  node.fetchNetworkStatus();
}

// Helpers
const sortedPeers = computed(() => {
  // Sort by ping (fastest first)
  return [...node.peers].sort((a, b) => a.pingtime - b.pingtime);
});

const totalSent = computed(() => {
  const bytes = node.peers.reduce((acc, p) => acc + p.bytessent, 0);
  return formatBytes(bytes);
});

const totalRecv = computed(() => {
  const bytes = node.peers.reduce((acc, p) => acc + p.bytesrecv, 0);
  return formatBytes(bytes);
});

function cleanSubver(sub: string) {
  return sub.replace(/\//g, '').replace(/:/g, ' ');
}

function getPingColor(sec: number) {
  const ms = sec * 1000;
  if (ms < 100) return 'text-emerald-400';
  if (ms < 300) return 'text-amber-400';
  return 'text-rose-400';
}

function formatBytes(bytes: number, decimals = 1) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}
</script>

<style scoped>
.animation-delay-100 {
  animation-delay: 100ms;
}
</style>
