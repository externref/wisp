<script lang="ts">
    import type { FileEntry } from "../lib/interfaces";
    import { fileToIcon } from "../utils/icons";

    export let files: FileEntry[];
    let show: { [x: string]: boolean } = {};

    function manageShow(path: string) {
        if (!show[path as keyof typeof show]) {
            show[path] = false;
        }
        show[path as keyof typeof show] = !show[path as keyof typeof show];
    }
</script>
<div class="px-3">
    {#each files as file}
        <div class="grid grid-col justify-items-start">
            {#if file.is_directory}
                <button
                    id={file.path}
                    on:click={() => {
                        manageShow(file.path);
                    }}
                    >{@html show[file.path]
                        ? `<i class="bi bi-folder2-open"></i>`
                        : `<i class="bi bi-folder2"></i>`}
                    {file.name}</button
                >
                {#if show[file.path]}
                    <svelte:self files={file.children}></svelte:self>
                {/if}
            {/if}
        </div>
        <div class="grid grid-col justify-items-start">
            {#if !file.is_directory}
                <button id={file.path} class="hover:">
                    {@html fileToIcon(file.name)}&nbsp; {file.name}
                </button>
            {/if}
        </div>
    {/each}
</div>
