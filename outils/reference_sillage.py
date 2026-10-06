"""S519 — la référence du sillage de W, indépendante de W (C07, eau profonde).

La réponse linéaire exacte en temps de l'eau profonde à une pression gaussienne `p0·exp(−r²/2σ²)` (`p0 = F/2πσ²`, la charge de
`wake_source`) partie du repos à t = 0 en `x0` et menée à `U` constante selon x. Par mode,

    η̂(k, T) = −(k/ρ)·p̂(k)·∫₀ᵀ sin ω(T−s)/ω · e^{−i kx U s} ds,   ω = √(g k),   p̂(k) = F·exp(−k²σ²/2)·e^{−i kx x0},

en forme fermée (`I = T/(2iω)·[e^{−iΩT}·φ(i(ω+Ω)T) − e^{−iωT}·φ(i(ω−Ω)T)]`, `Ω = kx U`, `φ(z) = (e^z − 1)/z`), sommée par FFT sur une
grille périodique assez grande pour que le transitoire n'y revienne pas. Le spectre est tronqué au disque `|k| ≤ coupure`, comme la recette
de W.

    python outils/reference_sillage.py instrument        — l'instrument de S517 (maximum des rayons) sur la référence, (σ, U), trois grilles
    python outils/reference_sillage.py instrument_fige   — l'instrument figé (le bord d'Airy, 4–6 λ₀) sur la référence, trois grilles
    python outils/reference_sillage.py comparer <fichier W>  — W contre la référence aux mêmes points (fichier de `c07_sillage`)
"""
import sys
import numpy as np

G, RHO, F = 9.81, 1025.0, 19_620.0


def phi(z):
    """(e^z − 1)/z, stable près de zéro."""
    out = np.empty_like(z)
    petit = np.abs(z) < 1e-4
    zs = z[petit]
    out[petit] = 1 + zs / 2 + zs * zs / 6
    zg = z[~petit]
    out[~petit] = np.expm1(zg) / zg
    return out


def champ(sigma, u, t, lx, ly, dx, x0, coupure):
    """η au temps t sur la grille [−lx/2, lx/2) × [−ly/2, ly/2), pas dx ; la source part de (x0, 0)."""
    nx, ny = int(round(lx / dx)), int(round(ly / dx))
    kx = 2 * np.pi * np.fft.fftfreq(nx, dx)
    ky = 2 * np.pi * np.fft.fftfreq(ny, dx)
    KX, KY = np.meshgrid(kx, ky)
    k = np.hypot(KX, KY)
    w = np.sqrt(G * k)
    om = KX * u
    ws = np.where(w > 0, w, 1.0)
    i_t = t / (2j * ws) * (np.exp(-1j * om * t) * phi(1j * (ws + om) * t) - np.exp(-1j * ws * t) * phi(1j * (ws - om) * t))
    # La grille commence à −lx/2 : la FFT indexe depuis l'origine de la grille.
    xg0, yg0 = -lx / 2, -ly / 2
    ph = F * np.exp(-0.5 * (k * sigma) ** 2) * np.exp(-1j * (KX * (x0 - xg0) + KY * (0 - yg0)))
    eta_k = -(k / RHO) * ph * i_t
    eta_k[k == 0] = 0
    eta_k[k > coupure] = 0
    eta = np.fft.ifft2(eta_k).real / dx**2
    xs = xg0 + dx * np.arange(nx)
    ys = yg0 + dx * np.arange(ny)
    return xs, ys, eta


def bilineaire(xs, ys, eta, x, y):
    dx = xs[1] - xs[0]
    fi, fj = (x - xs[0]) / dx, (y - ys[0]) / dx
    i, j = np.floor(fi).astype(int), np.floor(fj).astype(int)
    a, b = fi - i, fj - j
    return (eta[j, i] * (1 - a) * (1 - b) + eta[j, i + 1] * a * (1 - b) + eta[j + 1, i] * (1 - a) * b + eta[j + 1, i + 1] * a * b)


def rayons(echant, xs_src, d_min, d_max, pas=0.25):
    """L'instrument de S517 : la moyenne de |η| le long des rayons issus de la source, de d_min à d_max, vers l'arrière, des deux côtés ;
    l'angle du maximum (5 à 40°, pas de 0,25°) et le profil."""
    angles = np.arange(5.0, 40.0001, 0.25)
    r = np.arange(d_min, d_max + 1e-9, pas)
    prof = []
    for a in angles:
        th = np.radians(a)
        x = xs_src - r * np.cos(th)
        y = r * np.sin(th)
        prof.append(0.5 * (np.abs(echant(x, y)).mean() + np.abs(echant(x, -y)).mean()))
    prof = np.array(prof)
    return angles[np.argmax(prof)], angles, prof


# Le bord d'Airy : près de la ligne des cuspides, l'amplitude suit Ai(z), la ligne de Kelvin à z = 0 ; Ai(0) = 0,35503, le maximum de Ai
# sur les z négatifs Ai(−1,0188) = 0,53566 — le bord est où l'amplitude, passé son maximum, retombe à leur rapport.
RAPPORT_AIRY = 0.35503 / 0.53566


def bord_airy(angles, prof, a_min=12.0):
    """L'angle de Kelvin lu sur un profil (moyenne de |η| le long des rayons) : le maximum au-delà de `a_min`, puis l'angle, au-delà,
    où le profil retombe à `RAPPORT_AIRY` de ce maximum (interpolé) ; nan s'il n'y retombe pas."""
    i = int(np.argmax(np.where(angles >= a_min, prof, -1.0)))
    seuil = RAPPORT_AIRY * prof[i]
    j = i
    while j + 1 < len(prof) and prof[j + 1] > seuil:
        j += 1
    if j + 1 >= len(prof):
        return angles[i], float("nan")
    return angles[i], angles[j] + (angles[j + 1] - angles[j]) * (prof[j] - seuil) / (prof[j] - prof[j + 1])


