"""S363 — la couture verticale du ciel : le saut de luminance entre deux colonnes voisines, rapporté aux sauts voisins.

Usage : python outils/couture_ciel.py <image.png> [colonne] [...]

Sur la bande du ciel (des rangées 5 à 150), la différence absolue moyenne de luminance linéaire (Rec. 709, sRGB décodé)
entre les colonnes `c` et `c + 1`. La couture de Godot tombait au centre, entre 639 et 640 sur une image de 1 280
colonnes. Le rapport à la médiane des sauts de 560 à 720 dit si ce saut se distingue du grain du ciel : un ciel sans
couture le garde près de 1.
"""
import sys

import numpy as np
from PIL import Image


def lineaire(c):
    c = c / 255.0
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


for chemin in sys.argv[1:]:
    lum = lineaire(np.asarray(Image.open(chemin).convert("RGB"), dtype=np.float64)) @ np.array([0.2126, 0.7152, 0.0722])
    bande = lum[5:150]
    sauts = np.abs(bande[:, 1:] - bande[:, :-1]).mean(axis=0)
    centre = bande.shape[1] // 2 - 1
    voisins = np.median(sauts[560:720])
    print(f"COUTURE_S363 image={chemin} saut_centre={sauts[centre]:.5f} mediane_voisins={voisins:.5f} "
          f"rapport={sauts[centre] / voisins:.2f} plus_grand_saut_colonne={int(np.argmax(sauts))}")
