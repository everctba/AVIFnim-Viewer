#!/usr/bin/env bash
# Normaliza o .deb gerado pelo Tauri:
# - Package: avi-fnim-viewer -> avifnim-viewer (Tauri deriva do productName com kebab-case)
# - desktop file "AVIFnim Viewer.desktop" -> avifnim-viewer.desktop
# - regenera md5sums e reempacota como root:root
set -euo pipefail

deb="${1:-}"
if [[ -z "$deb" ]]; then
  deb="$(ls -1t src-tauri/target/release/bundle/deb/*.deb | head -n1)"
fi
if [[ ! -f "$deb" ]]; then
  echo "erro: .deb nao encontrado" >&2
  exit 1
fi

orig_pkg="$(dpkg-deb -f "$deb" Package)"
version="$(dpkg-deb -f "$deb" Version)"
arch="$(dpkg-deb -f "$deb" Architecture)"
echo "origem: $deb (Package: $orig_pkg, $version, $arch)"

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
dpkg-deb -R "$deb" "$work/pkg"

control="$work/pkg/DEBIAN/control"
sed -i -E 's/^Package: .*/Package: avifnim-viewer/' "$control"

# Transicao suave para quem tem o pacote antigo instalado.
if ! grep -q '^Provides:' "$control"; then
  {
    echo "Provides: avi-fnim-viewer"
    echo "Replaces: avi-fnim-viewer"
    echo "Conflicts: avi-fnim-viewer"
  } >>"$control"
fi

apps="$work/pkg/usr/share/applications"
if [[ -d "$apps" ]]; then
  first=1
  for f in "$apps"/*.desktop; do
    [[ -e "$f" ]] || continue
    if [[ $first -eq 1 ]]; then
      mv "$f" "$apps/avifnim-viewer.desktop"
      first=0
    else
      rm -f "$f"
    fi
  done
fi

# md5sums precisam refletir o desktop renomeado.
if [[ -d "$work/pkg/DEBIAN" ]]; then
  (
    cd "$work/pkg"
    find . -path ./DEBIAN -prune -o -type f -printf '%P\0' |
      sort -z |
      xargs -0 md5sum >DEBIAN/md5sums
  )
fi

out="$(dirname "$deb")/avifnim-viewer_${version}_${arch}.deb"
dpkg-deb --root-owner-group -b "$work/pkg" "$out" >/dev/null
rm -f "$deb"

new_pkg="$(dpkg-deb -f "$out" Package)"
if [[ "$new_pkg" != "avifnim-viewer" ]]; then
  echo "erro: Package inesperado: $new_pkg" >&2
  exit 1
fi
echo "ok: $out (Package: $new_pkg)"
