<template>
  <div class="h-full flex flex-col relative">
    
    <!-- Scrollable Content Area -->
    <div class="flex-1 overflow-y-auto custom-scrollbar p-6 pb-32">
      
      <!-- Header -->
      <div class="flex items-center justify-between mb-8 animate-fade-in">
        <div>
          <h1 class="text-2xl font-bold text-white tracking-tight">Send Funds</h1>
          <p class="text-gray-400 text-sm">Transfer assets or create batch transactions.</p>
        </div>
        <!-- Balance Display -->
        <div class="text-right hidden sm:block">
          <p class="text-xs text-gray-500 uppercase tracking-wider font-bold mb-1">
            {{ selectedSource ? 'Source Balance' : 'Total Available' }}
          </p>
          <div class="flex items-center justify-end gap-2">
            <span class="text-xl font-mono font-bold text-emerald-400 tracking-tight">
              {{ currentMaxBalance.toFixed(4) }}
            </span>
            <span class="text-sm font-bold text-emerald-500/50">JUNO</span>
          </div>
        </div>
      </div>

      <!-- SUCCESS STATE -->
      <transition name="page" mode="out-in">
        <div v-if="successTxId" class="h-[60vh] flex flex-col items-center justify-center text-center animate-scale-in">
          <div class="relative w-24 h-24 mb-8 group cursor-default">
             <div class="absolute inset-0 bg-emerald-500/20 rounded-full blur-xl group-hover:blur-2xl transition-all duration-500"></div>
             <div class="relative w-full h-full bg-gradient-to-br from-emerald-500 to-teal-600 rounded-full flex items-center justify-center shadow-2xl">
                <svg class="w-10 h-10 text-white drop-shadow-md" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3"><path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" /></svg>
             </div>
          </div>
          
          <h2 class="text-3xl font-bold text-white mb-2 tracking-tight">Transaction Broadcast</h2>
          <p class="text-gray-400 mb-8">Your funds are successfully on their way to the network.</p>
          
          <div class="glass border border-white/10 rounded-xl p-4 w-full max-w-lg mb-8 flex flex-col items-start gap-1">
            <span class="text-[10px] uppercase font-bold text-gray-500 tracking-widest">Operation ID</span>
            <span class="font-mono text-xs text-indigo-300 break-all text-left leading-relaxed select-all">
              {{ successTxId }}
            </span>
          </div>
          
          <button @click="resetForm" class="px-8 py-3 bg-white/5 hover:bg-white/10 border border-white/10 text-white font-medium rounded-xl transition-all hover:scale-105 active:scale-95">
            Send Another Payment
          </button>
        </div>

        <!-- FORM STATE -->
        <form v-else @submit.prevent="handleSubmit" class="space-y-6 max-w-4xl mx-auto">
          
          <!-- 1. SOURCE SELECTOR (Coin Control) -->
          <div class="glass rounded-2xl p-1 animate-slide-up relative z-20">
            <div class="bg-black/20 rounded-xl border border-white/5 p-4">
              <label class="block text-xs font-bold text-gray-500 uppercase tracking-wide mb-3 ml-1">Source Wallet</label>
              
              <div class="relative group">
                <select 
                  v-model="selectedSource" 
                  class="w-full appearance-none bg-black/40 border border-white/10 rounded-xl py-3 pl-4 pr-12 text-sm text-white font-mono shadow-inner focus:outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all cursor-pointer"
                >
                  <option value="" class="bg-gray-900 text-gray-300">Auto-Select (Recommended for Privacy)</option>
                  <option disabled class="bg-gray-800 text-gray-500 font-sans font-bold pt-2">─── Specific Addresses ───</option>
                  <option 
                    v-for="addr in wallet.spendableAddresses" 
                    :key="addr.address" 
                    :value="addr.address"
                    class="bg-gray-900 text-gray-200"
                  >
                     {{ addr.balance.toFixed(4) }} JUNO  —  {{ shorten(addr.address) }} ({{ formatTypeLabel(addr.addr_type) }})
                  </option>
                </select>
                
                <!-- Custom Chevron -->
                <div class="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none text-gray-500 group-hover:text-indigo-400 transition-colors">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 9l6 6 6-6"/></svg>
                </div>
              </div>
            </div>
          </div>

          <!-- 2. RECIPIENT LIST -->
          <transition-group name="list" tag="div" class="space-y-4 relative z-10">
            <div 
              v-for="(recipient, index) in recipients" 
              :key="recipient.id"
              class="glass rounded-2xl p-6 border border-white/5 relative group transition-all duration-300 hover:border-white/10 hover:shadow-lg hover:shadow-black/20"
            >
              <!-- Card Header / Remove Button -->
              <div class="absolute top-4 right-4 z-10">
                <button 
                  v-if="recipients.length > 1"
                  type="button" 
                  @click="removeRecipient(index)"
                  class="p-2 text-gray-600 hover:text-red-400 hover:bg-red-500/10 rounded-lg transition-all opacity-0 group-hover:opacity-100"
                  title="Remove Recipient"
                >
                  <svg width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg>
                </button>
              </div>

              <div class="grid grid-cols-1 md:grid-cols-12 gap-6">
                
                <!-- Address Input with Address Book Integration -->
                <div class="md:col-span-8 space-y-2">
                  <div class="flex items-center justify-between ml-1 mb-2">
                    <label class="block text-xs font-bold text-gray-500 uppercase tracking-wide">
                      Recipient {{ recipients.length > 1 ? `#${index + 1}` : '' }} Address
                    </label>
                    <button 
                      type="button"
                      @click="openAddressBook(index)"
                      class="text-xs font-bold text-indigo-400 hover:text-white transition-colors uppercase tracking-wider bg-indigo-500/10 hover:bg-indigo-500 px-2 py-1 rounded flex items-center gap-1"
                    >
                      <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
                      Address Book
                    </button>
                  </div>
                  <div class="relative">
                    <input 
                      v-model="recipient.address" 
                      type="text" 
                      placeholder="Paste j1... or t1... address" 
                      class="w-full pl-10 pr-4 py-3 bg-black/20 border border-white/10 rounded-xl text-gray-200 font-mono text-sm focus:outline-none focus:border-indigo-500/50 focus:bg-black/30 transition-all placeholder-gray-600"
                      required
                    />
                    <!-- Icon -->
                    <div class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500">
                      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
                    </div>
                    
                    <!-- Address Book Label (if recognized) -->
                    <div v-if="getAddressLabel(recipient.address)" class="mt-1.5 flex items-center gap-1.5 text-xs text-indigo-400">
                      <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 24 24"><path d="M9 11.75A2.25 2.25 0 1 1 11.25 9 2.25 2.25 0 0 1 9 11.75zm7.5 4.5A.75.75 0 0 0 15.75 15h-1.5a.75.75 0 0 0-.75.75v1.5a.75.75 0 0 0 .75.75h1.5a.75.75 0 0 0 .75-.75zm-7.5 0A.75.75 0 0 0 8.25 15h-1.5a.75.75 0 0 0-.75.75v1.5a.75.75 0 0 0 .75.75h1.5a.75.75 0 0 0 .75-.75zM21 3v18H3V3zm-1.5 1.5h-15v15h15z"/></svg>
                      <span class="font-medium">{{ getAddressLabel(recipient.address) }}</span>
                    </div>
                  </div>
                </div>

                <!-- Amount Input -->
                <div class="md:col-span-4 space-y-2">
                  <div class="flex justify-between items-center ml-1">
                    <label class="block text-xs font-bold text-gray-500 uppercase tracking-wide">Amount</label>
                    <button type="button" @click="setMax(index)" class="text-[10px] font-bold text-indigo-400 hover:text-white transition-colors uppercase tracking-wider bg-indigo-500/10 hover:bg-indigo-500 px-1.5 py-0.5 rounded">
                      MAX
                    </button>
                  </div>
                  <div class="relative">
                    <input 
                      v-model.number="recipient.amount" 
                      type="number" 
                      step="0.0001" 
                      min="0"
                      class="w-full pl-4 pr-14 py-3 bg-black/20 border border-white/10 rounded-xl text-white font-mono text-lg font-bold text-right focus:outline-none focus:border-indigo-500/50 focus:bg-black/30 transition-all placeholder-gray-700"
                      placeholder="0.00"
                      required
                    />
                    <span class="absolute right-4 top-1/2 -translate-y-1/2 text-gray-500 text-xs font-bold pointer-events-none">JUNO</span>
                  </div>
                </div>

                <!-- Memo Input (Full Width) -->
                 <div class="md:col-span-12">
                   <div class="relative">
                     <input 
                       v-model="recipient.memo"
                       type="text"
                       placeholder="Add an encrypted memo (optional, shielded only)..."
                       class="w-full pl-10 pr-4 py-2.5 bg-white/[0.03] border border-white/5 rounded-lg text-gray-300 text-sm focus:outline-none focus:border-white/20 focus:bg-black/40 transition-colors placeholder-gray-600"
                     />
                     <div class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-600">
                       <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>
                     </div>
                   </div>
                </div>
              </div>
            </div>
          </transition-group>

          <!-- Add Recipient Button -->
          <button 
            type="button" 
            @click="addRecipient" 
            class="w-full py-4 border-2 border-dashed border-white/10 rounded-2xl text-gray-400 hover:text-white hover:border-indigo-500/30 hover:bg-indigo-500/5 transition-all flex items-center justify-center gap-2 group animate-slide-up"
          >
            <div class="bg-white/10 rounded-full p-1 group-hover:bg-indigo-500 group-hover:text-white transition-colors">
               <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M12 5v14"/><path d="M5 12h14"/></svg>
            </div>
            <span class="font-bold text-sm">Add Another Recipient</span>
          </button>

        </form>
      </transition>
    </div>

    <!-- FLOATING FOOTER (Glassmorphism Action Bar) -->
    <transition name="slide-up">
      <div v-if="!successTxId" class="absolute bottom-6 left-6 right-6 z-50">
        <div class="glass backdrop-blur-xl border border-white/10 rounded-2xl p-4 shadow-2xl shadow-black/50 flex flex-col md:flex-row items-center justify-between gap-4">
           
           <!-- Summary -->
           <div class="flex items-center gap-6 px-2">
             <div class="text-right md:text-left">
               <p class="text-[10px] text-gray-500 uppercase font-bold tracking-wider">Estimated Total</p>
               <div class="flex items-baseline gap-2">
                  <span class="text-2xl font-bold tracking-tight transition-colors duration-300" :class="totalRequired > currentMaxBalance ? 'text-red-400' : 'text-white'">
                    {{ totalRequired.toFixed(4) }}
                  </span>
                  <span class="text-sm font-medium text-gray-500">JUNO</span>
               </div>
             </div>
             
             <!-- Separator -->
             <div class="hidden md:block w-px h-8 bg-white/10"></div>
             
             <div class="hidden md:block">
               <p class="text-[10px] text-gray-500 uppercase font-bold tracking-wider">Network Fee</p>
               <p class="text-sm font-mono text-gray-300">0.0001 JUNO</p>
             </div>
           </div>

           <!-- Action Button -->
           <button 
              @click="handleSubmit" 
              :disabled="wallet.isLoading || totalRequired > currentMaxBalance"
              class="w-full md:w-auto px-8 py-3.5 bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 text-white font-bold rounded-xl shadow-[0_0_20px_rgba(99,102,241,0.3)] hover:shadow-[0_0_30px_rgba(168,85,247,0.5)] disabled:opacity-50 disabled:shadow-none disabled:cursor-not-allowed transition-all transform active:scale-95 flex items-center justify-center gap-3"
           >
             <span v-if="wallet.isLoading" class="animate-spin w-5 h-5 border-2 border-white/30 border-t-white rounded-full"></span>
             <span>{{ wallet.isLoading ? 'Broadcasting...' : 'Sign & Send Transaction' }}</span>
             <svg v-if="!wallet.isLoading" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>
           </button>
        </div>
      </div>
    </transition>

    <!-- Address Book Modal -->
    <transition name="modal">
      <div v-if="addressBookOpen !== null" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm" @click.self="closeAddressBook">
        <div class="glass rounded-2xl border border-white/10 p-6 w-full max-w-2xl max-h-[80vh] flex flex-col shadow-2xl animate-scale-in" @click.stop>
          <div class="flex items-center justify-between mb-6">
            <h2 class="text-xl font-bold text-white">Select from Address Book</h2>
            <button @click="closeAddressBook" class="p-2 text-gray-500 hover:text-white rounded-lg hover:bg-white/5">
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M18 6L6 18M6 6l12 12"/></svg>
            </button>
          </div>

          <!-- Search -->
          <div class="relative mb-4">
            <input 
              v-model="addressBookSearch"
              type="text"
              placeholder="Search addresses..."
              class="w-full pl-10 pr-4 py-3 bg-black/20 border border-white/10 rounded-xl text-gray-200 text-sm focus:outline-none focus:border-indigo-500/50 transition-all placeholder-gray-600"
            />
            <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
          </div>

          <!-- Address List -->
          <div class="flex-1 overflow-y-auto custom-scrollbar space-y-2">
            <div v-if="filteredAddressBookEntries.length === 0" class="text-center py-12 text-gray-500">
              <p>No addresses found</p>
            </div>
            <button
              v-for="entry in filteredAddressBookEntries"
              :key="entry.id"
              type="button"
              @click="selectFromAddressBook(entry.address)"
              class="w-full p-4 bg-black/20 hover:bg-black/40 border border-white/5 hover:border-indigo-500/30 rounded-xl text-left transition-all group"
            >
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-indigo-500/20 to-purple-600/20 border border-indigo-500/30 flex items-center justify-center shrink-0">
                  <span class="text-indigo-400 font-bold text-sm">{{ entry.label[0].toUpperCase() }}</span>
                </div>
                <div class="flex-1 min-w-0">
                  <h3 class="text-white font-bold text-sm truncate group-hover:text-indigo-400 transition-colors">{{ entry.label }}</h3>
                  <p class="text-xs text-gray-500 font-mono truncate">{{ entry.address }}</p>
                </div>
              </div>
            </button>
          </div>
        </div>
      </div>
    </transition>

  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useWalletStore, type TxTarget } from '../stores/walletStore';
