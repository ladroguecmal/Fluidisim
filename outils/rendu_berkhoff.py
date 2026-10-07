#!/usr/bin/env python3
"""Le rendu d'atelier du haut-fond de Berkhoff — S663 (la séance visuelle ; R41).

Relit `calculs/s663_berkhoff.bin` (les trois modèles de S659–S662) et `calculs/s663_controle.csv` (le contrôle), puis rend, numpy et PIL :

- `<sortie>_carte.png` : l'amplitude vue de dessus (le modèle non linéaire), le haut-fond, la pente, les lignes de mesure ;
- `<sortie>_sections.png` : les quatre sections — les mesures (points) contre les trois modèles (lignes) ;
- `<sortie>_surface.gif` : la surface `η = a₀·Re(A·e^(i(ψ − ωt)))` sur une période, vue de dessus.

**Les mesures ont l'axe `y` inversé** : elles sont tracées à `−y`.

    python outils/rendu_berkhoff.py calculs/s663_berkhoff.bin calculs/s663_controle.csv calculs/s663
"""
import math
import struct
import sys

import numpy as np
from PIL import Image, ImageDraw, ImageFont

try:
    POLICE = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 13)
except OSError:
    POLICE = ImageFont.load_default()
# Une police avec les accents pour tous les dessins (la police par défaut de PIL ne les a pas).
ImageDraw.ImageDraw.font = POLICE

A0 = 0.0232


def lire(chemin):
    d = open(chemin, 'rb').read()
    nx, ny = struct.unpack_from('<2I', d, 0)
    x0, dx, y0, dy = struct.unpack_from('<4d', d, 8)
    o = 40
    psi = np.frombuffer(d, '<f8', nx, o)
    o += 8 * nx
    champs = []
    for _ in range(3):
        re = np.frombuffer(d, '<f4', nx * ny, o).reshape(nx, ny)
        o += 4 * nx * ny
        im = np.frombuffer(d, '<f4', nx * ny, o).reshape(nx, ny)
        o += 4 * nx * ny
        champs.append(re.astype(np.float64) + 1j * im.astype(np.float64))
    assert o == len(d)
    return nx, ny, x0, dx, y0, dy, psi, champs


def amplitude(champ, x0, dx, y0, dy, x, y):
    """Bilinéaire, comme `Champ::amplitude`."""
    sx, sy = (x - x0) / dx, (y - y0) / dy
    i, j = min(int(math.floor(sx)), champ.shape[0] - 2), min(int(math.floor(sy)), champ.shape[1] - 2)
    fx, fy = sx - i, sy - j
    m = np.abs(champ)
    return (1 - fx) * ((1 - fy) * m[i, j] + fy * m[i, j + 1]) + fx * ((1 - fy) * m[i + 1, j] + fy * m[i + 1, j + 1])


def berkhoff(x, y):
    c, s = math.cos(math.radians(20)), math.sin(math.radians(20))
    xr, yr = x * c - y * s, x * s + y * c
    z0 = (5.82 + xr) / 50 if xr >= -5.82 else 0
    zs = -0.3 + 0.5 * math.sqrt(max(0, 1 - (xr / 3.75) ** 2 - (yr / 5) ** 2)) if (xr / 3) ** 2 + (yr / 4) ** 2 <= 1 else 0
    return 0.45 - z0 - zs


def palette(v, vmax):
    """0 → bleu nuit, 1 → bleu, vmax → jaune puis blanc."""
    s = np.clip(v / vmax, 0, 1)
    pts = np.array([0, 0.4, 0.7, 1.0])
    cols = np.array([[10, 20, 60], [40, 110, 200], [250, 210, 60], [255, 255, 240]])
    return np.stack([np.interp(s, pts, cols[:, k]) for k in range(3)], -1).astype(np.uint8)


