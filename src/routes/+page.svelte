<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import * as themes from "thememirror";
    import type { Extension } from "@codemirror/state";
    import { themeSchemes } from "$lib/theming";
    import type {
        Configs,
        Session,
        FileEntry,
        ThemeSchema,
    } from "../lib/interfaces";
    import FileTree from "../components/FileTree.svelte";
    import Home from "../components/Home.svelte";
    import {open} from "@tauri-apps/plugin-dialog"
    import Loading from "../components/Loading.svelte";
    import { handlePathName } from "$lib/strings";

    let configs: Configs;
    let session: Session | null;

    let configsLoaded = false;
    let currentDirectory: string;

    let buffer_theme: Extension;
    let editor_theme: ThemeSchema;
    let buffer_font_size: number;
    let editor_font_size: number;

    let fileTree: FileEntry[];

    let showOpenLoading=false;
    async function open_new_directory(){
        let dirname = await open({directory: true})
        if (dirname!=null){
            currentDirectory=dirname; 
            showOpenLoading=true;
            fileTree = await invoke("open_directory", {
                path: currentDirectory,
            });
            showOpenLoading=false;
        }
    }

    async function commandHandler(e:KeyboardEvent) {
        if (e.ctrlKey&&( e.key.toLowerCase()=="o")){
            e.preventDefault()
            open_new_directory()
        }
    }

    let sidebarWidth = 160;
    let a =1
    function startResizing(e: MouseEvent) {
        const startX = e.clientX;
        const startWidth = sidebarWidth;

        function onMouseMove(e: MouseEvent) {
            sidebarWidth = Math.max(150, startWidth + e.clientX - startX);
        }

        function onMouseUp() {
            window.removeEventListener("mousemove", onMouseMove);
            window.removeEventListener("mouseup", onMouseUp);
        }

        window.addEventListener("mousemove", onMouseMove);
        window.addEventListener("mouseup", onMouseUp);
    }

    onMount(async () => {
        configs = await invoke("read_configs");
        session = await invoke("last_session");
        if (session) {
            currentDirectory = session.directory;
            fileTree = await invoke("open_directory", {
                path: currentDirectory,
            });
        }
        buffer_font_size = configs.buffer_font_size || 14;
        editor_font_size = configs.editor_font_size || 12;
        // @ts-ignore
        buffer_theme =
            themes[configs.theme as keyof typeof themes] || themes.dracula;
        editor_theme = themeSchemes[configs.theme as keyof typeof themes];
        configsLoaded = true;
        document.addEventListener("keydown", commandHandler)
    }); 
</script>

{#if configsLoaded}
    {#if !currentDirectory}
        <div
            style="background-color: {editor_theme.background};color: {editor_theme.foreground}; font-size: {editor_font_size}px"
            class="h-screen w-screen"
        >
            <Home theme={editor_theme}></Home>

        </div>
    {:else}
        <div
            id="__base__"
            style="background-color: {editor_theme.background};color: {editor_theme.foreground}; font-size: {editor_font_size}px"
        >
            <div class="flex">
                <div class="h-screen overflow-auto no-scrollbar border-r text-nowrap" style="width: {sidebarWidth}px;">
                    <div
                        class="font-kode m-2 italic"
                        style="font-size: {editor_font_size + 2}px;"
                    >
                        {handlePathName(currentDirectory)}
                    </div>
                    <FileTree bind:files={fileTree}></FileTree>
                </div>
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div
                class="cursor-ew-resize w-[2px] bg-white"
                on:mousedown={startResizing}
            ></div>
            </div>
            
        </div>
    {/if}

    {#if showOpenLoading}
    <Loading  theme={editor_theme} msg="Opening Folder: {currentDirectory}">

    </Loading>
    {/if}
     
{/if}

<style>
    .no-scrollbar::-webkit-scrollbar {
        display: none;
    }
    .no-scrollbar {
        -ms-overflow-style: none;
        scrollbar-width: none;
    }

</style>


  