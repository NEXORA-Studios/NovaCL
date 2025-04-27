<script setup lang="ts">
    import { Nav } from "@/components";
</script>

<template>
    <div
        id="app"
        class="w-screen h-screen bg-base-100">
        <Nav />
        <section
            class="w-full transition-opacity duration-300 ease-in-out h-full"
            id="main">
            <Suspense>
                <RouterView />
            </Suspense>
        </section>
    </div>
</template>

<style lang="scss" module></style>
