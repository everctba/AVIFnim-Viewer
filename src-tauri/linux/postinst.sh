#!/bin/sh
# postinst do pacote .deb: cria o atalho do AVIFnim Viewer na area de trabalho
# do usuario que executou o sudo (apt/pkexec), usando xdg-user-dir com fallback.
set -eu

DESKTOP_FILE="/usr/share/applications/avifnim-viewer.desktop"
TARGET_USER="${SUDO_USER:-}"

if [ -z "$TARGET_USER" ] && [ -n "${PKEXEC_UID:-}" ]; then
  TARGET_USER="$(getent passwd "$PKEXEC_UID" 2>/dev/null | cut -d: -f1 || true)"
fi

# Sem usuario identificado (instalacao como root direto): nao cria atalho.
[ -n "$TARGET_USER" ] || exit 0
[ "$TARGET_USER" = "root" ] && exit 0
[ -f "$DESKTOP_FILE" ] || exit 0

passwd_entry="$(getent passwd "$TARGET_USER" 2>/dev/null || true)"
[ -n "$passwd_entry" ] || exit 0
home_dir="$(printf '%s' "$passwd_entry" | cut -d: -f6)"
gid="$(printf '%s' "$passwd_entry" | cut -d: -f4)"
group="$(getent group "$gid" 2>/dev/null | cut -d: -f1 || true)"
[ -d "$home_dir" ] || exit 0
[ -n "${group:-}" ] || group="$(id -gn "$TARGET_USER" 2>/dev/null || echo "$TARGET_USER")"

as_user() {
  _as_user="$1"
  shift
  if command -v runuser >/dev/null 2>&1; then
    runuser -u "$_as_user" -- "$@" 2>/dev/null
  elif command -v sudo >/dev/null 2>&1; then
    sudo -u "$_as_user" -- "$@" 2>/dev/null
  else
    return 1
  fi
}

desktop_dir=""
if command -v xdg-user-dir >/dev/null 2>&1; then
  desktop_dir="$(as_user "$TARGET_USER" env HOME="$home_dir" xdg-user-dir DESKTOP 2>/dev/null || true)"
fi

if [ -z "$desktop_dir" ] || [ "$desktop_dir" = "$home_dir" ]; then
  user_dirs="$home_dir/.config/user-dirs.dirs"
  if [ -r "$user_dirs" ]; then
    desktop_dir="$(sed -n 's/^XDG_DESKTOP_DIR="\(.*\)"$/\1/p' "$user_dirs" | head -n1)"
    case "$desktop_dir" in
      '$HOME'*) desktop_dir="$home_dir${desktop_dir#'$HOME'}" ;;
    esac
  fi
fi

[ -n "$desktop_dir" ] || desktop_dir="$home_dir/Desktop"
[ "$desktop_dir" = "$home_dir" ] && desktop_dir="$home_dir/Desktop"

# Se o ambiente nao tem area de trabalho (ex.: servidor/CI), nao cria a pasta.
[ -d "$desktop_dir" ] || exit 0

install -o "$TARGET_USER" -g "$group" -m 0755 "$DESKTOP_FILE" "$desktop_dir/avifnim-viewer.desktop"
as_user "$TARGET_USER" gio set "$desktop_dir/avifnim-viewer.desktop" metadata::trusted true >/dev/null 2>&1 || true
echo "AVIFnim Viewer: atalho criado em $desktop_dir/avifnim-viewer.desktop"
exit 0
