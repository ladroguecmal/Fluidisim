#!/usr/bin/env python3
"""Le diagnostic de S4 — S735 (TEMOINS-SELECTEUR-S734 : le front figé sur la pente de 1:3).

Lit les instantanés `calculs/s735_S4_lisse_<quoi>_<t ms>_{particules,colonnes}.csv` écrits par `the_s4_front_diagnostic_s735` et dit, pour
chaque colonne de la plage, deux lectures indépendantes de l'épaisseur d'eau (ADR-280 D1) :

- **lue** par φ (`h_phi`, ce que voit le solveur) ;
- **comptée** par les particules de la rangée du milieu : `n·(dx³/8)/dx² = n·dx/8`.

Le critère du plan :
- **« eau collée »** : au-delà de 12,5 m, après 4,5 s, des particules (au moins 5 mm comptés sur une colonne), presque immobiles
  (|v| < 5 cm/s en moyenne) ;
- **« lecture fausse »** : 5 mm lus là où les particules en donnent moins de 1 mm.

    python outils/diagnostic_s735.py
"""
import glob
import re
import sys
from pathlib import Path

import numpy as np

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
RACINE = Path(__file__).resolve().parents[1]
DX = 0.025


def colonnes(chemin):
    out = []
    for ligne in open(chemin, encoding='utf-8').read().splitlines()[1:]:
        x, fm, fl, h, n, mailles = ligne.split(';', 5)
        out.append((float(x), float(fm), float(fl), float(h), int(n), mailles))
    return out


def particules(chemin):
    a = np.loadtxt(chemin, delimiter=';', skiprows=1, ndmin=2)
    return a


def main():
    fichiers = sorted(glob.glob(str(RACINE / 'calculs' / 's735_S4_lisse_*_colonnes.csv')))
    if not fichiers:
        sys.exit("aucun instantané (lancer the_s4_front_diagnostic_s735)")
    for f in fichiers:
        quoi, tms = re.search(r'lisse_(\w+?)_(\d+)_colonnes', f).groups()
        t = int(tms) / 1000
        cols = colonnes(f)
        p = particules(f.replace('_colonnes', '_particules'))
        print(f"== {quoi} à {t:.3f} s ({len(p)} particules dans la fenêtre)")
        if quoi == 'retournement':
            for x, fm, fl, h, n, m in cols:
                print(f"   x = {x:.3f} m : fond {fm:.3f} / lisse {fl:.3f} ; h_φ {h * 1e3:.1f} mm ; {n} particules ; mailles {m[:200]}")
            continue
        collee, fausse = [], []
        for x, fm, fl, h, n, m in cols:
            hc = n * DX / 8
            if x > 12.5 and t >= 4.5 and hc >= 0.005:
                sel = (p[:, 0] >= x - DX / 2) & (p[:, 0] < x + DX / 2)
                v = p[sel, 5].mean() if sel.any() else float('nan')
                if v < 0.05:
                    collee.append((x, hc, v))
            if h > 0.005 and hc < 0.001:
                fausse.append((x, h, hc))
        devant = [c for c in cols if c[3] > 0.001 or c[4] > 0]
        if devant:
            xf = max(c[0] for c in devant)
            print(f"   la dernière colonne mouillée (lue ou comptée) : {xf:.3f} m")
            for x, fm, fl, h, n, m in cols:
                if xf - 0.3 <= x <= xf + 0.01:
                    print(f"   x = {x:.3f} : lisse {fl:.3f} m ; h_φ {h * 1e3:5.1f} mm ; compté {n * DX / 8 * 1e3:5.1f} mm ({n})")
        print(f"   « eau collée » : {len(collee)} colonnes {[(round(x, 3), round(hc * 1e3, 1), round(v, 3)) for x, hc, v in collee[:8]]}")
        print(f"   « lecture fausse » : {len(fausse)} colonnes {[(round(x, 3), round(h * 1e3, 1), round(hc * 1e3, 2)) for x, h, hc in fausse[:8]]}")
        if len(p):
            haut = p[p[:, 0] > 12.5]
            if len(haut):
                print(f"   particules au-delà de 12,5 m : {len(haut)}, |v| moyen {haut[:, 5].mean():.3f} m/s, z de {haut[:, 2].min():.3f} à {haut[:, 2].max():.3f} m")
    return 0


if __name__ == '__main__':
    sys.exit(main())
