<script lang="ts">
    import { Router, Route } from "svelte-routing";
    import ConversationsList from "./pages/ConversationsList.svelte";
    import Conversation from "./pages/Conversation.svelte";
    import { Body } from "svelte-body";
    import Header from "./lib/Header.svelte";
    export let url = "/";
</script>

<Body class="text-zinc-50 min-h-screen" />
<Router {url}>
    <Header />
    <Route path="/" component={ConversationsList} />
    <Route path="/new" component={Conversation} />
    <Route path="/:id" component={Conversation} />
</Router>

<style>
    :global(#app) {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }
</style>
