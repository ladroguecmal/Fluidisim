#!/usr/bin/env python3
"""Mesure la structure spatiale d'une capture PPM — S306.

Pourquoi cet outil. Le guide reçu (§1, §10.3) place les normales fines et l'environnement
lumineux avant la topologie dans les causes d'un rendu trop strié. Pour trancher, il faut une
mesure de la **structure haute fréquence** de l'image, pas de son contraste global : un
écart-type de luma ne distingue pas une grande masse d'eau d'un tapis de stries.

Ce que l'outil publie, par image :

- `luma_moy`, `luma_et`   : moyenne et écart-type de luma sur les pixels d'eau.
- `hf_rms`               : RMS de la différence à un pixel (passe-haut à la fréquence de
                           Nyquist de l'image). C'est l'énergie des stries.
- `hf_part`              : `hf_rms / luma_et` — la part de la structure qui est haute
                           fréquence. C'est **le** nombre qui sépare « grandes masses » de
                           « tapis de stries », parce qu'il est insensible à l'exposition.
- `anisotropie`          : `RMS(dy) / RMS(dx)`. Des stries parallèles à l'horizon donnent un
                           rapport franchement supérieur à 1.
- `p99_hf`               : 99e centile de |gradient|, pour voir les stries les plus vives sans
                           les noyer dans la moyenne.

Les pixels de ciel sont exclus quand l'image est un diagnostic : la couleur du coin haut-gauche
sert de témoin, plutôt que de supposer une valeur (la cible est en sRGB).

Python standard, sans réseau, sans dépendance.

    python outils/spectre_image.py viewer/captures/s306/*.ppm
"""

import sys
import glob
import math


def lire_ppm(chemin):
    with open(chemin, "rb") as f:
        donnees = f.read()
    if not donnees.startswith(b"P6"):
        raise ValueError(f"{chemin} : PPM binaire attendu")
    champs, i = [], 2
    while len(champs) < 3:
        while i < len(donnees) and donnees[i : i + 1].isspace():
            i += 1
        if donnees[i : i + 1] == b"#":
            while donnees[i : i + 1] not in (b"\n", b""):
                i += 1
            continue
        j = i
        while j < len(donnees) and not donnees[j : j + 1].isspace():
            j += 1
        champs.append(int(donnees[i:j]))
        i = j
    largeur, hauteur, maxi = champs
    if maxi != 255:
        raise ValueError(f"{chemin} : 255 attendu, {maxi} lu")
    return largeur, hauteur, donnees[i + 1 :]


def luma(pixels, n):
    """Luma Rec.709 par pixel, sur les valeurs encodées (ce que l'œil voit à l'écran)."""
    out = [0.0] * n
    for k in range(n):
        o = 3 * k
        out[k] = 0.2126 * pixels[o] + 0.7152 * pixels[o + 1] + 0.0722 * pixels[o + 2]
    return out


def mesurer(chemin):
    largeur, hauteur, pixels = lire_ppm(chemin)
    n = largeur * hauteur
    ciel = (pixels[0], pixels[1], pixels[2])
    # Une image de diagnostic a un ciel d'une seule couleur ; un rendu, non. On exclut les
    # pixels exactement égaux au coin seulement s'ils sont assez nombreux pour être un ciel.
    egaux = sum(
        1
        for k in range(0, n, 37)
        if (pixels[3 * k], pixels[3 * k + 1], pixels[3 * k + 2]) == ciel
    )
    exclure = egaux > (n // 37) * 0.02
    y = luma(pixels, n)
    eau = [
        k
        for k in range(n)
        if not exclure
        or (pixels[3 * k], pixels[3 * k + 1], pixels[3 * k + 2]) != ciel
    ]
    if not eau:
        raise ValueError(f"{chemin} : aucun pixel d'eau")
    moyenne = sum(y[k] for k in eau) / len(eau)
    variance = sum((y[k] - moyenne) ** 2 for k in eau) / len(eau)
    ecart = math.sqrt(max(variance, 0.0))

    est_eau = [True] * n
    if exclure:
        for k in range(n):
            est_eau[k] = (pixels[3 * k], pixels[3 * k + 1], pixels[3 * k + 2]) != ciel

    sx, sy, m, pires = 0.0, 0.0, 0, []
    for j in range(hauteur):
        base = j * largeur
        for i in range(largeur - 1):
            k = base + i
            if est_eau[k] and est_eau[k + 1]:
                d = y[k + 1] - y[k]
                sx += d * d
                m += 1
                pires.append(abs(d))
    mx = m
    m = 0
    for j in range(hauteur - 1):
        base = j * largeur
        for i in range(largeur):
            k = base + i
            if est_eau[k] and est_eau[k + largeur]:
                d = y[k + largeur] - y[k]
                sy += d * d
                m += 1
                pires.append(abs(d))
    rx = math.sqrt(sx / max(mx, 1))
    ry = math.sqrt(sy / max(m, 1))
    hf = math.sqrt((sx + sy) / max(mx + m, 1))
    pires.sort()
    p99 = pires[int(0.99 * (len(pires) - 1))] if pires else 0.0
    return {
        "pixels_eau": len(eau),
        "luma_moy": moyenne,
        "luma_et": ecart,
        "hf_rms": hf,
        "hf_part": hf / ecart if ecart > 0 else float("nan"),
        "anisotropie": ry / rx if rx > 0 else float("nan"),
        "p99_hf": p99,
    }


def main(argv):
    chemins = []
    for motif in argv[1:]:
        trouve = glob.glob(motif)
        chemins.extend(trouve if trouve else [motif])
    if not chemins:
        print(__doc__)
        return 1
    for chemin in sorted(chemins):
        try:
            r = mesurer(chemin)
        except (OSError, ValueError) as e:
            print(f"SPECTRE_IMAGE image={chemin} erreur={e}")
            continue
        nom = chemin.replace("\\", "/").rsplit("/", 1)[-1]
        print(
            f"SPECTRE_IMAGE image={nom} pixels_eau={r['pixels_eau']} "
            f"luma_moy={r['luma_moy']:.3f} luma_et={r['luma_et']:.3f} "
            f"hf_rms={r['hf_rms']:.4f} hf_part={r['hf_part']:.4f} "
            f"anisotropie={r['anisotropie']:.4f} p99_hf={r['p99_hf']:.3f}"
        )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
