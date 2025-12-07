<template>
  <div class="p-8 max-w-2xl">
    <h1 class="text-3xl font-bold mb-6 text-gray-800">Send Funds</h1>

    <div class="bg-white p-6 rounded-xl shadow border border-gray-200">
      <form @submit.prevent="handleSubmit" class="space-y-6">
        
        <!-- Recipient -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Recipient Address</label>
          <input 
            v-model="toAddress" 
            type="text" 
            placeholder="zs1... or t1..." 
            class="w-full p-3 border border-gray-300 rounded focus:ring-2 focus:ring-purple-500 outline-none"
            required
          />
        </div>

        <!-- Amount -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Amount (JUNO)</label>
          <div class="relative">
            <input 
              v-model.number="amount" 
              type="number" 
              step="0.0001" 
              min="0"
              class="w-full p-3 border border-gray-300 rounded focus:ring-2 focus:ring-purple-500 outline-none"
              required
            />
            <button 
              type="button"
              @click="setMax" 
              class="absolute right-3 top-3 text-xs text-purple-600 font-bold hover:underline"
            >
              MAX
            </button>
          </div>
          <p class="text-right text-xs text-gray-500 mt-1">Available: {{ wallet.balance.total }} JUNO</p>
        </div>

        <!-- Memo (Optional) -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Memo (Shielded Only)</label>
          <textarea 
            v-model="memo"
            rows="3"
            placeholder="Optional private message..."
            class="w-full p-3 border border-gray-300 rounded focus:ring-2 focus:ring-purple-500 outline-none"
          ></textarea>
        </div>

        <!-- Error/Success Messages -->
        <div v-if="error" class="p-3 bg-red-100 text-red-700 rounded text-sm">{{ error }}</div>
        <div v-if="successMsg" class="p-3 bg-green-100 text-green-700 rounded text-sm break-all">
          Success! Op ID: {{ successMsg }}
        </div>

        <!-- Submit -->
        <button 
          type="submit" 
          :disabled="wallet.isLoading"
          class="w-full bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 rounded transition-colors disabled:opacity-50"
        >
          {{ wallet.isLoading ? 'Sending...' : 'Send Transaction' }}
        </button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useWalletStore } from '../stores/walletStore';

const wallet = useWalletStore();

const toAddress = ref('');
const amount = ref<number | null>(null);
const memo = ref('');
const error = ref('');
const successMsg = ref('');

function setMax() {
  // Simple estimation: total - fee (0.0001)
  const max = wallet.balance.total - 0.0001;
  amount.value = max > 0 ? max : 0;
}

async function handleSubmit() {
  error.value = '';
  successMsg.value = '';

  if (!amount.value || amount.value <= 0) {
    error.value = "Invalid amount";
    return;
  }

  try {
    const opId = await wallet.sendTransaction(toAddress.value, amount.value, memo.value);
    successMsg.value = opId;
    // Clear form
    toAddress.value = '';
    amount.value = null;
    memo.value = '';
    // Refresh balance after a short delay
    setTimeout(() => wallet.fetchBalance(), 2000);
  } catch (e: any) {
    error.value = e.toString();
  }
}
</script>
