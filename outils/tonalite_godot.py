"""S363 — les courbes de tonalité de Godot 4.4.1 rejouées sur une capture HDR, mesurées contre la photographie.

**Pourquoi cet outil existe.** P2 de S363 a mesuré Godot contre la photographie de référence (`cible_image.py`, S308) :
AgX, la courbe par défaut, écrase la **dynamique** (9,8 pour 23,7) et la **fraction claire** (0,0001 pour 0,074). Quelle
courbe de Godot, à quelle exposition et quel blanc, s'approche le plus des quatre cibles ? Rendre chaque réglage dans
Godot coûte quatre secondes ; le rejouer ici sur **une** capture HDR linéaire coûte un dixième de seconde, et le
gagnant se revérifie dans Godot. Même démarche que `courbe_tonalite.py` pour l'afficheur (S308 P7).

**Ce qui est recopié, d'où.** `servers/rendering/renderer_rd/shaders/effects/tonemap.glsl`, étiquette `4.4.1-stable` :
l'exposition multiplie, puis la courbe (`tonemap_reinhard`, `_filmic`, `_aces`, `_agx` — AgX **ignore le blanc** en
4.4.1), puis `linear_to_srgb`, qui **écrête** à [0, 1], puis l'écriture sur huit bits. Le halo est hors du modèle : la
capture HDR et les rendus comparés s'en passent (`HALO=0`).

**Ce que l'outil mesure.** Les quatre grandeurs comparables de `cible_image.py`, recopiées à l'identique — horizon par
la plus forte chute de luminance, centiles par indice, contraste local 9 × 9 au pas de 7, fraction au-dessus de quatre
fois la médiane de la mer — en `numpy` ; `verifier` le confronte à `cible_image.py` sur une même image.

    TONALITE=lineaire HALO=0 HDR=1 POSES=proche <godot> --path godot -- --captures     # la capture HDR
    python outils/tonalite_godot.py mesurer image.png [...]                               # les quatre grandeurs
    python outils/tonalite_godot.py verifier image.png                                    # contre cible_image.py
    python outils/tonalite_godot.py rejouer capture.pfm agx 1 1 [--ppm=sortie.ppm]        # une courbe rejouée
    python outils/tonalite_godot.py comparer capture.pfm agx 1 1 godot.png                # le modèle contre Godot
"""
import math
import os
import subprocess
import sys

import numpy as np
from PIL import Image

# Les quatre grandeurs de la photographie de référence (S308 P2), celles de `courbe_tonalite.py`.
CIBLES = {"p05_sur_p50": 0.1926, "dynamique": 23.70, "contraste": 0.4549, "fraction_claire": 0.07415}
LUM = np.array([0.2126, 0.7152, 0.0722])
COURBES = ("lineaire", "reinhard", "filmic", "aces", "agx")


def srgb_lin(v):
    c = np.asarray(v, dtype=np.float64) / 255.0
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


TABLE = srgb_lin(np.arange(256))


# --- Lecture et écriture -------------------------------------------------------------------------------------------

def lire_pfm(chemin):
    """PFM couleur : `PF`, largeur hauteur, échelle (négative : petit-boutiste), rangées du bas vers le haut."""
    with open(chemin, "rb") as f:
        if f.readline().strip() != b"PF":
            raise ValueError(f"{chemin} : pas un PFM couleur")
        largeur, hauteur = map(int, f.readline().split())
        echelle = float(f.readline())
        donnees = np.frombuffer(f.read(), dtype="<f4" if echelle < 0 else ">f4")
    return donnees.reshape(hauteur, largeur, 3)[::-1].astype(np.float64)


def lire_image(chemin):
    return np.asarray(Image.open(chemin).convert("RGB"), dtype=np.uint8)


def ecrire_ppm(chemin, octets):
    hauteur, largeur, _ = octets.shape
    with open(chemin, "wb") as f:
        f.write(f"P6\n{largeur} {hauteur}\n255\n".encode("ascii"))
        f.write(np.ascontiguousarray(octets, dtype=np.uint8).tobytes())


