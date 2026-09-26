"""S383 — contrôle des gerbes de la pluie (critères 3 à 5 de S383, ADR-205 pièce 4).

Lit le journal de `godot --path godot res://piscine.tscn -- --controle-gerbes` (argument) et les images
`godot/captures/controle_gerbes_*.png` qu'il a écrites : vue d'aplomb, 2 mm par pixel ; en rouge, les cœurs d'anneaux de
moins de 30 ms (disques d'1 cm de rayon, contrôle des rides de S379) ; en vert, un carré d'1 cm au centre de chaque gerbe
de moins de 30 ms.

- Critère 3 : chaque carré vert au centre d'un disque rouge (centroïdes à moins d'1 mm), et chaque disque rouge porte un
  carré vert — les gerbes naissent aux impacts des rides, à leur instant.
- Critère 4 : le nombre de carrés verts contre `taux × 0,03 s × aire × images` (à ±5 %, ou à deux écarts-types de Poisson).
- Critère 5 : le sommet de la gerbe de référence rendue, contre le plus haut des relevés de P2 sur les quatre instants de la
  pose (la silhouette y est moyennée) ; tolérance : un pixel de la figure source, 0,59 mm.

    python outils/controle_gerbes.py journal.txt
"""
import glob
import math
import re
import sys

import numpy as np
from PIL import Image

T = [0, 1, 3, 7, 12, 18, 41, 52, 80]
H = [0, 5.9, 11.2, 17.1, 19.4, 24.7, 23.5, 21.8, 0]
POSE_MS = 1000.0 / 60.0


def taches(masque):
    """Composantes 4-connexes d'un masque booléen : listes de (y, x)."""
    vus = np.zeros_like(masque)
    ys, xs = np.nonzero(masque)
    out = []
    for y0, x0 in zip(ys, xs):
        if vus[y0, x0]:
            continue
        pile = [(y0, x0)]
        vus[y0, x0] = True
        comp = []
        while pile:
            y, x = pile.pop()
            comp.append((y, x))
            for dy, dx in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                yy, xx = y + dy, x + dx
                if 0 <= yy < masque.shape[0] and 0 <= xx < masque.shape[1] and masque[yy, xx] and not vus[yy, xx]:
                    vus[yy, xx] = True
                    pile.append((yy, xx))
        out.append(comp)
    return out


def main():
    journal = open(sys.argv[1], encoding="utf-8", errors="replace").read()
    m = re.search(r"CONTROLE_GERBES_S383 pluie_mm_h=(\S+) taux=([\d.]+) aire_m2=([\d.]+) m_par_px=([\d.]+)", journal)
    taux, aire, mpp = float(m.group(2)), float(m.group(3)), float(m.group(4))
    images = sorted(glob.glob("godot/captures/controle_gerbes_*.png"))
    verts_total = 0
    rouges_sans_vert = 0
    au_bord = 0
    pire = 0.0
    for chemin in images:
        a = np.asarray(Image.open(chemin).convert("RGB")).astype(int)
        # Le vert se lit à son canal (les bords du carré, mêlés au rouge par l'anticrénelage, sont olive) ; le disque, à la
        # somme des deux canaux.
        vert = a[:, :, 1] > 60
        disques = taches((a[:, :, 0] + a[:, :, 1]) > 128)
        for d in disques:
            pts = np.array(d)
            # Un disque coupé par le bord de l'image : son centre peut être hors champ et son centroïde est faussé.
            if pts[:, 0].min() == 0 or pts[:, 1].min() == 0 or pts[:, 0].max() == a.shape[0] - 1 or pts[:, 1].max() == a.shape[1] - 1:
                au_bord += 1
                if vert[pts[:, 0], pts[:, 1]].any():
                    verts_total += 1
                continue
            gv = a[pts[:, 0], pts[:, 1], 1].astype(float)
            dedans = vert[pts[:, 0], pts[:, 1]]
            if not dedans.any():
                rouges_sans_vert += 1
                continue
            # Plusieurs carrés dans un disque : des impacts qui se touchent — comptés à part, par leurs taches vertes.
            sous = [g for g in taches(np.pad(vert, 0) & np.isin(np.arange(vert.size).reshape(vert.shape), pts[:, 0] * vert.shape[1] + pts[:, 1]))]
            if len(sous) > 1:
                verts_total += len(sous)
                continue
            verts_total += 1
            cg = np.average(pts, axis=0, weights=gv)
            cd = np.mean(pts, axis=0)
            pire = max(pire, float(np.hypot(*(cg - cd))) * mpp)
    attendu = taux * 0.03 * aire * len(images)
    ecart = (verts_total - attendu) / attendu
    sigma = 1.0 / math.sqrt(attendu)
    tenu3 = pire <= 0.001 and rouges_sans_vert == 0
    tenu4 = abs(ecart) <= max(0.05, 2.0 * sigma)
    print(f"critère 3 : {verts_total} gerbes ; écart de centre le plus grand {1000 * pire:.2f} mm ; disques sans gerbe {rouges_sans_vert} ; au bord de l'image, écartés {au_bord} : {'tenu' if tenu3 else 'manqué'}")
    print(f"critère 4 : {verts_total} comptées, {attendu:.1f} attendues ({len(images)} images, {aire:.3f} m², taux {taux:.1f}) ; écart {100 * ecart:+.2f} % (1 σ de Poisson {100 * sigma:.2f} %) : {'tenu' if tenu4 else 'manqué'}")
    pire5 = 0.0
    for t_ms, sommet in re.findall(r"critere=5 t_ms=([\d.]+) sommet_mm=([\d.]+)", journal):
        t, s = float(t_ms), float(sommet)
        attendu5 = max(float(np.interp(t - (i + 0.5) * 0.25 * POSE_MS, T, H)) for i in range(4))
        pire5 = max(pire5, abs(s - attendu5))
        print(f"critère 5 : t = {t:4.0f} ms  sommet rendu {s:5.2f} mm  relevés sur la pose {attendu5:5.2f} mm  écart {s - attendu5:+.2f}")
    print(f"critère 5 : pire écart {pire5:.2f} mm, tolérance 0,59 : {'tenu' if pire5 <= 0.59 else 'manqué'}")


if __name__ == "__main__":
    main()
