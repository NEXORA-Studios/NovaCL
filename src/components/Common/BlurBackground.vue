<script lang="ts" setup>
    defineProps<{
        backgroundImage?: string;
    }>();
</script>

<template>
    <div class="relative w-full h-full overflow-hidden">
        <!-- 背景层 -->
        <div
            class="absolute inset-0 bg-cover bg-center blur-sm"
            :style="{
                backgroundImage: $props.backgroundImage,
            }"></div>

        <!-- 正常内容 -->
        <div class="relative z-10 w-full h-full">
            <slot></slot>
        </div>
    </div>
</template>
