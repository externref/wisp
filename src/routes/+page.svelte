<script lang="ts">
    import Editor from "./Editor.svelte";
    import Greeting from "./Greeting.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { slide } from "svelte/transition";
    let filedata: {
        name: string;
        code: string;
        path: string
    };
    interface SearchResultsData {
        name: string,
        path: string,
        is_dir: boolean
    }

    let searchResultOptions: SearchResultsData[] = []
    let searchResultValue: SearchResultsData[] = searchResultOptions
    let showModal: boolean = false;
    let targetSearchFilePath: string;
    let inpt: HTMLInputElement;
    function openFile(filepath: string) {
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
                setTimeout(() => inpt?.focus(), 0);
            }
        }
    }
   async function searchInputsUpdate(){
        if( targetSearchFilePath.endsWith("/") || targetSearchFilePath.endsWith("\\")){
            // alert(targetSearchFilePath)
            await invoke("file_search_simulate", {pathStr: targetSearchFilePath}).then(
        (val) => {searchResultOptions=val as SearchResultsData[]}
        
        
        )
    searchResultValue = searchResultOptions;
    return
        }
        searchResultValue = []
        for( const entry of searchResultOptions){
            if (entry.path.startsWith(targetSearchFilePath)){
                searchResultValue.push(entry)
            }
        }
    }

    onMount(() => {
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
    <div
        class="fixed inset-0 bg-black bg-opacity-50 flex flex-row justify-center z-50"
    >
        <div
            transition:slide={{ axis: "y" }}
            class="bg-[#44475a] h-min rounded w-[60%] mt-10"
        >
            <div class="mx-4 mt-4">Enter file path</div>
            <div
                class="border bg-[#282a36] m-4 mt-1 rounded border-purple-500 text-purple-500"
            >
                &nbsp;> <input
                    bind:this={inpt}
                    class="bg-[#282a36] outline-none text-[#f8f8f2] w-[95%]"
                    bind:value={targetSearchFilePath}
                    type="text"
                    on:input={searchInputsUpdate}
                    on:keydown={(e) =>
                        e.key === "Enter" && openFile(targetSearchFilePath)}
                />
            </div>
            {#if searchResultValue.length}

            {#each searchResultValue.slice(0, 6) as search}
                
            <div class="mx-4 my-2 bg-[#282a36] text-sm px-4">{@html search.is_dir? `<i class="bi bi-folder2-open"></i>`: `<i class="bi bi-file-earmark-code"></i>`}&nbsp{search.path}</div>
            {/each}
            <div class="mb-1">&nbsp;</div>
        {/if}
        
        </div>
    </div>
  
{/if}

