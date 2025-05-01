<script setup lang="ts">
    defineProps<{
        title: string;
        canSwap?: boolean;
        isSwapped?: boolean;
    }>();
</script>

<template>
    <div
        tabindex="0"
        class="collapse bg-base-100 border-base-300 border"
        :class="{
            'collapse-open': !$props.canSwap,
            'collapse-arrow': $props.canSwap,
        }">
        <input
            type="checkbox"
            :checked="$props.canSwap && !$props.isSwapped"
            class="-my-2" />
        <div class="collapse-title font-semibold -my-2">
            {{ $props.title }}
        </div>
        <div class="collapse-content">
            <slot />
        </div>
    </div>
</template>

<style lang="scss" module></style>
