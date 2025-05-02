<script setup lang="ts">
    import { BlurBackground, LogoLoader } from "@/components";
</script>
<template>
    <BlurBackground>
        <div
            id="Settings"
            class="mt-12 pt-4 pb-8 px-8 flex flex-col justify-center items-center"
            style="
                height: calc(100% - 48px);
                max-height: calc(100% - 48px);
                overflow: hidden;
            ">
            <LogoLoader text="此页面施工中……" />
            <!-- <div class="tabs tabs-lift">
                <label class="tab">
                    <input
                        type="radio"
                        name="setting_tab_1"
                        checked />
                    <i
                        class="icon-[meteor-icons--gamepad-modern] scale-110 mr-2" />
                    游戏设置
                </label>
                <div class="tab-content border-base-300 bg-base-100 p-4 h-full">
                    <section class="flex flex-col gap-4">
                        <div class="divider mt-0">启动选项</div>
                    </section>
                </div>

                <label class="tab">
                    <input
                        type="radio"
                        name="setting_tab_1" />
                    <i class="icon-[meteor-icons--palette] scale-110 mr-2" />
                    个性化设置
                </label>
                <div class="tab-content border-base-300 bg-base-100 p-4 h-full">
                    <section class="flex flex-col gap-4"></section>
                </div>

                <label class="tab">
                    <input
                        type="radio"
                        name="setting_tab_1" />
                    <i class="icon-[meteor-icons--microchip] scale-110 mr-2" />
                    启动器设置
                </label>
                <div class="tab-content border-base-300 bg-base-100 p-4 h-full">
                    <section class="flex flex-col gap-4"></section>
                </div>
            </div> -->
        </div>
    </BlurBackground>
</template>