# --- Godot 4.4.1, tonemap.glsl, recopié ------------------------------------------------------------------------------

def reinhard(c, blanc):
    b2 = blanc * blanc
    return (b2 * c + c * c) / (b2 * c + b2)


def filmic(c, blanc):
    biais = 2.0
    a, b, cc, d, e, f = 0.22 * biais * biais, 0.30 * biais, 0.10, 0.20, 0.01, 0.30

    def h(x):
        return (x * (a * x + cc * b) + d * e) / (x * (a * x + b) + d * f) - e / f

    return h(c) / h(blanc)


def aces(c, blanc):
    biais = 1.8
    a, b, cc, d, e = 0.0245786, 0.000090537, 0.983729, 0.432951, 0.238081
    # `color *= mat3(c0, c1, c2)` : vecteur ligne fois matrice, la composante j vaut dot(color, c_j).
    rrt = biais * np.array([[0.59719, 0.35458, 0.04823], [0.07600, 0.90834, 0.01566], [0.02840, 0.13383, 0.83777]])
    odt = np.array([[1.60475, -0.53108, -0.07367], [-0.10208, 1.10813, -0.00605], [-0.00327, -0.07276, 1.07602]])
    x = c @ rrt.T
    x = (x * (x + a) - b) / (x * (cc * x + d) + e)
    x = x @ odt.T
    w = blanc * biais
    return x / ((w * (w + a) - b) / (w * (cc * w + d) + e))


def agx(c):
    # `mat3(a0, …, a8) * color` : colonnes (a0, a1, a2), (a3, a4, a5), (a6, a7, a8) ; en ligne, `color @ A`.
    entree = np.array([0.54490813676363087053, 0.14044005884001287035, 0.088827411851915368603,
                       0.37377945959812267119, 0.75410959864013760045, 0.17887712465043811023,
                       0.081384976686407536266, 0.10543358536857773485, 0.73224999956948382528]).reshape(3, 3)
    sortie = np.array([1.9645509602733325934, -0.29932243390911083839, -0.16436833806080403409,
                       -0.85585845117807513559, 1.3264510741502356555, -0.23822464068860595117,
                       -0.10886710826831608324, -0.027084020983874825605, 1.402665347143271889]).reshape(3, 3)
    ev_min, ev_max = -12.4739311883324, 4.02606881166759
    x = np.maximum(c, 2e-10) @ entree
    x = (np.clip(np.log2(x), ev_min, ev_max) - ev_min) / (ev_max - ev_min)
    x2 = x * x
    x4 = x2 * x2
    x = 0.021 * x + 4.0111 * x2 - 25.682 * x2 * x + 70.359 * x4 - 74.778 * x4 * x + 27.069 * x4 * x2
    return np.power(x, 2.4) @ sortie


def tonalite(hdr, courbe, exposition, blanc):
    """Les octets sRGB que Godot écrit : exposition, courbe, écrêtage à [0, 1], sRGB, arrondi sur huit bits."""
    c = hdr * exposition
    if courbe == "reinhard":
        c = reinhard(np.maximum(c, 0.0), blanc)
    elif courbe == "filmic":
        c = filmic(np.maximum(c, 0.0), blanc)
    elif courbe == "aces":
        c = aces(np.maximum(c, 0.0), blanc)
    elif courbe == "agx":
        c = agx(c)
    elif courbe != "lineaire":
        raise ValueError(f"courbe inconnue : {courbe}")
    c = np.clip(c, 0.0, 1.0)
    s = np.where(c < 0.0031308, 12.92 * c, 1.055 * np.power(c, 1.0 / 2.4) - 0.055)
    return np.floor(s * 255.0 + 0.5).astype(np.uint8)


# --- Les quatre grandeurs de `cible_image.py`, à l'identique ---------------------------------------------------------

def luma(octets):
    return TABLE[octets] @ LUM


