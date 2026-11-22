#!/bin/sh
# postrm do pacote .deb: remove o atalho criado pelo postinst.
#
# Nao dependemos de SUDO_USER/PKEXEC_UID: na desinstalacao pela interface
# grafica (aptdaemon/PolicyKit) essas variaveis normalmente nao existem.
# Removemos o arquivo exato de todos os usuarios humanos (UID >= 1000),
# inclusive Desktop localizado (XDG_DESKTOP_DIR).
set -u

# Em upgrade o postinst recria o atalho; so removemos em remove/purge.
case "${1:-}" in
  remove|purge) ;;
  *) exit 0 ;;
esac

remove_for_home() {
  home="$1"
  [ -n "$home" ] || return 0
  [ -d "$home" ] || return 0

  rm -f "$home/Desktop/avifnim-viewer.desktop"

  user_dirs="$home/.config/user-dirs.dirs"
  if [ -r "$user_dirs" ]; then
    desktop_dir="$(sed -n 's/^XDG_DESKTOP_DIR="\(.*\)"$/\1/p' "$user_dirs" | head -n1)"
    case "$desktop_dir" in
      '$HOME'*) desktop_dir="$home${desktop_dir#'$HOME'}" ;;
    esac
    if [ -n "$desktop_dir" ]; then
      rm -f "$desktop_dir/avifnim-viewer.desktop"
    fi
  fi
}

# Itera todas as contas humanas; cobre homes fora de /home tambem.
if command -v getent >/dev/null 2>&1; then
  getent passwd 2>/dev/null | while IFS=: read -r _user _passwd uid _gid _gecos home _shell; do
    case "$uid" in
      ''|*[!0-9]*) continue ;;
    esac
    [ "$uid" -ge 1000 ] || continue
    [ "$home" = "/nonexistent" ] && continue
    remove_for_home "$home"
  done
else
  for home in /home/*; do
    [ -d "$home" ] && remove_for_home "$home"
  done
fi

exit 0
