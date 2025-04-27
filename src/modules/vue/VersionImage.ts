export function getImageNameFromVersionCode(versionCode: string) {
    // 使用正则表达式提取版本号中的a和b部分
    const match = versionCode.match(/1\.(\d{1,2})\.(\d)/);

    // 如果格式不匹配，返回默认图片
    if (!match) {
        return "legacy.png";
    }

    // 提取a和b的值并转换为数字
    const a = parseInt(match[1], 10);
    const b = parseInt(match[2], 10);

    // 根据条件返回对应的图片名称
    if (a === 21) {
        // 当a=21时，根据b的值返回不同图片
        return b >= 4 ? "21b.png" : "21a.png";
    } else if (a >= 3 && a < 21) {
        // 当3≤a<21时，返回a.png
        return `${a}.png`;
    } else {
        // 其他情况返回legacy.png
        return "legacy.png";
    }
}
