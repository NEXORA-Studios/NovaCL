import { httpGet } from "@/api/Network";
import { WebRequestConstant } from "@/utils";
import {
    IClientVersionItemRaw,
    IClientVersionItem,
    IClientVersionList,
} from "@/types/Client";

const isAprilFirst = (date: Date) => {
    return date.getMonth() === 3 && date.getDate() === 1;
};

export async function getMinecraftClientVersions(): Promise<IClientVersionList> {
    const res = await httpGet(
        "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
        WebRequestConstant.DEFAULT_REQUEST_USERAGENT
    );
    const clientVersionRawAll = JSON.parse(res);
    const clientVersionsRaw = clientVersionRawAll["versions"];
    const clientVersions: IClientVersionList = {
        releases: [],
        snapshots: [],
        old_beta: [],
        old_alpha: [],
        april_fools: [],
    };
    for (let i = 0; i < clientVersionsRaw.length; i++) {
        const clientVersionRaw: IClientVersionItemRaw = clientVersionsRaw[i];
        const clientVersion: IClientVersionItem = {
            id: clientVersionRaw["id"],
            type: clientVersionRaw["type"],
            url: clientVersionRaw["url"],
            time: new Date(clientVersionRaw["time"]),
            releaseTime: new Date(clientVersionRaw["releaseTime"]),
            sha1: clientVersionRaw["sha1"],
            complianceLevel: clientVersionRaw["complianceLevel"],
        };
        /**
         * 逻辑判断
         * 1. 时间为 4 月 1 日则为愚人节版本
         * 2. 根据类型分类
         */
        if (isAprilFirst(clientVersion.releaseTime)) {
            switch (clientVersion.id) {
                case "21w13a":
                    clientVersions["snapshots"].push(clientVersion);
                    break;
                default:
                    clientVersions["april_fools"].push(clientVersion);
                    break;
            }
        }
        switch (clientVersion.type) {
            case "release":
                clientVersions["releases"].push(clientVersion);
                break;
            case "snapshot":
                clientVersions["snapshots"].push(clientVersion);
                break;
            case "old_alpha":
                clientVersions["old_alpha"].push(clientVersion);
                break;
            case "old_beta":
                clientVersions["old_beta"].push(clientVersion);
                break;
        }
    }
    return clientVersions;
}