import { useAddressBookStore } from '../stores/addressBookStore';

const wallet = useWalletStore();
const addressBook = useAddressBookStore();

const successTxId = ref('');
const selectedSource = ref('');
const addressBookOpen = ref<number | null>(null);
const addressBookSearch = ref('');

// Recipient List
let nextId = 1;
const recipients = ref<Array<{id: number} & TxTarget>>([
  { id: 0, address: '', amount: 0, memo: '' }
]);

onMounted(() => {
  wallet.fetchSpendableAddresses();
  addressBook.loadFromStorage();
});

// Computed Helpers
const currentMaxBalance = computed(() => {
  if (!selectedSource.value) return wallet.balance.total;
  const addr = wallet.spendableAddresses.find(a => a.address === selectedSource.value);
  return addr ? addr.balance : 0;
});

const totalRequired = computed(() => {
  const sum = recipients.value.reduce((acc, curr) => acc + (curr.amount || 0), 0);
  return sum + 0.0001; 
});

const filteredAddressBookEntries = computed(() => {
  if (!addressBookSearch.value.trim()) {
    return addressBook.sortedEntries;
  }
  return addressBook.searchEntries(addressBookSearch.value);
});

// Presentation Helpers
function formatTypeLabel(t: string) {
  if(t === 'unified') return 'Unified';
  if(t === 'sapling') return 'Sapling';
  return 'Transparent';
}

