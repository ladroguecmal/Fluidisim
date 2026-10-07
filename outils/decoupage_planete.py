"""Le découpage de la planète, mesuré : HEALPix (celui de DyingStar) contre la cube-sphère (équiangulaire et gnomonique).

Pour un même nombre de cellules (~49 000) : le rapport des aires extrêmes, le rapport des côtés extrêmes dans une cellule (l'anisotropie),
l'angle de coin le plus fermé (la forme), le rapport des côtés extrêmes sur toute la sphère. Les aires par sous-division fine (m × m
triangles sphériques par cellule). HEALPix : les formules de `healpix.gd` de DyingStar (vérifiées par lui contre healpy, nside 64).
"""
import numpy as np

JRLL = np.array([2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4])
JPLL = np.array([1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7])


def healpix_vec(face, fx, fy, ns):
    jr = JRLL[face] * ns - fx - fy
    nord, sud = jr < ns, jr > 3 * ns
    nr = np.where(nord, jr, np.where(sud, 4 * ns - jr, ns))
    z = np.where(nord, 1 - nr * nr / (3 * ns * ns), np.where(sud, -1 + nr * nr / (3 * ns * ns), (2 * ns - jr) * 2 / (3 * ns)))
    kp = JPLL[face] * nr + fx - fy
    with np.errstate(divide='ignore', invalid='ignore'):
        phi = np.where(nr > 0, kp * np.pi / (4 * nr), 0.0)
    st = np.sqrt(np.maximum(1 - z * z, 0))
    return np.stack([st * np.cos(phi), st * np.sin(phi), z], -1)


def cube_vec(face, a, b, equi):
    # a, b ∈ [-1, 1] ; équiangulaire : tan(π/4·a)
    if equi:
        a, b = np.tan(np.pi / 4 * a), np.tan(np.pi / 4 * b)
    one = np.ones_like(a)
    axes = [(one, a, b), (-one, -a, b), (a, one, b), (-a, -one, b), (a, b, one), (-a, b, -one)]
    v = np.stack(axes[face], -1)
    return v / np.linalg.norm(v, axis=-1, keepdims=True)


def tri_area(p, q, r):
    num = np.abs(np.einsum('...i,...i', p, np.cross(q, r)))
    den = 1 + np.einsum('...i,...i', p, q) + np.einsum('...i,...i', q, r) + np.einsum('...i,...i', r, p)
    return 2 * np.arctan2(num, den)


def mesures(cellules, m=8):
    """`cellules` : une fonction (s, t) ∈ [0,1]² → points, par cellule (tableaux [ncell, ...]). Rend les quatre mesures."""
    s = np.linspace(0, 1, m + 1)
    S, T = np.meshgrid(s, s, indexing='ij')
    P = cellules(S, T)                       # [ncell, m+1, m+1, 3]
    a = tri_area(P[:, :-1, :-1], P[:, 1:, :-1], P[:, 1:, 1:]) + tri_area(P[:, :-1, :-1], P[:, 1:, 1:], P[:, :-1, 1:])
    aire = a.sum(axis=(1, 2))
    c = [P[:, 0, 0], P[:, -1, 0], P[:, -1, -1], P[:, 0, -1]]
    cote = np.stack([np.arccos(np.clip(np.einsum('ij,ij->i', c[k], c[(k + 1) % 4]), -1, 1)) for k in range(4)], -1)
    ang = []
    for k in range(4):
        o, u, w = c[k], c[(k + 1) % 4], c[(k - 1) % 4]
        t1 = u - np.einsum('ij,ij->i', u, o)[:, None] * o
        t2 = w - np.einsum('ij,ij->i', w, o)[:, None] * o
        ang.append(np.degrees(np.arccos(np.clip(np.einsum('ij,ij->i', t1, t2) / np.linalg.norm(t1, axis=1) / np.linalg.norm(t2, axis=1), -1, 1))))
    ang = np.stack(ang, -1)
    return dict(n=len(aire), somme=aire.sum() / (4 * np.pi), aires=aire.max() / aire.min(),
                aniso=(cote.max(1) / cote.min(1)).max(), angle_min=ang.min(), cotes=cote.max() / cote.min())


def healpix(ns):
    ix, iy = np.meshgrid(np.arange(ns), np.arange(ns), indexing='ij')
    faces = np.repeat(np.arange(12), ns * ns)
    ix, iy = np.tile(ix.ravel(), 12), np.tile(iy.ravel(), 12)
    return lambda S, T: healpix_vec(faces[:, None, None], ix[:, None, None] + S, iy[:, None, None] + T, ns)


def cube(n, equi):
    i, j = np.meshgrid(np.arange(n), np.arange(n), indexing='ij')
    i, j = i.ravel(), j.ravel()

    def f(S, T):
        out = []
        for face in range(6):
            a = -1 + 2 * (i[:, None, None] + S) / n
            b = -1 + 2 * (j[:, None, None] + T) / n
            out.append(cube_vec(face, a, b, equi))
        return np.concatenate(out)
    return f


if __name__ == '__main__':
    for nom, f in [("HEALPix nside 64", healpix(64)), ("cube-sphère équiangulaire 91²×6", cube(91, True)),
                   ("cube-sphère gnomonique 91²×6", cube(91, False))]:
        r = mesures(f)
        print(f"{nom:34s} cellules {r['n']:6d}  Σaire/4π {r['somme']:.6f}  aires max/min {r['aires']:.4f}  "
              f"anisotropie max {r['aniso']:.3f}  angle min {r['angle_min']:.1f}°  côtés max/min {r['cotes']:.3f}")
