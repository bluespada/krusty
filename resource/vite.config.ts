import type { UserConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import tailwindcss from '@tailwindcss/vite';

export default {
    plugins: [
        vue(),
        tailwindcss(),
    ]
} satisfies UserConfig;