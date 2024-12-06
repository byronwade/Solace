<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";
  import { writable } from 'svelte/store';
  import { Button } from "$lib/components/ui/button";
  import { Switch } from "$lib/components/ui/switch";
  import TabManager from "$lib/components/browser/TabManager.svelte";

  let isPrivate = false;

  async function handleWindowControl(action: 'minimize' | 'maximize' | 'close') {
    try {
      await invoke(`${action}_window`);
    } catch (error) {
      console.error(`Failed to ${action} window:`, error);
    }
  }
</script>

<div class="h-screen flex flex-col">
  <!-- Window Controls -->
  <header class="h-8 flex items-center justify-between px-4 bg-card/80 backdrop-blur supports-[backdrop-filter]:bg-card/60 border-b z-50" data-tauri-drag-region>
    <span class="text-sm font-medium">Solace Browser</span>
    <div class="flex items-center space-x-2">
      <Button 
        variant="ghost" 
        size="icon" 
        class="h-6 w-6" 
        on:click={() => handleWindowControl('minimize')}>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
      </Button>
      <Button 
        variant="ghost" 
        size="icon" 
        class="h-6 w-6" 
        on:click={() => handleWindowControl('maximize')}>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
        </svg>
      </Button>
      <Button 
        variant="ghost" 
        size="icon" 
        class="h-6 w-6 hover:bg-destructive hover:text-destructive-foreground" 
        on:click={() => handleWindowControl('close')}>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </Button>
    </div>
  </header>

  <!-- Settings Bar -->
  <div class="bg-card/80 backdrop-blur supports-[backdrop-filter]:bg-card/60 border-b p-2 z-50">
    <div class="flex items-center justify-end max-w-md mx-auto">
      <div class="flex items-center space-x-2">
        <span>Private:</span>
        <Switch 
          checked={isPrivate} 
          on:click={() => {
            isPrivate = !isPrivate;
            invoke('set_private_mode', { enabled: isPrivate });
          }} 
        />
      </div>
    </div>
  </div>

  <!-- Browser Content -->
  <div class="flex-1 relative">
    <TabManager isPrivate={isPrivate} />
  </div>
</div>

<style>
  :global(html) {
    background: transparent !important;
  }
  
  :global(body) {
    background: transparent !important;
  }
</style>
