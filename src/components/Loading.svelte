<script lang="ts">
    import type { ThemeSchema } from "$lib/interfaces";
    import { scale } from "svelte/transition";

    export let theme: ThemeSchema
    export let msg: string
</script>

<div class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50 font-exo2">
    
    <div transition:scale class="p-2 rounded-sm border text-sm pr-3 align-baseline" style="background-color: {theme.background};color: {theme.foreground}; border-color: {theme.primary}">
        <div class="text-xl">Please Wait</div>
        <div class="h-[1px] mb-2" style="background-color: {theme.primary};"></div>
        <div class="flex">
            <img src="/logo.png" width="25px" alt="logo" class="animate-bounce">&nbsp; {msg}</div>
    </div>
</div>