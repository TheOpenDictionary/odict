// @ts-check
import { defineConfig, fontProviders } from "astro/config";
import starlight from "@astrojs/starlight";
import tailwindcss from "@tailwindcss/vite";

// https://astro.build/config
export default defineConfig({
  fonts: [
    {
      provider: fontProviders.google(),
      name: "Hedvig Letters Serif",
      cssVariable: "--ff-display",
    },
    {
      provider: fontProviders.google(),
      name: "Inter",
      weights: ["100 900"],
      options: {
        experimental: {
          variableAxis: {
            opsz: [["14", "32"]],
          },
        },
      },
      cssVariable: "--ff-sans",
    },
  ],
  integrations: [
    starlight({
      components: {
        Head: "./src/components/Head.astro",
      },
      title: "My Docs",
      logo: {
        light: "./src/assets/odict-light.svg",
        dark: "./src/assets/odict-dark.svg",
        replacesTitle: true,
      },
      customCss: [
        // Path to your Tailwind base styles:
        "./src/styles/global.css",
      ],
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/withastro/starlight",
        },
      ],
      sidebar: [
        {
          label: "Getting Started",
          items: [
            { label: "Introduction", slug: "getting-started/introduction" },
            { label: "Installation", slug: "getting-started/installation" },
            { label: "Quick Start", slug: "getting-started/quickstart" },
          ],
        },
        {
          label: "XML Schema",
          items: [
            { label: "Overview", slug: "schema/overview" },
            { label: "Reference", slug: "schema/reference" },
          ],
        },
        {
          label: "Guides",
          items: [
            { label: "Compiling Dictionaries", slug: "guides/compiling" },
            { label: "Looking Up Entries", slug: "guides/lookup" },
            { label: "Searching Dictionaries", slug: "guides/search" },
            { label: "Tokenizing Text", slug: "guides/tokenize" },
          ],
        },
        {
          label: "CLI",
          items: [{ label: "Command Reference", slug: "cli/reference" }],
        },
        {
          label: "API Reference",
          items: [
            { label: "Rust", slug: "api/rust" },
            { label: "Python", slug: "api/python" },
            { label: "JavaScript", slug: "api/javascript" },
          ],
        },
      ],
    }),
  ],

  vite: {
    plugins: [tailwindcss()],
  },
});
