import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { useNodeStore } from './nodeStore';

// Interfaces for Type Safety
interface BalanceResponse {
  total: string;
  transparent: string;
  private: string;
}

interface Transaction {
  txid: string;
  amount: number;
  confirmations: number;
  category: 'send' | 'receive' | 'generate' | 'immature';
  time: number;
  address?: string;
}

export const useWalletStore = defineStore('wallet', {
  state: () => ({
    balance: {
      total: 0.0,
      transparent: 0.0,
      private: 0.0,
    },
    transactions: [] as Transaction[],
    isLoading: false,
    lastError: null as string | null,
    addresses: [] as string[],
  }),

  getters: {
    // Helper to format balance for display
    formattedBalance: (state) => state.balance.total.toFixed(4) + ' JUNO',
    recentTransactions: (state) => {
      return state.transactions.slice().sort((a, b) => b.time - a.time);
    }
  },

  actions: {
    async fetchBalance() {
      const node = useNodeStore();
      if (!node.isConnected) return;

      try {
        const res = await invoke<BalanceResponse>('get_balance', {
          port: node.rpcPort,
          user: node.rpcUser,
          pass: node.rpcPass,
        });

        // The RPC might return strings or numbers depending on version, safely parse them
        this.balance = {
          total: parseFloat(res.total),
          transparent: parseFloat(res.transparent),
          private: parseFloat(res.private),
        };
      } catch (err: any) {
        console.error('Failed to fetch balance:', err);
        this.lastError = err.toString();
      }
    },

    async fetchAddresses() {
      const node = useNodeStore();
      if (!node.isConnected) return;

      try {
        const res = await invoke<any>('get_all_addresses', {
          port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass,
        });

        // Parse Juno 'listaddresses' output
        // It returns an array of accounts. We want to extract "unified" and "transparent" addresses.
        const flatList: string[] = [];

        if (Array.isArray(res)) {
          for (const account of res) {
            // 1. Get Transparent
            if (account.transparent && Array.isArray(account.transparent.addresses)) {
              flatList.push(...account.transparent.addresses);
            }
            // 2. Get Unified (Orchard/Juno Standard)
            if (Array.isArray(account.unified)) {
              for (const uni of account.unified) {
                if (Array.isArray(uni.addresses)) {
                  for (const addrObj of uni.addresses) {
                    if (addrObj.address) flatList.push(addrObj.address);
                  }
                }
              }
            }
            // 3. Get Sapling (Legacy Shielded)
            if (Array.isArray(account.sapling)) {
              for (const sap of account.sapling) {
                flatList.push(sap.address);
              }
            }
          }
        }

        // Remove duplicates and save
        this.addresses = [...new Set(flatList)];
      } catch (e) {
        console.error("Failed to fetch addresses", e);
      }
    },

    async fetchTransactions() {
      const node = useNodeStore();
      if (!node.isConnected) return;

      try {
        const res = await invoke<Transaction[]>('list_transactions', {
          port: node.rpcPort,
          user: node.rpcUser,
          pass: node.rpcPass,
        });
        this.transactions = res;
      } catch (err: any) {
        console.error('Failed to fetch transactions:', err);
      }
    },

    async sendTransaction(toAddress: string, amount: number, memo: string = "") {
      const node = useNodeStore();
      this.isLoading = true;
      this.lastError = null;

      try {
        // "From" address is set to "*" to let the node pick inputs automatically
        const opId = await invoke<string>('send_transaction', {
          fromAddress: "*",
          toAddress,
          amount,
          memo,
          port: node.rpcPort,
          user: node.rpcUser,
          pass: node.rpcPass,
        });

        this.isLoading = false;
        return opId; // Return the Operation ID so the UI can track it
      } catch (err: any) {
        this.isLoading = false;
        this.lastError = err.toString();
        throw err;
      }
    },

    // A polling helper to keep UI updated
    startPolling(intervalMs = 10000) {
      this.fetchBalance();
      this.fetchTransactions();

      const interval = setInterval(() => {
        this.fetchBalance();
        this.fetchTransactions();
      }, intervalMs);

      return () => clearInterval(interval); // Return cleanup function
    }
  },
});
