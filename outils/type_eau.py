#!/usr/bin/env python3
"""Le type d'eau — S474, ADR-217 : les propriétés optiques d'une eau tirées de ses constituants.

ADR-177 dérive la couleur de l'eau **pure** : `R(0⁻) = 0,33·b_b/(a + b_b)`, `Kd = (a + b_b)/μ̄_d`, `c = a + b`, aux bandes 650, 550
et 450 nm (l'ordre des canaux R, G, B du rendu). Une eau réelle y ajoute trois constituants, mélangés linéairement dans `a`, `b` et
`b_b` — d'où leur mélange d'un endroit à l'autre de la carte (ADR-217 D1) :

- **le phytoplancton**, `Chl` (mg/m³), cas 1 de Morel et Maritorena (2001, JGR 106(C4), 7163–7180) : `Kd = Kw + χ·Chl^e` (Table 2 ;
  la part du chlorophylle, `χ·Chl^e`, ajoutée à notre `Kd` pur, d'où `a_chl = μ̄_d·χ·Chl^e − b_bp`) ; `b_p(550) = 0,416·Chl^0,766`,
  `b_p ∝ λ⁻¹` (éq. 12) ; `b_bp = [0,002 + 0,01·(0,5 − 0,25·log₁₀ Chl)·(λ/550)^ν]·b_p(550)`, `ν = 0,5·(log₁₀ Chl − 0,3)` entre 0,02
  et 2 mg/m³, 0 au-delà (éq. 13–14) ;
- **la matière organique dissoute** (CDOM), `a_g(440)` (m⁻¹) : `a_g(λ) = a_g(440)·exp(−0,0176·(λ − 440))` — la pente moyenne de
  Babin et al. (2003, eaux côtières d'Europe), citée par l'Ocean Optics Web Book ;
- **les particules minérales** (matière en suspension, `MES`, g/m³) : `a_nap(443) = 0,04·MES` (Babin et al. 2003 ; plage 0,03–1,0
  selon le fer : la valeur basse, une argile sans oxyde), pente 0,0123 nm⁻¹ ; `b_p(555) = 0,5·MES` (Babin et al. 2003, eaux côtières
  turbides : 0,5 à 1), `∝ λ⁻¹` ; `b_bp/b_p` = 0,018 (rétrodiffusion des particules minérales — **valeur typique retenue, à vérifier
  sur source**).

La visibilité horizontale : `4,8/c(550)` (Davies-Colley 1988, la distance de vue d'un disque noir — **valeur typique retenue**).

    python outils/type_eau.py            # le tableau des préréglages
    python outils/type_eau.py --egalite=<sortie de mer.tscn -- --controle-type-eau>
"""

import math
import sys

BANDES = (650.0, 550.0, 450.0)
# ADR-177 : l'eau pure (Pope & Fry 1997 ; Morel 1974), `μ̄_d` de S359.
A_EAU = (0.340, 0.0565, 0.00922)
BB_EAU = (0.00071, 0.00145, 0.00344)
MU_D = 0.8
# Morel et Maritorena (2001), Table 2 : (χ, e) à 650, 550, 450 nm.
CHI = (0.04500, 0.04111, 0.10165)
EXPO = (0.67200, 0.64927, 0.67692)

PRESETS = {
    # nom : (Chl mg/m³, a_g(440) m⁻¹, MES g/m³) — dans les plages publiées pour chaque milieu ; réglés ensuite par référence.
    "pure": (0.0, 0.0, 0.0),
    "ocean_clair": (0.03, 0.0, 0.0),
    "mediterranee": (0.1, 0.01, 0.0),
    "cotier": (1.5, 0.1, 2.0),
    "lac": (5.0, 0.5, 3.0),
    "riviere": (3.0, 1.5, 15.0),
    "trouble": (2.0, 0.5, 50.0),
}


def proprietes(chl, ag440, mes):
    """`(a, b, b_b, R0, kd, c)` aux trois bandes (R, G, B)."""
    sortie = {k: [] for k in ("a", "b", "bb", "R0", "kd", "c")}
    for i, lam in enumerate(BANDES):
        a, bb, b = A_EAU[i], BB_EAU[i], 2.0 * BB_EAU[i]
        if chl > 0.0:
            l10 = math.log10(chl)
            bp550 = 0.416 * chl ** 0.766
            nu = 0.5 * (min(max(l10, math.log10(0.02)), math.log10(2.0)) - 0.3) if chl <= 2.0 else 0.0
            bbp = (0.002 + 0.01 * (0.5 - 0.25 * l10) * (lam / 550.0) ** nu) * bp550
            a += max(MU_D * CHI[i] * chl ** EXPO[i] - bbp, 0.0)
            bb += bbp
            b += bp550 * 550.0 / lam
        a += ag440 * math.exp(-0.0176 * (lam - 440.0))
        if mes > 0.0:
            a += 0.04 * mes * math.exp(-0.0123 * (lam - 443.0))
            bpm = 0.5 * mes * 555.0 / lam
            b += bpm
            bb += 0.018 * bpm
        sortie["a"].append(a)
        sortie["b"].append(b)
        sortie["bb"].append(bb)
        sortie["R0"].append(0.33 * bb / (a + bb))
        sortie["kd"].append((a + bb) / MU_D)
        sortie["c"].append(a + b)
    return sortie


def egalite(chemin):
    """Les lignes `TYPE_EAU_S474 <nom> <clé> x y z` de `mer.tscn -- --controle-type-eau` contre ce calcul."""
    pire = 0.0
    n = 0
    for ligne in open(chemin, encoding="utf-8", errors="replace"):
        m = ligne.split()
        if len(m) != 6 or m[0] != "TYPE_EAU_S474" or m[1] not in PRESETS:
            continue
        p = proprietes(*PRESETS[m[1]])[m[2]]
        for i in range(3):
            e = abs(float(m[3 + i]) - p[i]) / max(abs(p[i]), 1e-12)
            pire = max(pire, e)
            n += 1
    # 10⁻⁵ : `Vector3` de Godot est en flottants de 32 bits (S474 : le critère écrit avant disait 10⁻⁶, trop strict pour eux).
    print(f"TYPE_EAU egalite valeurs={n} pire_ecart_relatif={pire:.2e} {'tenu' if n and pire <= 1e-5 else 'NON TENU'}")
    return 0 if n and pire <= 1e-5 else 2


def main():
    sys.stdout.reconfigure(encoding="utf-8")
    for a in sys.argv[1:]:
        if a.startswith("--egalite="):
            return egalite(a.split("=", 1)[1])
    print("| préréglage | Chl | a_g(440) | MES | R0 (R, G, B) | B/G de R0 | kd (R, G, B) | c(550) | visibilité (m) |")
    print("|---|---:|---:|---:|---|---:|---|---:|---:|")
    for nom, (chl, ag, mes) in PRESETS.items():
        p = proprietes(chl, ag, mes)
        r0, kd, c = p["R0"], p["kd"], p["c"]
        print(f"| {nom} | {chl:g} | {ag:g} | {mes:g} | {r0[0]:.5f}, {r0[1]:.5f}, {r0[2]:.5f} | {r0[2] / r0[1]:.2f} | "
              f"{kd[0]:.4f}, {kd[1]:.4f}, {kd[2]:.4f} | {c[1]:.4f} | {4.8 / c[1]:.1f} |")
    return 0


if __name__ == "__main__":
    sys.exit(main())
