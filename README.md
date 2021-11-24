# Rum (former chad launcher)
Libre game launcher for GNU/Linux hackers powered by [Tauri](https://tauri.studio).

Developed by the johncena141 hacker group from 1337x. For questions or contribution, talk with us on [Matrix](https://matrix.to/#/!SlYhhmreXjJylcsjfn:tedomum.net?via=matrix.org&via=tedomum.net).

## Portable use

Download the compiled binary from [releases](https://notabug.org/johncena141/rum/releases). Required dependency is webkit2gtk.

## Arch Linux

Get the package from AUR:

* [rum-bin](https://aur.archlinux.org/packages/rum-bin/) for pre-compiled binary.

* [rum](https://aur.archlinux.org/packages/rum/) to build it locally from latest release.

* [rum-git](https://aur.archlinux.org/packages/rum-git/): build master branch.~~

## Gentoo
This is a guide on installing with eselect-repository for gentoo.
Make sure eselect-repository is installed
```
yes | emerge --update app-eselect/eselect-repository
```
After that is done you are able to add the [rum-repo](https://notabug.org/agdfrhjlbzvf/rum-repo) repository
```
eselect repository add rum-repo git https://notabug.org/agdfrhjlbzvf/rum-repo.git
emaint sync -r rum-repo
```
And finally install rum itself
```
emerge -av rum
```

## Build from source

We recommend using `pnpm` to build this project ([AUR](https://aur.archlinux.org/packages/pnpm/)), but any other
package manager like `npm` or `yarn` should work too.

```
pnpm install
pnpm build
pnpm tauri build
```

## Running development server

```
pnpm dev
pnpm tauri dev
```

## More info on [Wiki](https://notabug.org/johncena141/rum/wiki)

<img src="https://i.postimg.cc/nL9MJ4Df/ytryrty.png">
<img src="https://i.postimg.cc/wTF1cTpZ/6456.pngg">

<img src="https://i.postimg.cc/cC2cG149/434.png">

Donations (Monero): 4ABGQLAeAgiauvay11VRrWXRRtraRCU6oaC6uG9RUnNCHN4eepzWjEB6sHF92sUrSED5b8GyY7Ayh57R1jUdcKZg7is2DW3
