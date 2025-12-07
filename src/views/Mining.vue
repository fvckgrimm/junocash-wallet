<template>
  <div class="p-6">
    <h1 class="text-2xl font-bold mb-6">Mining Dashboard</h1>

    <!-- 1. Statistics Grid -->
    <!-- Adjusted grid to fit 5 items on extra large screens -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5 gap-4 mb-6">
      
      <!-- Status -->
      <div class="bg-white p-4 rounded shadow border-l-4" :class="isMining ? 'border-green-500' : 'border-red-500'">
        <div class="text-xs text-gray-500 uppercase font-bold">Status</div>
        <div class="text-xl font-bold flex items-center gap-2">
          <div :class="isMining ? 'bg-green-500 animate-pulse' : 'bg-red-500'" class="w-3 h-3 rounded-full"></div>
          {{ isMining ? 'Running' : 'Stopped' }}
        </div>
      </div>

      <!-- My Hashrate -->
      <div class="bg-white p-4 rounded shadow border-l-4 border-blue-500">
        <div class="text-xs text-gray-500 uppercase font-bold">My Hashrate</div>
        <div class="text-xl font-bold text-gray-800">
          {{ formatHashrate(miningInfo.localsolps) }}
        </div>
      </div>

      <!-- Network Hashrate -->
      <div class="bg-white p-4 rounded shadow border-l-4 border-purple-500">
        <div class="text-xs text-gray-500 uppercase font-bold">Network Hashrate</div>
        <div class="text-xl font-bold text-gray-800">
          {{ formatHashrate(miningInfo.networksolps) }}
        </div>
      </div>

      <!-- Difficulty -->
      <div class="bg-white p-4 rounded shadow border-l-4 border-yellow-500">
        <div class="text-xs text-gray-500 uppercase font-bold">Difficulty</div>
        <div class="text-xl font-bold text-gray-800">
          {{ formatDiff(miningInfo.difficulty) }}
        </div>
      </div>

      <!-- Block Reward (New) -->
      <div class="bg-white p-4 rounded shadow border-l-4 border-indigo-500">
        <div class="text-xs text-gray-500 uppercase font-bold">Block Reward</div>
        <div class="text-xl font-bold text-gray-800">
          {{ miningInfo.reward }} JUNO
        </div>
        <div class="text-[10px] text-gray-400 font-mono">Height: {{ miningInfo.blocks }}</div>
      </div>

    </div>

    <!-- 2. Controls Area -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      
      <!-- Mining Toggle -->
      <div class="bg-white p-6 rounded shadow">
        <h2 class="text-lg font-bold mb-4 text-gray-700">Control Miner</h2>
        <div class="flex items-center justify-between">
          <p class="text-sm text-gray-500">
            Algorithm: RandomX (CPU) <br> Threads: All Available
          </p>
          <button 
            @click="toggleMining" 
            class="px-8 py-3 rounded font-bold transition-colors shadow-sm"
            :class="isMining ? 'bg-red-50 text-red-600 border border-red-200 hover:bg-red-100' : 'bg-green-600 text-white hover:bg-green-700'"
          >
            {{ isMining ? 'Stop Mining' : 'Start Mining' }}
          </button>
        </div>
      </div>

      <!-- Shielding Settings -->
      <div class="bg-gray-800 text-white p-6 rounded shadow">
        <div class="flex justify-between items-start mb-4">
          <div>
            <h2 class="font-bold text-lg">Auto-Shielding</h2>
            <p class="text-xs text-gray-400 mt-1">Shield coinbase rewards to spend them.</p>
          </div>
          <div class="flex items-center gap-2">
            <input type="checkbox" id="autoshield" v-model="autoShieldEnabled" class="w-4 h-4 accent-purple-500" />
            <label for="autoshield" class="cursor-pointer select-none text-sm">Enable (5m)</label>
          </div>
        </div>

        <div class="mb-4">
          <select v-model="shieldToAddress" class="w-full p-2 rounded bg-gray-700 border border-gray-600 text-xs font-mono text-white focus:outline-none focus:border-purple-500">
            <option value="" disabled>Select Shield-To Address...</option>
            <option v-for="addr in wallet.addresses" :key="addr" :value="addr">
              {{ addr }}
            </option>
          </select>
        </div>

        <button 
          @click="performShield" 
          :disabled="!shieldToAddress || isShielding"
          class="w-full py-2 bg-purple-600 hover:bg-purple-700 text-white text-sm font-bold rounded disabled:opacity-50 disabled:cursor-not-allowed transition"
        >
          {{ isShielding ? 'Shielding...' : 'Shield Now (Manual)' }}
        </button>

        <div class="mt-3 text-[10px] font-mono text-gray-400 break-all" v-if="lastOperation">
          Last Op: {{ lastOperation }}
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNodeStore } from '../stores/nodeStore';
import { useWalletStore } from '../stores/walletStore';

