"""S386 — C2 (ADR-207 D2) : la dispersion d'une colonne à colonne haute ; bibliothèque standard seulement.

**Le schéma fin (S295).** Pour un mode horizontal `(kx, ky)` du pas linéaire de δ, la pression est, colonne par colonne,
la solution d'un problème vertical : `n` mailles de hauteur `dx`, Neumann au fond, Dirichlet à demi-maille au couvercle
(valeur 1). Écrit sous forme `A = D·M⁻¹·Dᵀ` : `D` la divergence d'une maille vers ses faces — une face « horizontale » par
maille, de coefficient `κ·dx` (`κ²` la valeur propre discrète du laplacien horizontal), les faces verticales internes, et la
face du couvercle, de poids `M⁻¹ = 2` (demi-distance). La réponse `φ` donne la profondeur effective `s = Σ φ_k·dx`,
`ω² = g·κ²·s`, puis le pas de temps : `Ω = acos(1 − ω²·dt²/2)/dt`. C'est `scheme_frequency` de `tests_delta3d.rs`.

**La colonne haute.** Les `m` mailles du bas n'ont plus qu'une pression **linéaire**, `p_k = a + b·(k − c)`,
`c = (m − 1)/2` : une restriction `P`, deux inconnues au lieu de `m`. Le schéma réduit est la **restriction de Galerkin**
du schéma fin — contraintes testées par `Pᵀ`, vitesses dans un espace `Q`, masse `M̂ = Qᵀ·M·Q` :

    Aᵣ = (Pᵀ·D·Q)·M̂⁻¹·(Pᵀ·D·Q)ᵀ,    second membre Pᵀ·d,

symétrique défini positif par construction. Deux choix de `Q` :

- **G** — les vitesses restent libres (`Q = I`) : `Aᵣ = Pᵀ·A·P` ;
- **Q** — la vitesse horizontale linéaire dans la colonne haute (exact : le gradient d'une pression linéaire l'est), et
  la vitesse verticale interne liée à celle du haut de la colonne haute, `w_f = (f/m)·w_m`.

Dans les deux cas, la vitesse horizontale d'une pression linéaire est représentée exactement : `s = Σ (P·φ̂)_k·dx` et
`ω² = g·κ²·s` valent toujours.

**Deux généralisations**, mesurées parce qu'une seule colonne haute linéaire manque le critère (S386, P4) :

- **N** — restriction de Galerkin **linéaire par morceaux** : des nœuds aux centres de mailles fines, un par maille dans
  les couches cubiques du haut, puis des écarts qui croissent d'un rapport `r` vers le fond ; `Aᵣ = Pᵀ·A·P`. La colonne
  haute unique en est le cas à deux nœuds ;
- **E** — la grille **étirée** en volumes finis : couches d'épaisseur `h_k`, faces verticales à la distance des centres,
  couvercle à demi-couche ; tridiagonale.

    python outils/colonnes_hautes.py [--tout]   # contrôle S295 et balayage de la porte B
"""
import math
import sys


def solve(a, b):
    """Gauss avec pivot partiel ; `a` carrée (liste de listes), `b` vecteur. Copie ses entrées."""
    n = len(b)
    a = [row[:] + [b[i]] for i, row in enumerate(a)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(a[r][col]))
        a[col], a[piv] = a[piv], a[col]
        for r in range(col + 1, n):
            f = a[r][col] / a[col][col]
            if f:
                for c in range(col, n + 1):
                    a[r][c] -= f * a[col][c]
    x = [0.0] * n
    for r in range(n - 1, -1, -1):
        x[r] = (a[r][n] - math.fsum(a[r][c] * x[c] for c in range(r + 1, n))) / a[r][r]
    return x


def matmul(a, b):
    bt = list(zip(*b))
    return [[math.fsum(x * y for x, y in zip(row, col)) for col in bt] for row in a]


def transpose(a):
    return [list(r) for r in zip(*a)]