def carte(nx, ny, x0, dx, y0, dy, champ, sortie):
    amp = np.abs(champ)                      # [nx, ny]
    e = 2                                    # px par point de grille
    img_a = palette(amp.T[::-1, :], 2.5)     # y vers le haut
    img = Image.fromarray(img_a).resize((nx * e, ny * e), Image.NEAREST)
    dr = ImageDraw.Draw(img)
    px = lambda x: (x - x0) / dx * e
    py = lambda y: (ny - 1 - (y - y0) / dy) * e
    # Le contour du haut-fond (ellipse tournée) et une isobathe de la pente (le pied, x' = −5,82).
    c, s = math.cos(math.radians(20)), math.sin(math.radians(20))
    pts = [(3 * math.cos(t), 4 * math.sin(t)) for t in np.linspace(0, 2 * math.pi, 120)]
    dr.line([(px(xr * c + yr * s), py(-xr * s + yr * c)) for xr, yr in pts] + [(px(3 * c), py(-3 * s))], fill=(255, 255, 255), width=1)
    for yy in np.linspace(-10, 10, 2):
        pass
    pied = [(px(-5.82 * c + yr * s), py(5.82 * s + yr * c)) for yr in np.linspace(-15, 15, 2)]
    dr.line(pied, fill=(200, 200, 200), width=1)
    # Les lignes de mesure.
    for xs in (3, 5, 9):
        dr.line([(px(xs), py(-4.5)), (px(xs), py(4.5))], fill=(255, 80, 80), width=2)
        dr.text((px(xs) + 3, py(4.5)), f"section x = {xs} m", fill=(255, 120, 120))
    dr.line([(px(0), py(0)), (px(10), py(0))], fill=(255, 80, 80), width=2)
    dr.text((px(10) + 3, py(0) - 6), "section y = 0", fill=(255, 120, 120))
    dr.text((6, 6), "Amplitude de la houle / amplitude incidente (modèle non linéaire, S662). Houle venant de la gauche.", fill=(255, 255, 255))
    dr.text((6, 22), "Blanc : contour du haut-fond et pied de la pente 1:50 ; rouge : lignes de mesure de Berkhoff (1982). En bas à droite : effet de la paroi latérale du modèle.", fill=(255, 255, 255))
    leg = Image.fromarray(np.repeat(palette(np.linspace(0, 2.5, img.width), 2.5)[None, :, :], 14, axis=0))
    out = Image.new('RGB', (img.width, img.height + 30), (10, 20, 60))
    out.paste(img, (0, 0))
    out.paste(leg, (0, img.height + 2))
    ImageDraw.Draw(out).text((4, img.height + 16), "rapport d'amplitude : 0 (bleu nuit) — 1 (bleu) — 1,75 (jaune) — 2,5 (blanc)", fill=(255, 255, 255))
    out.save(sortie)


