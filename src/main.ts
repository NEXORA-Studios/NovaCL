import { createApp } from "vue";
import App from "./App.vue";
import { ModRouter } from "@/modules";

createApp(App).use(ModRouter).mount("body");

// #region 挂载后操作
// 加载样式
import "@/assets/main.css";
import "@/assets/tailwind.css";
// 检查配置项
// #endregion
