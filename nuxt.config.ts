// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	compatibilityDate: "2026-10-24",
	devtools: { enabled: true },
	ssr: false,
	modules: ["@nuxt/ui", "@nuxtjs/mdc", "@vueuse/nuxt"],
	future: {
		compatibilityVersion: 4, // Ensures Nuxt 4 folder structures
	},
	css: ["~/assets/css/main.css"],
	mdc: {
		highlight: {
			theme: "github-dark",
			langs: [
				"js",
				"ts",
				"html",
				"css",
				"md",
				"json",
				"yaml",
				"vue",
				"bash",
				"sql",
				"go",
			],
		},
	},
	// 1. Auto-imports types ONLY for Frontend pages/components
	imports: {
		dirs: [
			// Scans this directory for any exported interfaces/types
			"server/types",
		],
	},
	// 2. Auto-imports types ONLY for Server Layer (api files & server utils)
	nitro: {
		imports: {
			dirs: ["../server/types"], // Nitro scans relative to the server/ directory
		},
		prerender: {
			routes: ["/"],
		},
	},
});
