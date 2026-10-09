#!/usr/bin/env python3
"""Le rendu de la séance visuelle R43 — S720 : la vague de bout en bout contre le tout-3D.

Relit les films de `record_the_end_to_end_wave_for_the_visual_session_s720` (`calculs/s720_tout3d.bin`, `calculs/s720_bout.bin`) et rend,
numpy et PIL seulement (d'après `rendu_rouleau.py`, S658) :

- `<sortie>_plage.gif` : la plage entière vue de côté, le tout-3D en haut, de bout en bout en bas, à la même heure ;
- `<sortie>_deferlement.gif` : le déferlement de près, de même ;
- `<sortie>_instants.png` : quatre instants, pour l'aperçu.

La 2D (SGN au large, Saint-Venant au rivage, puis la plage entière après la mort de la 3D) est en bleu uni sous sa surface ; les
particules de la 3D (une rangée sur quatre) sont colorées par leur vitesse.

    python outils/rendu_bout_en_bout.py calculs/s720_tout3d.bin calculs/s720_bout.bin calculs/s720

S730 (ADR-284) : deux titres en option (le film du haut, celui du bas), et `<sortie>_jet.png`, le jet de près à quatre instants :

    python outils/rendu_bout_en_bout.py calculs/s720_bout.bin calculs/s730_bout_12.bin calculs/s730 "Avant" "Après"
"""
import struct
import sys

import numpy as np
from PIL import Image, ImageDraw, ImageFont

try:
    POLICE = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 13)
except OSError:
    POLICE = ImageFont.load_default()
ImageDraw.ImageDraw.font = POLICE

DX, X_PIED, COT = 0.025, 5.696, 12.0
VMAX = 3.0


def lire(chemin):
    """Les images : (t, surface de la 2D, 3D vivante, particules [n, 3])."""
    d = open(chemin, 'rb').read()
    o, images = 0, []
    while o < len(d):
        t, n, ns, vivante = struct.unpack_from('<4f', d, o)
        o += 16
        n, ns = int(n), int(ns)
        surf = np.frombuffer(d, '<f4', ns, o)
        o += 4 * ns
        part = np.frombuffer(d, '<f4', 3 * n, o).reshape(n, 3)
        o += 12 * n
        images.append((t, surf, vivante > 0.5, part))
    assert o == len(d), "le film se termine au milieu d'une image"
    return images


def couleur(v):
    """Bleu profond (0) → cyan (1/3 VMAX) → blanc (VMAX)."""
    s = np.clip(v / VMAX, 0, 1)[:, None]
    bas, mil, haut = np.array([20, 60, 140]), np.array([40, 190, 230]), np.array([250, 250, 255])
    return np.where(s < 1 / 3, bas + (mil - bas) * (s * 3), mil + (haut - mil) * ((s - 1 / 3) * 1.5)).astype(np.uint8)


def rendre(image, cadre, echelle, titre):
    t, surf, vivante, part = image
    x0, x1, z0, z1 = cadre
    w, h = int(round((x1 - x0) * echelle)), int(round((z1 - z0) * echelle))
    px = lambda x: (x - x0) * echelle
    pz = lambda z: h - (z - z0) * echelle
    ciel = np.linspace([12, 18, 32], [40, 52, 78], h).astype(np.uint8)
    a = np.repeat(ciel[:, None, :], w, axis=1).copy()
    # La 2D : la colonne d'eau sous sa surface, là où elle est active.
    for i, e in enumerate(surf):
        x = (i + 0.5) * DX
        if np.isfinite(e) and x0 <= x < x1:
            c0, c1 = int(px(x - DX / 2)), int(px(x + DX / 2)) + 1
            haut = int(np.clip(pz(e), 0, h))
            a[haut:, max(c0, 0):min(c1, w)] = (30, 80, 150)
    # Les particules de la 3D : la vitesse moyenne par tache.
    if len(part):
        sel = (part[:, 0] >= x0) & (part[:, 0] < x1) & (part[:, 1] >= z0) & (part[:, 1] < z1)
        p = part[sel]
        tache = max(2, int(round(0.5 * DX * echelle)))
        ci = np.clip(px(p[:, 0]).astype(int), 0, w - tache)
        cj = np.clip(pz(p[:, 1]).astype(int), 0, h - tache)
        somme, compte = np.zeros((h, w)), np.zeros((h, w))
        for di in range(tache):
            for dj in range(tache):
                np.add.at(somme, (cj + dj, ci + di), p[:, 2])
                np.add.at(compte, (cj + dj, ci + di), 1)
        m = compte > 0
        a[m] = couleur(somme[m] / compte[m])
    img = Image.fromarray(a)
    dr = ImageDraw.Draw(img)
    xg = np.arange(x0, x1, DX / 4)
    zb = np.maximum(xg - X_PIED, 0) / COT
    dr.polygon([(px(x), pz(z)) for x, z in zip(xg, zb)] + [(w, h), (0, h)], fill=(170, 140, 90))
    etat = "3D active" if vivante else "3D éteinte : Saint-Venant seul"
    dr.text((6, 4), f"{titre} — {etat}", fill=(255, 220, 120))
    dr.text((6, h - 14), f"t = {t:4.2f} s", fill=(255, 255, 255))
    return img


