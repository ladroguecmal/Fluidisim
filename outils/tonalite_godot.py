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
    python outils/tonalite_godot.py mesurer [--horizon=223] image.png [...]               # les quatre grandeurs
    python outils/tonalite_godot.py verifier image.png                                    # contre cible_image.py
    python outils/tonalite_godot.py rejouer capture.pfm agx 1 1 [--ppm=sortie.ppm]        # une courbe rejouée
    python outils/tonalite_godot.py comparer capture.pfm agx 1 1 godot.png                # le modèle contre Godot
    python outils/tonalite_godot.py balayer --horizon=223 capture.pfm                    # la meilleure contre la photo
    python outils/tonalite_godot.py compromis --horizon=223 capture.pfm                  # luminance et teinte des creux
"""
import math
import os
import subprocess
import sys

import numpy as np
from PIL import Image

# Les quatre grandeurs de la photographie de référence (S308 P2), celles de `courbe_tonalite.py`.
CIBLES = {"p05_sur_p50": 0.1926, "dynamique": 23.70, "contraste": 0.4549, "fraction_claire": 0.07415}
# Et sa teinte (S308 P2, `e4174abc`) : creux sous p25, crêtes au-dessus de p75 de la luminance de mer. Comparables
# **sous réserve** de la balance des blancs du capteur, inconnue : un écart de teinte se lit, pas quelques pour cent.
TEINTES = {"creux_BsurG": 5.54, "creux_BsurR": 30.0, "cretes_BsurG": 2.89, "cretes_BsurR": 8.95}
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


def tonalite(hdr, courbe, exposition, blanc, bcs=None):
    """Les octets sRGB que Godot écrit : exposition, courbe, écrêtage à [0, 1], sRGB, arrondi sur huit bits. `bcs` :
    les ajustements de l'environnement (`adjustment_brightness`, `_contrast`, `_saturation`), que `apply_bcs` applique
    **après** le passage en sRGB — contraste autour de 0,5, saturation autour de la moyenne des trois canaux."""
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
    if bcs is not None:
        b, k, sat = bcs
        s = 0.5 + (s * b - 0.5) * k
        m = s.sum(axis=-1, keepdims=True) * 0.33333
        s = np.clip(m + (s - m) * sat, 0.0, 1.0)
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


def teintes(octets, lu, ligne):
    """`teinte()` de `cible_image.py` : moyennes linéaires de B, G et R sur les creux et sur les crêtes de la mer."""
    rgb = TABLE[octets[ligne + 3:]].reshape(-1, 3)
    mer = lu[ligne + 3:].ravel()
    tri = np.sort(mer)
    n = len(tri)
    p25, p75 = (tri[min(n - 1, max(0, int(p / 100.0 * (n - 1))))] for p in (25, 75))
    v = {}
    for nom, masque in (("creux", mer <= p25), ("cretes", mer >= p75)):
        m = rgb[masque].mean(axis=0)
        v[nom + "_BsurG"] = m[2] / max(m[1], 1e-9)
        v[nom + "_BsurR"] = m[2] / max(m[0], 1e-9)
    return v


def pire_ecart(v):
    ecarts = {k: math.log(max(v[k], 1e-9) / CIBLES[k]) for k in CIBLES}
    dure = max(ecarts, key=lambda k: abs(ecarts[k]))
    return abs(ecarts[dure]), dure


def ligne_mesure(etiquette, v, ligne, chute):
    q, dure = pire_ecart(v)
    return (f"{etiquette} horizon={ligne} chute={chute:.4f} p05_sur_p50={v['p05_sur_p50']:.4f} "
            f"dynamique={v['dynamique']:.2f} contraste={v['contraste']:.4f} fraction_claire={v['fraction_claire']:.5f} "
            f"satures={v['satures']:.5f} pire_ecart_log={q:.4f} cible_la_plus_dure={dure}"
            + ("" if "creux_BsurG" not in v else " TEINTE " + " ".join(
                f"{k}={v[k]:.3f}/{TEINTES[k]}" for k in TEINTES) + f" pire_ecart_log_teinte="
                f"{max(abs(math.log(v[k] / TEINTES[k])) for k in TEINTES):.4f}"))


def mesurer_octets(octets, force=None, couleur=False):
    """Les quatre grandeurs, plus la part des pixels de mer dont un canal est écrêté (255). **L'horizon se force**
    pour comparer deux rendus d'une même pose : la détection s'est trompée deux fois en silence (S308, A301)."""
    lu = luma(octets)
    ligne, chute = (force, float("nan")) if force is not None else horizon(lu)
    v = grandeurs(lu, ligne)
    v["satures"] = float((octets[ligne + 3:].max(axis=2) == 255).mean())
    if couleur:
        v.update(teintes(octets, lu, ligne))
    return v, ligne, chute


def options(args, **defauts):
    """`--nom=valeur` → dictionnaire, converti au type du défaut (flottant si le défaut est `None`) ; le reste, dans
    l'ordre."""
    vals, reste = dict(defauts), []
    for a in args:
        if a.startswith("--") and "=" in a:
            k, val = a[2:].split("=", 1)
            k = k.replace("-", "_")
            vals[k] = type(defauts[k])(val) if defauts.get(k) is not None else float(val)
        else:
            reste.append(a)
    return vals, reste


