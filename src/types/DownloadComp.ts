interface ICompSearchMenuConfig {
    type: "mod" | "modpack" | "resourcepack" | "shaderpack" | "datapack";
    content: Array<ICompSearchMenuConfigItem>;
}

interface ICompSearchMenuConfigItem {
    title: string;
    children: Array<ICompSearchMenuConfigItemChild>;
}

interface ICompSearchMenuConfigItemChild {
    svg?: string;
    text: string;
    mrkey: string;
    excludable?: boolean;
}
