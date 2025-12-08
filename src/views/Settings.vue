<template>
  <div class="max-w-4xl mx-auto min-h-full flex flex-col p-2 pb-20 relative">
    
    <!-- Header -->
    <div class="mb-8 px-2 animate-fade-in">
      <h1 class="text-2xl font-bold text-white tracking-tight">System Configuration</h1>
      <p class="text-gray-400 text-sm">Configure your local node connection, mining preferences, and security.</p>
    </div>

    <!-- Main Settings Card -->
    <div class="glass rounded-3xl p-8 relative overflow-hidden animate-slide-up">
      
      <!-- Background Decorations -->
      <div class="absolute top-0 right-0 w-96 h-96 bg-indigo-600/10 rounded-full blur-[100px] pointer-events-none -mr-32 -mt-32"></div>

      <div class="relative z-10 space-y-10">

        <!-- SECTION 1: NODE FILES -->
        <div class="space-y-5">
          <div class="flex items-center gap-3 border-b border-white/5 pb-2">
            <div class="p-1.5 bg-indigo-500/20 rounded text-indigo-400">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
            </div>
            <h2 class="text-sm font-bold text-gray-200 uppercase tracking-wider">Node Filesystem</h2>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Binary Path -->
            <div class="space-y-2 group">
              <label class="text-xs font-semibold text-gray-500 ml-1 group-focus-within:text-indigo-400 transition-colors">Executable Path (junocashd)</label>
              <div class="flex gap-2">
                <div class="relative flex-1">
                  <input 
                    v-model="binPath" 
                    type="text" 
                    readonly
                    class="w-full pl-4 pr-4 py-3 bg-black/20 border border-white/10 rounded-xl text-xs font-mono text-gray-300 truncate focus:outline-none focus:border-indigo-500/50 transition-colors"
                    placeholder="Not selected"
                  />
                </div>
                <button 
                  @click="pickBinary" 
                  class="px-4 bg-white/5 hover:bg-white/10 border border-white/5 rounded-xl text-gray-300 hover:text-white transition-all active:scale-95"
                  title="Browse Files"
                >
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
                </button>
              </div>
            </div>

            <!-- Data Directory -->
            <div class="space-y-2 group">
              <label class="text-xs font-semibold text-gray-500 ml-1 group-focus-within:text-indigo-400 transition-colors">Blockchain Storage (Data Dir)</label>
              <div class="flex gap-2">
                <div class="relative flex-1">
                  <input 
                    v-model="dataDir" 
                    type="text" 
                    readonly
                    class="w-full pl-4 pr-4 py-3 bg-black/20 border border-white/10 rounded-xl text-xs font-mono text-gray-300 truncate focus:outline-none focus:border-indigo-500/50 transition-colors"
                    placeholder="Default directory"
                  />
                </div>
                <button 
                  @click="pickDir" 
                  class="px-4 bg-white/5 hover:bg-white/10 border border-white/5 rounded-xl text-gray-300 hover:text-white transition-all active:scale-95"
                  title="Browse Folders"
                >
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- SECTION 2: RPC SECURITY -->
        <div class="space-y-5">
           <div class="flex items-center gap-3 border-b border-white/5 pb-2">
            <div class="p-1.5 bg-indigo-500/20 rounded text-indigo-400">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>
            </div>
            <h2 class="text-sm font-bold text-gray-200 uppercase tracking-wider">RPC Access Control</h2>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Username -->
            <div class="space-y-2">
              <label class="text-xs font-semibold text-gray-500 ml-1">RPC User</label>
              <input
                v-model="rpcUser"
                type="text"
                class="w-full px-4 py-3 bg-black/20 border border-white/10 rounded-xl text-sm text-gray-200 placeholder-gray-600 focus:outline-none focus:border-indigo-500/50 transition-colors"
              />
            </div>

            <!-- Password -->
            <div class="space-y-2 relative">
              <label class="text-xs font-semibold text-gray-500 ml-1">RPC Password</label>
              <div class="relative">
                <input
                  v-model="rpcPass"
                  :type="showPass ? 'text' : 'password'"
                  class="w-full pl-4 pr-12 py-3 bg-black/20 border border-white/10 rounded-xl text-sm text-gray-200 placeholder-gray-600 focus:outline-none focus:border-indigo-500/50 transition-colors"
                />
                <button
                  @click="showPass = !showPass"
                  class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-white transition-colors"
                >
                  <svg v-if="!showPass" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
                  <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>
                </button>
              </div>
            </div>

            <!-- RPC Host -->
            <div class="space-y-2">
              <label class="text-xs font-semibold text-gray-500 ml-1">RPC Host</label>
              <input
                v-model="rpcHost"
                type="text"
                class="w-full px-4 py-3 bg-black/20 border border-white/10 rounded-xl text-sm text-gray-200 placeholder-gray-600 focus:outline-none focus:border-indigo-500/50 transition-colors"
                placeholder="127.0.0.1"
              />
            </div>

            <!-- RPC Port -->
            <div class="space-y-2">
              <label class="text-xs font-semibold text-gray-500 ml-1">RPC Port</label>
              <input
                v-model="rpcPort"
                type="number"
                class="w-full px-4 py-3 bg-black/20 border border-white/10 rounded-xl text-sm text-gray-200 placeholder-gray-600 focus:outline-none focus:border-indigo-500/50 transition-colors"
                placeholder="18232"
              />
            </div>
          </div>
          
          <div class="bg-amber-500/10 border border-amber-500/20 rounded-lg p-3 flex gap-3">
             <svg class="text-amber-400 shrink-0" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
             <p class="text-xs text-amber-200/80 leading-relaxed">
               These credentials must match your local <span class="font-mono text-amber-100">junocash.conf</span> file.
             </p>
          </div>
        </div>

        <!-- SECTION 3: MINING CONFIGURATION -->
        <div class="space-y-5">
           <div class="flex items-center gap-3 border-b border-white/5 pb-2">
            <div class="p-1.5 bg-purple-500/20 rounded text-purple-400">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2v20M2 12h20" /></svg>
            </div>
            <h2 class="text-sm font-bold text-gray-200 uppercase tracking-wider">Mining Configuration</h2>
          </div>

          <!-- RandomX Fast Mode Toggle -->
          <div class="flex items-center justify-between p-4 bg-white/[0.02] rounded-xl border border-white/5 transition-colors hover:bg-white/[0.04]">
            <div class="flex-1">
              <div class="flex items-center gap-2 mb-1">
                <h3 class="text-sm font-bold text-white">RandomX Fast Mode</h3>
                <span class="px-2 py-0.5 text-[10px] font-bold uppercase bg-purple-500/20 text-purple-300 rounded">2GB RAM</span>
                <span class="ml-2 text-[10px] text-gray-500 uppercase tracking-wide">(Requires Restart)</span>
              </div>
              <p class="text-xs text-gray-400 leading-relaxed max-w-2xl">
                Allocates ~2GB of memory for a faster RandomX lookup table. Significantly improves hashrate.
              </p>
            </div>
            
            <button 
              @click="randomxFastMode = !randomxFastMode"
              class="ml-4 w-12 h-6 rounded-full p-1 transition-colors duration-300 ease-in-out relative flex-shrink-0"
              :class="randomxFastMode ? 'bg-purple-600' : 'bg-gray-700'"
            >
              <div 
                class="w-4 h-4 bg-white rounded-full shadow-md transform transition-transform duration-300"
                :class="randomxFastMode ? 'translate-x-6' : 'translate-x-0'"
              ></div>
            </button>
          </div>

          <!-- Developer Donation Slider -->
          <div class="p-5 bg-white/[0.02] rounded-xl border border-white/5 space-y-4 transition-colors hover:bg-white/[0.04]">
             <div class="flex justify-between items-center">
                <div>
                   <div class="flex items-center gap-2">
                     <h3 class="text-sm font-bold text-white">Developer Donation</h3>
                     <span class="text-[10px] text-gray-500 uppercase tracking-wide">(Requires Restart)</span>
                   </div>
                   <p class="text-xs text-gray-400 mt-1">Automatically donate a percentage of block rewards to the developers.</p>
                </div>
                <div class="text-right">
                   <span class="text-2xl font-mono font-bold" 
                         :class="donationPercent > 0 ? 'text-emerald-400' : 'text-gray-500'">
                      {{ donationPercent }}%
                   </span>
                </div>
             </div>
             
             <!-- Slider -->
             <div class="relative pt-1">
                <input 
                  type="range" 
                  v-model.number="donationPercent" 
                  min="0" max="100" step="1"
                  class="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-emerald-500"
                />
             </div>
             
             <div class="flex justify-between text-[10px] text-gray-500 font-bold uppercase tracking-wider">
               <span>0% (Disabled)</span>
               <span>25%</span>
               <span>50%</span>
               <span>75%</span>
               <span>100%</span>
             </div>
          </div>
        </div>

        <!-- SECTION 4: SECURITY ZONE -->
        <div class="space-y-5">
           <div class="flex items-center gap-3 border-b border-white/5 pb-2">
            <div class="p-1.5 bg-red-500/20 rounded text-red-400">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><path d="M12 8v4"/><path d="M12 16h.01"/></svg>
            </div>
            <h2 class="text-sm font-bold text-gray-200 uppercase tracking-wider">Security Zone</h2>
          </div>

          <div class="p-6 bg-red-500/5 border border-red-500/10 rounded-2xl flex flex-col md:flex-row items-center justify-between gap-6">
            <div>
              <h3 class="text-base font-bold text-white flex items-center gap-2">
                Backup Recovery Phrase
                <span class="px-2 py-0.5 rounded text-[10px] font-bold bg-red-500/20 text-red-300 uppercase">Highly Sensitive</span>
              </h3>
              <p class="text-sm text-gray-400 mt-2 max-w-lg leading-relaxed">
                Your 24-word seed phrase is the only way to recover your funds if this computer fails. 
                Write it down on paper and store it securely. Never share it.
              </p>
            </div>
            <button 
              @click="startBackupFlow"
              class="px-6 py-3 rounded-xl bg-red-500/10 hover:bg-red-500/20 text-red-400 font-bold border border-red-500/20 hover:border-red-500/40 transition-all active:scale-95 whitespace-nowrap"
            >
              Reveal Seed Phrase
            </button>
          </div>
        </div>

        <!-- SECTION 5: ACTIONS -->
        <div class="pt-4 border-t border-white/5 space-y-4">
          
          <div class="flex flex-col sm:flex-row gap-4">
            <!-- Launch Button -->
            <button 
              @click="launchNode" 
              :disabled="status.state === 'loading'"
              class="flex-1 group relative overflow-hidden rounded-xl bg-emerald-600 p-[1px] shadow-lg shadow-emerald-900/20 hover:shadow-emerald-900/40 transition-all active:scale-[0.99]"
            >
              <div class="absolute inset-0 bg-gradient-to-r from-emerald-500 to-teal-500 opacity-80 group-hover:opacity-100 transition-opacity"></div>
              <div class="relative h-12 bg-black/10 flex items-center justify-center gap-2 rounded-xl group-hover:bg-transparent transition-colors">
                <span v-if="status.state === 'loading'" class="animate-spin text-white">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M12 2v4"/><path d="m16.2 7.8 2.9-2.9"/><path d="M18 12h4"/><path d="m16.2 16.2 2.9 2.9"/><path d="M12 18v4"/><path d="m4.9 19.1 2.9-2.9"/><path d="M2 12h4"/><path d="m4.9 4.9 2.9 2.9"/></svg>
                </span>
                <span class="font-bold text-white tracking-wide">Save & Launch Node</span>
              </div>
            </button>

            <!-- Connect Button -->
            <button 
              @click="connectOnly" 
              :disabled="status.state === 'loading'"
              class="flex-1 h-12 rounded-xl border border-white/10 hover:bg-white/5 text-gray-300 font-semibold hover:text-white transition-colors active:scale-[0.99]"
            >
              Save & Connect Only
            </button>
          </div>

          <!-- STATUS BAR -->
          <transition name="fade">
            <div v-if="status.message" 
                 class="rounded-lg p-3 text-sm font-medium flex items-center justify-center gap-2"
                 :class="status.type === 'error' ? 'bg-red-500/10 text-red-400 border border-red-500/20' : 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20'">
              
              <svg v-if="status.type === 'success'" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M20 6L9 17l-5-5"/></svg>
              <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
              
              {{ status.message }}
            </div>
          </transition>
        </div>

      </div>
    </div>

    <!-- ========================================== -->
    <!--          SECURE BACKUP MODAL               -->
    <!-- ========================================== -->
    <transition name="modal">
      <div v-if="backupState.isOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
        <!-- Backdrop with Blur -->
        <div class="absolute inset-0 bg-black/80 backdrop-blur-xl transition-opacity" @click="closeBackup"></div>

        <!-- Modal Content -->
        <div class="relative bg-[#181B24] w-full max-w-2xl rounded-3xl border border-white/10 shadow-2xl overflow-hidden flex flex-col max-h-[90vh]">
          
          <!-- Header -->
          <div class="p-6 border-b border-white/5 bg-white/[0.02] flex justify-between items-center">
            <h2 class="text-xl font-bold text-white tracking-tight">Wallet Backup</h2>
            <button @click="closeBackup" class="text-gray-500 hover:text-white transition-colors">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
          </div>

          <!-- STAGE 1: WARNING -->
          <div v-if="backupState.step === 1" class="p-10 flex flex-col items-center text-center space-y-6">
            <div class="w-20 h-20 bg-red-500/10 rounded-full flex items-center justify-center text-red-500 mb-2">
               <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
            </div>
            <h3 class="text-2xl font-bold text-white">Are you being watched?</h3>
            <p class="text-gray-400 text-sm max-w-md leading-relaxed">
              Anyone with these words can steal your funds instantly. Ensure you are in a private location and no cameras or screen sharing software are active.
            </p>
            <div class="pt-4 w-full max-w-xs">
              <button @click="fetchAndShowSeed" class="w-full py-3 bg-red-600 hover:bg-red-500 text-white font-bold rounded-xl transition-all shadow-lg shadow-red-900/20">
                I am ready to write it down
              </button>
            </div>
          </div>

          <!-- STAGE 2: DISPLAY SEED -->
          <div v-if="backupState.step === 2" class="p-8 flex flex-col h-full overflow-hidden">
            <div class="bg-amber-500/10 border border-amber-500/20 p-3 rounded-lg flex gap-3 mb-6">
               <svg class="text-amber-400 shrink-0" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>
               <p class="text-xs text-amber-200/90">Write these words down in order #1 to #24. You will be asked to verify them next.</p>
            </div>

            <div class="grid grid-cols-3 sm:grid-cols-4 gap-3 overflow-y-auto custom-scrollbar p-2 mb-6">
              <div v-for="(word, idx) in seedWords" :key="idx" 
                   class="relative group bg-black/40 border border-white/10 rounded-lg p-3 flex flex-col items-center justify-center select-none hover:bg-white/5 transition-colors">
                <span class="absolute top-1 left-2 text-[10px] text-gray-600 font-bold select-none">{{ idx + 1 }}</span>
                <span class="font-mono text-sm font-bold text-gray-200 tracking-wide select-all">{{ word }}</span>
              </div>
            </div>

            <button @click="startVerification" class="w-full py-3 bg-indigo-600 hover:bg-indigo-500 text-white font-bold rounded-xl transition-all mt-auto">
              I have written it down
            </button>
          </div>

          <!-- STAGE 3: VERIFY -->
          <div v-if="backupState.step === 3" class="p-10 flex flex-col items-center text-center space-y-6">
            <h3 class="text-xl font-bold text-white">Verify Backup</h3>
            <p class="text-gray-400 text-sm">Please enter <span class="text-white font-bold">Word #{{ verifyTargetIndex + 1 }}</span> to confirm you saved your phrase.</p>
            
            <div class="w-full max-w-xs space-y-4">
               <input 
                 v-model="verifyInput" 
                 type="text" 
                 class="w-full text-center py-3 bg-black/40 border border-white/20 rounded-xl text-white font-mono text-lg focus:border-indigo-500 outline-none transition-colors"
                 placeholder="Enter word here"
                 @keyup.enter="checkVerification"
               />
               
               <p v-if="verifyError" class="text-red-400 text-xs font-bold animate-pulse">{{ verifyError }}</p>

               <button @click="checkVerification" class="w-full py-3 bg-indigo-600 hover:bg-indigo-500 text-white font-bold rounded-xl transition-all">
                 Verify
               </button>
               
               <button @click="backupState.step = 2" class="text-xs text-gray-500 hover:text-gray-300 underline">
                 Back to words
               </button>
            </div>
          </div>

           <!-- STAGE 4: SUCCESS -->
          <div v-if="backupState.step === 4" class="p-10 flex flex-col items-center text-center space-y-6">
            <div class="w-20 h-20 bg-emerald-500/10 rounded-full flex items-center justify-center text-emerald-500 mb-2">
               <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M20 6L9 17l-5-5"/></svg>
            </div>
            <h3 class="text-2xl font-bold text-white">Backup Verified</h3>
            <p class="text-gray-400 text-sm max-w-md">
              Your recovery phrase has been securely zeroed out from memory. Keep your written copy safe.
            </p>
            <button @click="closeBackup" class="mt-4 px-8 py-3 bg-white/5 hover:bg-white/10 border border-white/10 text-white font-bold rounded-xl transition-all">
              Close
            </button>
          </div>

        </div>
      </div>
    </transition>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useNodeStore } from '../stores/nodeStore';
