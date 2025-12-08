<template>
  <div>
    <!-- Mobile Backdrop (Only visible when open on small screens) -->
    <div 
      v-if="isOpen" 
      @click="$emit('close')"
      class="fixed inset-0 bg-black/60 backdrop-blur-sm z-30 md:hidden transition-opacity"
    ></div>

    <!-- Sidebar Container -->
    <aside 
      ref="sidebarRef"
      class="fixed md:relative
            top-16 md:top-0
            left-0
            z-40
            flex flex-col
            border-r border-white/5
            bg-[#13161c] md:bg-transparent md:glass
            transition-all duration-300 ease-out md:transition-none
            h-[calc(100vh-4rem)] md:h-screen
    "
      :class="[
        isOpen ? 'translate-x-0 shadow-2xl' : '-translate-x-full md:translate-x-0',
        isResizing ? 'select-none transition-none' : ''
      ]"
      :style="{ width: currentWidth + 'px' }"
    >
      
      <!-- Logo Area -->
      <div class="h-20 md:h-24 flex items-center justify-between px-6 md:px-8 shrink-0 overflow-hidden">
        <div class="flex items-center gap-3 select-none min-w-max">
          <div class="relative w-8 h-8 md:w-10 md:h-10 flex items-center justify-center rounded-xl bg-gradient-to-br from-indigo-500 to-purple-600 shadow-lg shadow-indigo-500/20">
            <span class="text-white font-bold text-lg md:text-xl leading-none font-mono">J</span>
          </div>
          <!-- Hide text if sidebar gets too narrow -->
          <transition name="fade">
            <div v-if="currentWidth > 160">
              <h1 class="text-base md:text-lg font-bold tracking-tight text-white">Juno<span class="text-indigo-400">Cash</span></h1>
              <p class="text-[10px] uppercase tracking-widest text-gray-500 font-medium">Wallet</p>
            </div>
          </transition>
        </div>

        <!-- Mobile Close Button -->
        <button 
          @click="$emit('close')"
          class="md:hidden p-2 text-gray-400 hover:text-white bg-white/5 rounded-lg"
        >
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6L6 18M6 6l12 12"/></svg>
        </button>
      </div>

      <!-- Navigation -->
      <nav class="flex-1 px-4 py-4 md:py-6 overflow-y-auto custom-scrollbar overflow-x-hidden">
        <ul class="space-y-1">
          <li v-for="link in links" :key="link.path">
            <router-link 
              :to="link.path"
              @click="$emit('close')"
              class="group relative flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-200"
              active-class="bg-white/5 shadow-inner"
              v-slot="{ isActive }"
            >
              <!-- Active Indicator Bar -->
              <div 
                class="absolute left-0 h-6 w-1 rounded-r-full bg-indigo-500 transition-all duration-300 transform origin-left"
                :class="isActive ? 'scale-x-100 opacity-100' : 'scale-x-0 opacity-0'"
              ></div>

              <!-- Icon -->
              <component 
                :is="link.icon" 
                class="w-5 h-5 transition-colors duration-300 relative z-10 shrink-0"
                :class="isActive ? 'text-indigo-400' : 'text-gray-400 group-hover:text-gray-200'" 
              />
              
              <!-- Label (Hide if narrow) -->
              <transition name="fade">
                <span 
                  v-show="currentWidth > 160"
                  class="font-medium text-sm transition-colors duration-300 relative z-10 whitespace-nowrap"
                  :class="isActive ? 'text-white' : 'text-gray-400 group-hover:text-gray-200'"
                >
                  {{ link.name }}
                </span>
              </transition>
            </router-link>
          </li>
        </ul>
      </nav>

      <!-- Status Area (Pinned to bottom) -->
      <div class="p-4 border-t border-white/5 bg-black/20 shrink-0 overflow-hidden">
        <!-- Only show full status if wide enough, otherwise show dot -->
        <div v-if="currentWidth > 160">
          <Status />
        </div>
        <div v-else class="flex justify-center">
           <!-- Minified Status Dot -->
           <div class="w-3 h-3 rounded-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)] animate-pulse" title="Online"></div>
        </div>
      </div>

      <!-- RESIZER HANDLE (Desktop Only) -->
      <div 
        class="hidden md:block absolute top-0 right-0 bottom-0 w-1 cursor-col-resize hover:bg-indigo-500/50 active:bg-indigo-500 transition-colors z-50 group"
        @mousedown.prevent="startResize"
        @dblclick="resetWidth"
      >
        <!-- Visual Hint -->
        <div class="absolute right-0 top-1/2 -translate-y-1/2 h-8 w-1 bg-white/10 group-hover:bg-white/50 rounded-full"></div>
      </div>

    </aside>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import Status from './Status.vue';

defineProps<{
  isOpen: boolean
}>();

defineEmits(['close']);

// --- Resizing Logic ---
const sidebarRef = ref<HTMLElement | null>(null);
const isResizing = ref(false);
const sidebarWidth = ref(288);
const minWidth = 80;
const maxWidth = 450;

const currentWidth = computed(() => {
  return sidebarWidth.value;
});

function startResize() {
  isResizing.value = true;
  document.addEventListener('mousemove', handleResize);
  document.addEventListener('mouseup', stopResize);
  document.body.style.cursor = 'col-resize';
}

function handleResize(e: MouseEvent) {
  if (!isResizing.value) return;
  
  let newWidth = e.clientX;
  
  if (newWidth < minWidth) newWidth = minWidth;
  if (newWidth > maxWidth) newWidth = maxWidth;
  
  sidebarWidth.value = newWidth;
}

function stopResize() {
  isResizing.value = false;
  document.removeEventListener('mousemove', handleResize);
  document.removeEventListener('mouseup', stopResize);
  document.body.style.cursor = '';
  
  localStorage.setItem('sidebarWidth', sidebarWidth.value.toString());
}

function resetWidth() {
  sidebarWidth.value = 288;
  localStorage.setItem('sidebarWidth', '288');
}

onMounted(() => {
  const saved = localStorage.getItem('sidebarWidth');
  if (saved) {
    sidebarWidth.value = parseInt(saved);
  }
});

// --- Navigation Data ---
const DashboardIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="7" height="7" rx="1" /><rect x="14" y="3" width="7" height="7" rx="1" /><rect x="14" y="14" width="7" height="7" rx="1" /><rect x="3" y="14" width="7" height="7" rx="1" /></svg>` };
const SendIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><line x1="22" y1="2" x2="11" y2="13" /><polygon points="22 2 15 22 11 13 2 9 22 2" /></svg>` };
const ReceiveIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M12 3v12"/><path d="m8 11 4 4 4-4"/><path d="M8 21h8"/></svg>` };
const HistoryIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/><path d="M12 7v5l4 2"/></svg>` };
const AddressBookIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>` };
const MiningIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M6 3h12l4 6-10 13L2 9z"/><path d="M12 22V9"/><path d="m2 9 20 0"/><path d="m14.5 9-5-6"/><path d="m9.5 9 5-6"/></svg>` };
const SettingsIcon = { template: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.47a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>` };

const links = [
  { name: 'Dashboard', path: '/', icon: DashboardIcon },
  { name: 'Send', path: '/send', icon: SendIcon },
  { name: 'Receive', path: '/receive', icon: ReceiveIcon },
  { name: 'History', path: '/history', icon: HistoryIcon },
  { name: 'Address Book', path: '/address-book', icon: AddressBookIcon },
  { name: 'Mining', path: '/mining', icon: MiningIcon },
  { name: 'Settings', path: '/settings', icon: SettingsIcon },
];
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
