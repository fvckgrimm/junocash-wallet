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

export interface SpendableAddress {
  address: string;
  balance: number;
  addr_type: 'transparent' | 'sapling' | 'unified';
}

export interface TxTarget {
  address: string;
  amount: number;
  memo?: string;
}

interface OpResult {
  id: string;
  status: 'queued' | 'executing' | 'success' | 'failed';
  error?: { message: string };
  result?: { txid: string };
  creation_time: number;
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
    spendableAddresses: [] as SpendableAddress[],
    notifications: [] as OpResult[],
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

    async checkOperations() {
      const node = useNodeStore();
      if (!node.isConnected) return;

      try {
        // z_getoperationresult returns ONLY finished operations (success or failed)
        const res = await invoke<OpResult[]>('get_operation_status', {
          port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass
        });

        if (Array.isArray(res) && res.length > 0) {
          // Add to our notifications list to show the user
          // You might want to sort by time or filter duplicates
          this.notifications.unshift(...res);

          // If we found a success, trigger a balance refresh
          if (res.some(op => op.status === 'success')) {
            this.fetchBalance();
            this.fetchTransactions();
          }
        }
      } catch (e) {
        console.error(e);
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

    async fetchSpendableAddresses() {
      const node = useNodeStore();
      if (!node.isConnected) return;

      try {
        const res = await invoke<SpendableAddress[]>('get_spendable_addresses', {
          port: node.rpcPort, user: node.rpcUser, pass: node.rpcPass
        });
        this.spendableAddresses = res;
      } catch (e) {
        console.error(e);
      }
    },

    // UPDATE SEND
    async sendTransaction(targets: TxTarget[], fromAddress?: string) {
      const node = useNodeStore();
      this.isLoading = true;

      try {
        // Pass "fromAddress" or null (rust treats Option<String> as null|string)
        const opId = await invoke<string>('send_transaction', {
          fromAddress: fromAddress || null,
          targets,
          port: node.rpcPort,
          user: node.rpcUser,
          pass: node.rpcPass,
        });
        this.isLoading = false;
        return opId;
      } catch (err: any) {
        this.isLoading = false;
        throw err;
      }
    },

    // A polling helper to keep UI updated
    startPolling(intervalMs = 5000) {
      this.fetchBalance();
      this.fetchTransactions();
      this.fetchSpendableAddresses();

      const interval = setInterval(() => {
        this.fetchBalance();
        this.fetchTransactions();
        this.checkOperations();
      }, intervalMs);

      return () => clearInterval(interval); // Return cleanup function
    }
  },
});
