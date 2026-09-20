#!/usr/bin/env python3
"""Recherche de la courbe de tonalité contre les cibles mesurées — S308 P7.

**Pourquoi cet outil existe.** S308 a transformé la photographie de référence en cible chiffrée
(`cible_image.py`), puis a réglé la courbe de tonalité du rendu en balayant trois points à la
main. P6 a montré que ce réglage-là ne pouvait pas tenir : dans la plage de luma de la mer, la
courbe est presque une pure loi de puissance, donc **le même exposant commande la densité des
creux et la queue claire**. Caler l'un dérègle l'autre. Choisir devient un problème
d'optimisation à quatre cibles, et un problème d'optimisation ne se résout pas à la main.

**Ce que l'outil cherche, et sur quoi.** La courbe du rendu, telle qu'elle est écrite dans
`viewer/src/water.wgsl` :

    x  = (l · e) ^ g                        écrase les ombres  (le « pied »)
    l' = x · (1 + x / w²) / (1 + x)         comprime les hautes lumières  (« l'épaule »)

Elle ne touche qu'à la **luminance** — la teinte est conservée par un rapport. Les statistiques
de luma se recalculent donc **sans refaire l'image**, à partir d'un rendu fait *sans courbe* :
c'est ce que l'outil exploite pour balayer des milliers de réglages en quelques secondes.

**Ce que cela vaut, et ce que cela ne vaut pas.** Vérifié en S308 P6 sur la pose « proche » :
le modèle hors GPU redonne les chiffres de la carte à 2 % près (`0,1956 / 27,84 / 0,3830 /
0,17402` contre `0,1923 / 28,40 / 0,3821 / 0,17470`), et l'image sans courbe ne sature **aucun**
pixel de mer. **Le gagnant se revérifie malgré tout sur la carte** : cet outil sert à chercher,
pas à conclure.

**Le critère est déclaré avant la mesure, et il est unique** : minimiser le **pire** écart
relatif logarithmique sur les quatre grandeurs comparables. Un réglage n'achète pas une cible en
détruisant une autre, et le pire écart dit tout de suite quelle cible est la plus dure.

    python outils/courbe_tonalite.py rendu_sans_courbe.ppm
    python outils/courbe_tonalite.py --horizon=223 --garder=8 rendu_sans_courbe.ppm

Python standard, sans réseau, sans dépendance — comme le reste du dépôt.
"""

import math
import sys

from spectre_image import lire_ppm

# Les quatre grandeurs comparables mesurées sur la photographie de référence (S308 P2).
CIBLES = {"p05_sur_p50": 0.1926, "dynamique": 23.70, "contraste": 0.4549, "fraction_claire": 0.07415}
LUM = (0.2126, 0.7152, 0.0722)


def srgb_lin(v):
    c = v / 255.0
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def courbe(l, e, g, w):
    """La courbe du shader, à l'identique — **avec l'écrêtage de la sortie**, qui en fait partie.

    Croissante au sens large pour tout (e, g, w) > 0 : la dérivée avant écrêtage vaut
    (1 + 2x/w² + x²/w²)/(1 + x)², qui ne s'annule jamais. C'est cette monotonie qui permet de lire
    les centiles **sans retrier** l'image à chaque candidat.

    **L'écrêtage fait partie de la courbe.** Elle vaut 1 en `x = w` et dépasse 1 ensuite ; l'image
    écrit du sRGB sur huit bits, donc tout ce qui dépasse retombe à 1. Un réglage à petit point
    blanc écrête donc pour de bon, et un modèle sans `min` le noterait mieux qu'il n'est.

    *(Honnêteté sur l'origine de ce `min` : il a été ajouté en S308 P7 en croyant expliquer un
    écart GPU / hors GPU. Il ne l'expliquait pas — l'écart venait de la détection d'horizon, qui
    s'était trompée sur ce rendu-là. Le `min` reste juste ; ce n'était simplement pas la cause.)*"""
    x = (l * e) ** g
    return min(x * (1.0 + x / (w * w)) / (1.0 + x), 1.0)


