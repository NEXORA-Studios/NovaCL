<script setup lang="tsx">
    const model = defineModel<string>();
</script>

<template>
    <label class="input">
        <i class="icon-[mdi--magnify] opacity-75" />
        <input
            type="search"
            class="focus:outline-none"
            placeholder="搜索..."
            v-model="model" />
    </label>
</template>
