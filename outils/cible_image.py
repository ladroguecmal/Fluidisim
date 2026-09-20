#!/usr/bin/env python3
"""Statistiques d'image d'une mer — photographie ou rendu — S308.

**Pourquoi cet outil existe.** L'utilisateur a fourni une photographie de la mer qu'il vise. Elle
n'est **pas** une cible physique : vent, focale, exposition, heure, état du ciel et réponse
capteur sont inconnus, et aucune calibration n'est possible. Mais c'est une **image**, et une
image se mesure. Cet outil extrait les grandeurs qui se comparent entre une photo et un rendu
**sans connaître la prise de vue**, et refuse de publier comme comparable ce qui ne l'est pas.

**Ce qui est comparable, et pourquoi.**

| grandeur | comparable | raison |
|---|---|---|
| position de l'horizon | oui | géométrique, indépendante de l'exposition |
| `mer_p05/p50`, `p95/p50`, `p99/p50` | **oui** | rapports **normalisés par la médiane de la mer** : une exposition globale les laisse inchangés |
| `contraste_local` (÷ médiane) | **oui** | même raison |
| `B/G`, `B/R` par tranche de luma | **oui** *(sous réserve)* | la balance des blancs du capteur est inconnue ; un écart de **teinte** reste lisible, un écart de quelques pour cent ne l'est pas |
| `fraction_claire` | **oui** | définie par un seuil **relatif à la médiane de la mer** |
| `hf_part` | **oui** | énergie haute fréquence rapportée à l'écart-type |
| `ciel_haut_sur_horizon`, `ciel_bandes` | **non** *(corrigé S308)* | le profil est mesuré en **fraction de cadre** ; deux images de champs de vision différents ne couvrent pas la même plage d'élévation, et le même ciel y donne deux profils. Comparable seulement entre images de **même cadrage** |
| `ciel_BsurG`, `ciel_BsurR` | oui *(sous la même réserve de balance des blancs)* | teintes moyennes du ciel visible |
| luma absolue, `hf_rms` absolu | **non** | dépendent de l'exposition et du tone mapping, inconnus |

L'horizon est détecté comme la ligne où la **luma moyenne par ligne** chute le plus brutalement :
sur une mer sous un ciel clair, c'est la transition la plus franche de l'image. **Elle se trompe
quand le ciel porte lui-même un fort gradient** — constaté en S308 sur un rendu à ciel calé, où
la chute du ciel l'emporte sur celle de l'horizon (chute 0,017 contre 0,083 sur la photographie).
La position détectée et la hauteur de la chute sont donc **publiées toutes les deux** : une chute
faible est le signe qu'il faut forcer la ligne avec `--horizon=<y>`.

Python standard, sans réseau, sans dépendance. Lit le P6 binaire à 255 niveaux, comme le reste
du dépôt (une photographie se convertit hors du dépôt : rien de binaire n'y entre, SPEC-005
§11.3).

    python outils/cible_image.py reference.ppm rendu.ppm
    python outils/cible_image.py --horizon=355 reference.ppm
"""

import math
import sys

from spectre_image import lire_ppm


def srgb_lin(v):
    c = v / 255.0
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


LUM = (0.2126, 0.7152, 0.0722)


def charger(chemin):
    largeur, hauteur, octets = lire_ppm(chemin)
    lin = [srgb_lin(v) for v in range(256)]
    rgb = [(lin[octets[3 * k]], lin[octets[3 * k + 1]], lin[octets[3 * k + 2]])
           for k in range(largeur * hauteur)]
    luma = [LUM[0] * p[0] + LUM[1] * p[1] + LUM[2] * p[2] for p in rgb]
    return largeur, hauteur, rgb, luma


def horizon(largeur, hauteur, luma):
    """Ligne de la chute de luma la plus brutale, cherchée dans le tiers central."""
    moy = [sum(luma[j * largeur:(j + 1) * largeur]) / largeur for j in range(hauteur)]
    lissee = [sum(moy[max(0, j - 2):min(hauteur, j + 3)]) / len(moy[max(0, j - 2):min(hauteur, j + 3)])
              for j in range(hauteur)]
    debut, fin = hauteur // 6, 5 * hauteur // 6
    chute, ligne = 0.0, hauteur // 2
    for j in range(debut, fin - 1):
        d = lissee[j] - lissee[j + 1]
        if d > chute:
            chute, ligne = d, j + 1
    return ligne, chute


def centiles(valeurs, points):
    tri = sorted(valeurs)
    n = len(tri)
    return {p: tri[min(n - 1, max(0, int(p / 100.0 * (n - 1))))] for p in points}


