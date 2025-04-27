<script setup lang="ts">
    import { ref } from "vue";
    import { ModEventBus } from "@/modules";
    // import { SettingStore } from "../mods/Store";

    interface IThemeOption {
        text: string;
        value: string;
    }
    const ThemeOptions: IThemeOption[] = [
        // 自动
        { text: "自动（跟随系统）", value: "__auto__" },
        // Nova 专属颜色
        { text: "Nova 浅色", value: "novalight" },
        { text: "Nova 深色", value: "novadark" },
    ];

    const theme = ref<string>();
    ModEventBus.on("theme:change", (e: string) => {
        theme.value = e;
    });
</script>

<template>
    <section
        data-region-for="theme-controller"
        class="hidden">
        <input
            v-for="option in ThemeOptions"
            type="checkbox"
            :name="option.text"
            :value="option.value"
            class="toggle theme-controller"
            :checked="theme === option.value" />
    </section>
</template>