SIGMA, U, T, COUPURE = 0.5, 2.5, 24.0, 6.0


def fenetre(u=U, t=T):
    """La fenêtre de distances de l'instrument : de 4 λ₀ à 6 λ₀ derrière la source, dans la zone établie (U·T/2 − λ₀)."""
    lam = 2 * np.pi * u * u / G
    return 4 * lam, min(6 * lam, u * t / 2 - lam)


def instrument_fige():
    """S519 P2 — l'instrument figé (le bord d'Airy sur 4–6 λ₀) sur la référence, trois grilles : critère (1)."""
    x0 = -U * T / 2
    xs_src = x0 + U * T
    d0, d1 = fenetre()
    for (lx, ly, dx) in [(256.0, 128.0, 0.25), (512.0, 256.0, 0.25), (512.0, 256.0, 0.125)]:
        xs, ys, eta = champ(SIGMA, U, T, lx, ly, dx, x0, COUPURE)
        _, angles, prof = rayons(lambda x, y: bilineaire(xs, ys, eta, x, y), xs_src, d0, d1)
        a_max, a_k = bord_airy(angles, prof)
        print(f"REF_S519 instrument_fige grille={lx:.0f}x{ly:.0f}@{dx} sigma={SIGMA} U={U} fenetre=[{d0:.2f},{d1:.2f}] maximum_deg={a_max:.2f}"
              f" bord_airy_deg={a_k:.2f} kelvin_deg=19.47 ecart_deg={a_k - 19.47:.2f}", flush=True)


def comparer(chemin):
    """S519 P3 — W contre la référence aux mêmes points : l'écart quadratique relatif dans la zone établie (critère 2), l'instrument figé
    sur W et sur la référence (critère 3)."""
    with open(chemin, "rb") as f:
        tete = f.readline().split()
        nx, ny = int(tete[0]), int(tete[1])
        gx0, gy0, dx, xs_src = (float(v) for v in tete[2:6])
        w = np.frombuffer(f.read(), dtype="<f4").reshape(ny, nx).astype(float)
    x0 = -U * T / 2
    xs, ys, eta = champ(SIGMA, U, T, 512.0, 256.0, dx, x0, COUPURE)
    gx = gx0 + dx * np.arange(nx)
    gy = gy0 + dx * np.arange(ny)
    GX, GY = np.meshgrid(gx, gy)
    ref = bilineaire(xs, ys, eta, GX.ravel(), GY.ravel()).reshape(ny, nx)
    lam = 2 * np.pi * U * U / G
    d = xs_src - GX
    zone = (d >= 2 * lam) & (d <= U * T / 2 - 2 * lam) & (np.abs(GY) <= d * np.tan(np.radians(30.0)))
    ecart = np.sqrt(((w - ref)[zone] ** 2).sum() / (ref[zone] ** 2).sum())
    pire = np.abs(w - ref)[zone].max() / np.abs(ref[zone]).max()
    print(f"C07_S519 comparer points_zone={zone.sum()} ecart_quadratique_relatif={ecart:.4f} pire_rapporte_au_max={pire:.4f}"
          f" max_ref={np.abs(ref[zone]).max():.4f} max_w={np.abs(w[zone]).max():.4f}", flush=True)
    d0, d1 = fenetre()
    for nom, champ_ in [("W", w), ("reference", ref)]:
        ech = lambda x, y, c=champ_: bilineaire(gx, gy, c, x, y)
        _, angles, prof = rayons(ech, xs_src, d0, d1)
        a_max, a_k = bord_airy(angles, prof)
        print(f"C07_S519 instrument {nom} maximum_deg={a_max:.2f} bord_airy_deg={a_k:.2f} kelvin_deg=19.47 ecart_deg={a_k - 19.47:.2f}",
              flush=True)


def instrument():
    t, coupure = 24.0, 6.0
    for sigma in [0.5, 1.0, 2.0]:
        for u in [2.0, 2.5, 3.0]:
            lam = 2 * np.pi * u * u / G
            x0 = -u * t / 2
            xs_src = x0 + u * t
            d_min, d_max = 2 * lam, u * t / 2 - 2 * lam
            ligne = []
            for (lx, ly, dx) in [(256.0, 128.0, 0.25), (512.0, 256.0, 0.25), (512.0, 256.0, 0.125)]:
                xs, ys, eta = champ(sigma, u, t, lx, ly, dx, x0, coupure)
                a, angles, prof = rayons(lambda x, y: bilineaire(xs, ys, eta, x, y), xs_src, d_min, d_max)
                ligne.append(f"{lx:.0f}x{ly:.0f}@{dx}:{a:.2f}")
            print(f"REF_S519 sigma={sigma} U={u} Fr_sigma={u/np.sqrt(G*sigma):.2f} lambda0={lam:.2f} d=[{d_min:.1f},{d_max:.1f}] angle_max {' '.join(ligne)}"
                  f" profil {' '.join(f'{angles[i]:.0f}:{prof[i]:.2e}' for i in range(0, len(angles), 8))}", flush=True)


if __name__ == "__main__":
    if sys.argv[1] == "comparer":
        comparer(sys.argv[2])
    else:
        {"instrument": instrument, "instrument_fige": instrument_fige}[sys.argv[1]]()
