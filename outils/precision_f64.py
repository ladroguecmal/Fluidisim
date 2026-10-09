# -*- coding: utf-8 -*-
"""Les modules du cœur où `f64` domine, chacun rangé — S642, ADR-260 D2.

I-08 (le calcul d'eau en `f32` local) gouverne les **champs de production** de B, W et δ (ADR-260 D1). Un module du cœur où
`f64` domine doit être rangé ici, dans l'une des catégories d'ADR-260 :

    V  — la couche V : ses intermédiaires en f64 (ADR-139)
    R  — une référence ou un instrument : un juge, une référence CPU, un bilan de diagnostic ; hors du pas du jeu
    O  — un outil hors ligne ou une cuisson : ce qu'il produit est arrondi avant d'être publié
    S  — un calcul scalaire (phase, événement, un corps, une prédiction, une poche) arrondi avant d'entrer dans un champ
    P  — **une référence de champ** : elle doit sa version de production `f32` avant que son point de la liste soit validé

    python outils/precision_f64.py           # l'inventaire
    python outils/precision_f64.py --check   # échoue si un module où f64 domine n'est pas rangé

`etat_projet.py --check` appelle `ecarts`. « Domine » : plus de 15 mentions de `f64`, et plus que de `f32` (les essais exclus).
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "code" / "water-core" / "src"

RANGES = {
    "hydro_network": "V", "hydro_geometry": "V", "hydro_charge": "V",
    "dispersif": "R", "gaussian_pressure": "R", "pressure_mode": "R", "spectrum_reference_s147": "R", "bathymetrie": "R",
    "delta3d_balance": "R", "delta3d_closure": "R", "delta3d_transfer": "R",
    "riviere": "O", "geoide": "O", "portee": "O", "current_field": "O", "gaussian_spectrum": "O", "refraction": "O",
    "background": "S", "background_cwm": "S", "ballistic": "S", "bulle": "S", "rigid_body": "S", "maree": "S", "tsunami": "S",
    "deferlement": "S", "glace": "S", "regional_level": "S", "apic3d_poches": "S", "coule": "S",
    "shallow": "P", "saint_venant_2d": "P", "substitutif": "P", "changement_solveur": "P", "grand_evenement": "P",
    "delta3d_levels": "P", "graine": "P", "precalcul": "P", "impact_field": "P",
    "activite": "S", "body": "S", "goutte": "S", "eponge": "O", "modal_pressure": "R", "pente_douce": "O", "bathymetrie_cote2d": "O", "houle_moyenne": "O", "relais_rivage": "P", "relais_boite": "P", "serre_1d": "P",
}


def domine(texte: str) -> bool:
    n64, n32 = len(re.findall(r"\bf64\b", texte)), len(re.findall(r"\bf32\b", texte))
    return n64 > 15 and n64 > n32


def modules() -> dict:
    return {f.stem: f.read_text(encoding="utf-8") for f in sorted(SRC.glob("*.rs")) if not f.stem.startswith("tests_")}


def ecarts() -> list[str]:
    found = []
    for nom, texte in modules().items():
        if domine(texte) and nom not in RANGES:
            found.append(f"précision : `{nom}.rs` calcule surtout en f64 sans être rangé (ADR-260 D2) — outils/precision_f64.py")
    return found


def main() -> int:
    if "--check" in sys.argv:
        e = ecarts()
        for x in e:
            print(x)
        return 1 if e else 0
    mods = modules()
    for cat in "VROSP":
        noms = [n for n, c in RANGES.items() if c == cat]
        print(cat, len(noms), ", ".join(n for n in noms if n in mods and domine(mods[n])))
    return 0


if __name__ == "__main__":
    sys.exit(main())
