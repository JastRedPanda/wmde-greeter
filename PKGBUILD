# Maintainer: WMDE <https://wmde.fun>
# Contributor: System76 <info@system76.com> (original cosmic-greeter)
#
# Builds our fork Lin-WMDE/wmde-greeter (branch wmde). One binary with two roles,
# chosen by the effective user (src/main.rs): as the system user wmde-greeter it is
# the greetd greeter, as anyone else it is the ext-session-lock client. Owns
# fun.wmde.Greeter on the system bus. Installs alongside cosmic-greeter (own binary,
# own D-Bus name, own user, own greetd config), so NO conflicts/replaces and NO
# compat symlinks.
#
# Installing this package does NOT make it the display manager: the unit ships
# disabled, and `systemctl enable wmde-greeter.service` is what claims the
# display-manager.service alias. Deliberate - only one may hold it.
pkgname=wmde-greeter
pkgver=0.1.0
pkgrel=1
pkgdesc="WMDE greeter and screen locker (fork of cosmic-greeter) - owns fun.wmde.Greeter"
arch=('x86_64')
url="https://wmde.fun"
license=('GPL-3.0-only')
# runtime: greetd drives the greeter, wmde-comp hosts it (wmde-greeter-start execs
# `wmde-comp wmde-greeter`), pam authenticates, wayland/libxkbcommon come from the
# libcosmic winit client. wmde-icons-base carries the WMDE icon theme the UI names.
# Verify with namcap after first build.
depends=('glibc' 'gcc-libs' 'wayland' 'libxkbcommon' 'pam' 'greetd' 'wmde-comp'
         'wmde-icons-base')
optdepends=('wmde-randr: remember per-output layout at the greeter'
            'networkmanager: network status on the greeter'
            'upower: battery status on the greeter')
makedepends=('rust' 'cargo' 'just' 'git' 'clang' 'lld' 'pkgconf' 'wayland' 'libxkbcommon' 'pam')
backup=('etc/greetd/wmde-greeter.toml' 'etc/pam.d/wmde-greeter')
source=("$pkgname::git+https://github.com/Lin-WMDE/wmde-greeter.git#branch=wmde")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/$pkgname"
  # WMDE unified version: 1.5 (libcosmic base) . <commits since nearest tag> . g<short>.
  local desc
  desc=$(git describe --long --tags --abbrev=7 2>/dev/null || true)
  if [ -n "$desc" ]; then
    printf '1.5.%s.g%s' "$(printf '%s' "$desc" | sed -E 's/.*-([0-9]+)-g[0-9a-f]+$/\1/')" "$(git rev-parse --short=7 HEAD)"
  else
    printf '1.5.%s.g%s' "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
  fi
}

build() {
  cd "$srcdir/$pkgname"
  # x86-64-v3 (AVX2/BMI2) baseline for the WMDE repo; runs on Haswell+ (and the VM).
  export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C target-cpu=x86-64-v3"
  just build-release
}

package() {
  cd "$srcdir/$pkgname"
  # installs /usr/bin/wmde-greeter{,-daemon,-start}, the D-Bus policy, sysusers.d,
  # tmpfiles.d, both systemd units, /etc/greetd/wmde-greeter.toml and /etc/pam.d/wmde-greeter
  just rootdir="$pkgdir" prefix=/usr install
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