def horizon(largeur, hauteur, luma):
    """Même détection que `cible_image.py` : la chute de luma la plus brutale."""
    moy = [sum(luma[j * largeur:(j + 1) * largeur]) / largeur for j in range(hauteur)]
    lissee = [sum(moy[max(0, j - 2):min(hauteur, j + 3)]) / len(moy[max(0, j - 2):min(hauteur, j + 3)])
              for j in range(hauteur)]
    chute, ligne = 0.0, hauteur // 2
    for j in range(hauteur // 6, 5 * hauteur // 6 - 1):
        d = lissee[j] - lissee[j + 1]
        if d > chute:
            chute, ligne = d, j + 1
    return ligne, chute


def premier_indice(tri, seuil):
    """Premier indice `i` tel que `tri[i] > seuil`, par dichotomie."""
    bas, haut = 0, len(tri)
    while bas < haut:
        mil = (bas + haut) // 2
        if tri[mil] > seuil:
            haut = mil
        else:
            bas = mil + 1
    return bas


def inverse(cible, e, g, w):
    """`l` tel que `courbe(l) = cible`, par dichotomie sur une fonction croissante. Au-dessus de
    l'écrêtage la courbe est plate : aucun `l` n'y répond, et aucun pixel ne dépasse le seuil."""
    if cible >= 1.0:
        return float("inf")
    bas, haut = 0.0, 1.0
    while courbe(haut, e, g, w) < cible and haut < 1e6:
        haut *= 2.0
    for _ in range(60):
        mil = 0.5 * (bas + haut)
        if courbe(mil, e, g, w) < cible:
            bas = mil
        else:
            haut = mil
    return 0.5 * (bas + haut)


def pire_ecart(v):
    return max(abs(math.log(max(v[k], 1e-9) / CIBLES[k])) for k in CIBLES)


def etage1(tri, e, g, w):
    """Trois grandeurs sur quatre, en temps constant : les centiles se lisent dans le tableau déjà
    trié — la courbe est croissante, elle ne change pas l'ordre — et la fraction claire se compte
    par dichotomie sur le seuil ramené en entrée de courbe."""
    n = len(tri)
    l05, l50, l95 = (tri[int(p / 100.0 * (n - 1))] for p in (5, 50, 95))
    p05, p50, p95 = (courbe(l, e, g, w) for l in (l05, l50, l95))
    if p50 <= 0.0 or p05 <= 0.0:
        return None
    claire = (n - premier_indice(tri, inverse(4.0 * p50, e, g, w))) / n
    return {"p05_sur_p50": p05 / p50, "dynamique": p95 / p05, "fraction_claire": max(claire, 1e-5)}


def etage2(luma, largeur, hauteur, ligne, depart, tri, e, g, w):
    """La quatrième grandeur — le contraste local — demande la fenêtre 9×9 de `cible_image.py`,
    donc l'image entière. Elle n'est calculée que sur la liste courte."""
    mer = [courbe(l, e, g, w) for l in luma[depart:]]
    p50 = courbe(tri[int(0.50 * (len(tri) - 1))], e, g, w)
    demi, pas, locaux = 4, 7, []
    for j in range(ligne + 4 + demi, hauteur - demi, pas):
        for i in range(demi, largeur - demi, pas):
            s = s2 = 0.0
            for dj in range(-demi, demi + 1):
                base = (j + dj) * largeur + i - depart
                for di in range(-demi, demi + 1):
                    v = mer[base + di]
                    s += v
                    s2 += v * v
            m = s / 81.0
            locaux.append(math.sqrt(max(s2 / 81.0 - m * m, 0.0)))
    return sorted(locaux)[len(locaux) // 2] / max(p50, 1e-12)


def grille():
    """Exposition en progression géométrique, exposant linéaire, point blanc géométrique. Les
    bornes sont celles que `--tonalite` accepte."""
    points = []
    e = 0.5
    while e <= 20.0:
        g = 1.0
        while g <= 2.21:
            w = 0.5
            while w <= 32.0:
                points.append((e, g, w))
                w *= 2.0 ** 0.5
            g += 0.05
        e *= 1.2
    return points


def main(argv):
    force, garder, chemins = None, 6, []
    for a in argv[1:]:
        if a.startswith("--horizon="):
            force = int(a.split("=", 1)[1])
        elif a.startswith("--garder="):
            garder = int(a.split("=", 1)[1])
        else:
            chemins.append(a)
    if not chemins:
        print(__doc__)
        return 1
    chemin = chemins[0]
    largeur, hauteur, octets = lire_ppm(chemin)
    table = [srgb_lin(v) for v in range(256)]
    luma = [LUM[0] * table[octets[3 * k]] + LUM[1] * table[octets[3 * k + 1]] + LUM[2] * table[octets[3 * k + 2]]
            for k in range(largeur * hauteur)]
    ligne, chute = (force, float("nan")) if force is not None else horizon(largeur, hauteur, luma)
    depart = (ligne + 3) * largeur
    tri = sorted(luma[depart:])
    nom = chemin.replace("\\", "/").rsplit("/", 1)[-1]
    satures = sum(1 for k in range(depart, largeur * hauteur)
                  if max(octets[3 * k], octets[3 * k + 1], octets[3 * k + 2]) == 255)
    print(f"COURBE image={nom} {largeur}x{hauteur} horizon={ligne} chute={chute:.4f} "
          f"pixels_mer={len(tri)} satures={satures}")
    if satures > len(tri) // 1000:
        print("COURBE avertissement=image_saturee : la recherche sous-estime les hautes lumières")

    points = grille()
    courts = []
    for (e, g, w) in points:
        v = etage1(tri, e, g, w)
        if v is None:
            continue
        # Le contraste manque encore : on classe sur les trois autres, et on garde large.
        q = max(abs(math.log(v[k] / CIBLES[k])) for k in ("p05_sur_p50", "dynamique", "fraction_claire"))
        courts.append((q, e, g, w, v))
    courts.sort(key=lambda r: r[0])
    print(f"COURBE etage1 candidats={len(points)} retenus={garder} meilleur_sur_trois={courts[0][0]:.4f}")

    final = []
    for (_, e, g, w, v) in courts[:garder]:
        v = dict(v)
        v["contraste"] = etage2(luma, largeur, hauteur, ligne, depart, tri, e, g, w)
        final.append((pire_ecart(v), e, g, w, v))
    final.sort(key=lambda r: r[0])
    for (q, e, g, w, v) in final:
        ecarts = {k: math.log(v[k] / CIBLES[k]) for k in CIBLES}
        dure = max(ecarts, key=lambda k: abs(ecarts[k]))
        print(f"COURBE e={e:.3f} g={g:.3f} w={w:.3f} p05_sur_p50={v['p05_sur_p50']:.4f} "
              f"dynamique={v['dynamique']:.2f} contraste={v['contraste']:.4f} "
              f"fraction_claire={v['fraction_claire']:.5f} pire_ecart_log={q:.4f} "
              f"cible_la_plus_dure={dure}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
