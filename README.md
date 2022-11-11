<h2>Project cancelled. (due to no active developers)</h2>

<h3>State of project is not up to date with current release mechanics, bash scripts need chmod +x for detection.</h3>

### Portable use

Download the compiled binary from [releases](https://github.com/jc141x/rum/releases). Required dependency is webkit2gtk.

### Build from source

We recommend using `pnpm` to build this project ([AUR](https://aur.archlinux.org/packages/pnpm/)), but any other
package manager like `npm` or `yarn` should work too.

```
pnpm install
pnpm build
pnpm tauri build
```

### Running development server

```
pnpm dev
pnpm tauri dev
```
