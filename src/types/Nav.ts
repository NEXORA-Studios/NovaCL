interface IMenuItem {
    title: string;
    icon: MenuIcons;
    path: string;
    extraClass?: string;
}

enum MenuIcons {
    Home = "meteor-icons--home",
    Download = "meteor-icons--download",
    Settings = "meteor-icons--grid",
    More = "meteor-icons--ellipsis",
}

enum MenuPaths {
    Home = "/",
    Download = "/download",
    Settings = "/settings",
    More = "/more",
}

export { type IMenuItem, MenuIcons, MenuPaths };
