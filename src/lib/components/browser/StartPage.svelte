<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  
  let searchQuery = "";
  
  async function handleSearch(e: Event) {
    e.preventDefault();
    if (searchQuery.trim()) {
      // If it looks like a URL, navigate directly
      if (searchQuery.includes('.') && !searchQuery.includes(' ')) {
        await invoke('navigate', { url: searchQuery });
      } else {
        // Otherwise, search using DuckDuckGo
        await invoke('navigate', { 
          url: `https://duckduckgo.com/?q=${encodeURIComponent(searchQuery)}` 
        });
      }
    }
  }
</script>

<div class="flex flex-col items-center justify-center min-h-screen bg-background text-foreground p-4">
  <div class="max-w-2xl w-full space-y-8">
    <div class="text-center">
      <h1 class="text-4xl font-bold mb-2">Solace Browser</h1>
      <p class="text-muted-foreground">Fast, Private, and Secure</p>
    </div>
    
    <form on:submit={handleSearch} class="flex flex-col items-center space-y-4">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search or enter URL"
        class="w-full px-4 py-2 rounded-lg border bg-card text-card-foreground focus:outline-none focus:ring-2 focus:ring-primary"
      />
      <button
        type="submit"
        class="px-6 py-2 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
      >
        Search
      </button>
    </form>
  </div>
</div> 