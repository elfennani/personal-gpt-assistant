<script lang="ts">
    import { Body } from "svelte-body";
    import ConvContent from "../lib/ConvContent.svelte";
    import Input from "../lib/Input.svelte";
    import { onMount } from "svelte";
    import { navigate } from "svelte-routing";

    export let id: string = null;

    onMount(() => {
        // TODO: Implement chat creation
        setTimeout(() => {
            navigate("/hellothere", { replace: true });
        }, 2000);
    });
</script>

<Body class="bg-zinc-800" />
<div class="grid grid-rows-[1fr_min-content] h-full text-zinc-400 flex-1">
    <ConvContent isLoading={!id} />
    <Input />
</div>
