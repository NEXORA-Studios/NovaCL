import { invoke } from "@tauri-apps/api/core";

interface IDownloadProgress {
    total: number;
    downloaded: number;
    speed: number;
    status: "pending" | "downloading" | "paused" | "completed" | "failed";
    error?: string;
}

interface IDownloadTask {
    id: string;
    url: string;
    filename: string;
    save_path: string;
    progress: IDownloadProgress;
}

export async function startDownload(
    url: string,
    savePath: string,
    filename?: string,
    segments?: number
): Promise<string> {
    return await invoke("start_download", {
        url,
        save_path: savePath,
        filename,
        segments,
    });
}

export async function pauseDownload(taskId: string): Promise<void> {
    return await invoke("pause_download", { task_id: taskId });
}

export async function resumeDownload(taskId: string): Promise<void> {
    return await invoke("resume_download", { task_id: taskId });
}

export async function cancelDownload(taskId: string): Promise<void> {
    return await invoke("cancel_download", { task_id: taskId });
}

export async function getDownloadProgress(
    taskId: string
): Promise<IDownloadProgress> {
    return await invoke("get_download_progress", { task_id: taskId });
}

export async function getAllDownloads(): Promise<IDownloadTask[]> {
    const tasks = await invoke<Array<[string, IDownloadProgress]>>(
        "get_all_downloads"
    );
    // TODO: 需要从其他接口获取任务详情
    return tasks.map(([id, progress]) => ({
        id,
        progress,
        url: "",
        filename: "",
        save_path: "",
    }));
}
