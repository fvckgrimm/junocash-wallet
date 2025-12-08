<template>
  <div class="max-w-5xl mx-auto space-y-6">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 animate-fade-in">
      <div>
        <h1 class="text-2xl font-bold text-white tracking-tight">Address Book</h1>
        <p class="text-gray-400 text-sm">Save and manage frequently used addresses</p>
      </div>
      <button 
        @click="showAddModal = true"
        class="px-6 py-3 bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 text-white font-bold rounded-xl shadow-[0_0_20px_rgba(99,102,241,0.3)] transition-all transform active:scale-95 flex items-center gap-2"
      >
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M12 5v14"/><path d="M5 12h14"/></svg>
        Add Address
      </button>
    </div>

    <!-- Search Bar -->
    <div class="glass rounded-xl p-1 border border-white/5">
      <div class="relative">
        <input 
          v-model="searchQuery"
          type="text"
          placeholder="Search by label, address, or notes..."
          class="w-full pl-10 pr-4 py-3 bg-black/20 rounded-lg text-gray-200 text-sm focus:outline-none focus:bg-black/30 transition-all placeholder-gray-600"
        />
        <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
      </div>
    </div>

    <!-- Entries List -->
    <div v-if="displayedEntries.length === 0" class="glass rounded-2xl p-12 text-center border border-white/5">
      <div class="w-16 h-16 mx-auto mb-4 bg-white/5 rounded-full flex items-center justify-center">
        <svg class="w-8 h-8 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
      </div>
      <h3 class="text-lg font-bold text-gray-400 mb-2">No Addresses Yet</h3>
      <p class="text-sm text-gray-500">{{ searchQuery ? 'No results found' : 'Add your first address to get started' }}</p>
    </div>

    <transition-group v-else name="list" tag="div" class="space-y-3">
      <div 
        v-for="entry in displayedEntries" 
        :key="entry.id"
        class="glass rounded-xl border border-white/5 p-5 hover:border-white/10 transition-all group"
      >
        <div class="flex items-start justify-between gap-4">
          <!-- Main Info -->
          <div class="flex-1 min-w-0 space-y-2">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-indigo-500/20 to-purple-600/20 border border-indigo-500/30 flex items-center justify-center shrink-0">
                <span class="text-indigo-400 font-bold text-sm">{{ entry.label[0].toUpperCase() }}</span>
              </div>
              <div class="flex-1 min-w-0">
                <h3 class="text-white font-bold text-base truncate">{{ entry.label }}</h3>
                <p class="text-xs text-gray-500 font-mono truncate">{{ entry.address }}</p>
              </div>
            </div>

            <p v-if="entry.notes" class="text-sm text-gray-400 pl-13">{{ entry.notes }}</p>

            <div class="flex items-center gap-4 text-xs text-gray-600 pl-13">
              <span>Added {{ formatDate(entry.createdAt) }}</span>
              <span v-if="entry.lastUsed" class="flex items-center gap-1">
                <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
                Last used {{ formatDate(entry.lastUsed) }}
              </span>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity shrink-0">
            <button 
              @click="copyAddress(entry.address)"
              class="p-2 text-gray-500 hover:text-indigo-400 hover:bg-indigo-500/10 rounded-lg transition-all"
              title="Copy Address"
            >
              <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
            </button>
            <button 
              @click="startEdit(entry)"
              class="p-2 text-gray-500 hover:text-blue-400 hover:bg-blue-500/10 rounded-lg transition-all"
              title="Edit"
            >
              <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
            </button>
            <button 
              @click="confirmDelete(entry)"
              class="p-2 text-gray-500 hover:text-red-400 hover:bg-red-500/10 rounded-lg transition-all"
              title="Delete"
            >
              <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>
            </button>
          </div>
        </div>
      </div>
    </transition-group>

    <!-- Add/Edit Modal -->
    <transition name="modal">
      <div v-if="showAddModal || editingEntry" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm" @click.self="closeModal">
        <div class="glass rounded-2xl border border-white/10 p-6 w-full max-w-lg shadow-2xl animate-scale-in" @click.stop>
          <div class="flex items-center justify-between mb-6">
            <h2 class="text-xl font-bold text-white">{{ editingEntry ? 'Edit Address' : 'Add New Address' }}</h2>
            <button @click="closeModal" class="p-2 text-gray-500 hover:text-white rounded-lg hover:bg-white/5">
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M18 6L6 18M6 6l12 12"/></svg>
            </button>
          </div>

          <form @submit.prevent="saveEntry" class="space-y-4">
            <div>
              <label class="block text-xs font-bold text-gray-400 uppercase tracking-wide mb-2">Label</label>
              <input 
                v-model="formLabel"
                type="text"
                placeholder="e.g., Exchange Wallet, Friend's Address"
                class="w-full px-4 py-3 bg-black/20 border border-white/10 rounded-xl text-white focus:outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all"
                required
              />
            </div>

            <div v-if="!editingEntry">
              <label class="block text-xs font-bold text-gray-400 uppercase tracking-wide mb-2">Address</label>
              <input 
                v-model="formAddress"
                type="text"
                placeholder="j1... or t1..."
                class="w-full px-4 py-3 bg-black/20 border border-white/10 rounded-xl text-white font-mono text-sm focus:outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all"
                required
              />
            </div>

            <div>
              <label class="block text-xs font-bold text-gray-400 uppercase tracking-wide mb-2">Notes (Optional)</label>
              <textarea 
                v-model="formNotes"
                placeholder="Add any additional information..."
                rows="3"
                class="w-full px-4 py-3 bg-black/20 border border-white/10 rounded-xl text-white text-sm focus:outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all resize-none"
              ></textarea>
            </div>

            <div v-if="errorMessage" class="p-3 bg-red-500/10 border border-red-500/20 rounded-lg">
              <p class="text-red-400 text-sm">{{ errorMessage }}</p>
            </div>

            <div class="flex gap-3 pt-2">
              <button 
                type="button"
                @click="closeModal"
                class="flex-1 px-4 py-3 bg-white/5 hover:bg-white/10 border border-white/10 text-white font-medium rounded-xl transition-all"
              >
                Cancel
              </button>
              <button 
                type="submit"
                class="flex-1 px-4 py-3 bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 text-white font-bold rounded-xl transition-all transform active:scale-95"
              >
                {{ editingEntry ? 'Save Changes' : 'Add Address' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </transition>

    <!-- Delete Confirmation Modal -->
    <transition name="modal">
      <div v-if="deletingEntry" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm" @click.self="cancelDelete">
        <div class="glass rounded-2xl border border-white/10 p-6 w-full max-w-md shadow-2xl animate-scale-in">
          <div class="flex items-center gap-3 mb-4">
            <div class="w-12 h-12 rounded-full bg-red-500/20 flex items-center justify-center">
              <svg class="w-6 h-6 text-red-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/></svg>
            </div>
            <div>
              <h3 class="text-lg font-bold text-white">Delete Address?</h3>
              <p class="text-sm text-gray-400">This action cannot be undone</p>
            </div>
          </div>

          <p class="text-gray-300 mb-6">
            Are you sure you want to delete <strong class="text-white">{{ deletingEntry.label }}</strong>?
          </p>

          <div class="flex gap-3">
            <button 
              @click="cancelDelete"
              class="flex-1 px-4 py-3 bg-white/5 hover:bg-white/10 border border-white/10 text-white font-medium rounded-xl transition-all"
            >
              Cancel
            </button>
            <button 
              @click="executeDelete"
              class="flex-1 px-4 py-3 bg-red-600 hover:bg-red-500 text-white font-bold rounded-xl transition-all transform active:scale-95"
            >
              Delete
            </button>
          </div>
        </div>
      </div>
    </transition>

  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useAddressBookStore, type AddressBookEntry } from '../stores/addressBookStore';

const addressBook = useAddressBookStore();
const searchQuery = ref('');
const showAddModal = ref(false);
const editingEntry = ref<AddressBookEntry | null>(null);
const deletingEntry = ref<AddressBookEntry | null>(null);

const formLabel = ref('');
const formAddress = ref('');
const formNotes = ref('');
const errorMessage = ref('');

const displayedEntries = computed(() => {
  if (!searchQuery.value.trim()) {
    return addressBook.sortedEntries;
  }
  return addressBook.searchEntries(searchQuery.value);
});

function formatDate(timestamp: number): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return 'Today';
  if (diffDays === 1) return 'Yesterday';
  if (diffDays < 7) return `${diffDays} days ago`;
  if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`;
  if (diffDays < 365) return `${Math.floor(diffDays / 30)} months ago`;
  return date.toLocaleDateString();
}

function copyAddress(address: string) {
  navigator.clipboard.writeText(address).then(() => {
    // Could show a toast notification here
    console.log('Address copied!');
  });
}

function startEdit(entry: AddressBookEntry) {
  editingEntry.value = entry;
  formLabel.value = entry.label;
  formAddress.value = entry.address;
  formNotes.value = entry.notes || '';
  errorMessage.value = '';
}

function closeModal() {
  showAddModal.value = false;
  editingEntry.value = null;
  formLabel.value = '';
  formAddress.value = '';
  formNotes.value = '';
  errorMessage.value = '';
}

function saveEntry() {
  errorMessage.value = '';
  
  try {
    if (editingEntry.value) {
      addressBook.updateEntry(editingEntry.value.id, {
        label: formLabel.value,
        notes: formNotes.value || undefined,
      });
    } else {
      addressBook.addEntry(formLabel.value, formAddress.value, formNotes.value || undefined);
    }
    closeModal();
  } catch (e: any) {
    errorMessage.value = e.message;
  }
}

function confirmDelete(entry: AddressBookEntry) {
  deletingEntry.value = entry;
}

function cancelDelete() {
  deletingEntry.value = null;
}

function executeDelete() {
  if (deletingEntry.value) {
    addressBook.deleteEntry(deletingEntry.value.id);
    deletingEntry.value = null;
  }
}

onMounted(() => {
  addressBook.loadFromStorage();
});
</script>

<style scoped>
.list-enter-active,
.list-leave-active {
  transition: all 0.3s ease;
}
.list-enter-from {
  opacity: 0;
  transform: translateY(-10px);
}
.list-leave-to {
  opacity: 0;
  transform: translateX(-10px);
}

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
