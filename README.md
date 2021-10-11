# chad launcher
Libre/Free game launcher/GUI for running bash scripts.

<img src="https://i.postimg.cc/cHMfLtLy/3423423.png">

This project was previously about developing a pirate game store. But due to issues with maintaining the database and relying on online services, we decided to move our focus on creating an offline GUI where you build your library of games manually, as well as still developing some features that we still want such as automated backups of game saves.

We are making this as an alternative to Lutris that doesn't do more than it needs to. We believe that patching games needs to be done in bash instead of an ambiguous collection of settings trough a GUI. So the GUI should only serve as a frontend and not as a debugger or compatiblity tool.

Developed by the [GNU/Linux P2P Pirates](https://matrix.to/#/!SlYhhmreXjJylcsjfn:tedomum.net?via=matrix.org&via=tedomum.net) matrix community and johncena141 release group from 1337x.

## Important: looking for new developers/maintainers.

## Installation or portable use

### Portable use

Download the compiled binary from [releases](https://notabug.org/johncena141/chad-launcher/releases). Required dependency is webkit2gtk.

### Install from AUR

Two AUR packages are available:

- [chad-launcher-bin](https://aur.archlinux.org/packages/chad-launcher-bin/): Downloads GitLab CI artifact of latest release.
- [chad-launcher-git](https://aur.archlinux.org/packages/chad-launcher-git/): Builds the master branch from source.

### Install from MPR (Debian)
Enable MPR on your system:
```
wget -qO - 'https://proget.hunterwittenborn.com/debian-feeds/makedeb.pub' | \
gpg --dearmor | \
sudo tee /usr/share/keyrings/makedeb-archive-keyring.gpg &> /dev/null
```
```
echo 'deb [signed-by=/usr/share/keyrings/makedeb-archive-keyring.gpg arch=all] https://proget.hunterwittenborn.com/ makedeb main' | \
sudo tee /etc/apt/sources.list.d/makedeb.list
```
```
sudo apt update && sudo apt install makedeb
```
```
git clone "https://mpr.hunterwittenborn.com/makedeb.git"
git clone "https://mpr.hunterwittenborn.com/makedeb-makepkg.git"

cd makedeb-makepkg/
makedeb -si

cd ../makedeb/
makedeb -si
```

Install tap (MPR helper)
```
git clone https://mpr.hunterwittenborn.com/tap.git && cd tap && makedeb -si
```
Install chad launcher
```
tap install chad-launcher-bin
```


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

This will create a `chad-launcher` executable and a debian package.

## Development

See Developer Guide in the [wiki](https://notabug.org/johncena141/chad-launcher/wiki).

### Running development server

```
pnpm dev
pnpm tauri dev
```

## Wiki

Read the [wiki](https://notabug.org/johncena141/chad-launcher/wiki).

# Donations
Monero: 4ABGQLAeAgiauvay11VRrWXRRtraRCU6oaC6uG9RUnNCHN4eepzWjEB6sHF92sUrSED5b8GyY7Ayh57R1jUdcKZg7is2DW3

Powered by [Tauri](https://tauri.studio)