def faces(n, kdx):
    """Divergence `D` (n × F) et poids `M⁻¹` des faces : n horizontales, n − 1 verticales internes, le couvercle."""
    nf = n + (n - 1) + 1
    d = [[0.0] * nf for _ in range(n)]
    minv = [1.0] * nf
    for k in range(n):
        d[k][k] = kdx
    for f in range(1, n):  # face verticale f, entre la maille f − 1 (dessous) et f (dessus)
        col = n + (f - 1)
        d[f - 1][col] = 1.0
        d[f][col] = -1.0
    top = nf - 1
    d[n - 1][top] = 1.0
    minv[top] = 2.0
    return d, minv, top


def restriction_pressure(n, m):
    """`P` (n × r) : les `m` mailles du bas linéaires (a, b), les autres identité ; `m < 2` : l'identité."""
    if m < 2:
        return [[1.0 if i == j else 0.0 for j in range(n)] for i in range(n)]
    r = n - m + 2
    c = (m - 1) / 2
    p = [[0.0] * r for _ in range(n)]
    for k in range(m):
        p[k][0] = 1.0
        p[k][1] = k - c
    for k in range(m, n):
        p[k][2 + k - m] = 1.0
    return p


def restriction_velocity(n, m):
    """`Q` (F × F̂) de la variante Q : horizontales des `m` mailles du bas linéaires, verticales internes liées à `w_m`."""
    nf = 2 * n
    if m < 2:
        return [[1.0 if i == j else 0.0 for j in range(nf)] for i in range(nf)]
    c = (m - 1) / 2
    # Degrés gardés : (Ū, σ), horizontales k ≥ m, verticales f ≥ m (dont w_m), le couvercle.
    cols = ["U", "S"] + [("h", k) for k in range(m, n)] + [("w", f) for f in range(m, n)] + ["top"]
    index = {key: j for j, key in enumerate(cols)}
    q = [[0.0] * len(cols) for _ in range(nf)]
    for k in range(n):
        if k < m:
            q[k][index["U"]] = 1.0
            q[k][index["S"]] = k - c
        else:
            q[k][index[("h", k)]] = 1.0
    for f in range(1, n):
        row = n + (f - 1)
        if f < m:
            q[row][index[("w", m)]] = f / m
        else:
            q[row][index[("w", f)]] = 1.0
    q[nf - 1][index["top"]] = 1.0
    return q


def column(n, dx, kappa2, m, variant="G"):
    """Profondeur effective `s = Σ φ·dx`, la réponse `φ` et l'opérateur réduit, pour `m` mailles en colonne haute."""
    kdx = math.sqrt(kappa2) * dx
    d, minv, top = faces(n, kdx)
    p = restriction_pressure(n, m)
    pt = transpose(p)
    if variant == "G" or m < 2:
        dm = [[x * w for x, w in zip(row, minv)] for row in d]
        a = matmul(dm, transpose(d))
        ar = matmul(matmul(pt, a), p)
    else:
        q = restriction_velocity(n, m)
        mass = [1.0 / w for w in minv]
        mq = [[x * mass[i] for x in row] for i, row in enumerate(q)]
        mhat = matmul(transpose(q), mq)
        b = matmul(matmul(pt, d), q)
        # Aᵣ = B·M̂⁻¹·Bᵀ : M̂⁻¹·Bᵀ colonne par colonne.
        bt = transpose(b)
        cols = [solve(mhat, [row[j] for row in bt]) for j in range(len(b))]
        ar = matmul(b, transpose(cols))
    rhs_fine = [0.0] * n
    rhs_fine[n - 1] = 2.0  # le couvercle, valeur 1, à demi-maille
    rhs = [math.fsum(pt[i][k] * rhs_fine[k] for k in range(n)) for i in range(len(pt))]
    phi_r = solve(ar, rhs)
    phi = [math.fsum(p[k][j] * phi_r[j] for j in range(len(phi_r))) for k in range(n)]
    return math.fsum(x * dx for x in phi), phi, ar


def graded_nodes(n, kept, r):
    """Nœuds de la variante N : les `kept` mailles du haut, puis des écarts arrondis de `r`, `r²`… jusqu'au fond."""
    nodes = list(range(n - kept, n))
    gap, pos = 1.0, n - kept
    while pos > 0:
        gap *= r
        pos = max(0, pos - max(1, round(gap)))
        nodes.insert(0, pos)
    return nodes


