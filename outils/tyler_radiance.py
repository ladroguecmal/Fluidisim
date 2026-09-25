"""S366 — `f(ω)`, la lumière de l'eau selon la direction de visée, contre la radiance mesurée par Tyler (1960).

Usage : python outils/tyler_radiance.py

**La mesure.** Tyler (1960, *Radiance distribution as a function of depth in an underwater environment*, Bull. Scripps
Inst. Oceanogr. 7, 363–411), lac Pend Oreille : la radiance dans le plan du soleil, à 4,2 m, normalisée à 1 au nadir
(vue vers le bas) ; reproduite par Mobley, *Ocean Optics Web Book*, « The Asymptotic Radiance Distribution », fig. 7.
Les points ci-dessous sont **numérisés en S366** sur les pixels de cette figure (axe logarithmique, 32,75 pixels par
décade ; abscisse linéaire, 195,5 pixels pour 180°) : deux azimuts, côté soleil et à l'opposé, de 10° à 90° du nadir —
la moitié du champ que la vue sous l'eau emploie (le miroir et l'horizontale ; au-dessus, la surface). Le soleil, pic de
la courbe, à 30,4° du zénith dans l'eau.

**Les deux modèles.** S365 : `f = 3 − 2·cos θ` (1 au nadir, 3 à l'horizontale, sans azimut). S366 : un lobe avant de
Henyey-Greenstein autour du soleil réfracté, `f = a0 + β·(1 − cos θ) + K·HG(g, ω·s)`, `a0` tel que `f(nadir) = 1` ; β,
K, g ajustés en moindres carrés sur le logarithme. La mesure est celle d'une eau de lac, où les particules diffusent
vers l'avant ; notre eau est pure (ADR-177) — le lobe lui est emprunté, et c'est dit.
"""
import math

# (θ depuis le nadir en degrés, radiance à l'opposé du soleil, radiance côté soleil) — Tyler 1960, 4,2 m.
TYLER = [
    (10.0, 1.007, 1.007), (20.0, 1.057, 1.065), (30.0, 1.057, 1.151), (40.0, 1.159, 1.417), (50.0, 1.315, 1.755),
    (60.0, 1.517, 2.457), (70.0, 1.735, 3.638), (80.0, 2.020, 5.487), (90.0, 2.457, 8.710),
]
SOLEIL_ZENITH_EAU = 30.4


def hg(g, mu):
    return (1.0 - g * g) / (4.0 * math.pi * (1.0 + g * g - 2.0 * g * mu) ** 1.5)


def points():
    s = (math.sin(math.radians(SOLEIL_ZENITH_EAU)), math.cos(math.radians(SOLEIL_ZENITH_EAU)))
    sortie = []
    for theta, anti, soleil in TYLER:
        t = math.radians(theta)
        # Direction de visée (horizontale le long de l'azimut du soleil, verticale) ; vers le bas : −cos θ.
        for cote, mesure in ((1.0, soleil), (-1.0, anti)):
            omega = (cote * math.sin(t), -math.cos(t))
            sortie.append((theta, cote, mesure, omega[0] * s[0] + omega[1] * s[1], -omega[1]))
    return sortie, s


def f_s365(cos_theta):
    return 3.0 - 2.0 * cos_theta


def f_s366(params, mu, cos_theta, mu_nadir):
    beta, k, g = params
    a0 = 1.0 - k * hg(g, mu_nadir)
    return a0 + beta * (1.0 - cos_theta) + k * hg(g, mu)


def residu(params, pts, mu_nadir):
    return math.sqrt(sum(math.log(f_s366(params, mu, c, mu_nadir) / m) ** 2 for (_, _, m, mu, c) in pts) / len(pts))


def main():
    pts, s = points()
    mu_nadir = -s[1]
    avant = math.sqrt(sum(math.log(f_s365(c) / m) ** 2 for (_, _, m, _, c) in pts) / len(pts))
    # Recherche sur grille puis affinage (pas de dépendance) ; K > 0, 0 < g < 0,99.
    meilleur = None
    for beta in [0.25 * i for i in range(0, 13)]:
        for k in [0.5 * 1.3 ** i for i in range(0, 20)]:
            for g in [0.05 * i for i in range(1, 20)]:
                p = (beta, k, g)
                if min(f_s366(p, mu, c, mu_nadir) for (_, _, _, mu, c) in pts) <= 0:
                    continue
                r = residu(p, pts, mu_nadir)
                if meilleur is None or r < meilleur[0]:
                    meilleur = (r, p)
    r, p = meilleur
    pas = [0.1, 0.1 * p[1], 0.02]
    for _ in range(200):
        ameliore = False
        for i in range(3):
            for signe in (1, -1):
                q = list(p)
                q[i] += signe * pas[i]
                if q[2] <= 0 or q[2] >= 0.99 or q[1] <= 0:
                    continue
                if min(f_s366(q, mu, c, mu_nadir) for (_, _, _, mu, c) in pts) <= 0:
                    continue
                rq = residu(q, pts, mu_nadir)
                if rq < r:
                    r, p, ameliore = rq, tuple(q), True
        if not ameliore:
            pas = [x / 2 for x in pas]
    print(f"TYLER_S366 points={len(pts)} soleil_zenith_eau={SOLEIL_ZENITH_EAU} residu_log_rms S365={avant:.3f} "
          f"S366={r:.3f}")
    print(f"TYLER_S366 ajustement beta={p[0]:.4f} K={p[1]:.4f} g={p[2]:.4f} a0={1.0 - p[1] * hg(p[2], mu_nadir):.4f}")
    for (theta, cote, m, mu, c) in pts:
        print(f"TYLER_S366 theta={theta:4.0f} {'soleil' if cote > 0 else 'oppose'} mesure={m:6.3f} "
              f"S365={f_s365(c):6.3f} S366={f_s366(p, mu, c, mu_nadir):6.3f}")
    return 0


if __name__ == "__main__":
    main()
