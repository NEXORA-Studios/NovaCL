<script setup lang="ts"></script>

<template>
    <section :class="$style.compSearch">
        <aside
            class="h-full bg-base-200 rounded-lg"
            :class="$style.compSearch_aside"></aside>
    </section>
</template>

<style lang="scss" module>
    .compSearch {
        width: 100%;
        height: 100%;
        display: grid;
        grid-template-columns: 224px 1fr;
        grid-template-rows: 1fr;
        grid-template-areas: "aside main";
    }

    .compSearch_aside {
        width: 100%;
        height: 100%;
        grid-area: aside;
    }
    .compSearch_main {
        width: 100%;
        height: 100%;
        grid-area: main;
    }
</style>
