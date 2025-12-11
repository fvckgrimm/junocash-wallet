<template>
  <div class="h-full max-w-6xl mx-auto flex flex-col p-2">
    
    <!-- Header -->
    <div class="flex items-center justify-between mb-6 px-2 animate-fade-in">
      <div>
        <h1 class="text-2xl font-bold text-white tracking-tight">Receive Assets</h1>
        <p class="text-gray-400 text-sm">Manage your addresses and receive payments.</p>
      </div>
      <button 
        @click="openGenerator"
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
      
      <!-- LEFT COL: Address Selector -->
      <div class="lg:col-span-5 flex flex-col min-h-0 animate-slide-right">
        <div class="glass flex-1 rounded-2xl border border-white/5 overflow-hidden flex flex-col">
          <div class="px-5 py-4 border-b border-white/5 bg-white/[0.02]">
            <h2 class="text-xs font-bold text-gray-500 uppercase tracking-wider">Your Wallet Addresses</h2>
          </div>
          
          <div class="flex-1 overflow-y-auto custom-scrollbar p-3 space-y-2">
            <div v-if="wallet.addresses.length === 0" class="h-40 flex flex-col items-center justify-center text-center text-gray-500">
              <p>No addresses yet.</p>
              <button @click="openGenerator" class="text-indigo-400 text-sm hover:underline mt-2">Generate your first one</button>
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

      <!-- RIGHT COL: QR Stage -->
      <div class="lg:col-span-7 flex flex-col animate-scale-in">
        <div class="glass flex-1 rounded-2xl border border-white/5 p-8 flex flex-col items-center justify-center relative overflow-hidden">
          
          <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-64 h-64 bg-indigo-500/20 rounded-full blur-[100px] pointer-events-none"></div>

          <transition name="fade" mode="out-in">
            <div v-if="selectedAddress" :key="selectedAddress" class="relative z-10 flex flex-col items-center max-w-md w-full">
              
              <div class="bg-white p-4 rounded-2xl shadow-[0_0_50px_rgba(0,0,0,0.5)] mb-8 transform transition-transform hover:scale-105 duration-300">
                <QrcodeVue :value="selectedAddress" :size="240" level="M" render-as="svg" foreground="#000000" background="#ffffff" />
              </div>

              <div class="w-full mb-6 group cursor-pointer" @click="copyToClipboard">
                <p class="text-center text-xs text-gray-400 uppercase font-bold tracking-widest mb-2">Wallet Address</p>
                <div class="bg-black/40 border border-white/10 rounded-xl p-4 break-all font-mono text-sm text-center text-gray-300 group-hover:text-white group-hover:border-indigo-500/30 transition-colors">
                  {{ selectedAddress }}
                </div>
              </div>

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
            <div v-else class="text-center text-gray-500">
              Select an address to view details.
            </div>
          </transition>
        </div>
      </div>
    </div>

    <!-- ============================================= -->
    <!--         ADDRESS GENERATOR MODAL               -->
    <!-- ============================================= -->
    <transition name="modal">
      <div v-if="genModal.isOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
        <!-- Backdrop -->
        <div class="absolute inset-0 bg-black/80 backdrop-blur-xl" @click="genModal.isOpen = false"></div>

        <!-- Modal -->
        <div class="relative bg-[#181B24] w-full max-w-lg rounded-3xl border border-white/10 shadow-2xl overflow-hidden flex flex-col animate-scale-in">
          
          <!-- Header -->
          <div class="p-6 border-b border-white/5 bg-white/[0.02]">
            <h2 class="text-xl font-bold text-white">Generate Address</h2>
            <p class="text-gray-400 text-sm">Create a new receiving address.</p>
          </div>

          <div class="p-6 space-y-6">
            
            <!-- 1. Address Type Selection -->
            <div class="space-y-3">
              <label class="text-xs font-bold text-gray-500 uppercase tracking-wide">Address Type</label>
              <div class="grid grid-cols-2 gap-4">
                
                <!-- Unified Option -->
                <button 
                  @click="genModal.type = 'unified'"
                  class="relative p-4 rounded-xl border transition-all text-left flex flex-col gap-2"
                  :class="genModal.type === 'unified' ? 'bg-indigo-500/10 border-indigo-500 ring-1 ring-indigo-500' : 'bg-black/20 border-white/10 hover:border-white/30'"
                >
                   <div class="p-2 bg-indigo-500 rounded-lg w-fit text-white">
                      <component :is="ShieldIcon" class="w-5 h-5" />
                   </div>
                   <div>
                     <span class="block text-sm font-bold text-white">Unified</span>
                     <span class="text-[10px] text-gray-400">Private (Orchard)</span>
                   </div>
                   <div v-if="genModal.type === 'unified'" class="absolute top-3 right-3 text-indigo-500">
                     <svg width="16" height="16" fill="currentColor" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
                   </div>
                </button>

                <!-- Transparent Option -->
                <button 
                  @click="genModal.type = 'transparent'"
                  class="relative p-4 rounded-xl border transition-all text-left flex flex-col gap-2"
                  :class="genModal.type === 'transparent' ? 'bg-amber-500/10 border-amber-500 ring-1 ring-amber-500' : 'bg-black/20 border-white/10 hover:border-white/30'"
                >
                   <div class="p-2 bg-amber-500 rounded-lg w-fit text-white">
                      <component :is="GlobeIcon" class="w-5 h-5" />
                   </div>
                   <div>
                     <span class="block text-sm font-bold text-white">Transparent</span>
                     <span class="text-[10px] text-gray-400">Public (Mining)</span>
                   </div>
                   <div v-if="genModal.type === 'transparent'" class="absolute top-3 right-3 text-amber-500">
                     <svg width="16" height="16" fill="currentColor" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
                   </div>
                </button>

              </div>
            </div>

            <!-- 2. Account Selection (Only for Unified) -->
            <transition name="fade" mode="out-in">
              <div v-if="genModal.type === 'unified'" class="space-y-3">
                <label class="text-xs font-bold text-gray-500 uppercase tracking-wide">Destination Account</label>
                
                <div class="relative">
                  <select 
                    v-model="genModal.targetAccount"
                    class="w-full appearance-none bg-black/40 border border-white/10 rounded-xl py-3 pl-4 pr-10 text-sm text-white font-mono shadow-inner focus:outline-none focus:border-indigo-500/50"
                  >
                    <option :value="null">✚ Create New Account</option>
                    <option disabled>──────────────</option>
                    <option v-for="acc in genModal.availableAccounts" :key="acc" :value="acc">
                      Account #{{ acc }} {{ acc === 0 ? '(Main)' : '' }}
                    </option>
                  </select>
                  <div class="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none text-gray-500">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 9l6 6 6-6"/></svg>
                  </div>
                </div>

                <!-- Info text -->
                <p v-if="genModal.targetAccount === null" class="text-xs text-amber-400 bg-amber-500/10 p-2 rounded-lg border border-amber-500/20">
                  Creates a separate bucket of funds. These funds cannot be mixed with Account #0 without an on-chain transaction.
                </p>
                <p v-else class="text-xs text-gray-500">
                  Adds a new diversified address to Account #{{ genModal.targetAccount }}. Balances will be shared.
                </p>
              </div>

              <!-- Info for Transparent Mode -->
              <div v-else class="bg-blue-500/10 border border-blue-500/20 p-3 rounded-xl space-y-2">
                 <p class="text-xs text-blue-200 leading-relaxed">
                   <strong>For Mining Rewards:</strong> Transparent addresses are used to receive Coinbase rewards. These funds must be <strong>shielded</strong> before they can be spent.
                 </p>
                 <p class="text-[10px] text-blue-300/60">
                   Note: Transparent addresses are generated globally and do not belong to a specific Unified Account.
                 </p>
              </div>
            </transition>

          </div>

          <!-- Actions -->
          <div class="p-6 pt-0 flex gap-3">
            <button @click="genModal.isOpen = false" class="flex-1 py-3 rounded-xl bg-white/5 hover:bg-white/10 text-gray-400 font-bold transition-colors">
              Cancel
            </button>
            <button 
              @click="submitGeneration" 
              :disabled="genModal.isBusy"
              class="flex-1 py-3 rounded-xl font-bold text-white shadow-lg transition-all active:scale-95 flex items-center justify-center gap-2"
              :class="genModal.isBusy ? 'bg-gray-700' : 'bg-indigo-600 hover:bg-indigo-500 shadow-indigo-900/20'"
            >
              <span v-if="genModal.isBusy" class="animate-spin w-4 h-4 border-2 border-white/30 border-t-white rounded-full"></span>
              {{ genModal.isBusy ? 'Generating...' : 'Generate Address' }}
            </button>
          </div>

        </div>
      </div>
    </transition>

  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import QrcodeVue from 'qrcode.vue';
