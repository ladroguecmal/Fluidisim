"""S712 — le graphique de la plage canonique de Synolakis : les profils mesurés (points) et calculés (courbes) aux instants 15, 20, 25.

Lit `references/synolakis/feuille_1.csv` (les mesures) et `calculs/synolakis_<variante>_t<t>.csv` (les profils d'APIC, écrits par
l'essai `the_judge_against_synolakis_*_s712`), et écrit `captures/s712_synolakis.svg`. Sans dépendance : le SVG est écrit à la main.
"""
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
VARIANTES = [("sans", "3D, pas 10 ms (le juge)", "#d62728"), ("k0.05", "3D, projection faible", "#1f77b4"),
             ("sans_pas2500", "3D, pas 2,5 ms", "#2ca02c"), ("sans_fin", "3D à 1,25 cm", "#9467bd")]
INSTANTS = [15, 20, 25]
X_MIN, X_MAX, E_MIN, E_MAX = -6.0, 14.0, -0.1, 0.5
L, H, MARGE = 900, 230, 60


def mesures():
    lignes = (ROOT / "references/synolakis/feuille_1.csv").read_text(encoding="latin-1").splitlines()[1:]
    out = {t: [] for t in [15, 20, 25, 30]}
    for ligne in lignes:
        c = ligne.split(";")
        for n, t in enumerate([15, 20, 25, 30]):
            try:
                out[t].append((float(c[2 * n].replace(",", ".")), float(c[2 * n + 1].replace(",", "."))))
            except (IndexError, ValueError):
                pass
    return out


def profil(variante, t):
    p = ROOT / f"calculs/synolakis_{variante}_t{t}.csv"
    if not p.exists():
        return None
    pts = []
    for ligne in p.read_text(encoding="utf-8").splitlines():
        x, e = ligne.split(";")
        pts.append((float(x), float(e)))
    return pts


def main():
    m = mesures()
    hauteur = MARGE + len(INSTANTS) * (H + MARGE) + 40
    svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{L + 2 * MARGE}" height="{hauteur}" font-family="sans-serif" font-size="13">',
           f'<rect width="100%" height="100%" fill="white"/>',
           f'<text x="{MARGE}" y="28" font-size="16" font-weight="bold">Onde solitaire H/d = 0,3 sur la pente 1:19,85 — mesures de Synolakis (points) contre la 3D (courbes)</text>']
    for n, t in enumerate(INSTANTS):
        y0 = MARGE + n * (H + MARGE)
        px = lambda x: MARGE + (X_MAX - x) / (X_MAX - X_MIN) * L  # le large à gauche, la plage à droite
        py = lambda e: y0 + (E_MAX - e) / (E_MAX - E_MIN) * H
        svg.append(f'<rect x="{MARGE}" y="{y0}" width="{L}" height="{H}" fill="none" stroke="#999"/>')
        svg.append(f'<text x="{MARGE + 8}" y="{y0 + 18}" font-weight="bold">t·√(g/d) = {t}</text>')
        # Le niveau au repos et le fond.
        svg.append(f'<line x1="{px(X_MAX)}" y1="{py(0)}" x2="{px(X_MIN)}" y2="{py(0)}" stroke="#bbb" stroke-dasharray="4 3"/>')
        # Le fond, `η = −x/19,85`, coupé au panneau (il n'y entre qu'au-dessus de E_MIN, côté plage).
        x_bas = -E_MIN * 19.85
        svg.append(f'<line x1="{px(x_bas):.1f}" y1="{py(E_MIN):.1f}" x2="{px(X_MIN):.1f}" y2="{py(-X_MIN / 19.85):.1f}" stroke="#8c6d31" stroke-width="2"/>')
        for v in [0.0, 0.2, 0.4]:
            svg.append(f'<text x="{MARGE - 8}" y="{py(v) + 4}" text-anchor="end">{v:.1f}</text>')
        for x in range(int(X_MIN), int(X_MAX) + 1, 2):
            svg.append(f'<text x="{px(x)}" y="{y0 + H + 16}" text-anchor="middle">{x}</text>')
        for variante, nom, couleur in VARIANTES:
            p = profil(variante, t)
            if p:
                pts = " ".join(f"{px(x):.1f},{py(e):.1f}" for x, e in p if X_MIN <= x <= X_MAX and E_MIN <= e <= E_MAX)
                svg.append(f'<polyline points="{pts}" fill="none" stroke="{couleur}" stroke-width="2"/>')
        for x, e in m[t]:
            if X_MIN <= x <= X_MAX:
                svg.append(f'<circle cx="{px(x):.1f}" cy="{py(e):.1f}" r="2.6" fill="black"/>')
    yl = MARGE + len(INSTANTS) * (H + MARGE) - 20
    svg.append(f'<text x="{MARGE + L / 2}" y="{yl - 12}" text-anchor="middle">x/d (le large à gauche, le rivage au repos en 0, la plage à droite) ; en ordonnée η/d</text>')
    xl = MARGE
    for variante, nom, couleur in VARIANTES:
        present = any(profil(variante, t) for t in INSTANTS)
        svg.append(f'<line x1="{xl}" y1="{yl + 10}" x2="{xl + 30}" y2="{yl + 10}" stroke="{couleur}" stroke-width="2"/>')
        svg.append(f'<text x="{xl + 36}" y="{yl + 14}">{nom}{"" if present else " (à venir)"}</text>')
        xl += 215
    svg.append(f'<circle cx="{xl + 5}" cy="{yl + 10}" r="3" fill="black"/><text x="{xl + 14}" y="{yl + 14}">mesures (Synolakis, Caltech)</text>')
    svg.append('</svg>')
    sortie = ROOT / "captures/s712_synolakis.svg"
    sortie.write_text("\n".join(svg), encoding="utf-8")
    print(sortie)


if __name__ == "__main__":
    main()
