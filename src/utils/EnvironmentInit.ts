export function initByEnvironment() {
    const MODE = import.meta.env.MODE;
    function keyWrapper(key: string) {
        return key.toLowerCase();
    }
    if (MODE !== "development") {
        // 禁用右键菜单
        document.addEventListener("contextmenu", (e) => {
            e.preventDefault();
        });
        // 禁用查看源代码，Ctrl+U
        document.addEventListener("keydown", (e) => {
            if (e.ctrlKey && keyWrapper(e.key) === "u") {
                e.preventDefault();
            }
        });
        // 禁用开发者工具，包括 Ctrl+Shift+I | F12
        document.addEventListener("keydown", (e) => {
            if (
                (e.ctrlKey && e.shiftKey && keyWrapper(e.key) === "i") ||
                keyWrapper(e.key) === "f12"
            ) {
                e.preventDefault();
            }
        });
        // 禁用打印，包括 Ctrl+P | Ctrl+Shift+P
        document.addEventListener("keydown", (e) => {
            if (
                (e.ctrlKey && keyWrapper(e.key) === "p") ||
                (e.ctrlKey && e.shiftKey && keyWrapper(e.key) === "p")
            ) {
                e.preventDefault();
            }
        });
        // 禁用 Web 原生搜索，包括 Ctrl+F | Ctrl+G
        document.addEventListener("keydown", (e) => {
            if (
                (e.ctrlKey && keyWrapper(e.key) === "f") ||
                (e.ctrlKey && keyWrapper(e.key) === "g")
            ) {
                e.preventDefault();
            }
        });
        // 禁用下载面板，包括 Ctrl+J
        document.addEventListener("keydown", (e) => {
            if (e.ctrlKey && keyWrapper(e.key) === "j") {
                e.preventDefault();
            }
        });
        // 禁用刷新，包括 Ctrl+R | F5 | Ctrl+Shift+R | Ctrl+F5
        document.addEventListener("keydown", (e) => {
            if (
                (e.ctrlKey && keyWrapper(e.key) === "r") ||
                keyWrapper(e.key) === "f5" ||
                (e.ctrlKey && e.shiftKey && keyWrapper(e.key) === "r") ||
                (e.ctrlKey && keyWrapper(e.key) === "f5")
            ) {
                e.preventDefault();
            }
        });
    }
}
