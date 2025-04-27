import { createRouter, createWebHistory } from "vue-router";

const routes = [
    {
        path: "/",
        name: "Home",
        component: () => import("@/views/Home.vue"),
    },
    {
        path: "/download",
        name: "Download",
        component: () => import("@/views/Download.vue"),
    },
    {
        path: "/settings",
        name: "Settings",
        component: () => import("@/views/Settings.vue"),
    },
];

const ModRouter = createRouter({
    history: createWebHistory(),
    routes,
});

ModRouter.beforeEach((_to, _from, next) => {
    (document.getElementById("main") as HTMLDivElement).style.opacity = "0";
    setTimeout(() => {
        next();
    }, 300); // 300ms delay before redirecting
});

ModRouter.afterEach(() => {
    (document.getElementById("main") as HTMLDivElement).style.opacity = "1";
});

export { ModRouter };
