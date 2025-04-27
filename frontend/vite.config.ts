import react from '@vitejs/plugin-react';
import path from 'path';
import {defineConfig, loadEnv} from 'vite';

// https://vite.dev/config/
export default defineConfig(({mode}) => {
    const env = loadEnv(mode, process.cwd());

    return {
        resolve: {
            alias: {
                '@': path.resolve(__dirname, 'src'),
            },
        },
        server: {
            proxy: {
                '/api': {
                    target: env.VITE_API_URL,
                    changeOrigin: true,
                    rewrite: (path) => path.replace(/^\/api/, ''),
                },
            },
        },
        plugins: [react()],
        preview: {
            // Listen on all interfaces so the preview server is reachable from Docker or remote hosts
            // host: true, // or '0.0.0.0'
            // Explicitly allow your public hostname to avoid "Blocked request" errors
            allowedHosts: ['wap.zlapik.org',]
            // Optional: reuse port from env or fall back to Vite’s default
            // port: env.VITE_PREVIEW_PORT ? Number(env.VITE_PREVIEW_PORT) : 4173,
        },
    };
});
