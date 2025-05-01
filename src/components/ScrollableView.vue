<script setup lang="ts">
    import { ref, onMounted, onUnmounted } from "vue";

    const showBackToTop = ref(false);
    const scrollContainer = ref<HTMLElement | null>(null);

    const handleScroll = () => {
        if (scrollContainer.value) {
            showBackToTop.value = scrollContainer.value.scrollTop > 100;
        }
    };

    const scrollToTop = () => {
        if (scrollContainer.value) {
            scrollContainer.value.scrollTo({
                top: 0,
                behavior: "smooth",
            });
        }
    };

    onMounted(() => {
        if (scrollContainer.value) {
            scrollContainer.value.addEventListener("scroll", handleScroll);
        }
    });

    onUnmounted(() => {
        if (scrollContainer.value) {
            scrollContainer.value.removeEventListener("scroll", handleScroll);
        }
    });
</script>

<template>
    <div class="flex flex-col h-full relative">
        <div class="shrink-0">
            <slot name="header"></slot>
        </div>
        <div
            ref="scrollContainer"
            class="grow overflow-y-auto">
            <slot></slot>
        </div>
        <button
            :class="{ 'opacity-100': showBackToTop }"
            class="btn btn-circle btn-primary hover:btn-secondary fixed bottom-4 right-4 z-50 opacity-0 transition-all ease-in-out duration-150"
            @click="scrollToTop">
            <i class="icon-[mdi--arrow-up] size-8" />
        </button>
    </div>
</template>
