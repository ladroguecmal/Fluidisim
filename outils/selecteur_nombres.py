#!/usr/bin/env python3
"""Les nombres du sélecteur des domaines — S732 (SELECTEUR-DOMAINES-S732 ; ADR-243 : les nombres d'un plan écrits par leur script).

Pour chaque scène de la batterie (une onde solitaire `H/d` sur une plage plane `1:cot`, depuis la profondeur `d`) :

- **déferle-t-elle ?** Synolakis (1987) : une onde solitaire déferle pendant sa remontée si `H/d > 0,818·cot^(-10/9)` ; sinon sa remontée
  est `R/d = 2,831·√cot·(H/d)^(5/4)` (la loi de la remontée, exacte en théorie non linéaire des ondes longues) ;
- **comment ?** Grilli, Svendsen et Subramanya (1997) : `S₀ = 1,521·s/√(H/d)`, glissant sous 0,025, plongeant jusqu'à 0,30, frontal jusqu'à
  0,37, au-delà aucun (S647) ;
- **où retombe le jet ?** Une estimation balistique, à éprouver sur les témoins : la crête part à la célérité `c_b = √(g·(h_b + H_b))` et
  tombe de `H_b` ; `L_jet = c_b·√(2·H_b/g)`. `h_b` et `H_b` sont mesurés sur le témoin quand il existe (S647 : `h_b` = 0,142 m,
  `H_b/h_b` = 1,46) ; sinon l'indice de McCowan (`H_b = 0,78·h_b`) les estime, à revoir dans la session de la scène.

    python outils/selecteur_nombres.py
"""
import math
import sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
G = 9.81

# (nom, d m, H/d, cot, h_b m mesuré ou None, H_b/h_b mesuré ou None)
SCENES = [
    ("S1 — R43 : plongeante près du bord", 0.5, 0.30, 12.0, 0.142, 1.46),
    ("S2 — plongeante loin du bord (Synolakis, labo)", 0.30, 0.30, 19.85, None, None),
    ("S3 — glissante, plage douce", 0.30, 0.50, 90.0, None, None),
    ("S4 — ne déferle pas", 0.5, 0.03, 12.0, None, None),
]


def nombres(d, hd, cot, hb, ratio):
    seuil = 0.818 * cot ** (-10 / 9)
    s0 = 1.521 * (1 / cot) / math.sqrt(hd)
    type_ = "glissant" if s0 < 0.025 else "plongeant" if s0 < 0.30 else "frontal" if s0 < 0.37 else "aucun"
    out = {"seuil H/d": seuil, "déferle": hd > seuil, "S0": s0, "type": type_}
    if hd <= seuil:
        out["remontée R m"] = d * 2.831 * math.sqrt(cot) * hd ** 1.25
        return out
    if hb is None:
        # McCowan sur la loi de Green (ordre de grandeur seulement) : H(h) = H₀·(d/h)^¼ = 0,78·h.
        h0 = hd * d
        hb = (h0 * d ** 0.25 / 0.78) ** (1 / 1.25)
        ratio = 0.78
        out["h_b estimé (Green, McCowan)"] = hb
    Hb = ratio * hb
    cb = math.sqrt(G * (hb + Hb))
    ljet = cb * math.sqrt(2 * Hb / G)
    out.update({"h_b m": hb, "H_b m": Hb, "c_b m/s": cb, "L_jet m": ljet,
                "x_b depuis le pied m": (d - hb) * cot, "rivage depuis le pied m": d * cot})
    return out


def main():
    for nom, d, hd, cot, hb, ratio in SCENES:
        print(f"{nom} (d = {d} m, H/d = {hd}, 1:{cot:g})")
        for k, v in nombres(d, hd, cot, hb, ratio).items():
            print(f"    {k} : {v:.4g}" if isinstance(v, float) else f"    {k} : {v}")
    # Le contrôle sur S1 : le jet retombe, d'après le témoin de S730, où l'air s'enferme (10,375 m) ; le retournement à 9,938–9,988 m.
    s1 = nombres(0.5, 0.30, 12.0, 0.142, 1.46)
    x_b = 9.963
    print(f"S1 contrôle : x_b + L_jet = {x_b + s1['L_jet m']:.3f} m (l'air enfermé du témoin : 10,375 m)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
