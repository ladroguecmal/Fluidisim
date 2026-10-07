#!/usr/bin/env python3
"""Le rendu d'atelier du rouleau — S658 (la séance visuelle demandée par l'utilisateur ; R40).

Relit l'enregistrement de `record_the_roller_for_the_visual_session_s658` (`calculs/s658_rouleau.bin`) et rend deux animations GIF,
numpy et PIL seulement :

- **la plage**, vue de côté : Saint-Venant 2D au large (sa surface), APIC 3D sur la plage (les particules, colorées par leur vitesse),
  le fond en escalier, la sphère libre ;
- **le rouleau**, de près.

Toute la largeur du canal est projetée sur le plan de la vue : le corps masque l'eau derrière lui. La couleur dit la vitesse, bornée à
`VMAX` m/s (le jet plus rapide est blanc).

    python outils/rendu_rouleau.py calculs/s658_rouleau.bin calculs/s658   # écrit calculs/s658_plage.gif, calculs/s658_rouleau.gif
    python outils/rendu_rouleau.py --controle calculs/s658_rouleau.bin "<trajet t x z>"…   # l'instrument relu (ADR-266)
"""
import struct
import sys

import numpy as np
from PIL import Image, ImageDraw

DX, X_PIED, COT, NIVEAU, X_R = 0.05, 5.696, 12.0, 0.5, 5.0
VMAX = 3.0


def lire(chemin):
    """Les images de l'enregistrement : (t, x_r, surface de Saint-Venant, surface des colonnes, (xs, zs, r), particules [n, 4])."""
    d = open(chemin, 'rb').read()
    o, images = 0, []
    while o < len(d):
        t, n, x_r, n_sv, n_col = struct.unpack_from('<5f', d, o)
        o += 20
        n, n_sv, n_col = int(n), int(n_sv), int(n_col)
        sv = np.frombuffer(d, '<f4', n_sv, o)
        o += 4 * n_sv
        col = np.frombuffer(d, '<f4', n_col, o)
        o += 4 * n_col
        corps = struct.unpack_from('<3f', d, o)
        o += 12
        part = np.frombuffer(d, '<f4', 4 * n, o).reshape(n, 4)
        o += 16 * n
        images.append((t, x_r, sv, col, corps, part))
    assert o == len(d), "l'enregistrement se termine au milieu d'une image"
    return images


def fond_escalier(x):
    """Le fond d'APIC : chaque colonne arrondie aux mailles dont le centre est sous le fond (S639), le repère de la 3D (plat à 0)."""
    i = np.floor((x - X_R) / DX)
    xc = X_R + (i + 0.5) * DX
    zb = np.maximum(xc - X_PIED, 0) / COT
    # le nombre de mailles dont le centre est sous le fond : (k + ½)·dx < zb
    return np.ceil(zb / DX - 0.5) * DX


def couleur(v):
    """Bleu profond (0) → cyan (1/3 VMAX) → blanc (VMAX)."""
    s = np.clip(v / VMAX, 0, 1)[:, None]
    bas, mil, haut = np.array([20, 60, 140]), np.array([40, 190, 230]), np.array([250, 250, 255])
    return np.where(s < 1 / 3, bas + (mil - bas) * (s * 3), mil + (haut - mil) * ((s - 1 / 3) * 1.5)).astype(np.uint8)