def sections(nx, ny, x0, dx, y0, dy, champs, mesures, sortie):
    W, H, marge = 520, 260, 40
    noms = [("2", "section 2 : x = 3 m", 3.0), ("3", "section 3 : x = 5 m", 5.0), ("5", "section 5 : x = 9 m", 9.0), ("7", "section 7 : y = 0", None)]
    couleurs = [(150, 150, 150), (80, 160, 255), (255, 170, 40)]
    etiquettes = ["petits angles (S659)", "grand angle (S660)", "non linéaire (S662)"]
    out = Image.new('RGB', (2 * W, 2 * H + 30), (250, 250, 250))
    for k, (cle, titre, xs) in enumerate(noms):
        ox, oy = (k % 2) * W, (k // 2) * H
        dr = ImageDraw.Draw(out)
        u0, u1 = (-5, 5) if xs is not None else (0, 10)
        X = lambda u: ox + marge + (u - u0) / (u1 - u0) * (W - 2 * marge)
        Y = lambda v: oy + H - marge - v / 2.6 * (H - 2 * marge)
        dr.rectangle([X(u0), Y(2.6), X(u1), Y(0)], outline=(0, 0, 0))
        for v in (0.5, 1, 1.5, 2, 2.5):
            dr.line([X(u0), Y(v), X(u1), Y(v)], fill=(225, 225, 225))
            dr.text((X(u0) - 26, Y(v) - 6), f"{v:g}", fill=(0, 0, 0))
        dr.text((X(u0), oy + 6), titre + ("  (abscisse : y, m)" if xs is not None else "  (abscisse : x, m)"), fill=(0, 0, 0))
        for c, champ in enumerate(champs):
            us = np.linspace(u0, u1, 200)
            pts = [(X(u), Y(amplitude(champ, x0, dx, y0, dy, xs, u) if xs is not None else amplitude(champ, x0, dx, y0, dy, u, 0.0))) for u in us]
            dr.line(pts, fill=couleurs[c], width=2)
        for (sec, px_, py_, mes, _) in mesures:
            if sec == cle:
                u = py_ if xs is not None else px_
                dr.ellipse([X(u) - 3, Y(mes) - 3, X(u) + 3, Y(mes) + 3], fill=(0, 0, 0))
    dr = ImageDraw.Draw(out)
    for c, e in enumerate(etiquettes):
        dr.line([20 + c * 230, 2 * H + 15, 50 + c * 230, 2 * H + 15], fill=couleurs[c], width=3)
        dr.text((56 + c * 230, 2 * H + 9), e, fill=(0, 0, 0))
    dr.ellipse([710, 2 * H + 12, 716, 2 * H + 18], fill=(0, 0, 0))
    dr.text((722, 2 * H + 9), "mesures de Berkhoff (1982)", fill=(0, 0, 0))
    out.save(sortie)


def surface(nx, ny, x0, dx, y0, dy, psi, champ, sortie, images=20):
    imgs = []
    for n in range(images):
        t = n / images
        eta = A0 * np.real(champ * np.exp(1j * (psi[:, None] - 2 * math.pi * t)))
        v = np.clip(eta / (2.4 * A0), -1, 1).T[::-1, :]
        a = np.stack([30 + 90 * (v + 1) / 2, 70 + 140 * (v + 1) / 2, 120 + 130 * (v + 1) / 2], -1).astype(np.uint8)
        img = Image.fromarray(a).resize((nx * 2, ny * 2), Image.BILINEAR)
        ImageDraw.Draw(img).text((6, 6), f"surface de la houle (modèle non linéaire), t = {t:.2f} T", fill=(255, 255, 255))
        imgs.append(img.convert('P', palette=Image.ADAPTIVE, colors=128))
    imgs[0].save(sortie, save_all=True, append_images=imgs[1:], duration=60, loop=0, optimize=True)


def main(argv):
    nx, ny, x0, dx, y0, dy, psi, champs = lire(argv[0])
    mesures = []
    pire = 0.0
    for ligne in open(argv[1], encoding='utf-8'):
        sec, px_, py_, mes, amp = ligne.strip().split(',')
        px_, py_, mes, amp = float(px_), float(py_), float(mes), float(amp)
        pire = max(pire, abs(amplitude(champs[2], x0, dx, y0, dy, px_, py_) - amp))
        mesures.append((sec, px_, py_, mes, amp))
    print(f"contrôle 1 : l'amplitude relue contre Champ::amplitude, écart au plus {pire:.2e} (f32 écrit)")
    for sec in ("2", "3", "5", "7"):
        m = [(mes, amp) for s_, _, _, mes, amp in mesures if s_ == sec]
        e = math.sqrt(sum((a - b) ** 2 for b, a in m) / len(m))
        print(f"contrôle 2 : section {sec}, écart recalculé {e:.3f}")
    # Les mesures sont tracées à −y : la position lue par le contrôle est déjà (x, −y_mesure).
    carte(nx, ny, x0, dx, y0, dy, champs[2], argv[2] + '_carte.png')
    sections(nx, ny, x0, dx, y0, dy, champs, mesures, argv[2] + '_sections.png')
    surface(nx, ny, x0, dx, y0, dy, psi, champs[2], argv[2] + '_surface.gif')
    print("écrit", argv[2] + '_carte.png', argv[2] + '_sections.png', argv[2] + '_surface.gif')


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
