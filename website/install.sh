#!/bin/sh
# Open Longevity installer / updater for macOS and Linux
# Usage: curl -fsSL https://openlongevity.life/install.sh | sh

set -eu

SITE="https://openlongevity.life"
REPOSITORY="edison7009/OpenLongevity"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT INT TERM

say() { printf '  %s\n' "$1"; }
fail() { printf '\n  Installation failed: %s\n' "$1" >&2; exit 1; }

OS="$(uname -s)"
ARCH="$(uname -m)"
case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64|aarch64) PLATFORM="macos-arm" ;;
      x86_64) PLATFORM="macos-intel" ;;
      *) fail "Unsupported macOS architecture: $ARCH" ;;
    esac
    EXT="dmg"
    ;;
  Linux)
    [ "$ARCH" = "x86_64" ] || [ "$ARCH" = "amd64" ] || fail "Only x64 Linux is currently available."
    if command -v dpkg >/dev/null 2>&1; then PLATFORM="linux-deb"; EXT="deb"
    elif command -v rpm >/dev/null 2>&1; then PLATFORM="linux-rpm"; EXT="rpm"
    else PLATFORM="linux-appimage"; EXT="AppImage"
    fi
    ;;
  *) fail "Unsupported operating system: $OS" ;;
esac

printf '\n  Open Longevity Installer\n  ------------------------\n'
say "Checking the latest release..."
VERSION="$(curl -fsSL "$SITE/version.json?platform=$PLATFORM" 2>/dev/null | sed -n 's/.*"version"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' || true)"
DOWNLOAD="$SITE/download/$PLATFORM"

if [ -z "$VERSION" ]; then
  RELEASE="$(curl -fsSL -H 'Accept: application/vnd.github+json' -H 'User-Agent: Open-Longevity-Installer' "https://api.github.com/repos/$REPOSITORY/releases/latest" 2>/dev/null || true)"
  VERSION="$(printf '%s' "$RELEASE" | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"v\{0,1\}\([^"]*\)".*/\1/p' | head -n 1)"
  case "$PLATFORM" in
    macos-arm) FILE="Open.Longevity_${VERSION}_macOS_arm64.dmg" ;;
    macos-intel) FILE="Open.Longevity_${VERSION}_macOS_x64.dmg" ;;
    linux-deb) FILE="Open.Longevity_${VERSION}_Linux_x64.deb" ;;
    linux-rpm) FILE="Open.Longevity_${VERSION}_Linux_x64.rpm" ;;
    *) FILE="Open.Longevity_${VERSION}_Linux_x64.AppImage" ;;
  esac
  DOWNLOAD="https://github.com/$REPOSITORY/releases/download/v$VERSION/$FILE"
fi

[ -n "$VERSION" ] && [ -n "$DOWNLOAD" ] || fail "The installer is not available yet. Please try again in about 10 minutes."
say "Latest: v$VERSION"

INSTALLED_VERSION=""
if [ "$OS" = "Darwin" ]; then
  APP_PATH="/Applications/Open Longevity.app"
  if [ -d "$APP_PATH" ]; then
    INSTALLED_VERSION="$(defaults read "$APP_PATH/Contents/Info" CFBundleShortVersionString 2>/dev/null || true)"
  fi
elif [ "$EXT" = "deb" ]; then
  INSTALLED_VERSION="$(dpkg-query -W -f='${Version}' openlongevity 2>/dev/null || true)"
elif [ "$EXT" = "rpm" ]; then
  INSTALLED_VERSION="$(rpm -q --queryformat '%{VERSION}' openlongevity 2>/dev/null || true)"
elif [ -f "$HOME/.local/bin/open-longevity.version" ]; then
  INSTALLED_VERSION="$(cat "$HOME/.local/bin/open-longevity.version" 2>/dev/null || true)"
fi

if [ -n "$INSTALLED_VERSION" ]; then
  say "Installed: v$INSTALLED_VERSION"
  if [ "$INSTALLED_VERSION" = "$VERSION" ]; then
    printf '\n  Open Longevity is already up to date.\n\n'
    exit 0
  fi
  say "Upgrading v$INSTALLED_VERSION -> v$VERSION..."
else
  say "Performing a fresh installation..."
fi

PACKAGE="$TMP_DIR/Open-Longevity.$EXT"
say "Downloading..."
curl -fL --retry 2 -o "$PACKAGE" "$DOWNLOAD" || fail "The download could not be completed."
[ -s "$PACKAGE" ] || fail "The installer download is empty."

if [ "$OS" = "Darwin" ]; then
  MOUNT="$TMP_DIR/mount"
  mkdir -p "$MOUNT"
  hdiutil attach "$PACKAGE" -nobrowse -quiet -mountpoint "$MOUNT" || fail "Unable to mount the disk image."
  APP="$(find "$MOUNT" -maxdepth 1 -name '*.app' -print -quit)"
  [ -n "$APP" ] || fail "The disk image does not contain an application."
  say "Installing to /Applications..."
  if [ -w "/Applications" ]; then
    rm -rf "/Applications/Open Longevity.app"
    cp -R "$APP" "/Applications/Open Longevity.app"
  else
    sudo rm -rf "/Applications/Open Longevity.app"
    sudo cp -R "$APP" "/Applications/Open Longevity.app"
  fi
  xattr -cr "/Applications/Open Longevity.app" 2>/dev/null || true
  hdiutil detach "$MOUNT" -quiet || true
elif [ "$EXT" = "deb" ]; then
  say "Installing the Debian package..."
  sudo dpkg -i "$PACKAGE" || { sudo apt-get install -f -y && sudo dpkg -i "$PACKAGE"; }
elif [ "$EXT" = "rpm" ]; then
  say "Installing the RPM package..."
  if command -v dnf >/dev/null 2>&1; then sudo dnf install -y "$PACKAGE"; else sudo rpm -U "$PACKAGE"; fi
else
  TARGET="$HOME/.local/bin/open-longevity.AppImage"
  mkdir -p "$HOME/.local/bin"
  cp "$PACKAGE" "$TARGET"
  chmod +x "$TARGET"
  printf '%s\n' "$VERSION" > "$HOME/.local/bin/open-longevity.version"
  say "Installed AppImage to $TARGET"
fi

printf '\n  Open Longevity v%s is installed.\n\n' "$VERSION"
