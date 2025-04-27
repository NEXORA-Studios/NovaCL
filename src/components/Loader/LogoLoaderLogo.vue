<script setup lang="ts">
    import { ref, onMounted } from "vue";

    const props = defineProps<{ error?: boolean }>();
    const svg = ref<SVGSVGElement>();

    function sleep(ms: number) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }
    function randomFloat(min: number, max: number): number {
        return Math.random() * (max - min) + min;
    }
    function randomString(length: number): string {
        const chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let result = "";
        for (let i = 0; i < length; i++) {
            const index = Math.floor(Math.random() * chars.length);
            result += chars[index];
        }
        return result;
    }

    const randomId = "_" + randomString(8);

    function initSvg(): void {
        // 设置唯一 Class 以避免多次使用冲突
        svg.value?.querySelectorAll<SVGPathElement>("path").forEach((path) => path.classList.add(randomId));
        svg.value?.querySelectorAll<SVGCircleElement>("circle").forEach((circle) => circle.classList.add(randomId));
        // 初始化动画
        [".line-up", ".line-up-right", ".line-right", ".line-down-right", ".line-down", ".line-down-left", ".line-left", ".line-up-left"].forEach((selector: string) => {
            document.querySelectorAll<SVGPathElement>(`${selector}.${randomId}`).forEach((line: SVGPathElement) => {
                const lineFactor = props.error ? randomFloat(1.2, 1.8) : 2;
                line.style.strokeDasharray = String(line.getTotalLength() * 2);
                line.style.strokeDashoffset = String(line.getTotalLength() * lineFactor);
            });
        });
        [".dots", ".center"].forEach((selector: string) => {
            document.querySelectorAll<SVGCircleElement>(`${selector}.${randomId}`).forEach((dot) => {
                const dotFactor = props.error ? randomFloat(0, 1) : 0;
                dot.style.scale = String(dotFactor);
            });
        });
        props.error && svg.value ? (svg.value.style.transform = `rotate(${randomFloat(0, 360)}deg)`) : null;
    }

    function animateLine(selectors: string[], offset: 1 | 2) {
        selectors.forEach((selector: string) => {
            document.querySelectorAll<SVGPathElement>(`${selector}.${randomId}`).forEach((line: SVGPathElement) => {
                line.style.strokeDashoffset = String(line.getTotalLength() * offset);
            });
        });
    }
    function animateDots(selectors: string | string[], show: boolean) {
        const _handler = (selector: string) => document.querySelectorAll<SVGCircleElement>(`${selector}.${randomId}`).forEach((dot) => (dot.style.scale = show ? "1" : "0"));
        if (typeof selectors === "string") {
            _handler(selectors);
        } else {
            selectors.forEach((selector: string) => _handler(selector));
        }
    }

    async function doAnimation() {
        // 正向动画开始
        animateDots(".center", true);
        await sleep(1000);
        animateLine([".line-left", ".line-right"], 1);
        await sleep(1000);
        animateLine([".line-up", ".line-down"], 1);
        await sleep(1000);
        animateLine([".line-up-left", ".line-down-left", ".line-up-right", ".line-down-right"], 1);
        await sleep(1000);
        animateDots(".dots", true);
        // 正向动画结束
        await sleep(1000);
        // 反向动画开始
        animateDots(".dots", false);
        await sleep(1000);
        animateLine([".line-up-left", ".line-down-left", ".line-up-right", ".line-down-right"], 2);
        await sleep(1000);
        animateLine([".line-up", ".line-down"], 2);
        await sleep(1000);
        animateLine([".line-left", ".line-right"], 2);
        await sleep(1000);
        animateDots(".center", false);
        setTimeout(doAnimation, 2000);
    }

    onMounted(() => {
        initSvg();
        if (!props.error) {
            svg.value?.classList.add("apply");
            setTimeout(doAnimation, 1000);
        }
    });
</script>

