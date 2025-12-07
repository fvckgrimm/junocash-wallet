<template>
  <div class="p-6 h-full flex flex-col">
    <h1 class="text-2xl font-bold mb-4">Receive Juno</h1>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 h-full">
      
      <!-- Left: List of Addresses -->
      <div class="bg-white rounded shadow flex flex-col overflow-hidden">
        <div class="p-4 border-b bg-gray-50 flex justify-between items-center">
          <h2 class="font-bold text-gray-700">My Addresses</h2>
          <button @click="generateNew" class="text-xs bg-purple-600 text-white px-2 py-1 rounded">
            + New
          </button>
        </div>
        
        <div class="overflow-y-auto flex-1 p-2 space-y-2">
          <div 
            v-for="addr in wallet.addresses" 
            :key="addr"
            @click="selectedAddress = addr"
            class="p-3 rounded cursor-pointer border transition-colors"
            :class="selectedAddress === addr ? 'border-purple-500 bg-purple-50' : 'border-gray-200 hover:bg-gray-50'"
          >
            <div class="flex items-center gap-2">
              <span class="text-xs font-bold uppercase px-1 rounded" :class="getTypeClass(addr)">
                {{ getTypeLabel(addr) }}
              </span>
              <p class="text-xs font-mono break-all text-gray-600">{{ addr }}</p>
            </div>
          </div>

          <div v-if="wallet.addresses.length === 0" class="text-center p-4 text-gray-400 text-sm">
            No addresses found. Click "+ New" to generate one.
          </div>
        </div>
      </div>

      <!-- Right: QR Code -->
      <div class="bg-white rounded shadow p-6 flex flex-col items-center justify-center text-center">
        <div v-if="selectedAddress">
          <QrcodeVue :value="selectedAddress" :size="220" level="M" />
          <p class="mt-6 font-mono text-sm break-all text-gray-700 bg-gray-100 p-2 rounded">
            {{ selectedAddress }}
          </p>
          <button @click="copyToClipboard" class="mt-4 px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded">
            Copy to Clipboard
          </button>
        </div>
        <div v-else class="text-gray-400">
          Select an address from the left to view QR code.
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import QrcodeVue from 'qrcode.vue';
import { invoke } from '@tauri-apps/api/core';
import { useNodeStore } from '../stores/nodeStore';
import { useWalletStore } from '../stores/walletStore';

const node = useNodeStore();
const wallet = useWalletStore();
const selectedAddress = ref('');

onMounted(async () => {
  await wallet.fetchAddresses();
  // Select the first one by default if available
  if (wallet.addresses.length > 0) {
    // Prefer unified (j1) addresses
    const uni = wallet.addresses.find(a => a.startsWith('j1'));
    selectedAddress.value = uni || wallet.addresses[0];
  }
});

async function generateNew() {
  if(!confirm("Generate a new Unified Address?")) return;
  
  try {
    const newAddr = await invoke<string>('get_new_address', {
      typeParam: "unified", // Explicitly ask for Orchard/Unified
      port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass
    });
    await wallet.fetchAddresses();
    selectedAddress.value = newAddr;
  } catch(e) {
    alert("Error: " + e);
  }
}

function copyToClipboard() {
  navigator.clipboard.writeText(selectedAddress.value);
  alert("Copied!");
}

function getTypeLabel(addr: string) {
  if (addr.startsWith('j1')) return 'Unified';
  if (addr.startsWith('zs')) return 'Sapling';
  if (addr.startsWith('t1')) return 'Transparent';
  return 'Legacy';
}

function getTypeClass(addr: string) {
  if (addr.startsWith('j1')) return 'bg-purple-100 text-purple-700';
  if (addr.startsWith('zs')) return 'bg-yellow-100 text-yellow-700';
  return 'bg-gray-200 text-gray-700';
}
</script>
