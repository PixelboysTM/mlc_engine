<script lang="ts">
    import {createTreeView} from "@melt-ui/svelte";
    import {createEventDispatcher, setContext} from "svelte";
    import Tree, {type TreeItem} from "./Tree.svelte";
    import {info} from "../stores";
    import {openEffect} from "../../customTypings/Effect";

    const ctx = createTreeView({defaultExpanded: []});

    setContext("effectTree", ctx);

    info.subscribe((data) => {
        if (data === "EffectListChanged") {
            updateEffects();
        }
    });

    const {
        elements: {tree},
        states: {selectedItem},
    } = ctx;

    type EI = [string, string];

    let effectNames: EI[] = [];

    let treeItems: TreeItem[] = [];

    function updateEffects() {
        fetch("/effects/get").then((res) => {
            res.json().then((data) => {
                effectNames = data as EI[];
                treeItems = makeTree(effectNames);
            });
        });
    }

    updateEffects();

    //   [
    //     "Effect 1",
    //     "Effect 2",
    //     "Effect 3",
    //     "Folder A/Effect 4",
    //     "Folder A/Effect 5",
    //     "Folder A/Effect 6",
    //     "Folder B/Effect 7",
    //     "Folder B/Effect 8",
    //     "Folder B/Effect 9",
    //     "Folder B/Folder C/Effect 10",
    //     "Folder B/Folder C/Effect 11",
    //     "Folder B/Folder C/Effect 12",
    //     "Effect 13",
    //     "Effect 14",
    //     "Folder A/Effect 15",
    //     "Folder X/Effect 16",
    //   ];

    function makeTree(items: string[][]): TreeItem[] {
        const treeRaw: any = {children: {}};

        for (const item of items) {
            const parts = item[0].split("/");
            if (parts.length === 1) {
                treeRaw.children[item[0]] = {
                    title: item[0],
                    icon: "effect",
                    name: item[0],
                    id: item[1],
                };
            } else {
                let current = treeRaw.children;
                for (const part of parts.slice(0, -1)) {
                    if (!current[part]) {
                        current[part] = {children: {}, title: part, icon: "folder"};
                    }
                    current = current[part].children;
                }
                current[parts[parts.length - 1]] = {
                    title: parts[parts.length - 1],
                    icon: "effect",
                    name: item[0],
                    id: item[1],
                };
            }
        }

        function makeChild(item: any): TreeItem {
            if (item.children) {
                return {
                    title: item.title,
                    icon: "folder",
                    name: item.title,
                    id: "",
                    children: Object.values(item.children).map(makeChild),
                };
            } else {
                return {
                    title: item.title,
                    name: item.name,
                    id: item.id,
                    icon: "effect",
                };
            }
        }

        return Object.values(treeRaw.children).map((item: any) => {
            return makeChild(item);
        });
    }

    const dispatch = createEventDispatcher();
    openEffect.subscribe((data) => {
        if (data == "") return;
        console.log(data);
        dispatch("open", data);
    })
</script>

<div {...$tree}>
    <Tree {treeItems}
          level={1}/>
</div>