import { invoke } from '@tauri-apps/api/core';
import { useNodeStore } from '../stores/nodeStore';
import { useWalletStore } from '../stores/walletStore';

// Icons Components
const ShieldIcon = { template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>` };
const GhostIcon = { template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 10h.01"/><path d="M15 10h.01"/><path d="M12 2a8 8 0 0 0-8 8v12l3-3 2.5 2.5L12 19l2.5 2.5L17 19l3 3V10a8 8 0 0 0-8-8z"/></svg>` };
const GlobeIcon = { template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>` };

const node = useNodeStore();
const wallet = useWalletStore();
const selectedAddress = ref('');
const copied = ref(false);

// Generator Modal State
const genModal = reactive({
  isOpen: false,
  isBusy: false,
  type: 'unified' as 'unified' | 'transparent',
  targetAccount: 0 as number | null, // null means "New Account"
  availableAccounts: [] as number[]
});

onMounted(async () => {
  await wallet.fetchAddresses();
  if (wallet.addresses.length > 0) {
    const uni = wallet.addresses.find(a => a.startsWith('j1'));
    selectedAddress.value = uni || wallet.addresses[0];
  }
});

// Open Modal & Fetch Accounts
async function openGenerator() {
  genModal.isOpen = true;
  genModal.targetAccount = 0; // Default to Main
  genModal.type = 'unified';
  
  // Fetch accounts from Rust
  const accounts = await wallet.fetchAccounts();
  genModal.availableAccounts = accounts;
}

async function submitGeneration() {
  genModal.isBusy = true;
  
  try {
    const newAddr = await invoke<string>('get_new_address', {
      accountTarget: genModal.type === 'transparent' ? 0 : genModal.targetAccount, // Send dummy 0 for transparent
      typeParam: genModal.type,
      host: node.rpcHost,
      port: node.rpcPort, 
      user: node.rpcUser, 
      pass: node.rpcPass
    });
    
    // Refresh & Select
    await wallet.fetchAddresses();
    selectedAddress.value = newAddr;
    
    // Close Modal
    genModal.isOpen = false;
  } catch(e: any) {
    alert("Error: " + e.toString());
  } finally {
    genModal.isBusy = false;
  }
}

function copyToClipboard() {
  if (!selectedAddress.value) return;
  navigator.clipboard.writeText(selectedAddress.value);
  copied.value = true;
  setTimeout(() => copied.value = false, 2000);
}

// --- Display Helpers ---

function getTypeLabel(addr: string) {
  if (addr.startsWith('j1')) return 'Unified (Orchard)';
  if (addr.startsWith('zs')) return 'Sapling (Legacy)';
  if (addr.startsWith('t1')) return 'Transparent (Mining)';
  return 'Legacy Address';
}

function getTypeIcon(addr: string) {
  if (addr.startsWith('j1')) return ShieldIcon;
  if (addr.startsWith('zs')) return GhostIcon;
  return GlobeIcon;
}

function getTypeColor(addr: string, isActive: boolean) {
  if (isActive) return 'text-white bg-indigo-500';
  if (addr.startsWith('j1')) return 'text-indigo-400 bg-indigo-500/10';
  if (addr.startsWith('zs')) return 'text-emerald-400 bg-emerald-500/10';
  return 'text-amber-400 bg-amber-500/10';
}

function getPrivacyTitle(addr: string) {
  if (addr.startsWith('j1') || addr.startsWith('zs')) return 'Privacy Protected';
  return 'Publicly Visible';
}

function getPrivacyDesc(addr: string) {
  if (addr.startsWith('j1')) return 'Orchard Shielded. Maximum privacy. Funds are hidden from the blockchain.';
  if (addr.startsWith('zs')) return 'Legacy Shielded address. Private, but Unified is recommended.';
  return 'Transparent. Used for mining rewards. Visible on-chain. Must shield to spend.';
}
</script>

<style scoped>
.animate-scale-in {
  animation: scaleIn 0.3s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
}
@keyframes scaleIn {
  from { opacity: 0; transform: scale(0.95); }
  to { opacity: 1; transform: scale(1); }
}

.modal-enter-active, .modal-leave-active {
  transition: opacity 0.2s ease;
}
.modal-enter-from, .modal-leave-to {
  opacity: 0;
}

.fade-enter-active, .fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
  transform: translateY(5px);
}
</style>
