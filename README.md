# chad launcher

Libre/Free game launcher/pirate game store for users who dislike DRM or other restrictions of freedom.

<img src="https://i.postimg.cc/fRtHc5cM/test.png"/>

Developed by the [GNU/Linux P2P Pirates](https://matrix.to/#/!SlYhhmreXjJylcsjfn:tedomum.net?via=matrix.org&via=tedomum.net) matrix community and johncena141 release group from 1337x.

Powered by [Tauri](https://tauri.studio)

## Important: looking for new developers/maintainers.

I will not have a lot of time to work on chad launcher until maybe next summer, so we are looking for new people
to continue development.

## Installation or portable use

### Portable use

Download the compiled binary from [releases](https://gitlab.com/Gnurur/chad_launcher/-/releases). Required dependency is webkit2gtk.

Alternatively, compiled CI artifacts for each commit can be downloaded [here](https://gitlab.com/Gnurur/chad_launcher/-/pipelines).

### Install from AUR

Two AUR packages are available:

- [chad_launcher-bin](https://aur.archlinux.org/packages/chad_launcher-bin/): Downloads GitLab CI artifact of latest release.
- [chad_launcher-git](https://aur.archlinux.org/packages/chad_launcher-git/): Builds the master branch from source.

### Install from MPR (Debian)

- Work in progress.

### Install from Fedora Projects

- Work in progress.

### Build from source

We recommend using `pnpm` to build this project ([AUR](https://aur.archlinux.org/packages/pnpm/)), but any other 
package manager like `npm` or `yarn` should work too.

```
pnpm install
pnpm build
pnpm tauri build
```

This will create a `chad_launcher` executable and a debian package.

## Development

See Developer Guide in the [wiki](https://gitlab.com/Gnurur/chad_launcher/-/wikis/home).

### Running development server

```
pnpm dev
pnpm tauri dev
```

## Wiki

Read the [wiki](https://gitlab.com/Gnurur/chad_launcher/-/wikis/home).

# Donations
Monero: 4ABGQLAeAgiauvay11VRrWXRRtraRCU6oaC6uG9RUnNCHN4eepzWjEB6sHF92sUrSED5b8GyY7Ayh57R1jUdcKZg7is2DW3
