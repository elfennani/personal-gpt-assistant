<script lang="ts">
    export let isLoading: boolean = false;
</script>

<div class="flex items-center justify-center font-bold">
    {#if isLoading}
        Loading...
    {:else}
        Ask anything, get yout answer
    {/if}
</div>