import { useWalletStore } from '../stores/walletStore';

const node = useNodeStore();
const wallet = useWalletStore();

// Form State
const binPath = ref('');
const dataDir = ref('');
const rpcHost = ref('127.0.0.1');
const rpcPort = ref(18232);
const rpcUser = ref('');
const rpcPass = ref('');
const randomxFastMode = ref(false);
const donationPercent = ref(5);
const showPass = ref(false);

// UI State
const status = reactive({
  state: 'idle', // idle, loading
  type: 'success', // success, error
  message: ''
});

// Load settings
onMounted(async () => {
  await node.loadSettings();
  binPath.value = node.binPath;
  dataDir.value = node.dataDir;
  rpcHost.value = node.rpcHost;
  rpcPort.value = node.rpcPort;
  rpcUser.value = node.rpcUser;
  rpcPass.value = node.rpcPass;
  randomxFastMode.value = node.randomxFastMode;
  donationPercent.value = node.donationPercent;
});

// Actions
async function pickBinary() {
  const selected = await open({ multiple: false, directory: false });
  if (selected && typeof selected === 'string') binPath.value = selected;
}

async function pickDir() {
  const selected = await open({ directory: true, multiple: false });
  if (selected && typeof selected === 'string') dataDir.value = selected;
}

async function saveToStore() {
  await node.saveSettings(
    binPath.value,
    dataDir.value,
    rpcUser.value,
    rpcPass.value,
    rpcHost.value,
    rpcPort.value,
    randomxFastMode.value,
    donationPercent.value
  );
}

