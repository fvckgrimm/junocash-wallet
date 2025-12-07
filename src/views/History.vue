<template>
  <div class="space-y-6 min-h-full pb-8">
    
    <!-- Header -->
    <header class="flex items-center justify-between animate-fade-in">
      <div>
        <h1 class="text-2xl font-bold text-white tracking-tight">Transaction History</h1>
        <p class="text-gray-400 text-sm mt-1">Complete record of your activity</p>
      </div>
      <button 
        @click="refreshData" 
        class="group flex items-center gap-2 px-4 py-2 rounded-lg bg-white/5 hover:bg-white/10 border border-white/5 transition-all active:scale-95"
        :disabled="isRefreshing"
      >
        <span class="text-indigo-400 group-hover:text-indigo-300 transition-colors" :class="{ 'animate-spin': isRefreshing }">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/><path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/><path d="M16 21h5v-5"/></svg>
        </span>
        <span class="text-sm font-medium text-gray-300">{{ isRefreshing ? 'Syncing...' : 'Refresh' }}</span>
      </button>
    </header>

    <!-- Filters -->
    <div class="glass rounded-xl p-4 border border-white/5 flex gap-3">
      <button 
        v-for="filter in filters" 
        :key="filter.value"
        @click="activeFilter = filter.value"
        class="px-4 py-2 rounded-lg text-sm font-medium transition-all"
        :class="activeFilter === filter.value 
          ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30' 
          : 'bg-white/5 text-gray-400 hover:text-gray-200 hover:bg-white/10'"
      >
        {{ filter.label }}
      </button>
    </div>

    <!-- Transactions List -->
    <div class="glass rounded-2xl border border-white/5 overflow-hidden">
      
      <!-- Empty State -->
      <div v-if="filteredTransactions.length === 0" class="flex flex-col items-center justify-center p-16 text-center">
        <div class="w-20 h-20 bg-white/5 rounded-full flex items-center justify-center mb-4 text-gray-500">
          <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
            <path d="M12 8v4"/>
            <path d="M12 16h.01"/>
          </svg>
        </div>
        <p class="text-gray-400 font-medium text-lg">No transactions found</p>
        <p class="text-gray-600 text-sm mt-2">{{ getEmptyMessage() }}</p>
      </div>

      <!-- List -->
      <div v-else class="overflow-y-auto custom-scrollbar max-h-[calc(100vh-300px)]">
        <ul class="divide-y divide-white/5">
          <li v-for="(tx, index) in filteredTransactions" :key="tx.txid" 
              class="group px-6 py-5 flex items-center justify-between hover:bg-white/[0.03] transition-colors cursor-default"
              :style="{ animationDelay: `${index * 30}ms` }">
            
            <div class="flex items-center gap-4 flex-1 min-w-0">
              <!-- Icon -->
              <div class="w-12 h-12 rounded-full flex items-center justify-center border border-white/10 shadow-lg flex-shrink-0" :class="getTxStyles(tx.category).bg">
                <component :is="getTxIcon(tx.category)" class="w-6 h-6" :class="getTxStyles(tx.category).icon" />
              </div>
              
              <!-- Info -->
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-1">
                  <p class="font-bold text-gray-200 capitalize">{{ tx.category }}</p>
                  <span v-if="tx.category === 'generate'" class="px-2 py-0.5 rounded text-[10px] font-bold bg-purple-500/20 text-purple-300 uppercase">Mined</span>
                </div>
                <div class="flex items-center gap-2 flex-wrap">
                  <p class="text-xs text-gray-500 font-mono group-hover:text-indigo-400 transition-colors cursor-pointer truncate max-w-[300px]" 
                     :title="tx.txid"
                     @click="copyToClipboard(tx.txid)">
                    {{ tx.txid }}
                  </p>
                  <span class="text-xs text-gray-600">•</span>
                  <p class="text-xs text-gray-500">{{ formatDate(tx.time) }}</p>
                  <span class="text-xs text-gray-600">•</span>
                  <p class="text-xs" :class="tx.confirmations >= 10 ? 'text-emerald-500/70' : 'text-amber-500/70'">
                    {{ tx.confirmations >= 10 ? 'Confirmed' : `${tx.confirmations} confirmations` }}
                  </p>
                </div>
              </div>
            </div>

            <!-- Amount -->
            <div class="text-right flex-shrink-0 ml-4">
              <p class="font-bold text-xl tracking-tight font-mono" :class="tx.amount > 0 ? 'text-emerald-400' : 'text-gray-200'">
                {{ tx.amount > 0 ? '+' : '' }}{{ tx.amount.toFixed(4) }}
              </p>
              <p class="text-xs text-gray-500 font-sans font-medium">JUNO</p>
            </div>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useWalletStore } from '../stores/walletStore';

// Icons (reuse from Dashboard)
const ReceiveIcon = { template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 5v14"/><path d="m19 12-7 7-7-7"/></svg>` };
const SendIcon = { template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 19V5"/><path d="m5 12 7-7 7 7"/></svg>` };
const MineIcon = { template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/></svg>` };

const wallet = useWalletStore();
const isRefreshing = ref(false);
const activeFilter = ref('all');

const filters = [
  { label: 'All', value: 'all' },
  { label: 'Received', value: 'receive' },
  { label: 'Sent', value: 'send' },
  { label: 'Mined', value: 'generate' },
];

const filteredTransactions = computed(() => {
  if (activeFilter.value === 'all') {
    return wallet.transactions;
  }
  return wallet.transactions.filter(tx => tx.category === activeFilter.value);
});

async function refreshData() {
  isRefreshing.value = true;
  await wallet.fetchTransactions();
  setTimeout(() => isRefreshing.value = false, 500);
}

function getTxIcon(category: string) {
  if (category === 'receive') return ReceiveIcon;
  if (category === 'generate') return MineIcon;
  return SendIcon;
}

function getTxStyles(category: string) {
  if (category === 'receive') return { bg: 'bg-emerald-500/10 border-emerald-500/20', icon: 'text-emerald-400' };
  if (category === 'generate') return { bg: 'bg-purple-500/10 border-purple-500/20', icon: 'text-purple-400' };
  return { bg: 'bg-gray-700/30 border-gray-600/30', icon: 'text-gray-300' };
}

function formatDate(timestamp: number) {
  const date = new Date(timestamp * 1000);
  const today = new Date();
  if (date.toDateString() === today.toDateString()) {
    return `Today, ${date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`;
  }
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);
  if (date.toDateString() === yesterday.toDateString()) {
    return `Yesterday, ${date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`;
  }
  return date.toLocaleDateString([], { month: 'short', day: 'numeric', year: 'numeric', hour: '2-digit', minute: '2-digit' });
}

function getEmptyMessage() {
  switch (activeFilter.value) {
    case 'receive': return 'No received transactions yet';
    case 'send': return 'No sent transactions yet';
    case 'generate': return 'No mined blocks yet';
    default: return 'Your transaction history will appear here';
  }
}

async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text);
    // You could add a toast notification here
  } catch (err) {
    console.error('Failed to copy:', err);
  }
}
</script>
