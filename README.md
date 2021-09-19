# chad launcher
Libre/Free game launcher/pirate game store for users who dislike DRM or other restrictions of freedom.

Developed by the [GNU/Linux P2P Pirates](https://matrix.to/#/!SlYhhmreXjJylcsjfn:tedomum.net?via=matrix.org&via=tedomum.net) matrix community and johncena141 release group from 1337x.

Powered by [Tauri](https://tauri.studio)

## Installation or portable use

### Portable use

Download the compiled binary from releases. Required dependency is webkit2gtk.

### Install from AUR

```sh
git clone https://aur.archlinux.org/paru-bin.git && cd paru-bin && makepkg -si
paru -S chad_launcher-bin
# Alternative called chad_launcher-git is available which will compile every update from latest commit. Not recommended for regular use.
```

### Install from DUR (Debian)

- Work in progress.

### Install from Fedora Projects

- Work in progress.

### Build from source

We recommend using `pnpm` to build this project. ([AUR](https://aur.archlinux.org/packages/pnpm/))

```
pnpm install
pnpm build
pnpm tauri build
```

This will create a `chad_launcher` executable, an AppImage and a debian package.

## Setting up a torrent client
### qBittorrent
To set up qBittorent you will need to enable the web ui.
This can be done by going into options > webui ticking the checkbox. 
You can set a custom password but the default password is 
```
adminadmin
```
Then you can go into the chad Launcher settings and add it, the defaults will work, you just need to add in the password.

## Development

### Running development server

```
pnpm dev
pnpm tauri dev
```

## Current GUI
<img src="https://i.postimg.cc/fRtHc5cM/test.png"/>

# Donations
Monero: 4ABGQLAeAgiauvay11VRrWXRRtraRCU6oaC6uG9RUnNCHN4eepzWjEB6sHF92sUrSED5b8GyY7Ayh57R1jUdcKZg7is2DW3
