<script setup lang="ts">
    
    import LogoLoaderLogo from "./LogoLoaderLogo.vue";
    const props = defineProps<{
        text: string;
        error?: boolean;
        errorText?: string;
    }>();
</script>

<template>
    <div
        class="card bg-base-300/50 w-96 shadow-sm flex flex-col justify-center items-center">
        <LogoLoaderLogo
            class="scale-50 -m-16"
            error
            v-if="props.error" />
        <LogoLoaderLogo
            class="scale-50 -m-16"
            v-else />
        <div class="card-body -mt-4">
            <h2 class="card-title">
                {{ props.error ? props.errorText : props.text }}
            </h2>
        </div>
    </div>
</template>
