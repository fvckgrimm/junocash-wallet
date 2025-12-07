<template>
  <div class="max-w-4xl mx-auto pt-4 relative">
    
    <!-- Header -->
    <div class="flex items-center justify-between mb-8 animate-fade-in">
      <div>
        <h1 class="text-2xl font-bold text-white tracking-tight">Send Funds</h1>
        <p class="text-gray-400 text-sm">Transfer assets to another wallet.</p>
      </div>
      <div class="text-right hidden sm:block">
        <p class="text-xs text-gray-400 uppercase tracking-wider font-semibold">Available to Spend</p>
        <p class="text-xl font-mono font-bold text-emerald-400">{{ wallet.balance.total.toFixed(4) }} JUNO</p>
      </div>
    </div>

    <!-- Main Card -->
    <div class="glass rounded-3xl p-1 relative overflow-hidden animate-slide-up">
      
      <transition name="fade" mode="out-in">
        
        <!-- STATE: SUCCESS -->
        <div v-if="successTxId" class="py-20 flex flex-col items-center justify-center text-center">
          <div class="w-20 h-20 bg-emerald-500 rounded-full flex items-center justify-center mb-6 shadow-[0_0_30px_rgba(16,185,129,0.4)] animate-scale-in">
            <svg class="w-10 h-10 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3"><path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" /></svg>
          </div>
          <h2 class="text-3xl font-bold text-white mb-2">Transaction Sent!</h2>
          <p class="text-gray-400 mb-8 max-w-md">Your funds are on the blockchain. It may take a few minutes to confirm.</p>
          
          <!-- Tx ID Card -->
          <div class="bg-black/30 border border-white/10 rounded-xl p-4 flex items-center gap-4 max-w-lg w-full mb-8">
            <div class="flex-1 overflow-hidden">
              <p class="text-xs text-gray-500 uppercase font-bold mb-1">Operation ID</p>
              <p class="font-mono text-sm text-indigo-300 truncate">{{ successTxId }}</p>
            </div>
            <button class="p-2 hover:bg-white/10 rounded-lg text-gray-400 hover:text-white transition-colors" title="Copy">
              <svg width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" /></svg>
            </button>
          </div>

          <button @click="resetForm" class="px-8 py-3 bg-white/10 hover:bg-white/20 text-white font-medium rounded-xl transition-all">
            Send Another
          </button>
        </div>

        <!-- STATE: FORM -->
        <form v-else @submit.prevent="handleSubmit" class="p-6 sm:p-8 space-y-8 bg-gradient-to-b from-white/[0.02] to-transparent rounded-[20px]">
          
          <!-- 1. Address Input -->
          <div class="space-y-3">
            <label class="block text-sm font-medium text-gray-300 ml-1">Recipient Address</label>
            <div class="relative group">
              <input 
                v-model="toAddress" 
                type="text" 
                placeholder="Paste address (zs1... or t1...)" 
                class="w-full pl-5 pr-14 py-4 bg-black/20 border border-white/10 rounded-xl text-gray-200 placeholder-gray-600 font-mono focus:outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all shadow-inner"
                :class="{'border-red-500/50 focus:border-red-500': addressError}"
                required
              />
              <!-- Address Type Indicator Icon -->
              <div class="absolute right-4 top-1/2 -translate-y-1/2 transition-all duration-300">
                <div v-if="addressType === 'shielded'" class="text-emerald-400 bg-emerald-500/10 p-1.5 rounded-lg border border-emerald-500/20" title="Shielded Address">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
                </div>
                <div v-else-if="addressType === 'transparent'" class="text-amber-400 bg-amber-500/10 p-1.5 rounded-lg border border-amber-500/20" title="Transparent Address">
                   <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>
                </div>
                <div v-else class="text-gray-600">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 6L9 17l-5-5"/></svg>
                </div>
              </div>
            </div>
            <!-- Dynamic Help Text -->
             <div class="h-5 text-xs ml-1">
                <transition name="fade">
                  <span v-if="addressType === 'shielded'" class="text-emerald-400/80 flex items-center gap-1">
                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span> Valid Shielded Address
                  </span>
                  <span v-else-if="addressType === 'transparent'" class="text-amber-400/80 flex items-center gap-1">
                    <span class="w-1.5 h-1.5 rounded-full bg-amber-400"></span> Transparent Address (Metadata visible)
                  </span>
                   <span v-else-if="addressError" class="text-red-400 flex items-center gap-1">
                    Invalid format
                  </span>
                </transition>
             </div>
          </div>

          <!-- 2. Amount Input (Hero Input) -->
          <div>
            <div class="flex justify-between mb-3 ml-1">
              <label class="text-sm font-medium text-gray-300">Amount</label>
              <button type="button" @click="setMax" class="text-xs font-bold text-indigo-400 hover:text-indigo-300 uppercase tracking-wider transition-colors">
                Max Amount
              </button>
            </div>
            
            <div 
              class="relative bg-black/20 rounded-2xl border border-white/5 hover:border-white/10 transition-colors py-8 px-4 flex flex-col items-center justify-center group"
              :class="{'border-red-500/30 bg-red-500/5': amountError}"
            >
              <div class="flex items-baseline gap-2 relative">
                <input 
                  v-model.number="amount" 
                  type="number" 
                  step="0.0001" 
                  min="0"
                  placeholder="0.00"
                  class="bg-transparent text-center text-5xl font-bold text-white placeholder-gray-700 focus:outline-none w-[300px] z-10"
                  required
                />
                <span class="text-xl font-bold text-gray-500 absolute right-[-40px] top-4 pointer-events-none">JUNO</span>
              </div>
              
              <!-- USD Equivalent Mockup -->
              <p class="mt-2 text-sm font-medium text-gray-500 group-focus-within:text-indigo-400 transition-colors">
                 ≈ ${{ (amount ? amount * 0.45 : 0).toFixed(2) }} USD
              </p>
            </div>
             <p v-if="amountError" class="text-center text-red-400 text-sm mt-2">{{ amountError }}</p>
          </div>

          <!-- 3. Memo (Conditional Reveal) -->
          <div class="overflow-hidden transition-all duration-500" :class="canAddMemo ? 'max-h-40 opacity-100' : 'max-h-0 opacity-50'">
            <label class="block text-sm font-medium text-gray-300 mb-2 ml-1 flex justify-between">
              <span>Encrypted Memo</span>
              <span class="text-xs text-gray-500 font-normal">{{ memo.length }}/512</span>
            </label>
            <textarea 
              v-model="memo"
              rows="2"
              maxlength="512"
              placeholder="Add a private message..."
              class="w-full p-4 bg-black/20 border border-white/10 rounded-xl text-gray-200 placeholder-gray-600 focus:outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all resize-none text-sm"
            ></textarea>
          </div>

          <!-- 4. Action Bar -->
          <div class="pt-4">
            <button 
              type="submit" 
              :disabled="wallet.isLoading || !!addressError || !!amountError"
              class="relative w-full group h-14 rounded-xl overflow-hidden disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <!-- Gradient Background -->
              <div class="absolute inset-0 bg-gradient-to-r from-indigo-600 to-purple-600 transition-all duration-300 group-hover:scale-105"></div>
              
              <!-- Button Content -->
              <div class="relative flex items-center justify-center gap-2">
                <span v-if="wallet.isLoading" class="animate-spin">
                  <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                </span>
                <span class="font-bold text-white tracking-wide text-lg">
                  {{ wallet.isLoading ? 'Processing...' : 'Send Transaction' }}
                </span>
              </div>
            </button>
            
            <p class="text-center text-xs text-gray-500 mt-4">
              Network Fee: <span class="text-gray-300">0.0001 JUNO</span> • Estimated Time: <span class="text-gray-300">~1 min</span>
            </p>
          </div>

        </form>
      </transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useWalletStore } from '../stores/walletStore';