<template>
    <svg width="328.538330" height="329.540039" viewBox="0 0 328.538 329.54" fill="none" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" :class="$style.LogoSvg" ref="svg">
        <defs>
            <filter id="filter_7_745_dd" x="0.000000" y="0.000031" width="328.538330" height="329.540009" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                <feFlood flood-opacity="0" result="BackgroundImageFix" />
                <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0" result="hardAlpha" />
                <feOffset dx="0" dy="4" />
                <feGaussianBlur stdDeviation="1.33333" />
                <feComposite in2="hardAlpha" operator="out" k2="-1" k3="1" />
                <feColorMatrix type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.25 0" />
                <feBlend mode="normal" in2="BackgroundImageFix" result="effect_dropShadow_1" />
                <feBlend mode="normal" in="SourceGraphic" in2="effect_dropShadow_1" result="shape" />
            </filter>
            <linearGradient x1="4.000000" y1="0.000031" x2="324.538330" y2="321.540070" id="paint_linear_7_745_0" gradientUnits="userSpaceOnUse" v-if="$props.error">
                <stop offset="0.251810" stop-color="#8d2b2b" />
                <stop offset="0.892751" stop-color="#D34040" />
            </linearGradient>
            <linearGradient x1="4.000000" y1="0.000031" x2="324.538330" y2="321.540070" id="paint_linear_7_745_0" gradientUnits="userSpaceOnUse" v-else>
                <stop offset="0.251810" stop-color="#4069D3" />
                <stop offset="0.508173" stop-color="#6340D3" />
                <stop offset="0.702896" stop-color="#D39940" />
                <stop offset="0.892751" stop-color="#D34040" />
            </linearGradient>
        </defs>
        <mask id="mask7_744" mask-type="alpha" maskUnits="userSpaceOnUse" x="7.000000" y="8.000031" width="314.000000" height="305.000000">
            <!-- Up -->
            <path class="line-up" id="直线 1" d="M164 128L164 8" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-up" id="直线 2" d="M178 131L178 30" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-up" id="直线 3" d="M149 131L149 30" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-up" id="直线 4" d="M192 97L192 43" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-up" id="直线 5" d="M204 85L204 62" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-up" id="直线 6" d="M136 97L136 43" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-up" id="直线 7" d="M123 82L123 61" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <!-- Up Right -->
            <path class="line-up-right" id="直线 8" d="M190 115L250 55" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-up-right" id="直线 9" d="M190 138L275 53" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-up-right" id="直线 10" d="M210.74 137.75L269.25 79.24" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-up-right" id="直线 11" d="M245 120L280 85" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <!-- Right -->
            <path class="line-right" id="直线 12" d="M230 133L276 133" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-right" id="直线 13" d="M197 148L295 148" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-right" id="直线 14" d="M201 164L321 164" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-right" id="直线 15" d="M198 179L293 179" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-right" id="直线 16" d="M248 193L276 193" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <!-- Down Right -->
            <path class="line-down-right" id="直线 17" d="M229 190L279 240" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-down-right" id="直线 18" d="M213.74 190.24L269.25 245.75" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-down-right" id="直线 19" d="M193 188L276 270" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-down-right" id="直线 20" d="M188.49 207.49L251.5 270.5" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <!-- Down -->
            <path class="line-down" id="直线 21" d="M204 239L204 262" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-down" id="直线 22" d="M191 227L191 280" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-down" id="直线 23" d="M164 202L164 313" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-down" id="直线 24" d="M178 199L178 292" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-down" id="直线 25" d="M149 199L149 292" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-down" id="直线 26" d="M136 226L136 280" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-down" id="直线 27" d="M123 241L123 262" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <!-- Down Left -->
            <path class="line-down-left" id="直线 28" d="M139 205L77 271" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-down-left" id="直线 29" d="M135 189L52 271" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-down-left" id="直线 30" d="M115 190L62 243" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-down-left" id="直线 31" d="M98 190L50 238" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <!-- Left -->
            <path class="line-left" id="直线 32" d="M80 193L51 193" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-left" id="直线 33" d="M130 179L36 179" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-left" id="直线 34" d="M127 163L7 163" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-left" id="直线 35" d="M131 147L33 147" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-left" id="直线 36" d="M96 134L51 134" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <!-- Up Left -->
            <path class="line-up-left" id="直线 37" d="M137.01 115.01L75.98 53.98" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-up-left" id="直线 38" d="M135.5 136.5L51.49 52.49" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-up-left" id="直线 39" d="M115 136L58 79" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <path class="line-up-left" id="直线 40" d="M82 120L47 85" stroke="#000000" stroke-opacity="1.000000" stroke-width="6.000000" stroke-linecap="round" />
            <!-- Dots -->
            <circle class="dots" id="椭圆 1" cx="32.000000" cy="110.000031" r="5.000000" fill="#C4C4C4" />
            <circle class="dots" id="椭圆 2" cx="105.000000" cy="42.000031" r="5.000000" fill="#C4C4C4" />
            <circle class="dots" id="椭圆 3" cx="223.000000" cy="42.000031" r="5.000000" fill="#C4C4C4" />
            <circle class="dots" id="椭圆 4" cx="295.000000" cy="110.000031" r="5.000000" fill="#C4C4C4" />
            <circle class="dots" id="椭圆 5" cx="295.000000" cy="214.000031" r="5.000000" fill="#C4C4C4" />
            <circle class="dots" id="椭圆 6" cx="223.000000" cy="280.000031" r="5.000000" fill="#C4C4C4" />
            <circle class="dots" id="椭圆 7" cx="105.000000" cy="280.000031" r="5.000000" fill="#C4C4C4" />
            <circle class="dots" id="椭圆 8" cx="33.000000" cy="214.000031" r="5.000000" fill="#C4C4C4" />
            <!-- Center -->
            <circle class="center" id="center" cx="164.000000" cy="165.000031" r="20.000000" fill="#C4C4C4" />
        </mask>
        <g mask="url(#mask7_744)">
            <g filter="url(#filter_7_745_dd)">
                <rect id="矩形 1" x="4.000000" y="0.000031" width="320.538330" height="321.540009" fill="url(#paint_linear_7_745_0)" />
            </g>
        </g>
    </svg>
</template>

<style lang="scss" module>
    .LogoSvg mask {
        path,
        circle {
            transition: all 0.5s ease-in-out;
        }
    }
    .LogoSvg mask circle {
        transform-origin: 50% 50%;
    }
</style>