def horizon(lu):
    hauteur = lu.shape[0]
    moy = lu.mean(axis=1)
    lissee = np.array([moy[max(0, j - 2):min(hauteur, j + 3)].mean() for j in range(hauteur)])
    chute, ligne = 0.0, hauteur // 2
    for j in range(hauteur // 6, 5 * hauteur // 6 - 1):
        d = lissee[j] - lissee[j + 1]
        if d > chute:
            chute, ligne = d, j + 1
    return ligne, chute


def grandeurs(lu, ligne):
    hauteur, largeur = lu.shape
    mer = lu[ligne + 3:].ravel()
    tri = np.sort(mer)
    n = len(tri)
    c = {p: tri[min(n - 1, max(0, int(p / 100.0 * (n - 1))))] for p in (5, 50, 95)}
    med = max(c[50], 1e-9)
    # Contraste local : écart-type 9 × 9 aux centres (j, i) de pas 7, par image intégrale ; médiane haute.
    demi, pas = 4, 7
    s1 = np.pad(lu, ((1, 0), (1, 0))).cumsum(0).cumsum(1)
    s2 = np.pad(lu * lu, ((1, 0), (1, 0))).cumsum(0).cumsum(1)
    js = np.arange(ligne + 4 + demi, hauteur - demi, pas)[:, None]
    ii = np.arange(demi, largeur - demi, pas)[None, :]

    def boite(s):
        return (s[js + demi + 1, ii + demi + 1] - s[js - demi, ii + demi + 1]
                - s[js + demi + 1, ii - demi] + s[js - demi, ii - demi])

    m = boite(s1) / 81.0
    locaux = np.sort(np.sqrt(np.maximum(boite(s2) / 81.0 - m * m, 0.0)).ravel())
    contraste = locaux[len(locaux) // 2] if len(locaux) else 0.0
    return {"p05_sur_p50": c[5] / med, "dynamique": c[95] / max(c[5], 1e-9), "contraste": contraste / med,
            "fraction_claire": float((mer > 4.0 * med).sum()) / n}


def pire_ecart(v):
    ecarts = {k: math.log(max(v[k], 1e-9) / CIBLES[k]) for k in CIBLES}
    dure = max(ecarts, key=lambda k: abs(ecarts[k]))
    return abs(ecarts[dure]), dure


def ligne_mesure(etiquette, v, ligne, chute):
    q, dure = pire_ecart(v)
    return (f"{etiquette} horizon={ligne} chute={chute:.4f} p05_sur_p50={v['p05_sur_p50']:.4f} "
            f"dynamique={v['dynamique']:.2f} contraste={v['contraste']:.4f} fraction_claire={v['fraction_claire']:.5f} "
            f"pire_ecart_log={q:.4f} cible_la_plus_dure={dure}")


def mesurer_octets(octets):
    lu = luma(octets)
    ligne, chute = horizon(lu)
    return grandeurs(lu, ligne), ligne, chute


# --- Les commandes ---------------------------------------------------------------------------------------------------

def cmd_mesurer(chemins):
    for chemin in chemins:
        v, ligne, chute = mesurer_octets(lire_image(chemin))
        print(ligne_mesure(f"TONALITE_GODOT image={os.path.basename(chemin)}", v, ligne, chute))


def cmd_verifier(chemins):
    """La mesure `numpy` contre `cible_image.py`, sur la même image écrite en PPM."""
    ici = os.path.dirname(os.path.abspath(__file__))
    for chemin in chemins:
        octets = lire_image(chemin)
        ppm = os.path.join(os.environ.get("TEMP", "."), "tonalite_godot_verifier.ppm")
        ecrire_ppm(ppm, octets)
        sortie = subprocess.run([sys.executable, os.path.join(ici, "cible_image.py"), ppm],
                                capture_output=True, text=True, check=True).stdout
        ref = {}
        for mot in sortie.split():
            if "=" in mot:
                k, val = mot.split("=", 1)
                ref[k] = val
        attendu = {"p05_sur_p50": float(ref["mer_p05_sur_p50"]), "dynamique": float(ref["dynamique_p95_sur_p05"]),
                   "contraste": float(ref["contraste_local_sur_p50"]), "fraction_claire": float(ref["fraction_claire"])}
        v, ligne, _ = mesurer_octets(octets)
        # `cible_image.py` n'imprime que 2 à 5 décimales : la comparaison juste est celle de l'impression elle-même.
        formats = {"p05_sur_p50": ".4f", "dynamique": ".2f", "contraste": ".4f", "fraction_claire": ".5f"}
        imprime = {k: format(v[k], formats[k]) for k in formats}
        attendu_txt = {"p05_sur_p50": ref["mer_p05_sur_p50"], "dynamique": ref["dynamique_p95_sur_p05"],
                       "contraste": ref["contraste_local_sur_p50"], "fraction_claire": ref["fraction_claire"]}
        identiques = all(imprime[k] == attendu_txt[k] for k in formats) and str(ligne) == ref["horizon_y"]
        print(f"VERIFIER_TONALITE image={os.path.basename(chemin)} horizon={ligne}/{ref['horizon_y']} "
              + " ".join(f"{k}={imprime[k]}/{attendu_txt[k]}" for k in formats)
              + f" identiques_a_l_impression={'oui' if identiques else 'NON'}")


def cmd_rejouer(args):
    ppm = None
    reste = []
    for a in args:
        if a.startswith("--ppm="):
            ppm = a.split("=", 1)[1]
        else:
            reste.append(a)
    chemin, courbe, e, w = reste[0], reste[1], float(reste[2]), float(reste[3])
    octets = tonalite(lire_pfm(chemin), courbe, e, w)
    if ppm:
        ecrire_ppm(ppm, octets)
    v, ligne, chute = mesurer_octets(octets)
    print(ligne_mesure(f"REJOUER courbe={courbe} e={e:g} w={w:g}", v, ligne, chute))


def cmd_comparer(args):
    chemin, courbe, e, w, image = args[0], args[1], float(args[2]), float(args[3]), args[4]
    modele = tonalite(lire_pfm(chemin), courbe, e, w)
    godot = lire_image(image)
    ecart = np.abs(modele.astype(np.int16) - godot.astype(np.int16))
    vm, lm, _ = mesurer_octets(modele)
    vg, lg, _ = mesurer_octets(godot)

    def relatif(a, b):
        return 0.0 if a == b else (abs(a / b - 1.0) if b != 0.0 else float("inf"))

    ecarts = {k: relatif(vm[k], vg[k]) for k in vm}
    # La fraction claire peut tenir en quelques pixels : l'écart se dit aussi en pixels de mer.
    n_mer = (godot.shape[0] - lg - 3) * godot.shape[1]
    pixels = round(abs(vm["fraction_claire"] - vg["fraction_claire"]) * n_mer)
    print(f"COMPARER courbe={courbe} e={e:g} w={w:g} octets_ecart_max={int(ecart.max())} "
          f"part_ecart_sup_1={float((ecart > 1).mean()):.2e} ecart_moyen={float(ecart.mean()):.4f} horizon={lm}/{lg} "
          + " ".join(f"{k}={vm[k]:.5f}/{vg[k]:.5f}" for k in vm)
          + f" claire_pixels_modele_godot={round(vm['fraction_claire'] * n_mer)}/{round(vg['fraction_claire'] * n_mer)}"
          + f" ecart_claire_pixels={pixels} pire_ecart_relatif={max(ecarts.values()):.4f}"
          + f" sur_trois={max(ecarts[k] for k in ecarts if k != 'fraction_claire'):.4f}")


def main(argv):
    if len(argv) < 3:
        print(__doc__)
        return 1
    commande, args = argv[1], argv[2:]
    if commande == "mesurer":
        cmd_mesurer(args)
    elif commande == "verifier":
        cmd_verifier(args)
    elif commande == "rejouer":
        cmd_rejouer(args)
    elif commande == "comparer":
        cmd_comparer(args)
    else:
        print(__doc__)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
