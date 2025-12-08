<template>
  <div class="h-full max-w-6xl mx-auto flex flex-col p-2">
    
    <!-- Header -->
    <div class="flex items-center justify-between mb-6 px-2 animate-fade-in">
      <div>
        <h1 class="text-2xl font-bold text-white tracking-tight">Receive Assets</h1>
        <p class="text-gray-400 text-sm">Manage your addresses and receive payments.</p>
      </div>
      <button 
        @click="generateNew"
        class="group flex items-center gap-2 px-4 py-2 rounded-xl bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-300 hover:text-white border border-indigo-500/20 transition-all active:scale-95"
      >
        <span class="bg-indigo-500 rounded-md p-0.5 text-white group-hover:rotate-90 transition-transform duration-300">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M12 5v14"/><path d="M5 12h14"/></svg>
        </span>
        <span class="text-sm font-medium">New Address</span>
      </button>
    </div>

    <!-- Main Content Grid -->
    <div class="flex-1 grid grid-cols-1 lg:grid-cols-12 gap-6 min-h-0">
      
      <!-- LEFT COL: Address Selector (Scrollable) -->
      <div class="lg:col-span-5 flex flex-col min-h-0 animate-slide-right">
        <div class="glass flex-1 rounded-2xl border border-white/5 overflow-hidden flex flex-col">
          <div class="px-5 py-4 border-b border-white/5 bg-white/[0.02]">
            <h2 class="text-xs font-bold text-gray-500 uppercase tracking-wider">Your Wallet Addresses</h2>
          </div>
          
          <div class="flex-1 overflow-y-auto custom-scrollbar p-3 space-y-2">
            <div v-if="wallet.addresses.length === 0" class="h-40 flex flex-col items-center justify-center text-center text-gray-500">
              <p>No addresses yet.</p>
              <button @click="generateNew" class="text-indigo-400 text-sm hover:underline mt-2">Generate your first one</button>
            </div>

            <button 
              v-for="(addr, index) in wallet.addresses" 
              :key="addr"
              @click="selectedAddress = addr"
              class="w-full text-left p-3 rounded-xl border transition-all duration-200 group relative overflow-hidden"
              :class="selectedAddress === addr 
                ? 'bg-indigo-600/10 border-indigo-500/50 shadow-[0_0_15px_rgba(99,102,241,0.1)]' 
                : 'bg-white/[0.02] border-transparent hover:bg-white/[0.05] hover:border-white/10'"
            >
              <div class="flex items-center gap-3 relative z-10">
                <!-- Icon based on type -->
                <div class="w-10 h-10 rounded-lg flex items-center justify-center shrink-0 transition-colors"
                  :class="getTypeColor(addr, selectedAddress === addr)">
                  <component :is="getTypeIcon(addr)" class="w-5 h-5" />
                </div>

                <div class="min-w-0 flex-1">
                  <div class="flex items-center gap-2 mb-0.5">
                    <span class="text-xs font-bold uppercase tracking-wide" 
                      :class="selectedAddress === addr ? 'text-white' : 'text-gray-400 group-hover:text-gray-300'">
                      {{ getTypeLabel(addr) }}
                    </span>
                    <span v-if="index === 0" class="text-[10px] bg-white/10 px-1.5 py-0.5 rounded text-gray-400">Default</span>
                  </div>
                  <p class="font-mono text-xs truncate transition-colors" 
                     :class="selectedAddress === addr ? 'text-indigo-200' : 'text-gray-500 group-hover:text-gray-400'">
                    {{ addr }}
                  </p>
                </div>
              </div>
            </button>
          </div>
        </div>
      </div>

      <!-- RIGHT COL: QR Stage (The "Hero") -->
      <div class="lg:col-span-7 flex flex-col animate-scale-in">
        <div class="glass flex-1 rounded-2xl border border-white/5 p-8 flex flex-col items-center justify-center relative overflow-hidden">
          
          <!-- Background Glow Effect -->
          <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-64 h-64 bg-indigo-500/20 rounded-full blur-[100px] pointer-events-none"></div>

          <transition name="fade" mode="out-in">
            <div v-if="selectedAddress" :key="selectedAddress" class="relative z-10 flex flex-col items-center max-w-md w-full">
              
              <!-- QR Card -->
              <div class="bg-white p-4 rounded-2xl shadow-[0_0_50px_rgba(0,0,0,0.5)] mb-8 transform transition-transform hover:scale-105 duration-300">
                <QrcodeVue :value="selectedAddress" :size="240" level="M" render-as="svg" foreground="#000000" background="#ffffff" />
              </div>

              <!-- Address Display -->
              <div class="w-full mb-6 group cursor-pointer" @click="copyToClipboard">
                <p class="text-center text-xs text-gray-400 uppercase font-bold tracking-widest mb-2">Wallet Address</p>
                <div class="bg-black/40 border border-white/10 rounded-xl p-4 break-all font-mono text-sm text-center text-gray-300 group-hover:text-white group-hover:border-indigo-500/30 transition-colors">
                  {{ selectedAddress }}
                </div>
              </div>

              <!-- Actions -->
              <div class="flex gap-4 w-full">
                <button 
                  @click="copyToClipboard" 
                  class="flex-1 h-12 rounded-xl font-bold text-sm transition-all flex items-center justify-center gap-2"
                  :class="copied ? 'bg-emerald-500 text-white shadow-[0_0_20px_rgba(16,185,129,0.4)]' : 'bg-indigo-600 hover:bg-indigo-500 text-white shadow-lg shadow-indigo-900/20'"
                >
                  <span v-if="!copied">Copy Address</span>
                  <span v-else class="flex items-center gap-2">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M20 6L9 17l-5-5"/></svg>
                    Copied!
                  </span>
                </button>
              </div>

              <!-- Privacy Notice -->
              <div class="mt-8 flex items-start gap-3 p-4 rounded-xl bg-white/[0.03] border border-white/5 max-w-sm">
                <div :class="getTypeColor(selectedAddress, true)" class="p-1 rounded bg-transparent">
                  <component :is="getTypeIcon(selectedAddress)" class="w-5 h-5" />
                </div>
                <div>
                  <h4 class="text-sm font-bold text-gray-200">{{ getPrivacyTitle(selectedAddress) }}</h4>
                  <p class="text-xs text-gray-400 mt-1 leading-relaxed">{{ getPrivacyDesc(selectedAddress) }}</p>
                </div>
              </div>

            </div>

            <!-- Empty State -->
            <div v-else class="text-center text-gray-500">
              Select an address to view details.
            </div>
          </transition>
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

