# Maintainer: Nuzli Hernawan <nuzlilatief@gmail.com>
pkgname=hyprquran
pkgver=0.1.0
pkgrel=1
pkgdesc="A modern, Wayland-native desktop application for reading the Qur'an, built with Rust and GTK4"
arch=('x86_64')
url="https://github.com/foozio/hyprquran"
license=('MIT')
depends=('gtk4')
makedepends=('rust' 'cargo' 'git')
optdepends=('noto-fonts: For Arabic text rendering'
            'amiri-fonts: For optimal Arabic text rendering')
install=hyprquran.install
source=("hyprquran::git+https://github.com/foozio/hyprquran.git")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/$pkgname"
  printf "0.1.0"
}

build() {
  cd "$srcdir/$pkgname"
  cargo build --release --locked --features "gui,sqlite"
}

package() {
  cd "$srcdir/$pkgname"
  
  # Install the main binary
  install -Dm755 "target/release/hyprquran" "$pkgdir/usr/bin/hyprquran"
  
  # Install the import binaries
  install -Dm755 "target/release/import" "$pkgdir/usr/bin/hyprquran-import"
  install -Dm755 "target/release/tanzil_import" "$pkgdir/usr/bin/hyprquran-tanzil-import"
  
  # Install desktop file
  install -Dm644 "assets/desktop/hyprquran.desktop" "$pkgdir/usr/share/applications/hyprquran.desktop"
  
  # Install icons
  install -Dm644 "assets/icons/hyprquran.svg" "$pkgdir/usr/share/icons/hicolor/scalable/apps/hyprquran.svg"
  
  # Install license
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  
  # Install sample data
  cp -r "assets" "$pkgdir/usr/share/$pkgname/"
}