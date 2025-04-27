<script setup lang="ts">
    import { ref } from "vue";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { ModRouter } from "@/modules";
    import { NavButton } from "..";

    interface IMenuItem {
        title: string;
        icon: MenuIcons;
        path: string;
        extraClass?: string;
    }
    enum MenuIcons {
        Home = "meteor-icons--home",
        Download = "meteor-icons--download",
        Settings = "meteor-icons--grid",
        More = "meteor-icons--ellipsis",
    }
    enum MenuPaths {
        Home = "/",
        Download = "/download",
        Settings = "/settings",
        More = "/more",
    }
    const btnHome = {
        title: "",
        icon: MenuIcons.Home,
        path: MenuPaths.Home,
        extraClass: "scale-80",
    };
    const btnDownload = {
        title: "· 下载",
        icon: MenuIcons.Download,
        path: MenuPaths.Download,
        extraClass: "scale-80",
    };
    const btnSettings = {
        title: "· 设置",
        icon: MenuIcons.Settings,
        path: MenuPaths.Settings,
        extraClass: "scale-75",
    };
    const btnMore = {
        title: "· 更多",
        icon: MenuIcons.More,
        path: MenuPaths.More,
        extraClass: "scale-75",
    };

    const close = async () => {
        await getCurrentWindow().close();
    };

    const btns = [btnHome, btnDownload, btnSettings, btnMore];
    const menuItems = ref<IMenuItem[]>([]);
    const title = ref("");

    function loadNavContents(current_path: string) {
        const _l: IMenuItem[] = [];
        btns.forEach((btn) => {
            if (btn.path !== current_path) {
                _l.push(btn);
            } else {
                title.value = btn.title;
            }
        });
        menuItems.value = _l;
    }

    loadNavContents("/");
    ModRouter.afterEach((to, _f) => loadNavContents(to.path));
</script>

<template>
    <section
        class="fixed top-0 w-full h-12 flex items-center gap-2 px-4 text-base-content z-50 hover:bg-primary/50 transition-all duration-250 ease-in-out backdrop-blur-sm"
        data-tauri-drag-region>
        <!-- 隐藏：加载项 -->
        <i class="hidden icon-[meteor-icons--home]" />
        <i class="hidden icon-[meteor-icons--download]" />
        <i class="hidden icon-[meteor-icons--grid]" />
        <i class="hidden icon-[meteor-icons--ellipsis]" />
        <!-- 左侧 -->
        <img
            src="/Logo.png"
            class="-translate-y-[1px]"
            width="22"
            height="22" />
        <h1 class="text-lg mr-auto">NovaCL {{ title }}</h1>
        <!-- 右侧：新闻 -->
        <div
            class="tooltip tooltip-left"
            data-tip="最近都发生了什么？查看新闻">
            <NavButton @click="openUrl(`https://afdian.com/@NEXORA-Studios`)">
                <i
                    class="icon-[meteor-icons--newspaper] scale-75 size-5.5" />
            </NavButton>
        </div>
        <!-- 右侧：动态页面切换菜单 -->
        <NavButton
            v-for="item in menuItems"
            :key="item.path"
            @click="$router.push(item.path)">
            <i
                :class="[`icon-[${item.icon}]`, item.extraClass]"
                class="w-[22px] h-[22px]" />
        </NavButton>
        <!-- 右侧：关闭按钮 -->
        <NavButton @click="close">
            <i class="icon-[meteor-icons--xmark] w-[22px] h-[22px]" />
        </NavButton>
    </section>
</template>
