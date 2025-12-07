<template>
  <div class="h-full flex flex-col p-2 space-y-6 max-w-7xl mx-auto w-full">
    
    <!-- Header -->
    <div class="flex items-center justify-between px-2 animate-fade-in">
      <div>
        <h1 class="text-2xl font-bold text-white tracking-tight">Mining Node</h1>
        <p class="text-gray-400 text-sm">Contribute CPU power to secure the network and earn rewards.</p>
      </div>
      <div class="flex items-center gap-2 px-3 py-1 rounded-full border" 
           :class="isMining ? 'bg-emerald-500/10 border-emerald-500/20 text-emerald-400' : 'bg-gray-800/50 border-white/5 text-gray-500'">
        <div class="w-2 h-2 rounded-full" :class="isMining ? 'bg-emerald-500 animate-pulse' : 'bg-gray-500'"></div>
        <span class="text-xs font-bold uppercase tracking-wider">{{ isMining ? 'Online' : 'Standby' }}</span>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 flex-1 min-h-0">
      
      <!-- LEFT COLUMN: The "Rig" (Controls & Local Stats) -->
      <div class="lg:col-span-5 flex flex-col gap-6 animate-slide-up">
        
        <!-- MAIN CONTROL CARD -->
        <div class="glass rounded-3xl p-8 relative overflow-hidden flex flex-col items-center justify-center text-center transition-all duration-500"
             :class="isMining ? 'border-purple-500/30 shadow-[0_0_50px_rgba(147,51,234,0.1)]' : 'border-white/5'">
          
          <!-- Background Ambient Glow -->
          <div class="absolute inset-0 bg-gradient-to-br from-purple-500/10 to-transparent opacity-0 transition-opacity duration-1000" :class="{ 'opacity-100': isMining }"></div>
          
          <!-- POWER BUTTON -->
          <button 
            @click="toggleMining"
            class="relative z-10 w-32 h-32 rounded-full flex items-center justify-center transition-all duration-300 group"
            :class="isMining ? 'bg-gray-900 shadow-[0_0_40px_rgba(168,85,247,0.6)] scale-105' : 'bg-white/5 hover:bg-white/10 shadow-xl active:scale-95'"
          >
            <!-- Spinning Ring when active -->
            <div class="absolute inset-0 rounded-full border-2 border-dashed border-purple-500/50 animate-spin-slow" v-if="isMining"></div>
            
            <!-- Icon -->
            <svg class="w-12 h-12 transition-colors duration-300" 
                 :class="isMining ? 'text-purple-400 drop-shadow-[0_0_10px_rgba(168,85,247,1)]' : 'text-gray-400 group-hover:text-white'"
                 fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
          </button>

          <h2 class="relative z-10 mt-6 text-xl font-bold text-white transition-all duration-300" :class="{ 'text-purple-300': isMining }">
            {{ isMining ? 'Mining Active' : 'Miner Stopped' }}
          </h2>
          <p class="relative z-10 text-sm text-gray-500 mt-2 max-w-xs">
            {{ isMining ? 'Allocating CPU cycles to proof-of-work consensus.' : 'Start the miner to begin generating hashes.' }}
          </p>

          <!-- Thread Configuration (only visible when stopped) -->
          <div v-if="!isMining" class="relative z-10 mt-6 w-full max-w-xs">
            <label class="block text-xs font-bold text-gray-500 uppercase mb-2">CPU Threads</label>
            <div class="flex items-center gap-3">
              <input 
                type="range" 
                v-model.number="selectedThreads" 
                :min="1" 
                :max="maxThreads"
                class="flex-1 h-2 bg-gray-800 rounded-lg appearance-none cursor-pointer accent-purple-500"
              />
              <div class="w-16 text-center">
                <span class="text-lg font-bold text-white font-mono">{{ selectedThreads === -1 ? 'Auto' : selectedThreads }}</span>
              </div>
            </div>
            <div class="flex justify-between text-xs text-gray-600 mt-1">
              <span>1 Core</span>
              <button 
                @click="selectedThreads = -1" 
                class="text-purple-400 hover:text-purple-300 transition-colors"
              >
                Auto
              </button>
              <span>{{ maxThreads }} Cores</span>
            </div>
          </div>
        </div>

        <!-- LOCAL PERFORMANCE CARD -->
        <div class="glass rounded-2xl p-6 border border-white/5 flex-1 flex flex-col justify-center">
          <div class="flex items-center gap-3 mb-6">
             <div class="p-2 bg-blue-500/10 rounded-lg text-blue-400">
               <svg width="20" height="20" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" /></svg>
             </div>
             <h3 class="text-sm font-bold text-gray-300 uppercase tracking-wide">CPU Performance</h3>
          </div>

          <!-- Large Hashrate Display -->
          <div class="mb-6">
            <p class="text-xs text-gray-500 uppercase font-semibold mb-1">My Hashrate</p>
            <div class="flex items-baseline gap-2">
              <span class="text-4xl font-mono font-bold text-white tracking-tighter transition-all duration-300" :class="{ 'opacity-50 blur-sm': !isMining }">
                 {{ formatHashrateNumber(miningInfo.localsolps) }}
              </span>
              <span class="text-lg text-gray-500 font-medium">{{ formatHashrateUnit(miningInfo.localsolps) }}</span>
            </div>
            <!-- Fake visualizer bar -->
            <div class="mt-4 h-1.5 w-full bg-gray-800 rounded-full overflow-hidden">
              <div class="h-full bg-blue-500 rounded-full transition-all duration-1000" 
                   :class="{ 'animate-pulse': isMining }"
                   :style="{ width: isMining ? '60%' : '0%' }"></div>
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4">
             <div class="bg-white/[0.03] p-3 rounded-xl border border-white/5">
               <p class="text-[10px] text-gray-500 uppercase">Algorithm</p>
               <p class="text-sm font-bold text-gray-300 mt-0.5">Equihash</p>
             </div>
             <div class="bg-white/[0.03] p-3 rounded-xl border border-white/5">
               <p class="text-[10px] text-gray-500 uppercase">Active Threads</p>
               <p class="text-sm font-bold text-gray-300 mt-0.5">
                 {{ isMining ? (selectedThreads === -1 ? `~${Math.floor(maxThreads / 2)}` : selectedThreads) : '0' }}
               </p>
             </div>
          </div>
        </div>

      </div>

      <!-- RIGHT COLUMN: Network & Privacy -->
      <div class="lg:col-span-7 flex flex-col gap-6 animate-slide-up" style="animation-delay: 100ms;">
        
        <!-- NETWORK STATS GRID -->
        <div class="grid grid-cols-2 gap-4">
          <!-- Network Hash -->
          <div class="glass p-5 rounded-2xl border border-white/5 hover:bg-white/[0.02] transition-colors">
            <div class="text-xs text-gray-500 font-bold uppercase mb-2">Network Hashrate</div>
            <div class="text-xl font-mono font-bold text-indigo-300">
              {{ formatHashrate(miningInfo.networksolps) }}
            </div>
          </div>
          
          <!-- Difficulty -->
          <div class="glass p-5 rounded-2xl border border-white/5 hover:bg-white/[0.02] transition-colors">
            <div class="text-xs text-gray-500 font-bold uppercase mb-2">Difficulty</div>
            <div class="text-xl font-mono font-bold text-amber-300">
              {{ formatDiff(miningInfo.difficulty) }}
            </div>
          </div>

          <!-- Block Height -->
          <div class="glass p-5 rounded-2xl border border-white/5 hover:bg-white/[0.02] transition-colors">
            <div class="text-xs text-gray-500 font-bold uppercase mb-2">Block Height</div>
            <div class="text-xl font-mono font-bold text-emerald-300">
              {{ miningInfo.blocks.toLocaleString() }}
            </div>
          </div>

          <!-- Reward -->
          <div class="glass p-5 rounded-2xl border border-white/5 hover:bg-white/[0.02] transition-colors">
            <div class="text-xs text-gray-500 font-bold uppercase mb-2">Block Reward</div>
            <div class="text-xl font-mono font-bold text-pink-300">
              {{ miningInfo.reward }} JUNO
            </div>
          </div>
        </div>

        <!-- AUTO-SHIELDING CONSOLE -->
        <div class="flex-1 glass rounded-2xl border border-white/5 p-6 flex flex-col">
          <div class="flex items-center justify-between mb-6">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-lg bg-indigo-500/10 flex items-center justify-center text-indigo-400">
                <svg width="20" height="20" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" /></svg>
              </div>
              <div>
                <h3 class="text-lg font-bold text-white">Auto-Shielding</h3>
                <p class="text-xs text-gray-400">Automated privacy for mined coins.</p>
              </div>
            </div>
            
            <!-- Toggle Switch -->
            <button 
              @click="autoShieldEnabled = !autoShieldEnabled"
              class="w-12 h-6 rounded-full p-1 transition-colors duration-300 ease-in-out relative"
              :class="autoShieldEnabled ? 'bg-indigo-600' : 'bg-gray-700'"
            >
              <div 
                class="w-4 h-4 bg-white rounded-full shadow-md transform transition-transform duration-300"
                :class="autoShieldEnabled ? 'translate-x-6' : 'translate-x-0'"
              ></div>
            </button>
          </div>

          <div class="flex-1 flex flex-col gap-4">
             <!-- Target Address Selector -->
             <div class="relative group">
                <label class="block text-xs font-bold text-gray-500 uppercase mb-2 ml-1">Destination Shielded Address</label>
                <div class="relative">
                  <select 
                    v-model="shieldToAddress" 
                    class="w-full appearance-none bg-black/20 border border-white/10 rounded-xl px-4 py-3 text-sm font-mono text-gray-300 focus:outline-none focus:border-indigo-500/50 focus:bg-black/40 transition-colors cursor-pointer"
                  >
                    <option value="" disabled>Select Address...</option>
                    <option v-for="addr in wallet.addresses" :key="addr" :value="addr">{{ addr }}</option>
                  </select>
                  <div class="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none text-gray-500">
                    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 9l6 6 6-6"/></svg>
                  </div>
                </div>
             </div>

             <!-- Console Log Output -->
             <div class="mt-auto bg-black/40 rounded-xl p-4 border border-white/5 font-mono text-xs h-24 overflow-y-auto custom-scrollbar flex flex-col-reverse">
                <div v-if="lastOperation" class="text-emerald-400">
                  <span class="text-gray-600">[{{ new Date().toLocaleTimeString() }}]</span> Success: OpID {{ lastOperation.substring(0, 12) }}...
                </div>
                <div v-if="isShielding" class="text-indigo-400 animate-pulse">
                  <span class="text-gray-600">[{{ new Date().toLocaleTimeString() }}]</span> Initiating shielding transaction...
                </div>
                <div v-if="autoShieldEnabled" class="text-gray-500">
                   <span class="text-gray-600">[{{ new Date().toLocaleTimeString() }}]</span> Service active. Monitoring coinbase...
                </div>
                <div class="text-gray-600 italic">System ready.</div>
             </div>

             <button 
                @click="performShield"
                :disabled="!shieldToAddress || isShielding"
                class="w-full py-2.5 rounded-lg border border-white/10 hover:bg-white/5 text-xs font-bold text-gray-400 hover:text-white transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
             >
                Trigger Manual Shield
             </button>
          </div>
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

