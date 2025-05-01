<script setup lang="ts">
    import { BlurBackground } from "@/components";
    import { getImageNameFromVersionCode } from "@/utils";

    const MC_VERSION = "1.21.1";
    const MC_DESCRIPTION = "NeoForge 21.1.162";

    const backgroundUrl = "/version/" + getImageNameFromVersionCode(MC_VERSION);

    function getGreetingMessage(name: string) {
        const hour = new Date().getHours();
        if (hour < 5) {
            return `凌晨好！${name}`;
        } else if (hour < 12) {
            return `早上好！${name}`;
        } else if (hour < 18) {
            return `下午好！${name}`;
        } else if (hour < 24) {
            return `晚上好！${name}`;
        }
    }
</script>

<template>
    <BlurBackground :backgroundImage="`url(${backgroundUrl})`">
        <div
            id="Home"
            class="mt-12 pt-4 pb-8 px-8 flex flex-col"
            style="height: calc(100% - 48px)">
            <h1 class="text-2xl font-bold">
                {{ getGreetingMessage("MoYuan-CN") }}
            </h1>
            <section class="mt-auto flex justify-between items-center">
                <div left>
                    <h1 class="text-6xl font-bold">{{ MC_VERSION }}</h1>
                    <h3 class="text-3xl font-bold">{{ MC_DESCRIPTION }}</h3>
                </div>
                <div
                    right
                    class="h-full flex items-end gap-4">
                    <button
                        class="btn h-3/5 border-info text-2xl tracking-wider px-6 rounded-full">
                        <i
                            class="icon-[mdi--application-cog-outline] scale-120" />
                    </button>
                    <button
                        :class="$style.colorfulBtn"
                        class="btn h-3/5 border-info text-2xl tracking-wider rounded-full px-6">
                        <i class="icon-[mdi--rocket-launch] scale-120" />
                        <span>启动游戏</span>
                    </button>
                </div>
            </section>
        </div>
    </BlurBackground>
</template>

<style lang="scss" module>
    .colorfulBtn {
        background: linear-gradient(
            to right,
            var(--color-primary),
            var(--color-secondary)
        );
        border: none;
        transition: all 0.15s ease-in-out;
        &:hover {
            filter: hue-rotate(16deg);
        }
    }
</style>
