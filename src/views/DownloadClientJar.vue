<script setup lang="ts">
    import { ref, onMounted, Ref, watch } from "vue";
    import { IClientVersionItem, IClientVersionList } from "@/types/Client";
    import { getMinecraftClientVersions } from "@/modules";
    import { Card } from "@/components";
    import LogoLoader from "@/components/LogoLoader.vue";

    const props = defineProps<{
        filter: Ref<string | undefined>;
    }>();

    const isLoaded = ref(false);
    const clientVersionList = ref<IClientVersionList>();
    const clientVersionListFiltered = ref<Array<IClientVersionItem>>([]);
    const NameMapper = {
        release: ["正式版", "Grass"],
        snapshot: ["快照版", "CommandBlock"],
        old_beta: ["远古版", "CobbleStone"],
        old_alpha: ["远古版", "CobbleStone"],
        april_fools: ["愚人节版", "GoldBlock"],
    };

    function filterClientVersionList() {
        const allVersion = [
            ...clientVersionList.value!.releases,
            ...clientVersionList.value!.snapshots,
            ...clientVersionList.value!.old_beta,
            ...clientVersionList.value!.old_alpha,
            ...clientVersionList.value!.april_fools,
        ];
        if (props.filter.value?.length === 0) {
            clientVersionListFiltered.value = [];
            return;
        }
        clientVersionListFiltered.value = [
            ...allVersion.filter((item) => {
                return item.id.includes(props.filter.value!);
            }),
        ];
    }

    onMounted(async () => {
        clientVersionList.value = await getMinecraftClientVersions();
        isLoaded.value = true;
    });

    watch(
        () => props.filter.value,
        () => {
            filterClientVersionList();
        }
    );
</script>

