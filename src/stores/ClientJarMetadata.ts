import { defineStore } from "pinia";
import { ref } from "vue";
import type { IClientVersionList } from "@/types/Client";
import { getMinecraftClientVersions } from "@/modules";

export const useClientJarMetadataStore = defineStore(
    "clientJarMetadata",
    () => {
        // 存储客户端版本列表
        const clientVersionList = ref<IClientVersionList | null>(null);

        // 存储错误信息
        const error = ref<string | null>(null);

        /**
         * 设置客户端版本列表
         * @param list 要存储的版本列表数据
         */
        async function getClientVersionList() {
            if (!clientVersionList.value)
                clientVersionList.value = await getMinecraftClientVersions();
            return clientVersionList.value;
        }

        /**
         * 设置错误信息
         * @param err 错误信息
         */
        function setError(err: string) {
            error.value = err;
            clientVersionList.value = null;
        }

        return {
            clientVersionList,
            error,
            getClientVersionList,
            setError,
        };
    }
);
