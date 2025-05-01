<script setup lang="ts">
    import { IViewItem } from "@/types/ViewSwitch";

    defineProps<{
        options: Array<IViewItem>;
    }>();

    const model = defineModel<IViewItem>();

    function afterUpdate() {
        // @ts-ignore
        document.activeElement.blur();
    }
</script>

<template>
    <div class="dropdown">
        <div
            tabindex="0"
            role="button"
            class="btn w-full">
            {{ model?.label }}
        </div>
        <ul
            tabindex="0"
            class="dropdown-content menu bg-base-200 rounded-box z-1 p-2 shadow-sm w-full">
            <li
                v-for="option in $props.options"
                :key="option.id"
                @click="
                    $emit('update:modelValue', option);
                    afterUpdate();
                ">
                <a>
                    {{ option.label }}
                </a>
            </li>
        </ul>
    </div>
</template>