function setStatus(type: 'success' | 'error', msg: string) {
  status.type = type;
  status.message = msg;
  status.state = 'idle';
  // Auto clear success messages, keep errors
  if(type === 'success') {
    setTimeout(() => { status.message = '' }, 5000);
  }
}

// Option A: Launch
async function launchNode() {
  status.state = 'loading';
  status.message = '';
  
  await saveToStore();

  try {
    await invoke('launch_node', {
      binPath: binPath.value,
      dataDir: dataDir.value,
      rpcPort: rpcPort.value,
      rpcUser: rpcUser.value,
      rpcPass: rpcPass.value,
      randomxFastMode: randomxFastMode.value,
      donationPercent: donationPercent.value
    });

    setStatus('success', `Node launch command sent. Donations set to ${donationPercent.value}%.`);

    // Attempt to verify connection shortly after
    setTimeout(() => {
        wallet.fetchBalance();
    }, 3000);

  } catch(e: any) {
    setStatus('error', "Launch Failed: " + e.toString());
  }
}

// Option B: Connect
async function connectOnly() {
  status.state = 'loading';
  status.message = '';
  
  await saveToStore();
  
  try {
    await wallet.fetchBalance();
    if (wallet.lastError) {
      throw new Error(wallet.lastError);
    }
    setStatus('success', "Connected to JunoCash Node successfully.");
  } catch (e: any) {
    setStatus('error', "Connection Failed: Is the node running? (" + e.message + ")");
  }
}

