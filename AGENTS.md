# Repository Guidelines

## Project Structure & Module Organization

Frameport is a local-first media import and management desktop app. Rust code is split by responsibility: `app/` contains the Tauri shell, `engine/` media import logic, `db/` local database code, and `settings/` configuration handling. The Svelte frontend lives in `ui/`, with routes in `ui/src/routes`, shared exports in `ui/src/lib`, static files in `ui/static`, and UI assets in `ui/src/lib/assets`. Architecture notes and diagrams are kept in `docs/`.

## Build, Test, and Development Commands

- `__NV_DISABLE_EXPLICIT_SYNC=1 pnpm tauri dev`: starts the Tauri app from `app/`.
- `pnpm run build`: builds the Tauri desktop app.
- `pnpm run ui:dev`: runs the Vite/Svelte development server for frontend-only work.
- `pnpm run ui:build`: builds the Svelte app.
- `pnpm run ui:check`: runs `svelte-check` with `ui/tsconfig.json`.
- `pnpm --prefix ui run lint`: checks frontend formatting and ESLint rules.
- `cargo check` or `cargo test` inside a crate, such as `engine/` or `app/`: checks or tests that crate. Root scripts include workspace commands, but there is currently no root `Cargo.toml`.

## Coding Style & Naming Conventions

Use Rust 2021 in `app/` and Rust 2024 in the library crates. Format Rust with `cargo fmt`; keep module and file names in `snake_case`. Frontend code uses TypeScript, Svelte 5, Prettier, ESLint, and Tailwind CSS. Follow `ui/.prettierrc`: tabs, single quotes, no trailing commas, and a 100-column print width. Svelte route files should follow SvelteKit conventions like `+page.svelte`.

## Testing Guidelines

There are no dedicated test files yet. Add Rust unit tests next to the code they cover using `#[cfg(test)]`, and add integration tests under each crate’s `tests/` directory when behavior crosses module boundaries. For frontend changes, run `npm run ui:check` and `npm --prefix ui run lint`; add component or browser tests once a test framework is introduced.

## Commit & Pull Request Guidelines

Existing commits use short, lower-case summaries such as `added docs` and `project setup`. Keep commits focused and use concise imperative or descriptive subjects. Pull requests should describe the change, explain any user-visible behavior, link related issues when available, and include screenshots or short recordings for UI changes. Run the relevant checks before requesting review.

## Security & Configuration Tips

Frameport manages local media, so avoid logging private file paths or media metadata unnecessarily. Keep Tauri capability changes in `app/capabilities/` narrow and document why new permissions are needed.
