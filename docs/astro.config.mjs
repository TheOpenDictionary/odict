// @ts-check
import { defineConfig, fontProviders } from "astro/config";
import starlight from "@astrojs/starlight";
import tailwindcss from "@tailwindcss/vite";
import mermaid from "astro-mermaid";

// https://astro.build/config
export default defineConfig({
  site: "https://www.odict.org",
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
    mermaid({
      autoTheme: true,
      enableLog: false,
    }),
    starlight({
      components: {
        Head: "./src/components/Head.astro",
        Header: "./src/components/Header.astro",
        Hero: "./src/components/Hero.astro",
        Footer: "./src/components/Footer.astro",
      },
      title: "ODict",
      logo: {
        light: "./src/assets/odict-light.svg",
        dark: "./src/assets/odict-dark.svg",
        replacesTitle: true,
      },
      customCss: ["./src/styles/global.css"],
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/TheOpenDictionary/odict",
        },
      ],
      sidebar: [
        {
          label: "Getting Started",
          items: [
            { label: "Introduction", slug: "getting-started/introduction" },
            { label: "Feature Comparison", slug: "getting-started/comparison" },
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
            { label: "Aliases and Loading", slug: "guides/aliases" },
            { label: "Full-Text Search", slug: "guides/search" },
            { label: "Quick Lookups", slug: "guides/lookup" },
            { label: "Running a Dictionary Server", slug: "guides/serving" },
            { label: "Tokenizing Text", slug: "guides/tokenize" },
            { label: "Using Remote Dictionaries", slug: "guides/downloading" },
            { label: "Writing Dictionaries", slug: "guides/compiling" },
          ],
        },
        {
          label: "CLI",
          items: [
            { label: "Overview", slug: "cli/overview" },
            { label: "Command Reference", slug: "cli/reference" },
          ],
        },
        {
          label: "API Reference",
          items: [
            { label: "Rust", link: "https://docs.rs/odict" },
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
