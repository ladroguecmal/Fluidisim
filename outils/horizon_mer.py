"""S359 — la luminance de la mer sous l'horizon, rapportée à celle du ciel au-dessus.

Usage : python outils/horizon_mer.py <image.png> [...]

L'horizon est la rangée où la luminance moyenne chute le plus d'une rangée à la suivante. On décode le sRGB en linéaire,
on prend la luminance Rec. 709, et on moyenne toute la largeur sur `BANDE` rangées au-dessus et au-dessous, en laissant
`MARGE` rangées de chaque côté de la ligne. Le rapport mer/ciel est la grandeur : une mer réelle, au rasant, renvoie
l'essentiel du ciel qui la surplombe (Fresnel), et s'éclaircit vers l'horizon.
"""
import sys

import numpy as np
from PIL import Image

BANDE = 20
MARGE = 3


def lineaire(c):
    c = c / 255.0
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


def mesure(chemin):
    rgb = lineaire(np.asarray(Image.open(chemin).convert("RGB"), dtype=np.float64))
    lum = rgb @ np.array([0.2126, 0.7152, 0.0722])
    rangs = lum.mean(axis=1)
    h = int(np.argmin(rangs[1:] - rangs[:-1])) + 1
    ciel = rangs[h - MARGE - BANDE:h - MARGE].mean()
    mer = rangs[h + MARGE:h + MARGE + BANDE].mean()
    loin = rangs[h + MARGE:h + MARGE + 200].reshape(-1, 20).mean(axis=1)
    print(f"HORIZON_S359 image={chemin} rang={h} ciel={ciel:.4f} mer={mer:.4f} rapport={mer / ciel:.3f} "
          f"profil_sous_horizon={' '.join(f'{x:.4f}' for x in loin)}")


for chemin in sys.argv[1:]:
    mesure(chemin)
