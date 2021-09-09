# Chad Launcher

The new version of Chad Launcher powered by [Tauri](https://tauri.studio).

Developed by the [GNU/Linux P2P Pirates](https://matrix.to/#/!SlYhhmreXjJylcsjfn:tedomum.net?via=matrix.org&via=tedomum.net) community and johncena141 release group from 1337x.

We are followers of the GNU project philosophy. Even if the games we upload are proprietary, we believe that our actions are necessary in order to enable people to use less proprietary software such as Steam, Epic Games, Origin, Uplay, Discord, Stadia or GOG Galaxy. 

Some of them also use DRM or APIs that make games tied to their store. They spy and abuse people for their own gain, they are rich and want to get richer from the hands of the fools of this community that have orgasms from hearing 'port'.

Buying yourself out of the guilt of spying on people will only work for people that didn't mind it in the first place. We do not change our minds through such moronic thinking.

We are not intending to create an environment where we provide the games from these platforms for free. We are working towards the complete replacement for the people that value their security, privacy and which remember why they use GNU/Linux in the first place.
For the people that value these ideals as much as us, they are welcome to join our fight.

## Installation

### Install from AUR

```sh
sudo pacman -S --needed paru
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

## Current GUI
<img src="https://i.postimg.cc/zG5gBndF/11111.png"/>
