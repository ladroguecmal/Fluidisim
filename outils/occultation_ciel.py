"""S382 — la part du ciel vue, recalculée indépendamment du nuanceur (critère 2 de S382, ADR-206).

Lit les lignes `CONTROLE_OCCULTATION_S382` de `godot --path godot res://piscine.tscn -- --controle-occultation` (fichier en
argument, ou entrée standard) : les occultants (boîtes alignées, monde de Godot, y en haut) et, pour chaque point d'essai,
la part vue que le rendu a produite. Pour chacun, l'intégrale refaite **autrement** que le nuanceur : des rayons 3D contre
les boîtes (test de dalles en trois dimensions, pas de décomposition en azimut et élévation), sur une grille fine du ciel
en `u = sin h` et `φ` (`dω = du·dφ`), pondérée par `max(n·ω, 0)` — et par `(1 + 2u)/3` pour le ciel couvert de la CIE —,
rapportée au total de la même grille. Critère : écart ≤ 0,01 ; loin des occultants, 1 exactement.

    python outils/occultation_ciel.py journal.txt
"""
import json
import re
import sys

import numpy as np

N_U, N_PHI = 1200, 2400
SEUIL = 0.01


def directions():
    u = (np.arange(N_U) + 0.5) / N_U
    phi = (np.arange(N_PHI) + 0.5) * 2 * np.pi / N_PHI
    uu, pp = np.meshgrid(u, phi, indexing="ij")
    ch = np.sqrt(1 - uu ** 2)
    w = np.stack([ch * np.cos(pp), uu, ch * np.sin(pp)], -1).reshape(-1, 3)
    return w, uu.reshape(-1)


def part_vue(p, n, boites, w, u):
    q = np.asarray(p, float) + 1e-4 * np.asarray(n, float)
    vu = np.ones(len(w), bool)
    with np.errstate(divide="ignore", invalid="ignore"):
        inv = 1.0 / w
        for b in boites:
            lo, hi = np.array(b[:3]), np.array(b[3:])
            ta = (lo - q) * inv
            tb = (hi - q) * inv
            t0 = np.nanmax(np.minimum(ta, tb), axis=1)
            t1 = np.nanmin(np.maximum(ta, tb), axis=1)
            vu &= ~((t1 > np.maximum(t0, 0.0)))
    c = np.clip(w @ np.asarray(n, float), 0, None)
    cie = c * (1 + 2 * u) / 3
    return (c * vu).sum() / c.sum(), (cie * vu).sum() / cie.sum()


def main():
    texte = open(sys.argv[1], encoding="utf-8", errors="replace").read() if len(sys.argv) > 1 else sys.stdin.read()
    m = re.search(r"CONTROLE_OCCULTATION_S382 occultants=(\[.*\])", texte)
    boites = json.loads(m.group(1))
    w, u = directions()
    pire = 0.0
    exacts = True
    motif = r"point=(\S+) p=(\[[^\]]*\]) n=(\[[^\]]*\]) vu_uniforme=([\d.]+) vu_cie=([\d.]+)"
    for nom, p, n, ru, rc in re.findall(motif, texte):
        p, n = json.loads(p), json.loads(n)
        eu, ec = part_vue(p, n, boites, w, u)
        du, dc = float(ru) - eu, float(rc) - ec
        pire = max(pire, abs(du), abs(dc))
        loin = eu == 1.0
        if loin and (float(ru) != 1.0 or float(rc) != 1.0):
            exacts = False
        print(f"{nom:20s} rendu {float(ru):.5f} / {float(rc):.5f}   référence {eu:.5f} / {ec:.5f}   écart {du:+.5f} / {dc:+.5f}"
              + ("   (aucun occultant vu : 1 exact)" if loin else ""))
    print(f"{len(boites)} occultants ; pire écart {pire:.5f} ; seuil {SEUIL} : {'tenu' if pire <= SEUIL and exacts else 'manqué'}"
          + ("" if exacts else " (un point sans occultant n'est pas rendu à 1 exactement)"))


if __name__ == "__main__":
    main()
