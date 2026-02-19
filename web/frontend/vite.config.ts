import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

const BACKEND_HOST = process.env.BACKEND_HOST || 'localhost:3000';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	server: {
		port: 5173,
		proxy: {
			'/ws': { target: `ws://${BACKEND_HOST}`, ws: true }
		}
	}
});
