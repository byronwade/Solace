<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import StartPage from "./StartPage.svelte";

  export let url: string;
  export let isActive: boolean = true;
  export let isPrivate: boolean;
  export let engineType: string = "webkit";

  let currentUrl: string = url;
  let showStartPage = url === "https://www.google.com";

  // Watch for URL changes
  $: if (url !== currentUrl && isActive) {
    currentUrl = url;
    showStartPage = false;
    console.log('Tab: Navigating to', currentUrl);
    invoke('navigate_to', { url: currentUrl })
      .catch(error => {
        console.error('Failed to navigate:', error);
        showStartPage = true;
      });
  }

  // Watch for private mode changes
  $: if (isActive) {
    console.log('Tab: Setting private mode', isPrivate);
    invoke('set_private_mode', { enabled: isPrivate })
      .catch(error => {
        console.error('Failed to set private mode:', error);
      });
  }

  // Watch for engine changes
  $: if (isActive) {
    console.log('Tab: Setting engine to', engineType);
    invoke('switch_browser_engine', { engineType })
      .catch(error => {
        console.error('Failed to switch engine:', error);
      });
  }

  onMount(() => {
    if (isActive && !showStartPage) {
      console.log('Tab: Initial navigation to', currentUrl);
      invoke('navigate_to', { url: currentUrl })
        .catch(error => {
          console.error('Failed to navigate:', error);
          showStartPage = true;
        });
    }
  });
</script>

{#if showStartPage}
  <div class="w-full h-full relative {isActive ? '' : 'hidden'}">
    <StartPage />
  </div>
{:else}
  <div class="w-full h-full relative {isActive ? '' : 'hidden'}" />
{/if}

<style>
  div {
    border: none;
    margin: 0;
    padding: 0;
    display: block;
  }
</style> 