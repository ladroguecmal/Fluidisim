"""S365 — l'angle de la fenêtre de Snell sur une image prise sous l'eau, au zénith (liste 8.6, ADR-019 §2).

Usage : python outils/fenetre_snell.py [--fresnel] <image.png> <champ_vertical_degres> [indice]

La caméra vise le zénith sous une mer plate : la fenêtre est un disque centré sur l'image. Le long de huit rayons partant
du centre, on cherche la plus forte chute de luminance (sRGB décodé, Rec. 709) — au bord de la fenêtre, la transmission
eau → air tombe à zéro avec une pente infinie ; le pic de la dérivée est affiné par une parabole sur trois points. Le
rayon en pixels devient un angle par `atan(r/f)`, `f = (H/2)/tan(champ/2)` (Godot garde le champ vertical). Attendu :
`arcsin(1/n)`, 48,27° pour n = 1,34.

**Les nuages trompent la plus forte chute** : près du bord, la fenêtre porte l'horizon du ciel, tassé, et le bord d'un
nuage y chute parfois plus fort que la fenêtre elle-même (mesuré, S365). `--fresnel` lit donc l'image de contrôle
(`CONTROLE_EAU=4`) où la surface rend son coefficient de Fresnel eau → air : 1 exactement au-delà de l'angle critique. Le
bord est le premier pixel à 255 le long du rayon, à un demi-pixel près (0,06°) — `1 − R` y tombe de 0,01 à 0 en 10⁻⁴°.
"""
import math
import sys

import numpy as np
from PIL import Image


def lineaire(c):
    c = c / 255.0
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


def fresnel(chemin, champ, n):
    img = np.asarray(Image.open(chemin).convert("RGB"), dtype=np.int32).min(axis=2)
    h, w = img.shape
    cx, cy = w / 2.0, h / 2.0
    f = (h / 2.0) / math.tan(math.radians(champ) / 2.0)
    attendu = math.degrees(math.asin(1.0 / n))
    angles = []
    for k in range(16):
        a = k * math.pi / 8.0
        dx, dy = math.cos(a), math.sin(a)
        r = 1.0
        while True:
            x, y = int(cx + r * dx), int(cy + r * dy)
            if not (0 <= x < w and 0 <= y < h):
                r = float("nan")
                break
            if img[y, x] >= 255:
                break
            r += 0.25
        # Le centre du premier pixel à 255, moins un demi-pas : le bord à un demi-pixel près.
        angle = math.degrees(math.atan((r - 0.125) / f))
        angles.append(angle)
    pire = max(abs(x - attendu) for x in angles)
    print(f"SNELL_FRESNEL_S365 image={chemin} champ={champ} indice={n} attendu={attendu:.3f} "
          f"moyenne={sum(angles) / len(angles):.3f} min={min(angles):.3f} max={max(angles):.3f} pire_ecart={pire:.3f} "
          f"degres_par_pixel={math.degrees(math.cos(math.radians(attendu)) ** 2 / f):.3f}")
    return 0


def main(argv):
    if argv[1] == "--fresnel":
        return fresnel(argv[2], float(argv[3]), float(argv[4]) if len(argv) > 4 else 1.34)
    chemin, champ = argv[1], float(argv[2])
    n = float(argv[3]) if len(argv) > 3 else 1.34
    lum = lineaire(np.asarray(Image.open(chemin).convert("RGB"), dtype=np.float64)) @ np.array([0.2126, 0.7152, 0.0722])
    h, w = lum.shape
    cx, cy = w / 2.0, h / 2.0
    f = (h / 2.0) / math.tan(math.radians(champ) / 2.0)
    attendu = math.degrees(math.asin(1.0 / n))
    angles = []
    for k in range(8):
        a = k * math.pi / 4.0
        d = np.array([math.cos(a), math.sin(a)])
        # Échantillons au pas d'un demi-pixel, interpolation bilinéaire.
        rs = np.arange(0.0, min(cx, cy) - 2.0, 0.5)
        xs, ys = cx + rs * d[0] - 0.5, cy + rs * d[1] - 0.5
        x0, y0 = np.floor(xs).astype(int), np.floor(ys).astype(int)
        fx, fy = xs - x0, ys - y0
        v = (lum[y0, x0] * (1 - fx) * (1 - fy) + lum[y0, x0 + 1] * fx * (1 - fy) + lum[y0 + 1, x0] * (1 - fx) * fy
             + lum[y0 + 1, x0 + 1] * fx * fy)
        der = np.diff(v)
        i = int(np.argmin(der))
        # Parabole sur la dérivée autour de son minimum.
        if 0 < i < len(der) - 1:
            a0, a1, a2 = der[i - 1], der[i], der[i + 1]
            dec = 0.5 * (a0 - a2) / (a0 - 2 * a1 + a2) if (a0 - 2 * a1 + a2) != 0 else 0.0
        else:
            dec = 0.0
        r = (rs[i] + rs[i + 1]) / 2.0 + dec * 0.5
        angle = math.degrees(math.atan(r / f))
        angles.append(angle)
        print(f"SNELL_S365 direction={k * 45}deg rayon_px={r:.2f} angle={angle:.3f} chute={-der[i]:.4f}")
    moy = sum(angles) / len(angles)
    pire = max(abs(x - attendu) for x in angles)
    print(f"SNELL_S365 image={chemin} champ={champ} indice={n} attendu={attendu:.3f} moyenne={moy:.3f} "
          f"pire_ecart={pire:.3f} degres pixel={math.degrees(math.cos(math.radians(attendu)) ** 2 / f):.3f}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
