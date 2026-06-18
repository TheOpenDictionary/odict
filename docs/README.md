# ODict Docs

This is the Astro Starlight documentation site for ODict.

## Project Structure

```
.
├── public/
├── src/
│   ├── assets/
│   ├── content/
│   │   └── docs/
│   └── content.config.ts
├── astro.config.mjs
├── package.json
└── tsconfig.json
```

Starlight looks for `.md` or `.mdx` files in the `src/content/docs/` directory. Each file is exposed as a route based on its file name.

Images can be added to `src/assets/` and embedded in Markdown with a relative link.

Static assets, like favicons, can be placed in the `public/` directory.

## Commands

All commands are run from this `docs/` directory.

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `yarn install`            | Installs dependencies                            |
| `yarn dev`                | Starts local dev server at `localhost:4321`      |
| `yarn build`              | Builds the production site to `./dist/`          |
| `yarn preview`            | Previews the production build locally            |
| `yarn astro ...`          | Runs Astro CLI commands                          |

Use the main ODict workspace commands for library, CLI, Python, and Node development.
