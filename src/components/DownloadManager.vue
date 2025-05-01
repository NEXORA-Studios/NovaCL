<script setup lang="ts">
    import { ref, onMounted, onUnmounted } from "vue";
    import { invoke } from "@tauri-apps/api/core";

    interface DownloadProgress {
        downloaded: number;
        total: number;
        speed: number;
        eta: number;
        status:
            | "Pending"
            | "Downloading"
            | "Paused"
            | "Completed"
            | "Failed"
            | "Cancelled";
    }

    interface DownloadTask {
        id: string;
        progress: DownloadProgress;
    }

    // 下载任务列表
    const downloads = ref<DownloadTask[]>([]);
    // 下载URL输入
    const downloadUrl = ref("");
    // 保存路径
    const savePath = ref("");
    // 文件名（可选）
    const filename = ref("");
    // 是否正在加载
    const isLoading = ref(false);
    // 错误信息
    const errorMessage = ref("");

    // 定时刷新下载进度
    let progressInterval: number | null = null;

    // 组件挂载时开始定时刷新下载进度
    onMounted(() => {
        progressInterval = window.setInterval(refreshDownloads, 1000);
    });

    // 组件卸载时清除定时器
    onUnmounted(() => {
        if (progressInterval !== null) {
            clearInterval(progressInterval);
        }
    });

    // 刷新所有下载任务的进度
    async function refreshDownloads() {
        try {
            const tasks = await invoke<[string, DownloadProgress][]>(
                "get_all_downloads"
            );
            downloads.value = tasks.map(([id, progress]) => ({
                id,
                progress,
            }));
        } catch (error) {
            console.error("获取下载列表失败:", error);
        }
    }

    // 开始下载
    async function startDownload() {
        if (!downloadUrl.value || !savePath.value) {
            errorMessage.value = "请输入下载URL和保存路径";
            return;
        }

        try {
            isLoading.value = true;
            errorMessage.value = "";

            const taskId = await invoke<string>("start_download", {
                url: downloadUrl.value,
                savePath: savePath.value,
                filename: filename.value || null,
                segments: 4, // 使用4个分段
            });

            console.log("下载任务已创建:", taskId);
            await refreshDownloads();

            // 清空输入
            downloadUrl.value = "";
            filename.value = "";
        } catch (error) {
            errorMessage.value = `创建下载任务失败: ${error}`;
            console.error("创建下载任务失败:", error);
        } finally {
            isLoading.value = false;
        }
    }

    // 暂停下载
    async function pauseDownload(taskId: string) {
        try {
            await invoke("pause_download", { taskId });
            await refreshDownloads();
        } catch (error) {
            console.error("暂停下载失败:", error);
        }
    }

    // 恢复下载
    async function resumeDownload(taskId: string) {
        try {
            await invoke("resume_download", { taskId });
            await refreshDownloads();
        } catch (error) {
            console.error("恢复下载失败:", error);
        }
    }

    // 取消下载
    async function cancelDownload(taskId: string) {
        try {
            await invoke("cancel_download", { taskId });
            await refreshDownloads();
        } catch (error) {
            console.error("取消下载失败:", error);
        }
    }

    // 格式化文件大小
    function formatSize(bytes: number): string {
        if (bytes === 0) return "0 B";
        const k = 1024;
        const sizes = ["B", "KB", "MB", "GB", "TB"];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
    }

    // 格式化下载速度
    function formatSpeed(bytesPerSecond: number): string {
        return formatSize(bytesPerSecond) + "/s";
    }

    // 格式化剩余时间
    function formatEta(seconds: number): string {
        if (seconds === 0) return "计算中...";
        if (seconds < 60) return `${seconds}秒`;
        if (seconds < 3600)
            return `${Math.floor(seconds / 60)}分钟${seconds % 60}秒`;
        return `${Math.floor(seconds / 3600)}小时${Math.floor(
            (seconds % 3600) / 60
        )}分钟`;
    }
</script>

