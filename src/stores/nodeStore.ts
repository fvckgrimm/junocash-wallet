import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

// --- Interfaces for Network Data ---

export interface NetworkInfo {
  version: number;
  subversion: string;
  protocolversion: number;
  connections: number;
  relayfee: number;
}

export interface PeerInfo {
  id: number;
  addr: string;
  version: number;
  subver: string;
  pingtime: number; // in seconds
  bytessent: number;
  bytesrecv: number;
  synced_blocks: number;
  inbound: boolean;
}

// --- Store Definition ---

export const useNodeStore = defineStore("node", {
  state: () => ({
    // Initialize state directly from LocalStorage (Sync and Fast)
    binPath: localStorage.getItem("binPath") || "",
    dataDir: localStorage.getItem("dataDir") || "",
    rpcUser: localStorage.getItem("rpcUser") || "",
    rpcPass: localStorage.getItem("rpcPass") || "",
    rpcHost: localStorage.getItem("rpcHost") || "127.0.0.1",
    randomxFastMode: localStorage.getItem("randomxFastMode") === "true",
    donationPercent: parseInt(localStorage.getItem("donationPercent") || "5"),
    rpcPort: parseInt(localStorage.getItem("rpcPort") || "18232"),

    // Connection Status
    isConnected: false,

    // Network & Peer Data
    networkInfo: null as NetworkInfo | null,
    peers: [] as PeerInfo[],
  }),

  actions: {
    async loadSettings() {
      // Refresh state from storage (useful if modified elsewhere)
      this.binPath = localStorage.getItem("binPath") || "";
      this.dataDir = localStorage.getItem("dataDir") || "";
      this.rpcUser = localStorage.getItem("rpcUser") || "";
      this.rpcPass = localStorage.getItem("rpcPass") || "";
      this.rpcHost = localStorage.getItem("rpcHost") || "127.0.0.1";
      this.randomxFastMode = localStorage.getItem("randomxFastMode") === "true";
      this.donationPercent = parseInt(
        localStorage.getItem("donationPercent") || "5",
      );
      this.rpcPort = parseInt(localStorage.getItem("rpcPort") || "18232");
    },

    async saveSettings(
      bin: string,
      data: string,
      user: string,
      pass: string,
      host: string,
      port: number,
      randomxFastMode: boolean,
      donation: number,
    ) {
      // 1. Update Pinia State
      this.binPath = bin;
      this.dataDir = data;
      this.rpcUser = user;
      this.rpcPass = pass;
      this.rpcHost = host;
      this.randomxFastMode = randomxFastMode;
      this.donationPercent = donation;
      this.rpcPort = port;

      // 2. Save to LocalStorage
      localStorage.setItem("binPath", bin);
      localStorage.setItem("dataDir", data);
      localStorage.setItem("rpcUser", user);
      localStorage.setItem("rpcPass", pass);
      localStorage.setItem("rpcHost", host);
      localStorage.setItem("randomxFastMode", randomxFastMode.toString());
      localStorage.setItem("donationPercent", donation.toString());
      localStorage.setItem("rpcPort", port.toString());
    },

    async fetchNetworkStatus() {
      // We attempt to fetch even if isConnected is false, 
      // as a successful fetch effectively proves we are connected.
      try {
        // 1. Fetch General Network Info
        const netRes = await invoke<NetworkInfo>("get_network_info", {
          host: this.rpcHost,
          port: this.rpcPort,
          user: this.rpcUser,
          pass: this.rpcPass,
        });
        this.networkInfo = netRes;
        this.isConnected = true; // Mark as connected if successful

        // 2. Fetch Peer Info
        const peerRes = await invoke<PeerInfo[]>("get_peer_info", {
          host: this.rpcHost,
          port: this.rpcPort,
          user: this.rpcUser,
          pass: this.rpcPass,
        });
        this.peers = peerRes;
      } catch (e) {
        console.error("Failed to fetch network stats:", e);
        // We don't necessarily set isConnected=false here to avoid UI flickering,
        // but you could if you want strict checking.
      }
    },
  },
});
