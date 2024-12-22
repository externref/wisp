<script lang="ts">
    import Editor from "./Editor.svelte";
    import Greeting from "./Greeting.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { slide } from "svelte/transition";
    import { normalizePath, debounce,  } from "../editorUtils";
    import { homeDir as _hdir} from "@tauri-apps/api/path"

    let filedata: { name: string; code: string; path: string };
    interface SearchResultsData {
        name: string;
        path: string;
        is_dir: boolean;
    }
    let cmdInpt: HTMLInputElement;
    export let commandSearch: string;
    let commandOptions = [
            { icon: "bi bi-file-earmark-plus", name: "Open File", enum: 1 },
            { icon: "bi bi-plus-square", name: "New File", enum: 2 },
            { icon: "bi bi-palette-fill", name: "Theme", enum: 3 },
    ];
    let commandValues = commandOptions;
    let searchResultOptions: SearchResultsData[] = [];
    let searchResultValue: SearchResultsData[] = searchResultOptions;
    let showModal: boolean = false;
    let showCommands: boolean = false;
    let targetSearchFilePath: string;
    let fileInpt: HTMLInputElement;

    let homeDir: string;

 

    async function openFile(filepath: string) {
        invoke("get_file_data", { pathStr: filepath }).then((val) => {
            filedata = val as any;
            showModal = false;
        });
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.ctrlKey && event.key === "o") {
            event.preventDefault();
            showModal = !showModal;
            if (showModal) {
                setTimeout(() => fileInpt?.focus(), 0);
            }
        } else if (event.ctrlKey && event.shiftKey && event.key.toLowerCase() == "p") {
            event.preventDefault();
            showCommands = !showCommands;
            if (showCommands) {
                setTimeout(() => cmdInpt?.focus(), 0);
            }
        }
    }

    async function searchInputUpdateCmds() {}

    async function searchInputsUpdate() {
        let normalizedPath = normalizePath(targetSearchFilePath);

        // Replace ~ with the actual home directory
        if (normalizedPath.startsWith("~")) {
            normalizedPath = normalizedPath.replace("~", homeDir);
        }

        if (normalizedPath.endsWith("/")) {
            await invoke("file_search_simulate", { pathStr: normalizedPath })
                .then((val) => {
                    searchResultOptions = val as SearchResultsData[];
                })
                .catch(() => {
                    searchResultOptions = [];
                });
        } else {
            searchResultValue = searchResultOptions.filter((entry) =>
                entry.path.toLowerCase().includes(normalizedPath.toLowerCase())
            );
        }
        searchResultValue = searchResultValue.slice(0, 10);
    }

    const debouncedSearchUpdate = debounce(searchInputsUpdate,0);

    function handleSearchInput(event: Event) {
        const inputElement = event.target as HTMLInputElement;
        targetSearchFilePath = inputElement.value;
        debouncedSearchUpdate();
    }

    onMount(() => {
         _hdir().then((r)=> {homeDir=r})
        window.addEventListener("keydown", handleKeydown);
        return () => {
            window.removeEventListener("keydown", handleKeydown);
        };
    });
</script>

{#if filedata}
    <Editor {filedata}></Editor>
{:else}
    <Greeting></Greeting>
{/if}

{#if showModal}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex flex-row justify-center z-50">
        <div transition:slide={{ axis: "y" }} class="bg-[#44475a] h-min rounded w-[60%] mt-10">
            <div class="mx-4 mt-4">Enter file path</div>
            <div class="border bg-[#282a36] m-4 mt-1 rounded border-purple-500 text-purple-500">
                &nbsp;> 
                <input
                    bind:this={fileInpt}
                    class="bg-[#282a36] outline-none text-[#f8f8f2] w-[95%]"
                    bind:value={targetSearchFilePath}
                    type="text"
                    on:input={handleSearchInput}
                    on:keydown={(e) => e.key === "Enter" && openFile(targetSearchFilePath)}
                />
            </div>
            {#if searchResultValue.length}
                {#each searchResultValue as search}
                    <div class="mx-4 my-2 bg-[#282a36] text-sm px-4">
                        {@html search.is_dir ? `<i class="bi bi-folder2-open"></i>` : `<i class="bi bi-file-earmark-code"></i>`}&nbsp;{search.path}
                    </div>
                {/each}
                <div class="mb-1">&nbsp;</div>
            {/if}
        </div>
    </div>
{/if}

{#if showCommands}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex flex-row justify-center z-50">
        <div transition:slide={{ axis: "y" }} class="bg-[#44475a] h-min rounded w-[60%] mt-10">
            <div class="mx-4 mt-4">Enter command</div>
            <div class="border bg-[#282a36] m-4 mt-1 rounded border-purple-500 text-purple-500">
                &nbsp;> 
                <input
                    bind:this={cmdInpt}
                    class="bg-[#282a36] outline-none text-[#f8f8f2] w-[95%]"
                    bind:value={commandSearch}
                    type="text"
                    on:input={searchInputUpdateCmds}
                />
            </div>
            {#each commandValues as cmd}
                <div class="mx-4">
                    <div class="mx-4 my-2 bg-[#282a36] text-sm px-4">
                        {@html cmd.enum}
                    </div>
                </div>
            {/each}
            <div class="mb-4"></div>
        </div>
    </div>
{/if}
