<template>
    <!-- ... (Template stays exactly the same) ... -->
    <div class="p-6">
    <h1 class="text-2xl font-bold mb-6">Settings (BYON)</h1>

    <div class="space-y-6 max-w-3xl">
      
      <!-- SECTION 1: Paths -->
      <div class="bg-white p-4 rounded shadow space-y-4 text-gray-900">
        <h2 class="font-bold border-b pb-2 text-gray-700">Node Location</h2>
        <!-- Binary Selection -->
        <div>
          <label class="block text-sm font-medium mb-1">JunoCash Binary (junocashd)</label>
          <div class="flex gap-2">
            <input v-model="binPath" class="w-full p-2 border border-gray-300 rounded bg-gray-50" readonly />
            <button @click="pickBinary" class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">Browse</button>
          </div>
        </div>

        <!-- Data Dir Selection -->
        <div>
          <label class="block text-sm font-medium mb-1">Data Directory</label>
          <div class="flex gap-2">
            <input v-model="dataDir" class="w-full p-2 border border-gray-300 rounded bg-gray-50" readonly />
            <button @click="pickDir" class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">Browse</button>
          </div>
        </div>
      </div>

      <!-- SECTION 2: RPC Credentials -->
      <div class="bg-white p-4 rounded shadow space-y-4 text-gray-900">
        <h2 class="font-bold border-b pb-2 text-gray-700">RPC Configuration</h2>
        <p class="text-xs text-gray-500">
          If launching via Wallet, these will be set automatically. 
          If running node manually, these must match your <code>junocash.conf</code>.
        </p>
        
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium mb-1">RPC User</label>
            <input v-model="rpcUser" class="w-full p-2 border border-gray-300 rounded" />
          </div>
          <div>
            <label class="block text-sm font-medium mb-1">RPC Password</label>
            <input v-model="rpcPass" type="password" class="w-full p-2 border border-gray-300 rounded" />
          </div>
        </div>
      </div>

      <!-- SECTION 3: Actions -->
      <div class="flex gap-4 pt-4">
        <!-- Option A: Launch -->
        <button @click="launchNode" class="flex-1 py-3 bg-green-600 hover:bg-green-700 text-white font-bold rounded shadow transition">
          Save & Launch Node
        </button>

        <!-- Option B: Connect Only -->
        <button @click="connectOnly" class="flex-1 py-3 bg-gray-600 hover:bg-gray-700 text-white font-bold rounded shadow transition">
          Save & Connect (Node already running)
        </button>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useNodeStore } from '../stores/nodeStore';
import { useWalletStore } from '../stores/walletStore';

const node = useNodeStore();
const wallet = useWalletStore();

// Local refs for the form inputs
const binPath = ref('');
const dataDir = ref('');
const rpcUser = ref('');
const rpcPass = ref('');

// Load settings when the page opens
onMounted(async () => {
  await node.loadSettings(); // Pulls from localStorage
  binPath.value = node.binPath;
  dataDir.value = node.dataDir;
  rpcUser.value = node.rpcUser;
  rpcPass.value = node.rpcPass;
});

async function pickBinary() {
  const selected = await open({ multiple: false, directory: false });
  if (selected && typeof selected === 'string') binPath.value = selected;
}

async function pickDir() {
  const selected = await open({ directory: true, multiple: false });
  if (selected && typeof selected === 'string') dataDir.value = selected;
}

// Helper to save all fields
async function saveToStore() {
  await node.saveSettings(
    binPath.value, 
    dataDir.value,
    rpcUser.value,
    rpcPass.value
  );
}

// Option A: Launch
async function launchNode() {
  await saveToStore(); // Save first!
  try {
    await invoke('launch_node', {
      binPath: binPath.value,
      dataDir: dataDir.value,
      rpcPort: node.rpcPort,
      rpcUser: node.rpcUser,
      rpcPass: node.rpcPass
    });
    alert("Node Launch Command Sent!");
    setTimeout(() => wallet.fetchBalance(), 2000);
  } catch(e) {
    alert("Error: " + e);
  }
}

// Option B: Connect
async function connectOnly() {
  await saveToStore(); // Save first!
  alert("Settings saved. Connecting...");
  await wallet.fetchBalance();
  if (wallet.lastError) {
    alert("Connection Failed: " + wallet.lastError);
  } else {
    alert("Connected Successfully!");
  }
}
</script>
