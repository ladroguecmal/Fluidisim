"""S360 — l'écume vue au nadir : couverture et taille des taches.

Usage : python outils/ecume_taches.py <hauteur_m> <masque.png> [...]

Chaque masque est une capture au nadir du mode contrôle 3 de `godot/eau.gdshader` (la couverture d'écume en sortie
directe, tonalité linéaire). Un pixel est de l'écume au-dessus de 0,5. Taille au sol d'un pixel au centre :
`2·H·tan(fov/2)/hauteur_image`, champ vertical de 50° (celui de `mer.gd`) ; la mer ondule de quelques mètres sous une
caméra à 12 ou 40 m : l'échelle est approchée à 10 à 20 % près. Taches : composantes 4-connexes ; diamètre équivalent
`2·√(A/π)`.
"""
import math
import sys

import numpy as np
from PIL import Image


def taches(masque):
    hauteur, largeur = masque.shape
    vu = np.zeros_like(masque, dtype=bool)
    tailles = []
    for y0, x0 in zip(*np.nonzero(masque)):
        if vu[y0, x0]:
            continue
        pile = [(y0, x0)]
        vu[y0, x0] = True
        n = 0
        while pile:
            y, x = pile.pop()
            n += 1
            for dy, dx in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                v, u = y + dy, x + dx
                if 0 <= v < hauteur and 0 <= u < largeur and masque[v, u] and not vu[v, u]:
                    vu[v, u] = True
                    pile.append((v, u))
        tailles.append(n)
    return tailles


def main():
    h = float(sys.argv[1])
    toutes, couvertes, total = [], 0, 0
    for chemin in sys.argv[2:]:
        m = np.asarray(Image.open(chemin).convert("L"), dtype=np.float64) / 255.0 > 0.5
        pixel = 2.0 * h * math.tan(math.radians(25.0)) / m.shape[0]
        couvertes += int(m.sum())
        total += m.size
        toutes += [2.0 * math.sqrt(n * pixel * pixel / math.pi) for n in taches(m)]
    d = np.array(sorted(toutes)) if toutes else np.array([0.0])
    print(f"ECUME_TACHES_S360 hauteur_m={h} images={len(sys.argv) - 2} couverture={couvertes / total:.5f} "
          f"taches={len(toutes)} diametre_median_m={np.median(d):.3f} diametre_min_m={d.min():.3f} "
          f"diametre_max_m={d.max():.3f} part_sous_0_5m={np.mean(d < 0.5):.3f}")


main()
