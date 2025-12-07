<template>
  <div class="p-8">
    <h1 class="text-3xl font-bold mb-6 text-gray-800">Overview</h1>

    <!-- Balance Cards -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
      <!-- Total -->
      <div class="bg-gradient-to-br from-purple-600 to-indigo-700 p-6 rounded-xl shadow-lg text-white">
        <h3 class="text-sm opacity-80 uppercase tracking-wider">Total Balance</h3>
        <div class="text-3xl font-bold mt-2">{{ wallet.formattedBalance }}</div>
      </div>
      
      <!-- Shielded -->
      <div class="bg-white p-6 rounded-xl shadow border border-gray-200">
        <div class="flex items-center gap-2 mb-2">
          <div class="w-3 h-3 bg-green-500 rounded-full"></div>
          <h3 class="text-sm text-gray-500 uppercase font-semibold">Shielded</h3>
        </div>
        <div class="text-2xl font-bold text-gray-800">{{ wallet.balance.private.toFixed(4) }} <span class="text-sm font-normal text-gray-500">JUNO</span></div>
      </div>

      <!-- Transparent -->
      <div class="bg-white p-6 rounded-xl shadow border border-gray-200">
        <div class="flex items-center gap-2 mb-2">
          <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
          <h3 class="text-sm text-gray-500 uppercase font-semibold">Transparent</h3>
        </div>
        <div class="text-2xl font-bold text-gray-800">{{ wallet.balance.transparent.toFixed(4) }} <span class="text-sm font-normal text-gray-500">JUNO</span></div>
      </div>
    </div>

    <!-- Recent Transactions -->
    <div class="bg-white rounded-xl shadow border border-gray-200 overflow-hidden">
      <div class="px-6 py-4 border-b border-gray-200 flex justify-between items-center">
        <h2 class="font-bold text-lg text-gray-700">Recent Activity</h2>
        <button @click="wallet.fetchTransactions()" class="text-purple-600 text-sm hover:underline">Refresh</button>
      </div>

      <div v-if="wallet.transactions.length === 0" class="p-8 text-center text-gray-500">
        No recent transactions found.
      </div>

      <ul v-else class="divide-y divide-gray-100">
        <li v-for="tx in wallet.recentTransactions.slice(0, 10)" :key="tx.txid" class="px-6 py-4 flex items-center justify-between hover:bg-gray-50">
          <div class="flex items-center gap-4">
            <div class="p-2 rounded-full" :class="getTxClass(tx.category)">
              <!-- Simple Icon based on Type -->
              <span v-if="tx.category === 'receive' || tx.category === 'generate'" class="font-bold text-lg">↓</span>
              <span v-else class="font-bold text-lg">↑</span>
            </div>
            <div>
              <p class="font-bold text-gray-800 capitalize">{{ tx.category }}</p>
              <p class="text-xs text-gray-400 font-mono">{{ tx.txid.substring(0, 16) }}...</p>
            </div>
          </div>
          <div class="text-right">
            <p class="font-bold" :class="tx.amount > 0 ? 'text-green-600' : 'text-gray-800'">
              {{ tx.amount > 0 ? '+' : '' }}{{ tx.amount }} JUNO
            </p>
            <p class="text-xs text-gray-400">{{ new Date(tx.time * 1000).toLocaleDateString() }}</p>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useWalletStore } from '../stores/walletStore';

const wallet = useWalletStore();

onMounted(() => {
  wallet.fetchBalance();
  wallet.fetchTransactions();
});

function getTxClass(category: string) {
  if (category === 'receive') return 'bg-green-100 text-green-600';
  if (category === 'generate') return 'bg-purple-100 text-purple-600'; // Mining reward
  return 'bg-gray-100 text-gray-600'; // Send
}
</script>
