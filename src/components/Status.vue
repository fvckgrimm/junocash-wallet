<template>
  <div class="px-4 py-3 border-t border-gray-700 bg-gray-900 text-xs text-gray-400">
    <div class="flex items-center justify-between mb-1">
      <span>Status:</span>
      <span :class="statusClass" class="font-bold flex items-center gap-1">
        <span class="w-2 h-2 rounded-full" :class="dotClass"></span>
        {{ statusText }}
      </span>
    </div>
    
    <div class="flex items-center justify-between">
      <span>Block Height:</span>
      <span class="font-mono text-white">{{ blockHeight }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from 'vue';
import { useNodeStore } from '../stores/nodeStore';
import { invoke } from '@tauri-apps/api/core';

const node = useNodeStore();
const blockHeight = ref(0);
let interval: any = null;

const statusText = computed(() => node.isConnected ? 'Online' : 'Offline');
const statusClass = computed(() => node.isConnected ? 'text-green-400' : 'text-red-400');
const dotClass = computed(() => node.isConnected ? 'bg-green-400' : 'bg-red-400');

async function checkStatus() {
  if (!node.rpcPort) return;
  
  try {
    // We add a lightweight 'getblockcount' command to rust or just reuse get_info
    // Assuming you implement a simple `get_block_count` in Rust similar to get_balance
    // For now, let's assume if balance fetch works, we are online.
    
    // Call generic RPC directly for block count
    // NOTE: You need to expose a generic 'rpc_call' or specific 'get_block_count' in Rust
    // This is a placeholder for that specific RPC call:
    const count = await invoke<number>('get_block_count', { 
        port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass 
    });
    
    blockHeight.value = count;
    node.isConnected = true;
  } catch (e) {
    node.isConnected = false;
  }
}

onMounted(() => {
  checkStatus();
  interval = setInterval(checkStatus, 5000); // Check every 5s
});

onUnmounted(() => clearInterval(interval));
</script>