# --- Les commandes ---------------------------------------------------------------------------------------------------

def cmd_mesurer(args):
    o, chemins = options(args, horizon=None)
    force = int(o["horizon"]) if o["horizon"] is not None else None
    for chemin in chemins:
        v, ligne, chute = mesurer_octets(lire_image(chemin), force, couleur=True)
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


def reglage(args):
    """`fichier courbe exposition blanc [--contraste=k] [--saturation=s] [--horizon=y] [--ppm=f]`."""
    o, reste = options(args, contraste=1.0, saturation=1.0, horizon=None, ppm="")
    bcs = None if (o["contraste"], o["saturation"]) == (1.0, 1.0) else (1.0, o["contraste"], o["saturation"])
    force = int(o["horizon"]) if o["horizon"] is not None else None
    etiquette = (f"courbe={reste[1]} e={float(reste[2]):g} w={float(reste[3]):g} contraste={o['contraste']:g} "
                 f"saturation={o['saturation']:g}")
    return reste, bcs, force, o["ppm"], etiquette


def cmd_rejouer(args):
    reste, bcs, force, ppm, etiquette = reglage(args)
    octets = tonalite(lire_pfm(reste[0]), reste[1], float(reste[2]), float(reste[3]), bcs)
    if ppm:
        ecrire_ppm(ppm, octets)
    v, ligne, chute = mesurer_octets(octets, force, couleur=True)
    print(ligne_mesure(f"REJOUER {etiquette}", v, ligne, chute))


def cmd_comparer(args):
    reste, bcs, force, _, etiquette = reglage(args)
    chemin, courbe, e, w, image = reste[0], reste[1], float(reste[2]), float(reste[3]), reste[4]
    modele = tonalite(lire_pfm(chemin), courbe, e, w, bcs)
    godot = lire_image(image)
    ecart = np.abs(modele.astype(np.int16) - godot.astype(np.int16))
    vm, lm, _ = mesurer_octets(modele, force)
    vg, lg, _ = mesurer_octets(godot, force)

    def relatif(a, b):
        return 0.0 if a == b else (abs(a / b - 1.0) if b != 0.0 else float("inf"))

    ecarts = {k: relatif(vm[k], vg[k]) for k in CIBLES}
    # La fraction claire peut tenir en quelques pixels : l'écart se dit aussi en pixels de mer.
    n_mer = (godot.shape[0] - lg - 3) * godot.shape[1]
    pixels = round(abs(vm["fraction_claire"] - vg["fraction_claire"]) * n_mer)
    print(f"COMPARER {etiquette} octets_ecart_max={int(ecart.max())} "
          f"part_ecart_sup_1={float((ecart > 1).mean()):.2e} ecart_moyen={float(ecart.mean()):.4f} horizon={lm}/{lg} "
          + " ".join(f"{k}={vm[k]:.5f}/{vg[k]:.5f}" for k in CIBLES)
          + f" claire_pixels_modele_godot={round(vm['fraction_claire'] * n_mer)}/{round(vg['fraction_claire'] * n_mer)}"
          + f" ecart_claire_pixels={pixels} pire_ecart_relatif={max(ecarts.values()):.4f}"
          + f" sur_trois={max(ecarts[k] for k in ecarts if k != 'fraction_claire'):.4f}")