def restriction_nodes(n, nodes):
    """`P` (n × nœuds) : interpolation linéaire entre nœuds consécutifs ; le dernier nœud est la maille du haut."""
    p = [[0.0] * len(nodes) for _ in range(n)]
    for j in range(len(nodes) - 1):
        a, b = nodes[j], nodes[j + 1]
        for k in range(a, b):
            t = (k - a) / (b - a)
            p[k][j] = 1 - t
            p[k][j + 1] = t
    p[nodes[-1]][len(nodes) - 1] = 1.0
    return p


def column_nodes(n, dx, kappa2, nodes):
    """Variante N : `s = Σ (P·φ̂)_k·dx`, `Aᵣ = Pᵀ·A·P`."""
    kdx = math.sqrt(kappa2) * dx
    d, minv, _ = faces(n, kdx)
    a = matmul([[x * w for x, w in zip(row, minv)] for row in d], transpose(d))
    p = restriction_nodes(n, nodes)
    pt = transpose(p)
    ar = matmul(matmul(pt, a), p)
    rhs = [2.0 * pt[i][n - 1] for i in range(len(pt))]
    phi_r = solve(ar, rhs)
    phi = [math.fsum(p[k][j] * phi_r[j] for j in range(len(phi_r))) for k in range(n)]
    return math.fsum(x * dx for x in phi), ar


def stretched_layers(depth, dx, kept, r):
    """Épaisseurs de la variante E, du fond vers le haut : `kept` couches de `dx`, puis `dx·r`, `dx·r²`… ; un reste plus
    mince que la moitié de la couche suivante est absorbé par la dernière."""
    hs, z, h = [dx] * kept, kept * dx, dx
    below = []
    while z < depth - 1e-9:
        h = min(h * r, depth - z)
        if 0 < depth - z - h < 0.5 * h * r:
            h = depth - z
        below.append(h)
        z += h
    return below[::-1] + hs


def stretched_s(hs, kappa2):
    """Variante E : `s = Σ φ_k·h_k`, tridiagonale en volumes finis (Thomas)."""
    n = len(hs)
    a, b, c, d = [0.0] * n, [0.0] * n, [0.0] * n, [0.0] * n
    for k in range(n):
        b[k] = kappa2 * hs[k]
        if k > 0:
            dd = (hs[k - 1] + hs[k]) / 2
            b[k] += 1 / dd
            a[k] = -1 / dd
        if k + 1 < n:
            dd = (hs[k] + hs[k + 1]) / 2
            b[k] += 1 / dd
            c[k] = -1 / dd
        else:
            b[k] += 2 / hs[k]
            d[k] = 2 / hs[k]
    for k in range(1, n):
        f = a[k] / b[k - 1]
        b[k] -= f * c[k - 1]
        d[k] -= f * d[k - 1]
    phi = [0.0] * n
    phi[-1] = d[-1] / b[-1]
    for k in range(n - 2, -1, -1):
        phi[k] = (d[k] - c[k] * phi[k + 1]) / b[k]
    return math.fsum(p * h for p, h in zip(phi, hs))


def omega_of(ws2, dt):
    return math.acos(1 - ws2 * dt * dt / 2) / dt


def frequencies(kx, ky, dx, n, g, dt, m=0, variant="G"):
    """(Ω du schéma, ω continue) ; profondeur `n·dx`."""
    kappa2 = 4 / (dx * dx) * (math.sin(kx * dx / 2) ** 2 + math.sin(ky * dx / 2) ** 2)
    s, _, _ = column(n, dx, kappa2, m, variant)
    ws2 = g * kappa2 * s
    big = math.acos(1 - ws2 * dt * dt / 2) / dt
    k = math.hypot(kx, ky)
    return big, math.sqrt(g * k * math.tanh(k * n * dx))


