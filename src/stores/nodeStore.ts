import { defineStore } from "pinia";

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
    isConnected: false,
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
  },
});