// ==========================================
//          BACKUP LOGIC
// ==========================================
const seedWords = ref<string[]>([]);
const verifyInput = ref('');
const verifyTargetIndex = ref(0);
const verifyError = ref('');

const backupState = reactive({
  isOpen: false,
  step: 1 // 1=Warning, 2=Display, 3=Verify, 4=Success
});

function startBackupFlow() {
  backupState.step = 1;
  backupState.isOpen = true;
  verifyError.value = '';
}

async function fetchAndShowSeed() {
  try {
    const rawOutput = await wallet.getSeedPhrase();
    
    // Extract the content between the header and footer markers
    const headerMarker = "=== RECOVERY SEED PHRASE ===";
    const footerMarker = "=== IMPORTANT ===";
    
    const headerIndex = rawOutput.indexOf(headerMarker);
    const footerIndex = rawOutput.indexOf(footerMarker);
    
    let seedPart = rawOutput;
    if (headerIndex !== -1 && footerIndex !== -1) {
      seedPart = rawOutput.substring(headerIndex + headerMarker.length, footerIndex);
    }
    
    // Split on whitespace and filter out empty strings and marker artifacts
    const words = seedPart
        .trim()
        .split(/\s+/)
        .filter(w => w.length > 0 && !w.includes('==='));

    if (words.length !== 24) {
      throw new Error(`Expected 24 words but found ${words.length}. Please try again.`);
    }

    seedWords.value = words;
    backupState.step = 2;
  } catch(e: any) {
    alert("Error fetching seed: " + e.message);
    closeBackup();
  }
}

function startVerification() {
  verifyTargetIndex.value = Math.floor(Math.random() * 24);
  verifyInput.value = '';
  verifyError.value = '';
  backupState.step = 3;
}

function checkVerification() {
  const correctWord = seedWords.value[verifyTargetIndex.value];
  if (verifyInput.value.trim().toLowerCase() === correctWord.toLowerCase()) {
    // Success - ZERO OUT MEMORY
    seedWords.value = []; 
    backupState.step = 4;
  } else {
    verifyError.value = "Incorrect word. Please check your written backup.";
  }
}

function closeBackup() {
  // CRITICAL: ZERO OUT DATA
  seedWords.value = [];
  verifyInput.value = '';
  backupState.isOpen = false;
  setTimeout(() => { backupState.step = 1; }, 300);
}

onUnmounted(() => {
  seedWords.value = [];
});
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}
.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: #10B981;
  cursor: pointer;
  box-shadow: 0 0 10px rgba(16, 185, 129, 0.4);
  margin-top: -4px;
}
input[type="range"]::-moz-range-thumb {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: #10B981;
  cursor: pointer;
  border: none;
  box-shadow: 0 0 10px rgba(16, 185, 129, 0.4);
}
</style>
