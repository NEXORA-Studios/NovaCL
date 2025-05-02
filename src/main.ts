import { createApp } from "vue";
import App from "./App.vue";
import { ModRouter } from "@/router";
import { createPinia } from "pinia";

createApp(App).use(ModRouter).use(createPinia()).mount("body");

// 挂载后操作
// 加载样式
import "@/styles/main.css";
import "@/styles/tailwind.css";
// 根据环境禁用部分内容
import { initByEnvironment } from "@/utils";
initByEnvironment();
