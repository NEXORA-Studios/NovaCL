import { invoke } from "@tauri-apps/api/core";

export async function httpGet(
    url: string,
    userAgent?: string
): Promise<string> {
    return await invoke("http_get", { url, user_agent: userAgent });
}

export async function httpPostJson<T = any>(
    url: string,
    data: T,
    userAgent?: string
): Promise<string> {
    return await invoke("http_post_json", { url, data, user_agent: userAgent });
}
