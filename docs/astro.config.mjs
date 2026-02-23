// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

// https://astro.build/config
export default defineConfig({
  integrations: [
    starlight({
      title: "ODict",
      description:
        "The lightning-fast open-source dictionary file format for human languages",
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/TheOpenDictionary/odict",
        },
      ],
      editLink: {
        baseUrl:
          "https://github.com/TheOpenDictionary/odict/edit/main/docs/",
      },
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
          label: "CLI",
          items: [{ label: "Command Reference", slug: "cli/reference" }],
        },
        {
          label: "API",
          items: [
            { label: "Rust", slug: "api/rust" },
            { label: "Python", slug: "api/python" },
            { label: "JavaScript", slug: "api/javascript" },
          ],
        },
      ],
    }),
  ],
});
