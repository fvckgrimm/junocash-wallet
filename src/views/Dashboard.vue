<template>
  <div class="space-y-8 min-h-full pb-8">
    
    <!-- Header Section -->
    <header class="flex items-center justify-between animate-fade-in">
      <div>
        <h1 class="text-2xl font-bold text-white tracking-tight">Portfolio Overview</h1>
        <p class="text-gray-400 text-sm mt-1">Welcome back to your private bank.</p>
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

    <!-- Balance Cards Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      
      <!-- HERO CARD: Total Balance -->
      <div class="lg:col-span-2 relative overflow-hidden rounded-2xl bg-gradient-to-br from-indigo-900/50 to-purple-900/50 border border-white/10 shadow-2xl animate-slide-up" style="animation-delay: 100ms;">
        <!-- Background Mesh Effect -->
        <div class="absolute top-0 right-0 -mr-16 -mt-16 w-64 h-64 bg-indigo-500/30 rounded-full blur-3xl"></div>
        <div class="absolute bottom-0 left-0 -ml-16 -mb-16 w-64 h-64 bg-purple-500/20 rounded-full blur-3xl"></div>
        
        <div class="relative p-8 flex flex-col justify-between h-full">
          <div>
            <h3 class="text-indigo-200 font-medium text-sm uppercase tracking-wider mb-1">Total Balance</h3>
            <div class="flex items-baseline gap-2">
              <span class="text-5xl font-bold text-white tracking-tighter">
                {{ displayBalance.whole }}<span class="text-3xl text-indigo-300/70">.{{ displayBalance.decimal }}</span>
              </span>
              <span class="text-xl font-bold text-indigo-400">JUNO</span>
            </div>
          </div>

          <!-- Privacy Ratio Bar -->
          <div class="mt-8">
            <div class="flex justify-between text-xs font-medium text-gray-400 mb-2">
              <span class="flex items-center gap-1"><div class="w-2 h-2 rounded-full bg-emerald-400 shadow-[0_0_8px_rgba(52,211,153,0.5)]"></div> Shielded ({{ privacyRatio }}%)</span>
              <span class="flex items-center gap-1">Transparent <div class="w-2 h-2 rounded-full bg-amber-400"></div></span>
            </div>
            <div class="h-3 w-full bg-black/40 rounded-full overflow-hidden flex">
              <div 
                class="h-full bg-gradient-to-r from-emerald-500 to-emerald-400 shadow-[0_0_15px_rgba(16,185,129,0.4)] transition-all duration-1000 ease-out"
                :style="{ width: `${privacyRatio}%` }"
              ></div>
              <div class="flex-1 bg-white/5"></div>
            </div>
          </div>
        </div>
      </div>

      <!-- SIDE CARDS: Breakdown -->
      <div class="flex flex-col gap-4 animate-slide-up" style="animation-delay: 200ms;">
        
        <!-- Shielded Card -->
        <div class="flex-1 glass p-5 rounded-2xl flex flex-col justify-center relative overflow-hidden group">
          <div class="absolute right-[-10px] top-[-10px] opacity-10 group-hover:opacity-20 transition-opacity rotate-12">
             <svg width="80" height="80" viewBox="0 0 24 24" fill="currentColor" class="text-emerald-400"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
          </div>
          <div class="flex items-center gap-3 mb-2">
            <div class="p-2 bg-emerald-500/20 rounded-lg text-emerald-400">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
            </div>
            <span class="text-sm text-gray-300 font-medium">Shielded</span>
          </div>
          <p class="text-2xl font-bold text-white font-mono tracking-tight">
            {{ wallet.balance.private.toFixed(4) }}
          </p>
        </div>

        <!-- Transparent Card -->
        <div class="flex-1 glass p-5 rounded-2xl flex flex-col justify-center relative overflow-hidden group">
          <div class="absolute right-[-10px] top-[-10px] opacity-10 group-hover:opacity-20 transition-opacity rotate-12">
             <svg width="80" height="80" viewBox="0 0 24 24" fill="currentColor" class="text-amber-400"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
          </div>
          <div class="flex items-center gap-3 mb-2">
            <div class="p-2 bg-amber-500/20 rounded-lg text-amber-400">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
            </div>
            <span class="text-sm text-gray-300 font-medium">Transparent</span>
          </div>
          <p class="text-2xl font-bold text-white font-mono tracking-tight">
            {{ wallet.balance.transparent.toFixed(4) }}
          </p>
        </div>

      </div>
    </div>

    <!-- Recent Transactions -->
    <div class="animate-slide-up" style="animation-delay: 300ms;">
      <div class="glass rounded-2xl border border-white/5 overflow-hidden flex flex-col min-h-[400px]">
        <div class="px-6 py-5 border-b border-white/5 bg-white/[0.02] flex justify-between items-center">
          <h2 class="font-bold text-lg text-white">Activity</h2>
          <router-link to="/history" class="text-xs font-medium text-indigo-400 hover:text-indigo-300 uppercase tracking-wider transition-colors">View All</router-link>
        </div>

        <div class="flex-1 overflow-y-auto custom-scrollbar">
          <!-- Empty State -->
          <div v-if="wallet.transactions.length === 0" class="h-full flex flex-col items-center justify-center p-12 text-center">
            <div class="w-16 h-16 bg-white/5 rounded-full flex items-center justify-center mb-4 text-gray-500">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><path d="M12 8v4"/><path d="M12 16h.01"/></svg>
            </div>
            <p class="text-gray-400 font-medium">No transactions yet</p>
            <p class="text-gray-600 text-sm mt-1 max-w-xs">Your recent mining rewards, sent, and received transactions will appear here.</p>
          </div>

          <!-- List -->
          <ul v-else class="divide-y divide-white/5">
            <li v-for="(tx, index) in wallet.recentTransactions.slice(0, 10)" :key="tx.txid" 
                class="group px-6 py-4 flex items-center justify-between hover:bg-white/[0.03] transition-colors cursor-default"
                :style="{ animationDelay: `${index * 50}ms` }">
              
              <div class="flex items-center gap-4">
                <!-- Icon -->
                <div class="w-10 h-10 rounded-full flex items-center justify-center border border-white/10 shadow-lg" :class="getTxStyles(tx.category).bg">
                  <component :is="getTxIcon(tx.category)" class="w-5 h-5" :class="getTxStyles(tx.category).icon" />
                </div>
                
                <!-- Info -->
                <div>
                  <div class="flex items-center gap-2">
                    <p class="font-bold text-gray-200 capitalize">{{ tx.category }}</p>
                    <span v-if="tx.category === 'generate'" class="px-1.5 py-0.5 rounded text-[10px] font-bold bg-purple-500/20 text-purple-300 uppercase">Mined</span>
                  </div>
                  <div class="flex items-center gap-2 mt-0.5">
                     <p class="text-xs text-gray-500 font-mono group-hover:text-indigo-400 transition-colors cursor-pointer" title="Copy ID">
                      {{ tx.txid.substring(0, 8) }}...{{ tx.txid.substring(tx.txid.length - 8) }}
                    </p>
                    <span class="text-xs text-gray-600">•</span>
                    <p class="text-xs text-gray-500">{{ formatDate(tx.time) }}</p>
                  </div>
                </div>
              </div>

              <!-- Amount -->
              <div class="text-right">
                <p class="font-bold text-lg tracking-tight font-mono" :class="tx.amount > 0 ? 'text-emerald-400' : 'text-gray-200'">
                  {{ tx.amount > 0 ? '+' : '' }}{{ tx.amount }} <span class="text-sm font-sans font-medium text-gray-500">JUNO</span>
                </p>
                <p class="text-xs text-gray-500" v-if="tx.confirmations < 10">
                  {{ tx.confirmations }} Confirmations
                </p>
                <p class="text-xs text-emerald-500/70" v-else>
                  Confirmed
                </p>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue';