function shorten(addr: string) {
  return addr.substring(0, 6) + '...' + addr.substring(addr.length - 6);
}

function getAddressLabel(address: string): string | null {
  const entry = addressBook.getByAddress(address);
  return entry ? entry.label : null;
}

// Address Book Functions
function openAddressBook(index: number) {
  addressBookOpen.value = index;
  addressBookSearch.value = '';
}

function closeAddressBook() {
  addressBookOpen.value = null;
  addressBookSearch.value = '';
}

function selectFromAddressBook(address: string) {
  if (addressBookOpen.value !== null) {
    recipients.value[addressBookOpen.value].address = address;
    addressBook.markAsUsed(address);
    closeAddressBook();
  }
}

// Logic
function addRecipient() {
  recipients.value.push({ id: nextId++, address: '', amount: 0, memo: '' });
}

function removeRecipient(index: number) {
  recipients.value.splice(index, 1);
}

function setMax(index: number) {
  const others = recipients.value.reduce((acc, curr, idx) => idx === index ? acc : acc + (curr.amount || 0), 0);
  const fee = 0.0001;
  const remaining = currentMaxBalance.value - fee - others;
  
  recipients.value[index].amount = remaining > 0 ? Number(remaining.toFixed(4)) : 0;
}

function resetForm() {
  recipients.value = [{ id: nextId++, address: '', amount: 0, memo: '' }];
  successTxId.value = '';
}