<template>
    <section
        class="flex w-full h-full justify-center items-center"
        v-if="!isLoaded">
        <LogoLoader text="正在加载版本列表..." />
    </section>
    <section
        class="flex flex-col gap-2"
        v-else-if="clientVersionListFiltered?.length === 0">
        <Card title="最新版本">
            <ul class="list -mx-2 -my-3">
                <li class="list-row">
                    <div>
                        <img
                            src="/blocks/Grass.png"
                            class="size-10" />
                    </div>
                    <div>
                        <div>{{ clientVersionList?.releases[0].id }}</div>
                        <div class="text-xs opacity-60">
                            最新正式版，{{
                                clientVersionList?.releases[0].releaseTime.toLocaleString()
                            }}
                        </div>
                    </div>
                    <button class="btn btn-square btn-success w-fit px-3">
                        <i class="icon-[mdi--download] size-6 -ml-1" />
                        自动下载
                    </button>
                </li>
                <li class="list-row">
                    <div>
                        <img
                            src="/blocks/CommandBlock.png"
                            class="size-10" />
                    </div>
                    <div>
                        <div>{{ clientVersionList?.snapshots[0].id }}</div>
                        <div class="text-xs opacity-60">
                            最新快照版，{{
                                clientVersionList?.snapshots[0].releaseTime.toLocaleString()
                            }}
                        </div>
                    </div>
                    <button class="btn btn-square btn-success w-fit px-3">
                        <i class="icon-[mdi--download] size-6 -ml-1" />
                        自动下载
                    </button>
                </li>
            </ul>
        </Card>
        <!-- 正式版 -->
        <Card
            title="正式版"
            can-swap
            is-swapped>
            <ul class="list -mx-2 -my-4">
                <li
                    class="list-row"
                    v-for="release of clientVersionList?.releases">
                    <div>
                        <img
                            src="/blocks/Grass.png"
                            class="size-10" />
                    </div>
                    <div>
                        <div>{{ release.id }}</div>
                        <div class="text-xs opacity-60">
                            {{ release.releaseTime.toLocaleString() }}
                        </div>
                    </div>
                    <button class="btn btn-square btn-success w-fit px-3">
                        <i class="icon-[mdi--download] size-6 -ml-1" />
                        自动下载
                    </button>
                </li>
            </ul>
        </Card>
        <!-- 快照版 -->
        <Card
            title="快照版"
            can-swap
            is-swapped>
            <ul class="list -mx-2 -my-4">
                <li
                    class="list-row"
                    v-for="snapshot of clientVersionList?.snapshots">
                    <div>
                        <img
                            src="/blocks/CommandBlock.png"
                            class="size-10" />
                    </div>
                    <div>
                        <div>{{ snapshot.id }}</div>
                        <div class="text-xs opacity-60">
                            {{ snapshot.releaseTime.toLocaleString() }}
                        </div>
                    </div>
                    <button class="btn btn-square btn-success w-fit px-3">
                        <i class="icon-[mdi--download] size-6 -ml-1" />
                        自动下载
                    </button>
                </li>
            </ul>
        </Card>
        <!-- 远古版 -->
        <Card
            title="远古版"
            can-swap
            is-swapped>
            <ul class="list -mx-2 -my-4">
                <li
                    class="list-row"
                    v-for="beta of clientVersionList?.old_beta">
                    <div>
                        <img
                            src="/blocks/CobbleStone.png"
                            class="size-10" />
                    </div>
                    <div>
                        <div>{{ beta.id }}</div>
                        <div class="text-xs opacity-60">
                            {{ beta.releaseTime.toLocaleString() }}
                        </div>
                    </div>
                    <button class="btn btn-square btn-success w-fit px-3">
                        <i class="icon-[mdi--download] size-6 -ml-1" />
                        自动下载
                    </button>
                </li>
                <li
                    class="list-row"
                    v-for="alpha of clientVersionList?.old_alpha">
                    <div>
                        <img
                            src="/blocks/CobbleStone.png"
                            class="size-10" />
                    </div>
                    <div>
                        <div>{{ alpha.id }}</div>
                        <div class="text-xs opacity-60">
                            {{ alpha.releaseTime.toLocaleString() }}
                        </div>
                    </div>
                    <button class="btn btn-square btn-success w-fit px-3">
                        <i class="icon-[mdi--download] size-6 -ml-1" />
                        自动下载
                    </button>
                </li>
            </ul>
        </Card>
        <!-- 愚人节版 -->
        <Card
            title="愚人节版"
            can-swap
            is-swapped>
            <ul class="list -mx-2 -my-4">
                <li
                    class="list-row"
                    v-for="april_fool of clientVersionList?.april_fools">
                    <div>
                        <img
                            src="/blocks/GoldBlock.png"
                            class="size-10" />
                    </div>
                    <div>
                        <div>{{ april_fool.id }}</div>
                        <div class="text-xs opacity-60">
                            {{ april_fool.releaseTime.toLocaleString() }}
                        </div>
                    </div>
                    <button class="btn btn-square btn-success w-fit px-3">
                        <i class="icon-[mdi--download] size-6 -ml-1" />
                        自动下载
                    </button>
                </li>
            </ul>
        </Card>
    </section>
    <section
        class="flex flex-col gap-2"
        v-else>
        <Card title="搜索结果">
            <ul class="list -mx-2 -my-4">
                <li
                    class="list-row"
                    v-for="version of clientVersionListFiltered">
                    <div>
                        <img
                            :src="`/blocks/${NameMapper[version.type][1]}.png`"
                            class="size-10" />
                    </div>
                    <div>
                        <div>{{ version.id }}</div>
                        <div class="text-xs opacity-60">
                            {{ NameMapper[version.type][0] }}，
                            {{ version.releaseTime.toLocaleString() }}
                        </div>
                    </div>
                    <button class="btn btn-square btn-success w-fit px-3">
                        <i class="icon-[mdi--download] size-6 -ml-1" />
                        自动下载
                    </button>
                </li>
            </ul>
        </Card>
    </section>
</template>