def mesurer(chemin, force=None):
    largeur, hauteur, rgb, luma = charger(chemin)
    ligne, chute = (force, float("nan")) if force is not None else horizon(largeur, hauteur, luma)
    # Deux lignes de garde de part et d'autre : la ligne d'horizon elle-même mélange les deux.
    mer = [k for k in range(ligne * largeur + 3 * largeur, largeur * hauteur)]
    ciel = [k for k in range(0, max(0, (ligne - 3) * largeur))]
    if len(mer) < 1000 or len(ciel) < 1000:
        raise ValueError(f"{chemin} : horizon en {ligne}, une des deux zones est vide")

    lm = [luma[k] for k in mer]
    c = centiles(lm, (1, 5, 25, 50, 75, 95, 99))
    med = max(c[50], 1e-9)

    # Contraste local : écart-type de luma dans une fenêtre 9×9, médiane sur la mer.
    pas, demi, locaux = 7, 4, []
    for j in range(ligne + 4 + demi, hauteur - demi, pas):
        for i in range(demi, largeur - demi, pas):
            s = s2 = 0.0
            for dj in range(-demi, demi + 1):
                base = (j + dj) * largeur + i
                for di in range(-demi, demi + 1):
                    v = luma[base + di]
                    s += v
                    s2 += v * v
            n = (2 * demi + 1) ** 2
            m = s / n
            locaux.append(math.sqrt(max(s2 / n - m * m, 0.0)))
    contraste = sorted(locaux)[len(locaux) // 2] if locaux else 0.0

    # Couleur par tranche : creux (sous p25) et crêtes (au-dessus de p75).
    def teinte(predicat):
        s = [0.0, 0.0, 0.0]
        n = 0
        for k in mer:
            if predicat(luma[k]):
                p = rgb[k]
                s[0] += p[0]
                s[1] += p[1]
                s[2] += p[2]
                n += 1
        if n == 0:
            return (float("nan"),) * 2 + (0,)
        m = [v / n for v in s]
        return (m[2] / max(m[1], 1e-9), m[2] / max(m[0], 1e-9), n)

    creux = teinte(lambda v: v <= c[25])
    cretes = teinte(lambda v: v >= c[75])
    # Fraction « claire » : au-dessus de quatre fois la médiane de la mer — proxy d'écume.
    claire = sum(1 for v in lm if v > 4.0 * med) / len(lm)

    # Ciel : luma moyenne par bande, de l'horizon vers le haut, normalisée par la bande d'horizon.
    bandes = []
    if ligne > 20:
        for b in range(5):
            y1 = ligne - 3 - int(b * (ligne - 3) / 5.0)
            y0 = ligne - 3 - int((b + 1) * (ligne - 3) / 5.0)
            vals = [luma[j * largeur + i] for j in range(max(y0, 0), max(y1, 1)) for i in range(0, largeur, 5)]
            bandes.append(sum(vals) / len(vals) if vals else float("nan"))
    base_ciel = bandes[0] if bandes and bandes[0] > 0 else 1.0
    pente = bandes[-1] / base_ciel if bandes else float("nan")
    teinte_ciel = teinte_bande = None
    if ligne > 20:
        s = [0.0, 0.0, 0.0]
        n = 0
        for k in ciel:
            p = rgb[k]
            s[0] += p[0]
            s[1] += p[1]
            s[2] += p[2]
            n += 1
        m = [v / n for v in s]
        teinte_ciel = m[2] / max(m[1], 1e-9)
        teinte_bande = m[2] / max(m[0], 1e-9)

    nom = chemin.replace("\\", "/").rsplit("/", 1)[-1]
    print(
        f"CIBLE image={nom} {largeur}x{hauteur} horizon_y={ligne} horizon_frac={ligne / hauteur:.3f} "
        f"chute={chute:.4f} pixels_mer={len(mer)} pixels_ciel={len(ciel)}"
    )
    print(
        f"CIBLE image={nom} COMPARABLE mer_p05_sur_p50={c[5] / med:.4f} mer_p25_sur_p50={c[25] / med:.4f} "
        f"mer_p75_sur_p50={c[75] / med:.4f} mer_p95_sur_p50={c[95] / med:.4f} mer_p99_sur_p50={c[99] / med:.4f} "
        f"dynamique_p95_sur_p05={c[95] / max(c[5], 1e-9):.2f} contraste_local_sur_p50={contraste / med:.4f} "
        f"fraction_claire={claire:.5f}"
    )
    print(
        f"CIBLE image={nom} COMPARABLE creux_BsurG={creux[0]:.3f} creux_BsurR={creux[1]:.3f} "
        f"cretes_BsurG={cretes[0]:.3f} cretes_BsurR={cretes[1]:.3f} ciel_BsurG={teinte_ciel:.3f} "
        f"ciel_BsurR={teinte_bande:.3f}"
    )
    # Profil du ciel : **non comparable** entre cadrages différents (voir l'en-tête).
    print(
        f"CIBLE image={nom} CADRAGE ciel_haut_sur_horizon={pente:.4f} "
        f"ciel_bandes={'/'.join(f'{b / base_ciel:.3f}' for b in bandes)}"
    )
    print(
        f"CIBLE image={nom} NON_COMPARABLE mer_luma_p50={med:.5f} mer_luma_p05={c[5]:.5f} "
        f"mer_luma_p95={c[95]:.5f} contraste_local={contraste:.5f}"
    )


def main(argv):
    force = None
    chemins = []
    for a in argv[1:]:
        if a.startswith("--horizon="):
            force = int(a.split("=", 1)[1])
        else:
            chemins.append(a)
    if not chemins:
        print(__doc__)
        return 1
    for chemin in chemins:
        try:
            mesurer(chemin, force)
        except (OSError, ValueError) as e:
            print(f"CIBLE image={chemin} erreur={e}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