const node = useNodeStore();
const wallet = useWalletStore();

// UI State
const isMining = ref(false);
const autoShieldEnabled = ref(false);
const shieldToAddress = ref('');
const isShielding = ref(false);
const lastOperation = ref('');

// Data State
const miningInfo = reactive({
  localsolps: 0,
  networksolps: 0,
  difficulty: 0,
  blocks: 0,  // Added Block Height
  reward: 0,  // Added Block Reward
  generate: false
});

let statsInterval: any = null;
let shieldInterval: any = null;

onMounted(async () => {
  await wallet.fetchAddresses();
  const uni = wallet.addresses.find(a => a.startsWith('j1'));
  if (uni) shieldToAddress.value = uni;

  await fetchMiningInfo();
  statsInterval = setInterval(fetchMiningInfo, 5000); // Poll every 5s (2s is a bit aggressive for subsidy checks)
});

async function fetchMiningInfo() {
  try {
    // 1. Get General Info (includes 'blocks')
    const res = await invoke<any>('get_mining_info', {
      port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass
    });
    
    miningInfo.localsolps = res.localsolps || 0;
    miningInfo.networksolps = res.networksolps || 0;
    miningInfo.difficulty = res.difficulty || 0;
    miningInfo.blocks = res.blocks || 0;
    miningInfo.generate = res.generate || false;
    isMining.value = res.generate;

    // 2. If we have a block height, fetch the subsidy for it
    if (miningInfo.blocks > 0) {
      const subsidy = await invoke<number>('get_block_subsidy', {
        height: miningInfo.blocks,
        port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass
      });
      miningInfo.reward = subsidy;
    }

  } catch (e) {
    // console.error(e);
  }
}

async function toggleMining() {
  const newState = !isMining.value;
  isMining.value = newState; 
  
  try {
    await invoke('set_mining', {
      enabled: newState, 
      threads: -1, 
      port: node.rpcPort, 
      user: node.rpcUser, 
      pass: node.rpcPass
    });
    setTimeout(fetchMiningInfo, 500);
  } catch(e) {
    console.error(e);
    isMining.value = !newState; 
  }
}

// ... (Shielding logic stays same) ...
watch(autoShieldEnabled, (enabled) => {
  if (enabled) {
    shieldInterval = setInterval(() => {
        if(isMining.value) performShield();
    }, 300000); 
  } else {
    clearInterval(shieldInterval);
  }
});

async function performShield() {
  if (!shieldToAddress.value) {
    alert("Please select a Shield-To address.");
    return;
  }
  isShielding.value = true;
  try {
    const opId = await invoke<string>('shield_coinbase', {
      toAddress: shieldToAddress.value,
      port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass
    });
    lastOperation.value = opId;
  } catch (e) {
    console.error(e);
  } finally {
    isShielding.value = false;
  }
}

function formatHashrate(val: number) {
  if (!val) return '0 H/s';
  if (val >= 1_000_000) return (val / 1_000_000).toFixed(3) + ' MH/s';
  if (val >= 1_000) return (val / 1_000).toFixed(3) + ' kH/s';
  return val.toFixed(2) + ' H/s';
}

function formatDiff(val: number) {
  if (!val) return '0';
  if (val >= 1_000_000) return (val / 1_000_000).toFixed(2) + 'M';
  if (val >= 1_000) return (val / 1_000).toFixed(2) + 'k';
  return val.toFixed(2);
}

onUnmounted(() => {
  clearInterval(statsInterval);
  clearInterval(shieldInterval);
});
</script>