<template>
    <div class="download-manager">
        <h2>下载管理器</h2>

        <!-- 添加下载表单 -->
        <div class="download-form">
            <div class="form-group">
                <label for="download-url">下载URL</label>
                <input
                    id="download-url"
                    v-model="downloadUrl"
                    type="text"
                    placeholder="https://example.com/file.zip" />
            </div>

            <div class="form-group">
                <label for="save-path">保存路径</label>
                <input
                    id="save-path"
                    v-model="savePath"
                    type="text"
                    placeholder="C:\Downloads" />
            </div>

            <div class="form-group">
                <label for="filename">文件名 (可选)</label>
                <input
                    id="filename"
                    v-model="filename"
                    type="text"
                    placeholder="file.zip" />
            </div>

            <button
                @click="startDownload"
                :disabled="isLoading">
                {{ isLoading ? "处理中..." : "开始下载" }}
            </button>

            <div
                v-if="errorMessage"
                class="error-message">
                {{ errorMessage }}
            </div>
        </div>

        <!-- 下载列表 -->
        <div class="download-list">
            <h3>下载任务</h3>

            <div
                v-if="downloads.length === 0"
                class="empty-list">
                暂无下载任务
            </div>

            <div
                v-for="task in downloads"
                :key="task.id"
                class="download-item">
                <div class="download-info">
                    <div class="download-id">任务ID: {{ task.id }}</div>
                    <div class="download-status">
                        状态: {{ task.progress.status }}
                    </div>
                </div>

                <div class="download-progress">
                    <div class="progress-bar">
                        <div
                            class="progress-fill"
                            :style="{
                                width: `${
                                    (task.progress.downloaded /
                                        task.progress.total) *
                                    100
                                }%`,
                            }"></div>
                    </div>
                    <div class="progress-text">
                        {{ formatSize(task.progress.downloaded) }} /
                        {{ formatSize(task.progress.total) }} ({{
                            (
                                (task.progress.downloaded /
                                    task.progress.total) *
                                100
                            ).toFixed(1)
                        }}%)
                    </div>
                </div>

                <div class="download-details">
                    <div>速度: {{ formatSpeed(task.progress.speed) }}</div>
                    <div>剩余时间: {{ formatEta(task.progress.eta) }}</div>
                </div>

                <div class="download-actions">
                    <button
                        v-if="task.progress.status === 'Downloading'"
                        @click="pauseDownload(task.id)">
                        暂停
                    </button>
                    <button
                        v-if="task.progress.status === 'Paused'"
                        @click="resumeDownload(task.id)">
                        恢复
                    </button>
                    <button
                        v-if="
                            ['Downloading', 'Paused', 'Pending'].includes(
                                task.progress.status
                            )
                        "
                        @click="cancelDownload(task.id)">
                        取消
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
    .download-manager {
        padding: 20px;
        max-width: 800px;
        margin: 0 auto;
    }

    .download-form {
        background-color: rgba(255, 255, 255, 0.1);
        padding: 20px;
        border-radius: 8px;
        margin-bottom: 20px;
    }

    .form-group {
        margin-bottom: 15px;
    }

    label {
        display: block;
        margin-bottom: 5px;
        font-weight: 500;
    }

    input {
        width: 100%;
        padding: 8px 12px;
        border-radius: 4px;
        border: 1px solid rgba(255, 255, 255, 0.2);
        background-color: rgba(0, 0, 0, 0.2);
        color: white;
    }

    button {
        background-color: #4a6cf7;
        color: white;
        border: none;
        padding: 8px 16px;
        border-radius: 4px;
        cursor: pointer;
        font-weight: 500;
        transition: background-color 0.2s;
    }

    button:hover {
        background-color: #3a5ce5;
    }

    button:disabled {
        background-color: #6c7a9c;
        cursor: not-allowed;
    }

    .error-message {
        color: #ff6b6b;
        margin-top: 10px;
    }

    .download-list {
        background-color: rgba(255, 255, 255, 0.1);
        padding: 20px;
        border-radius: 8px;
    }

    .empty-list {
        text-align: center;
        padding: 20px;
        color: rgba(255, 255, 255, 0.6);
    }

    .download-item {
        background-color: rgba(0, 0, 0, 0.2);
        border-radius: 6px;
        padding: 15px;
        margin-bottom: 15px;
    }

    .download-info {
        display: flex;
        justify-content: space-between;
        margin-bottom: 10px;
    }

    .download-progress {
        margin-bottom: 10px;
    }

    .progress-bar {
        height: 8px;
        background-color: rgba(255, 255, 255, 0.1);
        border-radius: 4px;
        overflow: hidden;
        margin-bottom: 5px;
    }

    .progress-fill {
        height: 100%;
        background-color: #4a6cf7;
        border-radius: 4px;
    }

    .download-details {
        display: flex;
        justify-content: space-between;
        font-size: 0.9em;
        color: rgba(255, 255, 255, 0.8);
        margin-bottom: 10px;
    }

    .download-actions {
        display: flex;
        gap: 10px;
    }

    .download-actions button {
        font-size: 0.9em;
        padding: 6px 12px;
    }
</style>