def legende(w):
    v = np.linspace(0, VMAX, w)
    bande = np.repeat(couleur(v)[None, :, :], 10, axis=0)
    img = Image.new('RGB', (w, 30), (12, 18, 32))
    img.paste(Image.fromarray(bande), (0, 0))
    ImageDraw.Draw(img).text((2, 11), f"particules 3D : vitesse de 0 (bleu) à {VMAX:g} m/s (blanc) ; bleu uni : l'eau en 2D (SGN, Saint-Venant)", fill=(255, 255, 255))
    return img


TITRES = ["Tout-3D (la référence)", "De bout en bout : SGN, bande 3D, Saint-Venant (4 fois moins de calcul)"]


def paire(haut, bas, cadre, echelle):
    a = rendre(haut, cadre, echelle, TITRES[0])
    b = rendre(bas, cadre, echelle, TITRES[1])
    l = legende(a.width)
    c = Image.new('RGB', (a.width, a.height + b.height + l.height + 4), (0, 0, 0))
    c.paste(a, (0, 0))
    c.paste(b, (0, a.height + 4))
    c.paste(l, (0, a.height + b.height + 4))
    return c


def apparier(tout, bout):
    """Les images de même heure (le film de bout en bout donne l'horloge)."""
    out = []
    for b in bout:
        a = min(tout, key=lambda i: abs(i[0] - b[0]))
        if abs(a[0] - b[0]) < 0.02:
            out.append((a, b))
    return out


def animer(paires, cadre, echelle, sortie):
    imgs = [paire(a, b, cadre, echelle).convert('P', palette=Image.ADAPTIVE, colors=128) for a, b in paires]
    imgs[0].save(sortie, save_all=True, append_images=imgs[1:], duration=66, loop=0, optimize=True)


def main(argv):
    if len(argv) >= 5:
        TITRES[:] = argv[3:5]
    tout, bout = lire(argv[0]), lire(argv[1])
    paires = apparier(tout, bout)
    print(f"{len(tout)} et {len(bout)} images ; {len(paires)} paires ; t de {paires[0][1][0]:.3f} à {paires[-1][1][0]:.3f} s")
    animer(paires, (0.0, 12.8, 0.0, 1.0), 100, argv[2] + '_plage.gif')
    animer([p for p in paires if 2.0 <= p[1][0] <= 4.6], (8.6, 12.6, 0.2, 1.0), 300, argv[2] + '_deferlement.gif')
    choix = [min(paires, key=lambda p: abs(p[1][0] - t)) for t in (1.0, 2.6, 3.0, 3.9)]
    vues = [paire(a, b, (6.0, 12.8, 0.0, 1.0), 150) for a, b in choix]
    planche = Image.new('RGB', (vues[0].width, sum(v.height for v in vues) + 6 * len(vues)), (0, 0, 0))
    y = 0
    for v in vues:
        planche.paste(v, (0, y))
        y += v.height + 6
    planche.save(argv[2] + '_instants.png')
    # S730 : le jet de près.
    choix = [min(paires, key=lambda p: abs(p[1][0] - t)) for t in (2.70, 2.80, 2.90, 3.05)]
    vues = [paire(a, b, (9.2, 12.4, 0.3, 0.9), 220) for a, b in choix]
    planche = Image.new('RGB', (vues[0].width, sum(v.height for v in vues) + 6 * len(vues)), (0, 0, 0))
    y = 0
    for v in vues:
        planche.paste(v, (0, y))
        y += v.height + 6
    planche.save(argv[2] + '_jet.png')
    print("écrit", argv[2] + '_plage.gif', argv[2] + '_deferlement.gif', argv[2] + '_instants.png', argv[2] + '_jet.png')
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
