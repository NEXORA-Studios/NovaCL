<script setup lang="ts">
    import { h, ref } from "vue";
    import {
        BlurBackground,
        ScrollableView,
        SearchBar,
        ViewSwitch,
    } from "@/components";
    import { IViewItem } from "@/types/ViewSwitch";
    import DownloadClientjar from "@/views/DownloadClientJar.vue";

    const subView = ref<IViewItem>({
        label: "Minecraft 核心",
        id: "clientjar",
    });
    const subViewOptions: Array<IViewItem> = [
        {
            label: "Minecraft 核心",
            id: "clientjar",
        },
        { label: "模组", id: "mod" },
        { label: "整合包", id: "modpack" },
        { label: "资源包", id: "resourcepack" },
        { label: "光影包", id: "shaderpack" },
        { label: "数据包", id: "datapack" },
    ];
    const searchContent = ref<string>();
    const subViewComponents: {
        [key: string]: any | null;
    } = {
        clientjar: h(DownloadClientjar, { filter: searchContent }),
        mod: null,
        modpack: null,
        resourcepack: null,
        shaderpack: null,
        datapack: null,
    };
</script>

<template>
    <BlurBackground>
        <div
            id="Downloads"
            class="mt-12 pt-4 pb-8 px-8 flex flex-col"
            style="
                height: calc(100% - 48px);
                max-height: calc(100% - 48px);
                overflow: hidden;
            ">
            <ScrollableView>
                <template #header>
                    <section class="w-full flex gap-2">
                        <SearchBar
                            class="w-3/4"
                            v-model="searchContent" />
                        <ViewSwitch
                            class="w-1/4"
                            v-model="subView"
                            :options="subViewOptions" />
                    </section>
                </template>

                <section class="w-full h-full mt-4 flex flex-col gap-2">
                    <component :is="subViewComponents[subView.id]" />
                </section>
            </ScrollableView>
        </div>
    </BlurBackground>
</template>
