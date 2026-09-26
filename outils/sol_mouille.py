"""S392 — les surfaces mouillées : les nombres du film d'eau (ADR-205, pièce 5a).

Ångström (1925), Lekner et Dorf (1988, *Appl. Opt.* 27, 1278) : un matériau rugueux d'albédo `a` sous un film d'eau. La
lumière entre dans le film, se diffuse sur le matériau ; à chaque remontée, l'interface eau–air en renvoie vers le bas une
part `r_i` (réflexion totale au-delà de l'angle critique, partielle en deçà). Sommés, les rebonds donnent le flux sortant
`E·(1 − r_e)·a·(1 − r_i)/(1 − a·r_i)` ; la sortie d'un champ lambertien sous l'eau, réfracté, donne la radiance dans la
direction `θ` : `L(θ) = (E/π)·a·(1 − r_i)/(1 − a·r_i)·(1 − R(θ))` — le `(1 − r_e)` d'entrée et celui de la sortie se
compensent, par la réciprocité `1 − r_i = (1 − r_e)/n²`.

`r_e` et `r_i` sont calculés ici par **deux intégrations indépendantes** du Fresnel non polarisé, pondéré par le cosinus,
l'une vue de l'air, l'autre vue de l'eau ; la réciprocité les relie. Python standard, sans dépendance.

Usage : `python outils/sol_mouille.py [n]` — imprime `r_e`, `r_i`, `R(0)` et les rapports mouillé / sec de quelques albédos.
"""
import math
import sys

N_EAU = 1.333


def fresnel(cos_i: float, n1: float, n2: float) -> float:
    """Réflectance de Fresnel non polarisée, de l'indice `n1` vers `n2`, `cos_i` le cosinus d'incidence ; 1 au-delà de
    l'angle critique."""
    cos_i = min(max(cos_i, 0.0), 1.0)
    sin_t = n1 / n2 * math.sqrt(max(0.0, 1.0 - cos_i * cos_i))
    if sin_t >= 1.0:
        return 1.0
    cos_t = math.sqrt(1.0 - sin_t * sin_t)
    rs = ((n1 * cos_i - n2 * cos_t) / (n1 * cos_i + n2 * cos_t)) ** 2
    rp = ((n1 * cos_t - n2 * cos_i) / (n1 * cos_t + n2 * cos_i)) ** 2
    return 0.5 * (rs + rp)


def reflectance_diffuse(n1: float, n2: float, pas: int = 200_000) -> float:
    """Réflectance hémisphérique d'un éclairement lambertien venu de l'indice `n1` sur une interface vers `n2` :
    `∫ R(θ)·2·sin θ·cos θ dθ` sur `[0, π/2]`, point milieu."""
    h = (math.pi / 2) / pas
    total = 0.0
    for k in range(pas):
        t = (k + 0.5) * h
        total += fresnel(math.cos(t), n1, n2) * 2.0 * math.sin(t) * math.cos(t)
    return total * h


def film(n: float = N_EAU) -> dict:
    """`r_e` (vue de l'air), `r_i` (vue de l'eau), la réciprocité `1 − (1 − r_e)/n²`, et `R(0)`."""
    r_e = reflectance_diffuse(1.0, n)
    r_i = reflectance_diffuse(n, 1.0)
    return {"r_e": r_e, "r_i": r_i, "reciprocite": 1.0 - (1.0 - r_e) / (n * n), "R0": fresnel(1.0, 1.0, n)}


def albedo_mouille(a: float, r_i: float) -> float:
    """L'albédo effectif du matériau mouillé, hors reflet et hors Fresnel de sortie : `a·(1 − r_i)/(1 − a·r_i)`."""
    return a * (1.0 - r_i) / (1.0 - a * r_i)


def rapport(a: float, cos_vue: float, n: float = N_EAU, r_i: float | None = None) -> float:
    """Radiance diffuse mouillée rapportée à la sèche, vue sous `cos_vue` : `(1 − r_i)/(1 − a·r_i)·(1 − R(θ))`."""
    if r_i is None:
        r_i = film(n)["r_i"]
    return albedo_mouille(a, r_i) / a * (1.0 - fresnel(cos_vue, 1.0, n))


def main() -> None:
    n = float(sys.argv[1]) if len(sys.argv) > 1 else N_EAU
    f = film(n)
    print(f"SOL_MOUILLE_S392 n={n} r_e={f['r_e']:.5f} r_i={f['r_i']:.5f} reciprocite={f['reciprocite']:.5f} R0={f['R0']:.5f}")
    for a in (0.1, 0.2, 0.3, 0.42, 0.6, 0.9):
        print(f"SOL_MOUILLE_S392 a={a} mouille={albedo_mouille(a, f['r_i']):.4f} rapport_aplomb={rapport(a, 1.0, n, f['r_i']):.4f}")


if __name__ == "__main__":
    main()
