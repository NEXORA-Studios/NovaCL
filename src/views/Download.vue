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
            <LogoLoader text="此页面施工中……"  error-text="未知错误……"/>
        </div>
    </BlurBackground>
</template>