def rendre(image, cadre, echelle):
    """Une image : `cadre` = (x0, x1, z0, z1) m ; `echelle` px/m."""
    t, x_r, sv, col, (xs, zs, rs), part = image
    x0, x1, z0, z1 = cadre
    w, h = int(round((x1 - x0) * echelle)), int(round((z1 - z0) * echelle))
    px = lambda x: (x - x0) * echelle
    pz = lambda z: h - (z - z0) * echelle
    ciel = np.linspace([12, 18, 32], [40, 52, 78], h).astype(np.uint8)
    a = np.repeat(ciel[:, None, :], w, axis=1).copy()
    # Saint-Venant au large : la colonne d'eau sous sa surface, jusqu'à x_r.
    xs_sv = (np.arange(len(sv)) + 0.5) * DX
    for i, x in enumerate(xs_sv):
        if x0 <= x < min(x1, x_r):
            c0, c1 = int(px(x - DX / 2)), int(px(x + DX / 2)) + 1
            haut = int(np.clip(pz(sv[i]), 0, h))
            a[haut:, max(c0, 0):min(c1, w)] = (30, 80, 150)
    # La zone des colonnes de la 3D : la colonne d'eau sous sa surface `η`, de x_r à x_r + n_col·dx.
    for i, eta in enumerate(col):
        x = x_r + (i + 0.5) * DX
        if x0 <= x < x1:
            c0, c1 = int(px(x - DX / 2)), int(px(x + DX / 2)) + 1
            haut = int(np.clip(pz(eta), 0, h))
            a[haut:, max(c0, 0):min(c1, w)] = (30, 80, 150)
    # Les particules d'APIC : la vitesse moyenne par pixel (taches de la demi-maille), toute la largeur projetée.
    sel = (part[:, 0] >= x0) & (part[:, 0] < x1) & (part[:, 2] >= z0) & (part[:, 2] < z1)
    p = part[sel]
    ci = np.clip(px(p[:, 0]).astype(int), 0, w - 2)
    cj = np.clip(pz(p[:, 2]).astype(int), 0, h - 2)
    somme, compte = np.zeros((h, w)), np.zeros((h, w))
    tache = max(2, int(round(0.5 * DX * echelle)))
    ci, cj = np.clip(ci, 0, w - tache), np.clip(cj, 0, h - tache)
    for di in range(tache):
        for dj in range(tache):
            np.add.at(somme, (cj + dj, ci + di), p[:, 3])
            np.add.at(compte, (cj + dj, ci + di), 1)
    m = compte > 0
    a[m] = couleur(somme[m] / compte[m])
    img = Image.fromarray(a)
    dr = ImageDraw.Draw(img)
    # Le fond : plat, puis l'escalier de la 3D.
    xg = np.arange(x0, x1, DX / 4)
    zb = np.where(xg < x_r, 0.0, fond_escalier(xg))
    pts = [(px(x), pz(z)) for x, z in zip(xg, zb)] + [(w, h), (0, h)]
    dr.polygon(pts, fill=(170, 140, 90))
    # La frontière du relais.
    if x0 < x_r < x1:
        for y in range(0, h, 8):
            dr.line([(px(x_r), y), (px(x_r), y + 4)], fill=(255, 220, 120))
        dr.text((px(x_r) - 118, 4), "Saint-Venant 2D", fill=(255, 220, 120))
        dr.text((px(x_r) + 6, 4), "APIC 3D", fill=(255, 220, 120))
    # La sphère libre.
    if rs > 0:
        r = rs * echelle
        dr.ellipse([px(xs) - r, pz(zs) - r, px(xs) + r, pz(zs) + r], outline=(255, 140, 40), width=2)
    dr.text((6, h - 14), f"t = {t:4.2f} s", fill=(255, 255, 255))
    return img


def legende(w):
    """Une bande de couleurs : 0 à VMAX m/s."""
    v = np.linspace(0, VMAX, w)
    bande = np.repeat(couleur(v)[None, :, :], 10, axis=0)
    img = Image.new('RGB', (w, 24), (12, 18, 32))
    img.paste(Image.fromarray(bande), (0, 0))
    ImageDraw.Draw(img).text((2, 11), f"vitesse : de 0 (bleu) a {VMAX:g} m/s (blanc) ; la zone des colonnes et Saint-Venant en bleu uni", fill=(255, 255, 255))
    return img


def animer(images, cadre, echelle, sortie):
    imgs = []
    for im in images:
        a = rendre(im, cadre, echelle)
        l = legende(a.width)
        c = Image.new('RGB', (a.width, a.height + l.height))
        c.paste(a, (0, 0))
        c.paste(l, (0, a.height))
        imgs.append(c.convert('P', palette=Image.ADAPTIVE, colors=128))
    imgs[0].save(sortie, save_all=True, append_images=imgs[1:], duration=80, loop=0, optimize=True)


def main(argv):
    if argv and argv[0] == '--controle':
        images = lire(argv[1])
        print(f"{len(images)} images ; t de {images[0][0]:.3f} à {images[-1][0]:.3f} s ; {len(images[-1][5])} particules à la fin")
        pire = 0.0
        for ligne in argv[2:]:
            t, x, z = map(float, ligne.split())
            im = min(images, key=lambda i: abs(i[0] - t))
            if abs(im[0] - t) < 1e-3:
                pire = max(pire, abs(im[4][0] - x), abs(im[4][1] - z))
        print(f"la sphère, enregistrement contre relevé : écart au plus {pire * 1e3:.3f} mm")
        return 0
    images = lire(argv[0])
    animer(images, (3.0, 12.8, 0.0, 1.0), 140, argv[1] + '_plage.gif')
    animer(images, (8.8, 12.6, 0.15, 1.0), 300, argv[1] + '_rouleau.gif')
    print("écrit", argv[1] + '_plage.gif', argv[1] + '_rouleau.gif')
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