import { useWalletStore } from '../stores/walletStore';

// Icons
const ReceiveIcon = { template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 5v14"/><path d="m19 12-7 7-7-7"/></svg>` };
const SendIcon = { template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 19V5"/><path d="m5 12 7-7 7 7"/></svg>` };
const MineIcon = { template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/></svg>` };

const wallet = useWalletStore();
const isRefreshing = ref(false);

// Animated CountUp Logic
const animatedTotal = ref(0);

// Computed Display Properties
const displayBalance = computed(() => {
  const val = animatedTotal.value.toFixed(4);
  const [whole, decimal] = val.split('.');
  return { whole, decimal };
});

const privacyRatio = computed(() => {
  const total = wallet.balance.private + wallet.balance.transparent;
  if (total === 0) return 0;
  return Math.round((wallet.balance.private / total) * 100);
});

// Animation Loop for Balance
watch(() => wallet.formattedBalance, (newVal) => {
  animateValue(parseFloat(newVal));
}, { immediate: true });

function animateValue(target: number) {
  const start = animatedTotal.value;
  const duration = 1000;
  const startTime = performance.now();

  function update(currentTime: number) {
    const elapsed = currentTime - startTime;
    const progress = Math.min(elapsed / duration, 1);
    
    // Ease Out Quint
    const ease = 1 - Math.pow(1 - progress, 5);
    
    animatedTotal.value = start + (target - start) * ease;

    if (progress < 1) {
      requestAnimationFrame(update);
    }
  }
  requestAnimationFrame(update);
}

// Data Handling
onMounted(async () => {
  await refreshData();
});

async function refreshData() {
  isRefreshing.value = true;
  await Promise.all([
    wallet.fetchBalance(),
    wallet.fetchTransactions()
  ]);
  setTimeout(() => isRefreshing.value = false, 500); // Minimum spin time
}

// Helpers
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
  // If today
  const today = new Date();
  if (date.toDateString() === today.toDateString()) {
    return `Today, ${date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`;
  }
  return date.toLocaleDateString([], { month: 'short', day: 'numeric', year: 'numeric' });
}
</script>