async function handleSubmit() {
  if (totalRequired.value > currentMaxBalance.value) {
    return;
  }
  
  const targets = recipients.value.map(r => ({
    address: r.address,
    amount: r.amount,
    memo: r.memo
  }));

  try {
    const opId = await wallet.sendTransaction(targets, selectedSource.value);
    
    // Mark addresses as used in address book
    targets.forEach(t => {
      if (t.address) {
        addressBook.markAsUsed(t.address);
      }
    });
    
    successTxId.value = opId;
    setTimeout(() => {
        wallet.fetchBalance(); 
        wallet.fetchSpendableAddresses();
    }, 2000);
  } catch (e: any) {
    alert("Error: " + e.toString());
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

/* List Transitions */
.list-enter-active,
.list-leave-active {
  transition: all 0.4s cubic-bezier(0.25, 1, 0.5, 1);
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(-20px) scale(0.95);
}

/* Footer slide up */
.slide-up-enter-active, .slide-up-leave-active {
  transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1), opacity 0.4s ease;
}
.slide-up-enter-from, .slide-up-leave-to {
  transform: translateY(100%);
  opacity: 0;
}

/* Modal Transitions */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}
.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .glass,
.modal-leave-active .glass {
  transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}
.modal-enter-from .glass {
  transform: scale(0.9);
}
.modal-leave-to .glass {
  transform: scale(0.9);
}
</style>
