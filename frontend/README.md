# Frontend

## File structure

**TODO** - finish file structure

- `.husky/` - pre-commit hook
- `public/` - static files
- `src/` - source code
    - `assets/` - resources that are used in JavaScript and bundled with the final build
    - `components/` - reusable components
        - `ui/` - shadcn/ui components
    - `environment/` - environment variables object
    - `features/` - components which are used only once (AppBar, Menu, etc.)
    - `hooks/` - custom React hooks
    - `lib/` - helper functions
    - `pages/` - application pages
- `index.html` - entry point to application

## Pre-commit hook

Pre-commit hook is always installed to `.husky/_` directory after dependencies installation.

However, you can install it directly using:

```sh
npm run prepare
```

The hook runs these checks on staged files in the following order:

1. **ESLint**. Linter for JavaScript. Show problems in code.
1. **Prettier**. Format Javascript / Typescript code based on rules in `.prettierrc`.

To skip hook run Git command with `-n/--no-verify` option:

```sh
git commit -m "..." -n
```

## Prettier (Visual Studio Code setup)

1. Install [Prettier - Code formatter](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode) extension.
1. Add next lines to Workspace or User settings JSON.
    
    `CTRL + SHIFT + P` -> `Preferences: Open User Settings (JSON)`

    ```json
    {
        // Prettier
        "prettier.requireConfig": true,

        // JavaScript / TypeScript
        "[javascript][javascriptreact][typescript][typescriptreact]": {
            "editor.defaultFormatter": "esbenp.prettier-vscode",
            "editor.formatOnSave": true,
            "editor.codeActionsOnSave": {
                "source.organizeImports": "explicit"
            }
        }
    }
    ```

Also, import sorting has been added here.