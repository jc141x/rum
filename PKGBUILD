pkgname='chad_launcher-git'
_pkgname='chad_launcher'
pkgver=r83.2c4e917
pkgrel=1
pkgdesc='GNU/LINUX GAMING UNLEASHED!'
arch=('x86_64')
url='https://gitlab.com/Gnurur/chad_launcher'
license=('GPL3')
depends=(cargo webkit2gtk curl wget openssl appmenu-gtk-module gtk3 libappindicator-gtk3 libvips)
makedepends=(npm git squashfs-tools patchelf)
source=('git+https://gitlab.com/Gnurur/chad_launcher.git#branch=tauri')
md5sums=('SKIP')

pkgver() {
    cd "$srcdir/$_pkgname"
    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
    cd "$srcdir/$_pkgname"
    export RUSTUP_TOOLCHAIN=stable
    npm install
    npm run build
    npm run tauri build
}

package() {
    cd "$srcdir/$_pkgname"
    install -Dm0755 -t "$pkgdir/usr/bin/" "src-tauri/target/release/chad-launcher"
    install -Dm644 ./src/chad_launcher/chad_launcher.desktop "$pkgdir/usr/share/applications/chad-launcher.desktop"
    install -Dm644 ./src/chad_launcher/icon.svg "$pkgdir/usr/share/pixmaps/chad-launcher.svg"

}

