<script lang="ts">
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import Tab from "./Tab.svelte";
  import type { TabInfo } from "$lib/types";
  import { Button } from "$lib/components/ui/button";

  export let isPrivate: boolean;

  const tabs = writable<TabInfo[]>([]);
  let activeTabId: string | null = null;
  let currentEngine: string = "webkit";

  // Initialize with a new tab
  onMount(() => {
    createNewTab("https://www.google.com");
  });

  function createNewTab(url: string = "https://www.google.com"): void {
    const id = crypto.randomUUID();
    const newTab: TabInfo = {
      id,
      url,
      title: url === "https://www.google.com" ? "Google" : "New Tab",
      isActive: true,
      isPrivate,
      engineType: currentEngine
    };

    tabs.update((currentTabs: TabInfo[]) => {
      // Deactivate all other tabs
      const updatedTabs = currentTabs.map((tab: TabInfo) => ({
        ...tab,
        isActive: false
      }));
      return [...updatedTabs, newTab];
    });

    activeTabId = id;
  }

  function closeTab(id: string, event: Event): void {
    event.stopPropagation();
    tabs.update((currentTabs: TabInfo[]) => {
      const index = currentTabs.findIndex((tab: TabInfo) => tab.id === id);
      if (index === -1) return currentTabs;

      const newTabs = [...currentTabs];
      newTabs.splice(index, 1);

      // If we're closing the active tab, activate the next available tab
      if (id === activeTabId) {
        const nextTab = newTabs[index] || newTabs[index - 1];
        if (nextTab) {
          nextTab.isActive = true;
          activeTabId = nextTab.id;
        }
      }

      // If no tabs left, create a new one
      if (newTabs.length === 0) {
        setTimeout(() => createNewTab(), 0);
        return newTabs;
      }

      return newTabs;
    });
  }

  function activateTab(id: string): void {
    tabs.update((currentTabs: TabInfo[]) => {
      return currentTabs.map((tab: TabInfo) => ({
        ...tab,
        isActive: tab.id === id
      }));
    });
    activeTabId = id;
  }

  function switchEngine(newEngine: string): void {
    currentEngine = newEngine;
    tabs.update((currentTabs: TabInfo[]) => {
      return currentTabs.map((tab: TabInfo) => ({
        ...tab,
        engineType: newEngine
      }));
    });
  }

  function handleKeyDown(event: KeyboardEvent, id: string): void {
    if (event.key === 'Enter' || event.key === ' ') {
      activateTab(id);
    }
  }

  // Watch for private mode changes
  $: {
    if ($tabs.length > 0) {
      tabs.update((currentTabs: TabInfo[]) => {
        return currentTabs.map((tab: TabInfo) => ({
          ...tab,
          isPrivate
        }));
      });
    }
  }
</script>

<div class="flex flex-col h-full">
  <!-- Tab Bar -->
  <div class="flex items-center justify-between bg-card/80 backdrop-blur supports-[backdrop-filter]:bg-card/60 border-b h-8 px-2 z-50">
    <div class="flex items-center flex-1" role="tablist">
      {#each $tabs as tab (tab.id)}
        <div 
          role="tab"
          tabindex="0"
          aria-selected={tab.isActive}
          class="flex items-center h-7 px-3 rounded-t border-t border-x {tab.isActive ? 'bg-background/80 backdrop-blur border-border' : 'bg-muted/50 border-transparent'} 
                 hover:bg-background/80 cursor-pointer mr-1"
          on:click={() => activateTab(tab.id)}
          on:keydown={(e) => handleKeyDown(e, tab.id)}
        >
          <span class="text-sm truncate max-w-[200px]">{tab.title}</span>
          <button 
            type="button"
            class="ml-2 text-muted-foreground hover:text-foreground"
            on:click={(e) => closeTab(tab.id, e)}
            aria-label={`Close ${tab.title} tab`}
          >
            ×
          </button>
        </div>
      {/each}
      <button 
        type="button"
        class="h-7 w-7 flex items-center justify-center rounded hover:bg-muted"
        on:click={() => createNewTab()}
        aria-label="Create new tab"
      >
        +
      </button>
    </div>

    <!-- Engine Switcher -->
    <div class="flex items-center space-x-2">
      <Button
        variant={currentEngine === "webkit" ? "default" : "ghost"}
        size="sm"
        on:click={() => switchEngine("webkit")}
      >
        WebKit
      </Button>
      <Button
        variant={currentEngine === "chromium" ? "default" : "ghost"}
        size="sm"
        on:click={() => switchEngine("chromium")}
      >
        Chromium
      </Button>
    </div>
  </div>

  <!-- Tab Content -->
  <div class="flex-1 relative z-0" role="tabpanel">
    {#each $tabs as tab (tab.id)}
      <Tab 
        url={tab.url}
        isActive={tab.isActive}
        isPrivate={tab.isPrivate}
        engineType={tab.engineType}
      />
    {/each}
  </div>
</div> 