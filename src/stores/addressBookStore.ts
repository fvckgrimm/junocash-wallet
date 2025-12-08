import { defineStore } from 'pinia';

export interface AddressBookEntry {
  id: string;
  label: string;
  address: string;
  notes?: string;
  createdAt: number;
  lastUsed?: number;
}

export const useAddressBookStore = defineStore('addressBook', {
  state: () => ({
    entries: [] as AddressBookEntry[],
  }),

  getters: {
    sortedEntries: (state) => {
      return [...state.entries].sort((a, b) => {
        // Sort by last used (most recent first), then by label
        if (a.lastUsed && b.lastUsed) {
          return b.lastUsed - a.lastUsed;
        }
        if (a.lastUsed) return -1;
        if (b.lastUsed) return 1;
        return a.label.localeCompare(b.label);
      });
    },

    getByAddress: (state) => (address: string) => {
      return state.entries.find(e => e.address === address);
    },
  },

  actions: {
    loadFromStorage() {
      const stored = localStorage.getItem('addressBook');
      if (stored) {
        try {
          this.entries = JSON.parse(stored);
        } catch (e) {
          console.error('Failed to load address book:', e);
          this.entries = [];
        }
      }
    },

    saveToStorage() {
      localStorage.setItem('addressBook', JSON.stringify(this.entries));
    },

    addEntry(label: string, address: string, notes?: string): AddressBookEntry {
      // Check if address already exists
      const existing = this.entries.find(e => e.address === address);
      if (existing) {
        throw new Error('This address is already in your address book');
      }

      const entry: AddressBookEntry = {
        id: Date.now().toString() + Math.random().toString(36).substring(7),
        label,
        address,
        notes,
        createdAt: Date.now(),
      };

      this.entries.push(entry);
      this.saveToStorage();
      return entry;
    },

    updateEntry(id: string, updates: Partial<Pick<AddressBookEntry, 'label' | 'notes'>>) {
      const entry = this.entries.find(e => e.id === id);
      if (!entry) {
        throw new Error('Entry not found');
      }

      if (updates.label !== undefined) entry.label = updates.label;
      if (updates.notes !== undefined) entry.notes = updates.notes;

      this.saveToStorage();
    },

    deleteEntry(id: string) {
      this.entries = this.entries.filter(e => e.id !== id);
      this.saveToStorage();
    },

    markAsUsed(address: string) {
      const entry = this.entries.find(e => e.address === address);
      if (entry) {
        entry.lastUsed = Date.now();
        this.saveToStorage();
      }
    },

    searchEntries(query: string): AddressBookEntry[] {
      const q = query.toLowerCase();
      return this.entries.filter(e =>
        e.label.toLowerCase().includes(q) ||
        e.address.toLowerCase().includes(q) ||
        e.notes?.toLowerCase().includes(q)
      );
    },
  },
});
