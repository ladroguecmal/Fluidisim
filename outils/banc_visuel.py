#!/usr/bin/env python3
"""Le banc visuel — S471, ADR-216 : **mesurer plutôt que regarder**.

Le même calcul que `banc_visuel.js` (qui mesure les vidéos de référence dans la page qui les lit), pour nos rendus : une suite
d'images PNG (une séquence capturée par Godot, ou une image seule), des zones (rectangles en fractions du cadre), l'intervalle entre
images. Toute modification se porte dans les deux fichiers ; `--egalite` les confronte (ADR-216 D3 : 1 %).

Les grandeurs, par zone — comparables sans connaître la prise de vue parce que rapportées à la médiane de la zone (`cible_image.py`,
S308), plus le temps :

- statiques (médiane sur les images) : `p05 … p99` sur `p50` (luminance linéaire) ; `contraste` (écart-type local 9×9, médiane, sur
  `p50`) ; `creux_BsurG`, `creux_BsurR` (≤ p25), `cretes_BsurG`, `cretes_BsurR` (≥ p75) ; `claire` (> 4·p50) ; `ecume` (> 2·p50,
  saturation < 0,2) ; `hf_part` (RMS des différences à un pixel ÷ écart-type) ; `anisotropie` (RMS dy ÷ RMS dx) ; `p50_absolu` (**non
  comparable** : l'exposition) ;
- temporelles : `mouvement_par_s` (médiane de |L_t − L_{t−1}| moyen, ÷ p50, ÷ Δt) ; `periode_s` et `periode_nettete` (la luminance
  moyenne de la zone, tendance retirée : le pic du spectre, et pic ÷ médiane du spectre) ; `renouvellement_clairs_par_s` (la part des
  pixels clairs neufs d'une image à l'autre, par seconde).

L'image est réduite par moyenne de blocs `f × f` des valeurs linéaires.

    python outils/banc_visuel.py --zones='{"mer": [0, 0.5, 1, 1]}' --dt=0.0333 --facteur=4 images/*.png
    python outils/banc_visuel.py --egalite=<json de banc_visuel.js> --zones=... --facteur=... image.png
"""

import json
import sys

import numpy as np
from PIL import Image

LIN = np.array([v / 255 / 12.92 if v / 255 <= 0.04045 else ((v / 255 + 0.055) / 1.055) ** 2.4 for v in range(256)])
LUM = (0.2126, 0.7152, 0.0722)


def reduire(chemin, f):
    a = np.asarray(Image.open(chemin).convert("RGB"))
    h, w = a.shape[0] // f, a.shape[1] // f
    lin = LIN[a[: h * f, : w * f]]
    lin = lin.reshape(h, f, w, f, 3).mean(axis=(1, 3))
    r, g, b = lin[..., 0], lin[..., 1], lin[..., 2]
    return {"r": r, "g": g, "b": b, "l": LUM[0] * r + LUM[1] * g + LUM[2] * b}


def decouper(img, z):
    h, w = img["l"].shape
    x0, y0, x1, y1 = int(z[0] * w), int(z[1] * h), int(z[2] * w), int(z[3] * h)
    return {k: v[y0:y1, x0:x1] for k, v in img.items()}


def centile(tri, p):
    n = len(tri)
    return tri[min(n - 1, max(0, int(p / 100 * (n - 1))))]


