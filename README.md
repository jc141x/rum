<h2>Libre game launcher in beta, usage for testing and development only.</h2>

<img src="https://i.postimg.cc/7Ppyhq83/yes.png">

### Portable use

Download the compiled binary from [releases](https://github.com/jc141x/rum/releases). Required dependency is webkit2gtk.

### Arch Linux

Get the package from AUR:

* [rum-bin](https://aur.archlinux.org/packages/rum-bin/) for pre-compiled binary.

* [rum](https://aur.archlinux.org/packages/rum/) to build it locally from latest release.

* [rum-git](https://aur.archlinux.org/packages/rum-git/) build master branch.

### Debian

Package is avaivable on [MPR](https://makedeb.hunterwittenborn.com/mpr/using-the-mpr/installing-packages/) 

* [rum-bin](https://mpr.hunterwittenborn.com/packages/rum-bin)

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
