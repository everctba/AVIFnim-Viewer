#!/bin/sh
# postrm do pacote .deb: remove o atalho criado pelo postinst (somente do
# usuario que executou o sudo/pkexec).
set -eu

TARGET_USER="${SUDO_USER:-}"

if [ -z "$TARGET_USER" ] && [ -n "${PKEXEC_UID:-}" ]; then
  TARGET_USER="$(getent passwd "$PKEXEC_UID" 2>/dev/null | cut -d: -f1 || true)"
fi

[ -n "$TARGET_USER" ] || exit 0
[ "$TARGET_USER" = "root" ] && exit 0

passwd_entry="$(getent passwd "$TARGET_USER" 2>/dev/null || true)"
[ -n "$passwd_entry" ] || exit 0
home_dir="$(printf '%s' "$passwd_entry" | cut -d: -f6)"
[ -d "$home_dir" ] || exit 0

desktop_dir=""
if command -v xdg-user-dir >/dev/null 2>&1; then
  if command -v runuser >/dev/null 2>&1; then
    desktop_dir="$(runuser -u "$TARGET_USER" -- env HOME="$home_dir" xdg-user-dir DESKTOP 2>/dev/null || true)"
  elif command -v sudo >/dev/null 2>&1; then
    desktop_dir="$(sudo -u "$TARGET_USER" -- env HOME="$home_dir" xdg-user-dir DESKTOP 2>/dev/null || true)"
  fi
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

rm -f "$desktop_dir/avifnim-viewer.desktop"
exit 0