// Thread configuration
const maxThreads = ref(navigator.hardwareConcurrency || 4);
const selectedThreads = ref(-1); // -1 = auto (uses ~half)

// Data State
const miningInfo = reactive({
  localsolps: 0,
  networksolps: 0,
  difficulty: 0,
  blocks: 0,
  reward: 0,
  generate: false
});

let statsInterval: any = null;
let shieldInterval: any = null;

onMounted(async () => {
  await wallet.fetchAddresses();
  // Auto-select first unified address
  const uni = wallet.addresses.find(a => a.startsWith('j1'));
  if (uni) shieldToAddress.value = uni;
  else if (wallet.addresses.length > 0) shieldToAddress.value = wallet.addresses[0];

  await fetchMiningInfo();
  statsInterval = setInterval(fetchMiningInfo, 5000); 
});

async function fetchMiningInfo() {
  try {
    const res = await invoke<any>('get_mining_info', {
      port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass
    });
    
    miningInfo.localsolps = res.localsolps || 0;
    miningInfo.networksolps = res.networksolps || 0;
    miningInfo.difficulty = res.difficulty || 0;
    miningInfo.blocks = res.blocks || 0;
    miningInfo.generate = res.generate || false;
    isMining.value = res.generate;

    if (miningInfo.blocks > 0) {
      const subsidy = await invoke<number>('get_block_subsidy', {
        height: miningInfo.blocks,
        port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass
      });
      miningInfo.reward = subsidy;
    }
  } catch (e) {
    // Silent fail for polling
  }
}