// Icons
const ShieldIcon = { template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>` };
const GhostIcon = { template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 10h.01"/><path d="M15 10h.01"/><path d="M12 2a8 8 0 0 0-8 8v12l3-3 2.5 2.5L12 19l2.5 2.5L17 19l3 3V10a8 8 0 0 0-8-8z"/></svg>` }; // For Sapling
const GlobeIcon = { template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>` };

const node = useNodeStore();
const wallet = useWalletStore();
const selectedAddress = ref('');
const copied = ref(false);

onMounted(async () => {
  await wallet.fetchAddresses();
  if (wallet.addresses.length > 0) {
    const uni = wallet.addresses.find(a => a.startsWith('j1'));
    selectedAddress.value = uni || wallet.addresses[0];
  }
});

async function generateNew() {
  // Use a cleaner confirmation dialog in production, but browser confirm is okay for MVP
  if(!confirm("Generate a new Unified Address? This ensures maximum privacy.")) return;
  
  try {
    const newAddr = await invoke<string>('get_new_address', {
      typeParam: "unified",
      host: node.rpcHost,
      port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass
    });
    await wallet.fetchAddresses();
    selectedAddress.value = newAddr;
  } catch(e) {
    alert("Error: " + e);
  }
}

function copyToClipboard() {
  if (!selectedAddress.value) return;
  navigator.clipboard.writeText(selectedAddress.value);
  copied.value = true;
  setTimeout(() => copied.value = false, 2000);
}

// --- Helper Logic for UI Polish ---

function getTypeLabel(addr: string) {
  if (addr.startsWith('j1')) return 'Unified Address';
  if (addr.startsWith('zs')) return 'Sapling Address';
  if (addr.startsWith('t1')) return 'Transparent Address';
  return 'Legacy Address';
}

function getTypeIcon(addr: string) {
  if (addr.startsWith('j1')) return ShieldIcon;
  if (addr.startsWith('zs')) return GhostIcon;
  return GlobeIcon;
}

function getTypeColor(addr: string, isActive: boolean) {
  if (isActive) return 'text-white bg-indigo-500';
  
  if (addr.startsWith('j1')) return 'text-indigo-400 bg-indigo-500/10'; // Unified
  if (addr.startsWith('zs')) return 'text-emerald-400 bg-emerald-500/10'; // Sapling
  return 'text-amber-400 bg-amber-500/10'; // Transparent
}

function getPrivacyTitle(addr: string) {
  if (addr.startsWith('j1') || addr.startsWith('zs')) return 'Privacy Protected';
  return 'Publicly Visible';
}

function getPrivacyDesc(addr: string) {
  if (addr.startsWith('j1')) return 'This Unified Address supports all shielding pools. Balances and transaction history are hidden from the public blockchain.';
  if (addr.startsWith('zs')) return 'This is an older Shielded address generation. It is still private, but Unified addresses are recommended.';
  return 'Transactions to this address expose the amount and sender/receiver on the public blockchain. Use only when necessary.';
}
</script>

<style scoped>
/* Ensure QR Code has a slight bounce on hover */
.animate-scale-in {
  animation: scaleIn 0.4s ease-out forwards;
}
@keyframes scaleIn {
  from { opacity: 0; transform: scale(0.95); }
  to { opacity: 1; transform: scale(1); }
}
</style>
