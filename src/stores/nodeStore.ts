import { defineStore } from 'pinia';

export const useNodeStore = defineStore('node', {
  state: () => ({
    // Initialize state directly from LocalStorage (Sync and Fast)
    binPath: localStorage.getItem('binPath') || '',
    dataDir: localStorage.getItem('dataDir') || '',
    rpcUser: localStorage.getItem('rpcUser') || '',
    rpcPass: localStorage.getItem('rpcPass') || '',
    rpcPort: 18232, // Default port
    isConnected: false
  }),
  actions: {
    // Kept async to match existing component calls, but logic is sync
    async loadSettings() {
      // Refresh state from storage (useful if modified elsewhere)
      this.binPath = localStorage.getItem('binPath') || '';
      this.dataDir = localStorage.getItem('dataDir') || '';
      this.rpcUser = localStorage.getItem('rpcUser') || '';
      this.rpcPass = localStorage.getItem('rpcPass') || '';
    },

    async saveSettings(bin: string, data: string, user: string, pass: string) {
      // 1. Update Pinia State
      this.binPath = bin;
      this.dataDir = data;
      this.rpcUser = user;
      this.rpcPass = pass;

      // 2. Save to LocalStorage
      localStorage.setItem('binPath', bin);
      localStorage.setItem('dataDir', data);
      localStorage.setItem('rpcUser', user);
      localStorage.setItem('rpcPass', pass);
    }
  }
});
