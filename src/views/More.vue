<script setup lang="ts">
    import { BlurBackground } from "@/components";
    import { reactive } from "vue";
    import { getVersion } from "@tauri-apps/api/app";
    import { openUrl } from "@tauri-apps/plugin-opener";

    const $meta = import.meta.env;

    function channelWrapper(channel: string) {
        const _channel = {
            horizon: "🌅 Horizon 地平线更新渠道",
            echo: "🌃 Echo 回音更新渠道",
            rift: "🌌 裂隙 Rift 更新渠道",
        };
        return channel in _channel
            ? _channel[channel as keyof typeof _channel]
            : "⚠️ 未知";
    }

    const METADATA = reactive<{
        version: string;
        channel: string;
        hash: string;
        time: string;
    }>({
        version: await getVersion(),
        channel: channelWrapper($meta.VITE_CHANNEL),
        hash: $meta.VITE_GIT_HASH ?? "native",
        time: $meta.VITE_BUILD_TIME ?? "本地构建中",
    });
</script>
<template>
    <BlurBackground>
        <div
            id="Settings"
            class="mt-12 pt-4 pb-8 px-8 flex flex-col"
            style="
                height: calc(100% - 48px);
                max-height: calc(100% - 48px);
                overflow: hidden;
            ">
            <div class="divider">版本信息</div>
            <div class="stats bg-base-100 border border-base-300">
                <div class="stat">
                    <div class="stat-title">版本</div>
                    <div class="stat-value">{{ METADATA.version }}</div>
                    <div class="stat-actions">
                        <button
                            class="btn btn-xs btn-success"
                            disabled>
                            检查更新
                        </button>
                    </div>
                </div>
                <div class="stat">
                    <div class="stat-title">更新渠道</div>
                    <div class="stat-value">{{ METADATA.channel }}</div>
                    <div class="stat-actions">
                        <button
                            class="btn btn-xs"
                            disabled>
                            切换更新渠道
                        </button>
                    </div>
                </div>
                <div class="stat">
                    <div class="stat-title">构建信息</div>
                    <div class="stat-value">{{ METADATA.time }}</div>
                    <div class="stat-desc">
                        <span>来自提交：{{ METADATA.hash }}</span>
                    </div>
                </div>
            </div>
            <div class="divider">链接</div>
            <ul class="list bg-base-100 rounded-box shadow-md">
                <li class="list-row">
                    <div>
                        <i class="icon-[octicon--mark-github-16] size-9" />
                    </div>
                    <div>
                        <div>开源仓库</div>
                        <div class="text-xs uppercase font-semibold opacity-60">
                            查看项目源码
                        </div>
                    </div>
                    <button
                        class="btn btn-square btn-info"
                        @click="
                            openUrl('https://github.com/NEXORA-Studios/NovaCL')
                        ">
                        <i class="icon-[mdi--open-in-new] size-6" />
                    </button>
                </li>
                <li class="list-row">
                    <div>
                        <i
                            class="icon-[octicon--issue-tracks-16] text-accent size-9" />
                    </div>
                    <div>
                        <div>工单中心</div>
                        <div class="text-xs uppercase font-semibold opacity-60">
                            向我们提供建议或反馈问题
                        </div>
                    </div>
                    <button
                        class="btn btn-square btn-info"
                        @click="
                            openUrl(
                                'https://github.com/NEXORA-Studios/NovaCL/issues'
                            )
                        ">
                        <i class="icon-[mdi--open-in-new] size-6" />
                    </button>
                </li>
                <li class="list-row">
                    <div>
                        <i
                            class="icon-[simple-icons--afdian] text-secondary size-9" />
                    </div>
                    <div>
                        <div>爱发电</div>
                        <div class="text-xs uppercase font-semibold opacity-60">
                            <del class="opacity-50">给我们爆金币</del>
                            支持我们的创作
                        </div>
                    </div>
                    <button
                        class="btn btn-square btn-info"
                        @click="
                            openUrl('https://github.com/NEXORA-Studios/NovaCL')
                        ">
                        <i class="icon-[mdi--open-in-new] size-6" />
                    </button>
                </li>
            </ul>
        </div>
    </BlurBackground>
</template>