def cmd_balayer(args):
    """Le critère de S308, déclaré avant la mesure : le **pire** écart logarithmique sur les quatre grandeurs. Grille
    grossière — exposition de 0,25 à 34 par facteurs 1,2, blanc de 0,5 à 32 par facteurs √2 pour les trois courbes qui
    le lisent —, puis affinage autour du meilleur de chaque courbe (facteurs 1,2^(1/8) et 2^(1/8)).

    **Deux classements.** Le critère seul, et le même sous la contrainte `--satures-max` (part des pixels de mer dont
    un canal est écrêté) : ajoutée en S363 après le premier balayage, où le meilleur réglage écrêtait 18 % de la mer —
    il achetait la dynamique et la fraction claire en brûlant les hautes lumières. Les deux se publient."""
    o, reste = options(args, garder=8, horizon=None, satures_max=0.01)
    chemin = reste[0]
    force = int(o["horizon"]) if o["horizon"] is not None else None
    hdr = lire_pfm(chemin)
    resultats = []
    vus = set()

    def essai(courbe, e, w):
        cle = (courbe, round(e, 6), round(w, 6))
        if cle in vus:
            return
        vus.add(cle)
        v, ligne, _ = mesurer_octets(tonalite(hdr, courbe, e, w), force)
        q, dure = pire_ecart(v)
        resultats.append((q, courbe, e, w, v, ligne, dure))

    def sous_contrainte(r):
        return r[4]["satures"] <= o["satures_max"]

    blanc = ("reinhard", "filmic", "aces")
    for courbe in COURBES:
        for k in range(27):
            for j in (range(13) if courbe in blanc else (None,)):
                essai(courbe, 0.25 * 1.2 ** k, 0.5 * 2.0 ** (j / 2.0) if j is not None else 1.0)
    for courbe in COURBES:
        for filtre in (lambda r: True, sous_contrainte):
            candidats = [r for r in resultats if r[1] == courbe and filtre(r)]
            if not candidats:
                continue
            _, _, e0, w0, *_ = min(candidats, key=lambda r: r[0])
            for k in range(-8, 9):
                for j in (range(-4, 5) if courbe in blanc else (0,)):
                    essai(courbe, e0 * 1.2 ** (k / 8.0), w0 * 2.0 ** (j / 8.0))

    def imprimer(etiquette, r):
        q, courbe, e, w, v, ligne, dure = r
        print(f"{etiquette} courbe={courbe} e={e:.4f} w={w:.4f} horizon={ligne} p05_sur_p50={v['p05_sur_p50']:.4f} "
              f"dynamique={v['dynamique']:.2f} contraste={v['contraste']:.4f} fraction_claire={v['fraction_claire']:.5f} "
              f"satures={v['satures']:.5f} pire_ecart_log={q:.4f} cible_la_plus_dure={dure}")

    print(f"BALAYER capture={os.path.basename(chemin)} essais={len(resultats)} horizon_force={force} "
          f"satures_max={o['satures_max']} cibles=" + " ".join(f"{k}={CIBLES[k]}" for k in CIBLES))
    for etiquette, filtre in (("", lambda r: True), ("_sous_contrainte", sous_contrainte)):
        for courbe in COURBES:
            candidats = [r for r in resultats if r[1] == courbe and filtre(r)]
            if candidats:
                imprimer(f"BALAYER meilleur_par_courbe{etiquette}", min(candidats, key=lambda r: r[0]))
        for r in sorted((r for r in resultats if filtre(r)), key=lambda r: r[0])[:o["garder"]]:
            imprimer(f"BALAYER tete{etiquette}", r)


def cmd_compromis(args):
    """**La luminance et la teinte ensemble.** Le balayage retient ACES, mais ACES rend les creux 3,3 fois trop bleus
    (B/G 18 pour 5,54) — le « bleu saturé » que R14 refusait —, quand AgX, dont la couleur a été jugée « parfaite »
    (R20), les tient à 0,3 % près. Critère ajouté en S363 après ce constat, et dit tel : le pire des cinq écarts
    logarithmiques — les quatre grandeurs de luminance et le B/G des creux. Grille : AgX, exposition 0,2 à 1,8 par
    facteurs 1,2, contraste 1 à 2 par pas de 0,1, saturation 0,8 / 1 / 1,2 ; ACES au blanc 4 (aucun écrêtage),
    exposition 0,7 à 1,7 par facteurs 1,12, saturation 0,3 à 1 par pas de 0,1."""
    o, reste = options(args, garder=8, horizon=None)
    force = int(o["horizon"]) if o["horizon"] is not None else None
    hdr = lire_pfm(reste[0])
    res = []

    def essai(courbe, e, w, k, sat):
        v, ligne, _ = mesurer_octets(tonalite(hdr, courbe, e, w, (1.0, k, sat)), force, couleur=True)
        q, dure = pire_ecart(v)
        qt = abs(math.log(max(v["creux_BsurG"], 1e-9) / TEINTES["creux_BsurG"]))
        res.append((max(q, qt), q, qt, courbe, e, w, k, sat, v, ligne, dure if q >= qt else "creux_BsurG"))

    for i in range(13):
        for j in range(11):
            for sat in (0.8, 1.0, 1.2):
                essai("agx", 0.2 * 1.2 ** i, 1.0, 1.0 + 0.1 * j, sat)
    for i in range(9):
        for j in range(8):
            essai("aces", 0.7 * 1.12 ** i, 4.0, 1.0, 0.3 + 0.1 * j)
    print(f"COMPROMIS capture={os.path.basename(reste[0])} essais={len(res)} horizon_force={force}")
    for r in sorted(res, key=lambda r: r[0])[:o["garder"]]:
        m, q, qt, courbe, e, w, k, sat, v, ligne, dure = r
        print(f"COMPROMIS courbe={courbe} e={e:.4f} w={w:g} contraste={k:.1f} saturation={sat:.1f} horizon={ligne} "
              f"p05_sur_p50={v['p05_sur_p50']:.4f} dynamique={v['dynamique']:.2f} contraste_local={v['contraste']:.4f} "
              f"fraction_claire={v['fraction_claire']:.5f} satures={v['satures']:.5f} creux_BsurG={v['creux_BsurG']:.3f} "
              f"cretes_BsurG={v['cretes_BsurG']:.3f} pire_luminance={q:.4f} ecart_teinte={qt:.4f} pire_des_cinq={m:.4f} "
              f"cible_la_plus_dure={dure}")


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
    elif commande == "balayer":
        cmd_balayer(args)
    elif commande == "compromis":
        cmd_compromis(args)
    else:
        print(__doc__)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
