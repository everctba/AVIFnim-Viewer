"""Gera os assets de teste/QA do AVIFnim Viewer (requer Pillow).

Uso:
    python scripts/make-test-assets.py

Gera em tests/assets:
  - still.avif, anim.avif (8 frames), anim.webp, anim.gif, promo-avif.avif
  - p3.ppm (ASCII), p6.ppm (binario), broken.ppm (header malicioso)
  - seq/frame_0001..0012.png
  - big-1080p.avif (~3,4 MB) e big-5mb.avif (~5,4 MB) para benchmarks
    (estes dois ficam fora do git; sao regenerados por este script)
"""
import os
import random

from PIL import Image, ImageDraw

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ASSETS = os.path.join(BASE, "tests", "assets")
SRC_ASSETS = os.path.join(BASE, "src", "assets")
os.makedirs(os.path.join(ASSETS, "seq"), exist_ok=True)


def gradient(size=(96, 64)):
    img = Image.new("RGB", size)
    px = img.load()
    w, h = size
    for y in range(h):
        for x in range(w):
            px[x, y] = (int(255 * x / max(1, w - 1)), int(255 * y / max(1, h - 1)), (x * y) % 256)
    return img


def frames(size=(96, 64), count=8):
    out = []
    for i in range(count):
        img = gradient(size)
        d = ImageDraw.Draw(img)
        d.ellipse([8 + i * 9, 22, 28 + i * 9, 42], fill=(255, 210, 70))
        out.append(img)
    return out


def main():
    # stills e animacoes web
    gradient().save(os.path.join(ASSETS, "still.avif"), "AVIF", quality=60)
    animated = frames()
    animated[0].save(
        os.path.join(ASSETS, "anim.avif"), "AVIF",
        save_all=True, append_images=animated[1:], duration=100, loop=0, quality=60,
    )
    animated[0].save(
        os.path.join(ASSETS, "anim.webp"), "WEBP",
        save_all=True, append_images=animated[1:], duration=100, loop=0,
    )
    animated[0].save(
        os.path.join(ASSETS, "anim.gif"), "GIF",
        save_all=True, append_images=animated[1:], duration=100, loop=0,
    )

    # banner do Sobre (16 frames 160x160)
    promo = []
    for i in range(16):
        img = Image.new("RGB", (160, 160), (18, 18, 26))
        d = ImageDraw.Draw(img)
        angle = i * 22.5
        for radius, color in [(78, (124, 92, 255)), (58, (34, 211, 238)), (38, (255, 184, 107))]:
            d.arc(
                [80 - radius, 80 - radius, 80 + radius, 80 + radius],
                start=angle, end=angle + 150, fill=color, width=10,
            )
        promo.append(img)
    promo[0].save(
        os.path.join(SRC_ASSETS, "promo-avif.avif"), "AVIF",
        save_all=True, append_images=promo[1:], duration=80, loop=0, quality=55,
    )

    # PPM P3/P6 e header malicioso
    small = Image.new("RGB", (8, 8))
    for y in range(8):
        for x in range(8):
            small.putpixel((x, y), (x * 32, y * 32, 128))
    with open(os.path.join(ASSETS, "p3.ppm"), "w", encoding="ascii") as f:
        f.write("P3\n8 8\n255\n")
        for y in range(8):
            row = []
            for x in range(8):
                row.extend(str(c) for c in small.getpixel((x, y)))
            f.write(" ".join(row) + "\n")
    gradient((64, 64)).save(os.path.join(ASSETS, "p6.ppm"), "PPM")
    with open(os.path.join(ASSETS, "broken.ppm"), "w", encoding="ascii") as f:
        f.write("P6\n999999 999999\n255\n")

    # sequencia PNG (12 frames 64x64)
    for i in range(12):
        img = Image.new("RGB", (64, 64), (20, 20, 28))
        d = ImageDraw.Draw(img)
        d.rectangle([4 + i * 4, 20, 24 + i * 4, 44], fill=(124, 92, 255))
        d.text((6, 6), "f%02d" % (i + 1), fill=(230, 230, 240))
        img.save(os.path.join(ASSETS, "seq", "frame_%04d.png" % (i + 1)), "PNG")

    # amostras grandes para benchmark (nao versionadas)
    Image.frombytes("RGB", (1920, 1080), os.urandom(1920 * 1080 * 3)).save(
        os.path.join(ASSETS, "big-1080p.avif"), "AVIF", quality=100
    )
    Image.frombytes("RGB", (2400, 1350), os.urandom(2400 * 1350 * 3)).save(
        os.path.join(ASSETS, "big-5mb.avif"), "AVIF", quality=100
    )

    print("assets gerados em", ASSETS)


if __name__ == "__main__":
    main()