def statiques(z):
    l = z["l"]
    tri = np.sort(l.ravel())
    c = {p: centile(tri, p) for p in (5, 25, 50, 75, 95, 99)}
    med = max(c[50], 1e-9)
    et = l.std()
    h, w = l.shape
    locaux = []
    for j in range(4, h - 4, 7):
        for i in range(4, w - 4, 7):
            f = l[j - 4: j + 5, i - 4: i + 5]
            m = f.mean()
            locaux.append(np.sqrt(max((f * f).mean() - m * m, 0.0)))
    locaux.sort()
    contraste = locaux[len(locaux) // 2] if locaux else 0.0

    def teinte(masque):
        sr, sg, sb = z["r"][masque].sum(), z["g"][masque].sum(), z["b"][masque].sum()
        return sb / max(sg, 1e-12), sb / max(sr, 1e-12)

    creux, cretes = teinte(l <= c[25]), teinte(l >= c[75])
    mx = np.maximum(np.maximum(z["r"], z["g"]), z["b"])
    mn = np.minimum(np.minimum(z["r"], z["g"]), z["b"])
    ecume = ((l > 2 * med) & ((mx - mn) < 0.2 * np.maximum(mx, 1e-12))).mean()
    rx = np.sqrt((np.diff(l, axis=1) ** 2).mean()) if w > 1 else 0.0
    ry = np.sqrt((np.diff(l, axis=0) ** 2).mean()) if h > 1 else 0.0
    return {
        "p05": c[5] / med, "p25": c[25] / med, "p75": c[75] / med, "p95": c[95] / med, "p99": c[99] / med,
        "contraste": contraste / med,
        "creux_BsurG": creux[0], "creux_BsurR": creux[1], "cretes_BsurG": cretes[0], "cretes_BsurR": cretes[1],
        "claire": float((l > 4 * med).mean()), "ecume": float(ecume),
        "hf_part": np.sqrt((rx * rx + ry * ry) / 2) / max(et, 1e-12), "anisotropie": ry / max(rx, 1e-12),
        "p50_absolu": med,
    }


def mediane(a):
    t = np.sort(np.asarray(a, dtype=float))
    return t[len(t) // 2] if len(t) else float("nan")


def temporelles(suite, dt):
    n = len(suite)
    if n < 3:
        return {}
    meds = [mediane(z["l"].ravel()) for z in suite]
    med = mediane(meds)
    moy = [z["l"].mean() for z in suite]
    dif, renouv = [], []
    for t in range(1, n):
        a, b = suite[t - 1]["l"], suite[t]["l"]
        dif.append(np.abs(b - a).mean())
        clairs = b > 4 * meds[t]
        renouv.append((clairs & ~(a > 4 * meds[t - 1])).sum() / clairs.sum() if clairs.sum() > 0 else float("nan"))
    # Les coupes d'un montage : un écart de plus de six fois la médiane, et de 5 % de la luminance au moins (le bruit d'une zone
    # immobile n'en est pas une) ; exclues, et le spectre pris sur le plus long plan.
    seuil = max(6 * mediane(dif), 0.05 * med)
    coupe = [d > seuil for d in dif]
    difs = [d for d, c in zip(dif, coupe) if not c]
    renouvs = [r for r, c in zip(renouv, coupe) if not c and r == r]
    plan, deb = (0, 0), 0
    for t in range(1, n + 1):
        if t == n or coupe[t - 1]:
            if t - deb > plan[1] - plan[0]:
                plan = (deb, t)
            deb = t
    moyp = moy[plan[0]:plan[1]]
    coupes = sum(coupe)
    if len(moyp) < 8:
        return {"coupes": coupes, "images": n, "dt": dt}
    periode, nettete = spectre_de(moyp, dt)
    return {
        "mouvement_par_s": mediane(difs) / max(med, 1e-12) / dt,
        "periode_s": periode, "periode_nettete": nettete,
        "renouvellement_clairs_par_s": mediane(renouvs) / dt if renouvs else float("nan"),
        "coupes": coupes, "plan_s": len(moyp) * dt, "images": n, "dt": dt,
    }


def spectre_de(moy, dt):
    """Le spectre d'une suite (tendance linéaire retirée), de 1/(durée/2) à Nyquist : la période du pic, sa netteté."""
    n = len(moy)
    tt = np.arange(n, dtype=float)
    pente, orig = np.polyfit(tt, moy, 1)
    x = np.asarray(moy) - (orig + pente * tt)
    duree = n * dt
    spectre = []
    for k in range(2, n // 2 + 1):
        ang = 2 * np.pi * k * tt / n
        spectre.append((k / duree, (x * np.cos(ang)).sum() ** 2 + (x * np.sin(ang)).sum() ** 2))
    pic = max(spectre, key=lambda s: s[1])
    return 1 / pic[0], pic[1] / max(mediane([s[1] for s in spectre]), 1e-30)


def mesurer_suite(chemins, zones, dt, f):
    imgs = [reduire(c, f) for c in chemins]
    res = {}
    for nom, z in zones.items():
        suite = [decouper(i, z) for i in imgs]
        st = [statiques(s) for s in suite]
        res[nom] = {
            "statiques": {k: float(mediane([s[k] for s in st])) for k in st[0]},
            "temporelles": {k: float(v) for k, v in temporelles(suite, dt).items()},
        }
    return res


def arrondir(o):
    if isinstance(o, dict):
        return {k: arrondir(v) for k, v in o.items()}
    if isinstance(o, float):
        return float(f"{o:.4g}")
    return o


def egalite(js, py):
    """Les écarts relatifs entre le calcul de la page et celui-ci, grandeur par grandeur."""
    pire = 0.0
    for zone, m in py.items():
        for groupe in ("statiques", "temporelles"):
            for k, v in m[groupe].items():
                w = js[zone][groupe].get(k)
                if w is None or k in ("images", "dt", "coupes"):
                    continue
                e = abs(v - w) / max(abs(v), abs(w), 1e-12)
                pire = max(pire, e)
                if e > 0.01:
                    print(f"BANC_VISUEL egalite ecart zone={zone} {k} py={v:.5g} js={w:.5g} relatif={e:.4f}")
    print(f"BANC_VISUEL egalite pire_ecart_relatif={pire:.5f} {'tenu' if pire <= 0.01 else 'NON TENU'}")
    return pire


def main(argv):
    opts = {a.split("=", 1)[0]: a.split("=", 1)[1] for a in argv[1:] if a.startswith("--") and "=" in a}
    chemins = [a for a in argv[1:] if not a.startswith("--")]
    if not chemins or "--zones" not in opts:
        print(__doc__)
        return 1
    zones = json.loads(opts["--zones"])
    res = arrondir(mesurer_suite(chemins, zones, float(opts.get("--dt", "0.1")), int(opts.get("--facteur", "2"))))
    if "--egalite" in opts:
        js = json.load(open(opts["--egalite"], encoding="utf-8"))
        return 0 if egalite(js.get("mesures", js), res) <= 0.01 else 2
    print(json.dumps(res, ensure_ascii=False, indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
