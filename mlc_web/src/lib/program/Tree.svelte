<script context="module" lang="ts">
    import {
        ArrowLeft,
        Folder,
        FolderOpen,
        Flame,
        Clapperboard,
        Aperture,
    } from "lucide-svelte";
    import {writable} from "svelte/store";

    type Icon = "effect" | "folder";

    export type TreeItem = {
        title: string;
        icon: Icon;
        name: string;
        id: string;

        children?: TreeItem[];
    };

    export const icons = {
        folder: Folder,
        folderOpen: FolderOpen,
        highlight: ArrowLeft,
        effect: Aperture,
    };
</script>

<script lang="ts">
    import {melt, type TreeView} from "@melt-ui/svelte";
    import {getContext} from "svelte";
    import {openEffect} from "../../customTypings/Effect";

    export let treeItems: TreeItem[];
    export let level: 1;

    const {
        elements: {item, group},
        helpers: {isExpanded, isSelected},
    } = getContext<TreeView>("effectTree");

    let iconSize = "1rem";

    function updateSelection(id: string) {
        openEffect.set(id);
    }
</script>

{#each treeItems as {title, icon, children, name, id}, i}
    {@const itemId = title + "-" + i}
    {@const hasChildren = !!children?.length}

    <li class={level !== 1 ? "liClass" : ""} style="list-style: none;">
        <button
                use:melt={$item({ id: itemId, hasChildren })}
                class="buttonClass"
                on:dblclick={(v) => {
        if (!hasChildren) {
          updateSelection(id);
        }
      }}
        >
            {#if icon === "folder" && hasChildren && $isExpanded(itemId)}
                <svelte:component
                        this={icons["folderOpen"]}
                        size={iconSize}
                        class="iconClass"
                />
            {:else}
                <svelte:component
                        this={icons[icon]}
                        size={iconSize}
                        class="iconClass"
                />
            {/if}

            <span class="select-none">{title}</span>

            <!-- {#if $isSelected(itemId)}
              <svelte:component
                this={icons["highlight"]}
                size={iconSize}
                class="iconClass"
              />
            {/if} -->
        </button>

        {#if children}
            <ul use:melt={$group({ id: itemId })} class="ulClass">
                <svelte:self treeItems={children} level={level + 1}/>
            </ul>
        {/if}
    </li>
{/each}

<style>
    .iconClass {
        height: 0.5rem;
        width: 0.5rem;
    }

    .liClass {
        list-style: none;
    }

    .ulClass {
        list-style: none;
        margin: 0;
        padding-left: 1.5rem;
    }

    .buttonClass {
        display: flex;
        align-items: center;
        gap: 0.25rem;
        border-radius: 0.25rem;
        height: 2rem;
        padding: 0.25rem;
    }

    .buttonClass:focus {
        color: var(--color-accent);
    }

    .buttonClass:focus-visible {
        outline: 4px solid var(--color-accent);
    }
</style>
