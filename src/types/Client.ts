interface IClientVersionItemBase {
    id: string;
    type: "release" | "snapshot" | "old_alpha" | "old_beta";
    url: string;
    sha1: string;
    complianceLevel: number;
}

export interface IClientVersionItemRaw extends IClientVersionItemBase {
    time: string;
    releaseTime: string;
}

export interface IClientVersionItem extends IClientVersionItemBase {
    time: Date;
    releaseTime: Date;
}

export interface IClientVersionList {
    releases: Array<IClientVersionItem>;
    snapshots: Array<IClientVersionItem>;
    old_beta: Array<IClientVersionItem>;
    old_alpha: Array<IClientVersionItem>;
    april_fools: Array<IClientVersionItem>;
}