const wallet = useWalletStore();

const toAddress = ref('');
const amount = ref<number | null>(null);
const memo = ref('');
const successTxId = ref('');
const addressError = ref(false);

// --- Computed Logic ---

// Determine address type based on prefix (Zcash/Juno standard)
const addressType = computed(() => {
  if (toAddress.value.startsWith('z') || toAddress.value.startsWith('zs')) return 'shielded';
  if (toAddress.value.startsWith('t')) return 'transparent';
  return null;
});

// Memos are strictly for shielded transactions (simplified logic)
const canAddMemo = computed(() => addressType.value === 'shielded');

// Validation
const amountError = computed(() => {
  if (amount.value === null) return null;
  if (amount.value <= 0) return "Amount must be positive";
  if (amount.value > wallet.balance.total) return "Insufficient funds";
  return null;
});

// Watch address to clear error when typing
watch(toAddress, () => {
  addressError.value = false;
});

// --- Actions ---

function setMax() {
  const fee = 0.0001;
  const max = wallet.balance.total - fee;
  amount.value = max > 0 ? Number(max.toFixed(4)) : 0;
}

function resetForm() {
  toAddress.value = '';
  amount.value = null;
  memo.value = '';
  successTxId.value = '';
  addressError.value = false;
}

async function handleSubmit() {
  // Basic format validation
  if (!addressType.value) {
    addressError.value = true;
    return;
  }

  if (!amount.value || amountError.value) return;

  try {
    const opId = await wallet.sendTransaction(toAddress.value, amount.value, memo.value);
    
    // Play success flow
    successTxId.value = opId;
    
    // Refresh balance in background
    setTimeout(() => wallet.fetchBalance(), 2000);
  } catch (e: any) {
    // Show error via toast or alert (for now alert to keep it simple, 
    // but ideally a global toast system)
    alert(`Error: ${e.toString()}`);
  }
}
</script>

<style scoped>
/* Remove spinner from number input */
input[type=number]::-webkit-inner-spin-button, 
input[type=number]::-webkit-outer-spin-button { 
  -webkit-appearance: none; 
  margin: 0; 
}
input[type=number] {
  -moz-appearance: textfield;
}

/* Specific Scale animation for the checkmark */
.animate-scale-in {
  animation: scaleIn 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275) forwards;
}

@keyframes scaleIn {
  from { transform: scale(0); opacity: 0; }
  to { transform: scale(1); opacity: 1; }
}
</style>
