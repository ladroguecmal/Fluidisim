#!/usr/bin/env python3
"""La crête d'un film de la 3D — S733 (SELECTEUR-DOMAINES-S732, P1 : la crête du témoin contre celle du prédicteur).

Relit un film de `deux_raccords_porteur` (le format de S720, `outils/rendu_bout_en_bout.py`) et rend, tous les 0,1 s, la plus haute
particule de la première rangée, sa place, et son élévation au-dessus du niveau (plus `dx/4` : les particules sont posées au quart de maille
sous la surface, S684). **Une lecture ponctuelle** (ADR-280 D1) : rapportée, jamais un critère.

    python outils/crete_film.py calculs/s730_tout3d_12.bin [niveau=0.5] [t_max=2.9]
"""
import sys

import numpy as np

sys.stdout.reconfigure(encoding='utf-8', errors='replace')

from rendu_bout_en_bout import lire, DX


def main(argv):
    images = lire(argv[0])
    niveau = float(argv[1]) if len(argv) > 1 else 0.5
    t_max = float(argv[2]) if len(argv) > 2 else 2.9
    for t in np.arange(0.0, t_max + 1e-9, 0.1):
        im = min(images, key=lambda i: abs(i[0] - t))
        p = im[3]
        if not len(p):
            continue
        k = int(np.argmax(p[:, 1]))
        print(f"t = {im[0]:.2f} s : x = {p[k, 0]:.3f} m, η = {(p[k, 1] + DX / 4 - niveau) * 1e3:.1f} mm")
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