def s295():
    """Les deux cas de l'onde oblique de S295 : mode (1, 1) d'une cuve de 8 × 4 m, h = 4 m."""
    out = []
    for n, dt in [(16, 2e-3), (32, 1e-3)]:
        dx = 8 / n
        big, omega = frequencies(math.pi / 8, math.pi / 4, dx, round(4 / dx), 9.81, dt)
        out.append((n, big / omega - 1))
    return out


def wavelengths(h, dx):
    """De `4·dx` à `2·h`, 25 longueurs d'onde en progression géométrique."""
    return [4 * dx * (2 * h / (4 * dx)) ** (i / 24) for i in range(25)]


def worst_ratio(h, dx, dt, tall, g=9.81):
    """Le pire rapport de l'écart ajouté au critère 2 de S386, `|Ω_haut/Ω_fin − 1| / max(|Ω_fin/ω − 1|, 10⁻⁴)`, sur les
    longueurs d'onde ; `tall(kx, κ²)` rend `ω²` du schéma essayé. Rend (rapport, λ du pire)."""
    n = round(h / dx)
    worst, where = 0.0, None
    for lam in wavelengths(h, dx):
        kx = 2 * math.pi / lam
        fine, omega = frequencies(kx, 0, dx, n, g, dt)
        kappa2 = 4 / (dx * dx) * math.sin(kx * dx / 2) ** 2
        ratio = abs(omega_of(tall(kx, kappa2), dt) / fine - 1) / max(abs(fine / omega - 1), 1e-4)
        if ratio > worst:
            worst, where = ratio, lam
    return worst, where


def sweep(h, dx, dt, g=9.81):
    """Chaque variante et chaque réglage : (variante, réglage, inconnues par colonne, rapport, λ du pire)."""
    n = round(h / dx)
    rows = []
    for kept in range(1, n - 1):
        m = n - kept
        for variant in ("G", "Q"):
            tall = lambda kx, k2, m=m, v=variant: g * k2 * column(n, dx, k2, m, v)[0]
            rows.append((variant, f"couches_cubiques={kept}", kept + 2, *worst_ratio(h, dx, dt, tall, g)))
    for r in (1.2, 1.25, 1.3, 1.4, 1.5, 1.7, 2.0):
        for kept in range(1, 13):
            nodes = graded_nodes(n, kept, r)
            tall = lambda kx, k2, nodes=nodes: g * k2 * column_nodes(n, dx, k2, nodes)[0]
            rows.append(("N", f"r={r} couches_cubiques={kept}", len(nodes), *worst_ratio(h, dx, dt, tall, g)))
            hs = stretched_layers(h, dx, kept, r)
            tall = lambda kx, k2, hs=hs: g * k2 * stretched_s(hs, k2)
            rows.append(("E", f"r={r} couches_cubiques={kept}", len(hs), *worst_ratio(h, dx, dt, tall, g)))
    return rows


def main():
    print("S386 — contrôle : le schéma fin redonne S295")
    for n, e in s295():
        print(f"COLONNES_HAUTES_S386 controle n={n} omega_schema_sur_omega_moins_1={e:+.4e}")
    h, dx, dt = 7.0, 0.25, 1 / 30
    n = round(h / dx)
    print(f"S386 — porte B (h = {h} m, dx = {dx} m, dt = {dt:.5f} s, {n} couches fines) : le moins d'inconnues qui tient")
    rows = sweep(h, dx, dt)
    for variant in ("G", "Q", "E", "N"):
        held = [r for r in rows if r[0] == variant and r[3] <= 1]
        best = min(held, key=lambda r: (r[2], r[3])) if held else None
        if best:
            print(f"COLONNES_HAUTES_S386 variante={variant} {best[1]} inconnues={best[2]} division={n / best[2]:.2f} "
                  f"rapport_au_critere={best[3]:.3f} pire_lambda_m={best[4]:.2f}")
        else:
            print(f"COLONNES_HAUTES_S386 variante={variant} aucun reglage ne tient")
    if "--tout" in sys.argv:
        for v, reglage, inc, ratio, lam in rows:
            print(f"COLONNES_HAUTES_S386_TOUT variante={v} {reglage} inconnues={inc} rapport={ratio:.3f} pire_lambda_m={lam:.2f}")


if __name__ == "__main__":
    sys.exit(main())