async function toggleMining() {
  const newState = !isMining.value;
  isMining.value = newState; // Optimistic update
  
  try {
    await invoke('set_mining', {
      enabled: newState, 
      threads: selectedThreads.value, 
      port: node.rpcPort, 
      user: node.rpcUser, 
      pass: node.rpcPass
    });
    setTimeout(fetchMiningInfo, 1000);
  } catch(e) {
    console.error(e);
    isMining.value = !newState; // Revert on error
  }
}

// Watcher for Auto-Shielding
watch(autoShieldEnabled, (enabled) => {
  if (enabled) {
    shieldInterval = setInterval(() => {
        if(isMining.value) performShield();
    }, 300000); // 5 minutes
  } else {
    clearInterval(shieldInterval);
  }
});

async function performShield() {
  if (!shieldToAddress.value) return;
  
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

// Formatters
function formatHashrate(val: number) {
  if (!val) return '0 H/s';
  if (val >= 1_000_000) return (val / 1_000_000).toFixed(2) + ' MH/s';
  if (val >= 1_000) return (val / 1_000).toFixed(2) + ' kH/s';
  return val.toFixed(2) + ' H/s';
}

// Splitters for the large UI display
function formatHashrateNumber(val: number) {
  if (!val) return '0.00';
  if (val >= 1_000_000) return (val / 1_000_000).toFixed(2);
  if (val >= 1_000) return (val / 1_000).toFixed(2);
  return val.toFixed(2);
}

function formatHashrateUnit(val: number) {
  if (!val) return 'H/s';
  if (val >= 1_000_000) return 'MH/s';
  if (val >= 1_000) return 'kH/s';
  return 'H/s';
}

function formatDiff(val: number) {
  if (!val) return '0';
  if (val >= 1_000_000) return (val / 1_000_000).toFixed(2) + 'M';
  return val.toLocaleString();
}

onUnmounted(() => {
  clearInterval(statsInterval);
  clearInterval(shieldInterval);
});
</script>

<style scoped>
.animate-spin-slow {
  animation: spin 3s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* Custom range slider styling */
input[type="range"] {
  -webkit-appearance: none;
}

input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #a855f7;
  cursor: pointer;
  box-shadow: 0 0 8px rgba(168, 85, 247, 0.5);
}

input[type="range"]::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #a855f7;
  cursor: pointer;
  border: none;
  box-shadow: 0 0 8px rgba(168, 85, 247, 0.5);
}
</style>
