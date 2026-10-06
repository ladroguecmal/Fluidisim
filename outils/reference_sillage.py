"""S519 — la référence du sillage de W, indépendante de W (C07, eau profonde).

La réponse linéaire exacte en temps de l'eau profonde à une pression gaussienne `p0·exp(−r²/2σ²)` (`p0 = F/2πσ²`, la charge de
`wake_source`) partie du repos à t = 0 en `x0` et menée à `U` constante selon x. Par mode,

    η̂(k, T) = −(k/ρ)·p̂(k)·∫₀ᵀ sin ω(T−s)/ω · e^{−i kx U s} ds,   ω = √(g k),   p̂(k) = F·exp(−k²σ²/2)·e^{−i kx x0},

en forme fermée (`I = T/(2iω)·[e^{−iΩT}·φ(i(ω+Ω)T) − e^{−iωT}·φ(i(ω−Ω)T)]`, `Ω = kx U`, `φ(z) = (e^z − 1)/z`), sommée par FFT sur une
grille périodique assez grande pour que le transitoire n'y revienne pas. Le spectre est tronqué au disque `|k| ≤ coupure`, comme la recette
de W.

    python outils/reference_sillage.py instrument        — l'instrument de S517 (maximum des rayons) sur la référence, (σ, U), trois grilles
    python outils/reference_sillage.py instrument_fige   — l'instrument figé (le bord d'Airy, 4–6 λ₀) sur la référence, trois grilles
    python outils/reference_sillage.py coque             — S520 : l'instrument figé sur la référence de la coque (4 × 1,6 m), trois grilles
    python outils/reference_sillage.py delta <fichier>   — S520 : l'instrument figé sur la surface de δ (banc `--lineaire-sillage`, `SORTIE=`)
    python outils/reference_sillage.py profondeur <fichier> — S522 : W par 5 m de fond (`c07_profondeur`) contre la référence finie et profonde
    python outils/reference_sillage.py supercritique [W10 W15] — S523 : l'angle au-delà du critique, sur la référence (trois grilles) ou W
    python outils/reference_sillage.py calibration <W_T16.bin …> — S524 : l'écart de W contre la distance du chemin aux points (A331)
    python outils/reference_sillage.py resonance [W0.3 W0.5 W0.7 W0.9] — S525 : la résonance de C07 sur la référence ou W
    python outils/reference_sillage.py plancher [W10 W15]  — S527 : la dernière crête au-dessus du plancher, référence bruitée ou W
    python outils/reference_sillage.py comparer <fichier W>  — W contre la référence aux mêmes points (fichier de `c07_sillage`)
"""
import os
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


def champ(sigma, u, t, lx, ly, dx, x0, coupure, spectre=None, profondeur=None):
    """η au temps t sur la grille [−lx/2, lx/2) × [−ly/2, ly/2), pas dx ; la source part de (x0, 0). `spectre(KX, KY)` : la transformée
    de la pression de la source (N), la gaussienne de `wake_source` par défaut."""
    nx, ny = int(round(lx / dx)), int(round(ly / dx))
    kx = 2 * np.pi * np.fft.fftfreq(nx, dx)
    ky = 2 * np.pi * np.fft.fftfreq(ny, dx)
    KX, KY = np.meshgrid(kx, ky)
    k = np.hypot(KX, KY)
    # S522 : en profondeur finie, le nombre d'onde effectif `k tanh kh` dans la pulsation et le forçage.
    kk = k if profondeur is None else k * np.tanh(k * profondeur)
    w = np.sqrt(G * kk)
    om = KX * u
    ws = np.where(w > 0, w, 1.0)
    i_t = t / (2j * ws) * (np.exp(-1j * om * t) * phi(1j * (ws + om) * t) - np.exp(-1j * ws * t) * phi(1j * (ws - om) * t))
    # La grille commence à −lx/2 : la FFT indexe depuis l'origine de la grille.
    xg0, yg0 = -lx / 2, -ly / 2
    forme = F * np.exp(-0.5 * (k * sigma) ** 2) if spectre is None else spectre(KX, KY)
    ph = forme * np.exp(-1j * (KX * (x0 - xg0) + KY * (0 - yg0)))
    eta_k = -(kk / RHO) * ph * i_t
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


def rayons(echant, xs_src, d_min, d_max, pas=0.25, a_max=40.0):
    """L'instrument de S517 : la moyenne de |η| le long des rayons issus de la source, de d_min à d_max, vers l'arrière, des deux côtés ;
    l'angle du maximum (5 à 40°, pas de 0,25°) et le profil."""
    angles = np.arange(5.0, a_max + 0.0001, 0.25)
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


# S520 — la coque de la porte D (4 × 1,6 m, 500 kg/m³) : sa pression hydrostatique `ρ g d` sur son empreinte, `d` son tirant.
COQUE_L, COQUE_B, COQUE_D, COQUE_U, COQUE_T = 4.0, 1.6, 500.0 / 1025.0, 3.0, 30.0


def spectre_coque(KX, KY):
    """La transformée d'un rectangle `L × B` de pression `ρ g d` : `ρ g d · L B · sinc(kx L/2) · sinc(ky B/2)`."""
    sx = np.sinc(KX * COQUE_L / 2 / np.pi)
    sy = np.sinc(KY * COQUE_B / 2 / np.pi)
    return RHO * G * COQUE_D * COQUE_L * COQUE_B * sx * sy


def coque():
    """S520 P2 — l'instrument figé (le bord d'Airy, 4–6 λ₀, rayons issus du centre) sur la référence de la coque, trois grilles, la
    coupure au nombre d'onde de Nyquist de δ (π/0,25 m) : critère (1)."""
    u, t = COQUE_U, COQUE_T
    x0 = -u * t / 2
    xs_src = x0 + u * t
    d0, d1 = fenetre(u, t)
    for (lx, ly, dx) in [(256.0, 128.0, 0.25), (512.0, 256.0, 0.25), (512.0, 256.0, 0.125)]:
        xs, ys, eta = champ(0.0, u, t, lx, ly, dx, x0, np.pi / 0.25, spectre_coque)
        _, angles, prof = rayons(lambda x, y: bilineaire(xs, ys, eta, x, y), xs_src, d0, d1)
        a_max, a_k = bord_airy(angles, prof)
        par = []
        for n in range(2, 7):
            lam = 2 * np.pi * u * u / G
            _, ang1, pr1 = rayons(lambda x, y: bilineaire(xs, ys, eta, x, y), xs_src, n * lam, (n + 1) * lam)
            par.append(f"{n}-{n + 1}:{bord_airy(ang1, pr1)[1]:.2f}")
        print(f"REF_S520 coque grille={lx:.0f}x{ly:.0f}@{dx} U={u} T={t} fenetre=[{d0:.2f},{d1:.2f}] maximum_deg={a_max:.2f}"
              f" bord_airy_deg={a_k:.2f} kelvin_deg=19.47 ecart_deg={a_k - 19.47:.2f} par_lambda {' '.join(par)}", flush=True)


def delta(chemin):
    """S520 P3 — l'instrument figé sur la surface de δ (fichier du banc `--lineaire-sillage`, `SORTIE=`) : le bord d'Airy sur 4–6 λ₀ et
    par fenêtre d'1 λ₀, rayons issus du centre de la coque, contre la référence de la coque (critère 2')."""
    with open(chemin, "rb") as f:
        tete = f.readline().split()
        nx, ny = int(tete[0]), int(tete[1])
        gx0, gy0, dx, xs_src = (float(v) for v in tete[2:6])
        eta = np.frombuffer(f.read(), dtype="<f4").reshape(ny, nx).astype(float)
    gx = gx0 + dx * np.arange(nx)
    gy = gy0 + dx * np.arange(ny)
    ech = lambda x, y: bilineaire(gx, gy, eta, x, y)
    u, t = COQUE_U, COQUE_T
    lam = 2 * np.pi * u * u / G
    d0, d1 = fenetre(u, t)
    _, angles, prof = rayons(ech, xs_src, d0, d1)
    a_max, a_k = bord_airy(angles, prof)
    par = []
    for n in range(2, 7):
        _, a1, p1 = rayons(ech, xs_src, n * lam, (n + 1) * lam)
        par.append(f"{n}-{n + 1}:{bord_airy(a1, p1)[1]:.2f}")
    print(f"DELTA_S520 xs={xs_src:.2f} fenetre=[{d0:.2f},{d1:.2f}] maximum_deg={a_max:.2f} bord_airy_deg={a_k:.2f} reference_deg=16.40"
          f" ecart_reference_deg={a_k - 16.40:.2f} kelvin_deg=19.47 par_lambda {' '.join(par)}"
          f" profil {' '.join(f'{angles[i]:.0f}:{prof[i]:.2e}' for i in range(0, len(angles), 8))}", flush=True)


def profondeur(chemin):
    """S522 P3 — W en profondeur finie (fichier de `c07_profondeur`) contre la référence par 5 m de fond, convergée sur trois grilles,
    et contre la référence profonde : l'écart quadratique relatif sur la zone établie (critère 3)."""
    with open(chemin, "rb") as f:
        tete = f.readline().split()
        nx, ny = int(tete[0]), int(tete[1])
        gx0, gy0, dx, xs_src = (float(v) for v in tete[2:6])
        w = np.frombuffer(f.read(), dtype="<f4").reshape(ny, nx).astype(float)
    sigma, u, t, h, coupure = 2.0, 6.3, 40.0, 5.0, 3.0
    x0 = xs_src - u * t
    # L'onde transverse par 5 m de fond : U² = g tanh(kh)/k.
    k = G / u**2
    for _ in range(100):
        k -= (u * u * k - G * np.tanh(k * h)) / (u * u - G * h * (1 - np.tanh(k * h) ** 2))
    lam = 2 * np.pi / k
    gx = gx0 + dx * np.arange(nx)
    gy = gy0 + dx * np.arange(ny)
    GX, GY = np.meshgrid(gx, gy)
    d = xs_src - GX
    zone = (d >= lam) & (d <= u * t / 2 - lam) & (np.abs(GY) <= d * np.tan(np.radians(60.0)))
    print(f"C07_S522 lambda_transverse={lam:.2f} zone=[{lam:.1f},{u * t / 2 - lam:.1f}] points_zone={zone.sum()} max_w_zone={np.abs(w[zone]).max():.4f}", flush=True)
    for nom, prof, grilles in [("finie", h, [(1024.0, 1024.0, 1.0), (1536.0, 1536.0, 1.0), (1024.0, 1024.0, 0.5)]),
                               ("profonde", None, [(1024.0, 1024.0, 1.0)])]:
        for (lx, ly, ddx) in grilles:
            xs, ys, eta = champ(sigma, u, t, lx, ly, ddx, x0, coupure, profondeur=prof)
            ref = bilineaire(xs, ys, eta, GX.ravel(), GY.ravel()).reshape(ny, nx)
            ecart = np.sqrt(((w - ref)[zone] ** 2).sum() / (ref[zone] ** 2).sum())
            print(f"C07_S522 reference={nom} grille={lx:.0f}x{ly:.0f}@{ddx} ecart_quadratique_relatif={ecart:.4f} max_ref_zone={np.abs(ref[zone]).max():.4f}", flush=True)


SUPER_U, SUPER_XS, SUPER_H, SUPER_D = (10.0, 15.0), 90.0, 5.0, (80.0, 100.0)
# S523 : la durée par vitesse (`SUPER_T=40,16`, par défaut 40 et 40) — la distance du départ à la zone sous le rayon honnête de W.
SUPER_T = dict(zip(SUPER_U, (float(v) for v in os.environ.get("SUPER_T", "40,40").split(","))))


def derniere_crete(angles, prof):
    """L'instrument figé de S523 : l'angle du maximum local le plus extérieur du profil (la crête des ondes longues, juste en dedans du
    coin de Mach ; au-delà, le profil décroît)."""
    for i in range(len(prof) - 2, 0, -1):
        if prof[i] > prof[i - 1] and prof[i] >= prof[i + 1]:
            return angles[i]
    return float("nan")


def supercritique(fichiers=None):
    """S523 — C07 peu profond au-delà du critique. Le maximum des rayons (20–60 m, 24 s) lisait le sillage intérieur (14,5° et 11,25°) :
    changé avant W. L'instrument figé : la dernière crête de la moyenne de |η| le long des rayons, 80–100 m derrière, 40 s (le transitoire
    du départ hors de la zone). Sur la référence par 5 m de fond, trois grilles (critère 1) ; avec les fichiers de W (`c07_profondeur`),
    sur W et la référence aux mêmes points, et l'écart quadratique (critères 2, 3)."""
    for n, u in enumerate(SUPER_U):
        attendu = np.degrees(np.arcsin(np.sqrt(G * SUPER_H) / u))
        x0 = SUPER_XS - u * SUPER_T[u]
        if fichiers is None:
            for (lx, ly, dx) in [(2048.0, 768.0, 1.0), (2048.0, 768.0, 0.5), (2560.0, 1024.0, 0.5)]:
                xs, ys, eta = champ(2.0, u, SUPER_T[u], lx, ly, dx, x0, 3.0, profondeur=SUPER_H)
                _, angles, prof = rayons(lambda x, y: bilineaire(xs, ys, eta, x, y), SUPER_XS, *SUPER_D, a_max=80.0)
                a = derniere_crete(angles, prof)
                print(f"REF_S523 U={u} Fr_h={u / np.sqrt(G * SUPER_H):.3f} grille={lx:.0f}x{ly:.0f}@{dx} derniere_crete_deg={a:.2f} attendu_deg={attendu:.2f}"
                      f" ecart_deg={a - attendu:.2f} profil {' '.join(f'{angles[i]:.0f}:{prof[i]:.2e}' for i in range(0, len(angles), 12))}", flush=True)
            continue
        with open(fichiers[n], "rb") as f:
            tete = f.readline().split()
            nx, ny = int(tete[0]), int(tete[1])
            gx0, gy0, dx, xs_src = (float(v) for v in tete[2:6])
            w = np.frombuffer(f.read(), dtype="<f4").reshape(ny, nx).astype(float)
        gx = gx0 + dx * np.arange(nx)
        gy = gy0 + dx * np.arange(ny)
        GX, GY = np.meshgrid(gx, gy)
        coupure = float(os.environ.get("COUPURE", "3"))
        xs, ys, eta = champ(2.0, u, SUPER_T[u], 2048.0, 768.0, 0.5, x0, coupure, profondeur=SUPER_H)
        ref = bilineaire(xs, ys, eta, GX.ravel(), GY.ravel()).reshape(ny, nx)
        d = xs_src - GX
        zone = (d >= SUPER_D[0]) & (d <= SUPER_D[1])
        ecart = np.sqrt(((w - ref)[zone] ** 2).sum() / (ref[zone] ** 2).sum())
        for nom, c in [("W", w), ("reference", ref)]:
            _, angles, prof = rayons(lambda x, y, c=c: bilineaire(gx, gy, c, x, y), xs_src, *SUPER_D, a_max=80.0)
            a = derniere_crete(angles, prof)
            print(f"C07_S523 U={u} {nom} derniere_crete_deg={a:.2f} attendu_deg={attendu:.2f} ecart_deg={a - attendu:.2f}", flush=True)
        print(f"C07_S523 U={u} ecart_quadratique_relatif={ecart:.4f} points_zone={zone.sum()} max_ref_zone={np.abs(ref[zone]).max():.4f}", flush=True)


def derniere_crete_plancher(angles, prof, plancher=1e-3):
    """S527 — la dernière crête **au-dessus d'un plancher** : le maximum local le plus extérieur dont la valeur dépasse `plancher` fois le
    maximum du profil (ADR-234 D2 : ce qui est sous le bruit de l'objet ne compte pas ; seuil déclaré avant la mesure)."""
    seuil = plancher * prof.max()
    for i in range(len(prof) - 2, 0, -1):
        if prof[i] > prof[i - 1] and prof[i] >= prof[i + 1] and prof[i] >= seuil:
            return angles[i]
    return float("nan")


def plancher(fichiers=None):
    """S527 — l'angle supercritique par la dernière crête au-dessus du plancher. Sans fichier : la référence (5 m, σ 2 m, coupure 3,
    `SUPER_T`) **bruitée** d'un bruit gaussien de 10⁻⁷ m par point, trois tirages, trois grilles (critère 1). Avec les fichiers de W de
    S523 : W et la référence aux mêmes points (critère 2)."""
    for n, u in enumerate(SUPER_U):
        attendu = np.degrees(np.arcsin(np.sqrt(G * SUPER_H) / u))
        x0 = SUPER_XS - u * SUPER_T[u]
        if fichiers is None:
            for (lx, ly, dx) in [(2048.0, 768.0, 1.0), (2048.0, 768.0, 0.5), (2560.0, 1024.0, 0.5)]:
                xs, ys, eta = champ(2.0, u, SUPER_T[u], lx, ly, dx, x0, 3.0, profondeur=SUPER_H)
                lus = []
                for tirage in (1, 2, 3):
                    bruite = eta + np.random.default_rng(tirage).normal(0.0, 1e-7, eta.shape)
                    _, angles, prof = rayons(lambda x, y: bilineaire(xs, ys, bruite, x, y), SUPER_XS, *SUPER_D, a_max=80.0)
                    lus.append(derniere_crete_plancher(angles, prof))
                print(f"PLANCHER_S527 U={u} grille={lx:.0f}x{ly:.0f}@{dx} reference_bruitee_deg={' '.join(f'{a:.2f}' for a in lus)} attendu_deg={attendu:.2f}"
                      f" pire_ecart_deg={max(abs(a - attendu) for a in lus):.2f}", flush=True)
            continue
        with open(fichiers[n], "rb") as f:
            tete = f.readline().split()
            nx, ny = int(tete[0]), int(tete[1])
            gx0, gy0, dx, xs_src = (float(v) for v in tete[2:6])
            w = np.frombuffer(f.read(), dtype="<f4").reshape(ny, nx).astype(float)
        gx = gx0 + dx * np.arange(nx)
        gy = gy0 + dx * np.arange(ny)
        GX, GY = np.meshgrid(gx, gy)
        xs, ys, eta = champ(2.0, u, SUPER_T[u], 2048.0, 768.0, 0.5, x0, 3.0, profondeur=SUPER_H)
        ref = bilineaire(xs, ys, eta, GX.ravel(), GY.ravel()).reshape(ny, nx)
        for nom, c in [("W", w), ("reference", ref)]:
            _, angles, prof = rayons(lambda x, y, c=c: bilineaire(gx, gy, c, x, y), xs_src, *SUPER_D, a_max=80.0)
            a = derniere_crete_plancher(angles, prof)
            print(f"PLANCHER_S527 U={u} {nom} derniere_crete_plancher_deg={a:.2f} attendu_deg={attendu:.2f} ecart_deg={a - attendu:.2f}", flush=True)


def calibration(fichiers):
    """S524 — A331 : W (5 m de fond, 10 m/s, σ 2 m, recette 512 × 256 à coupure 3, rayon honnête 179 m) contre la référence, la zone de 80
    à 100 m derrière la source ; la durée lue dans le nom du fichier (`…_T<durée>.bin`). `D`, la plus grande distance d'un point du chemin
    (le départ) à un point de la zone, et l'écart quadratique relatif."""
    u, r_honnete = 10.0, 2 * np.pi * 256 / (3 * 3.0)
    for chemin in fichiers:
        t = float(chemin.rsplit("_T", 1)[1].split(".")[0])
        with open(chemin, "rb") as f:
            tete = f.readline().split()
            nx, ny = int(tete[0]), int(tete[1])
            gx0, gy0, dx, xs_src = (float(v) for v in tete[2:6])
            w = np.frombuffer(f.read(), dtype="<f4").reshape(ny, nx).astype(float)
        x0 = xs_src - u * t
        gx = gx0 + dx * np.arange(nx)
        gy = gy0 + dx * np.arange(ny)
        GX, GY = np.meshgrid(gx, gy)
        xs, ys, eta = champ(2.0, u, t, 2048.0, 768.0, 0.5, x0, 3.0, profondeur=SUPER_H)
        ref = bilineaire(xs, ys, eta, GX.ravel(), GY.ravel()).reshape(ny, nx)
        d = xs_src - GX
        zone = (d >= SUPER_D[0]) & (d <= SUPER_D[1])
        dist = np.hypot(GX[zone] - x0, GY[zone]).max()
        ecart = np.sqrt(((w - ref)[zone] ** 2).sum() / (ref[zone] ** 2).sum())
        print(f"CALIB_S524 T={t:.0f} D_m={dist:.1f} D_sur_R={dist / r_honnete:.2f} ecart_quadratique_relatif={ecart:.4f}", flush=True)


def resonance(fichiers=None):
    """S525 — la résonance de C07 : la dépression maximale à moins de 3σ de la source (σ 20 m, 5 m de fond, 64 s, coupure 0,4), rapportée
    à la statique `p₀/ρg`, à `Fr_h` ∈ {0,3 ; 0,5 ; 0,7 ; 0,9} ; la pente de `log A` contre `log|1 − Fr_h²|` sur quatre et sur trois points.
    Sans fichier : la référence sur sa grille ; avec les quatre fichiers de W (`c07_profondeur`), W et la référence aux mêmes points."""
    sigma, h, t, coupure = 20.0, 5.0, 64.0, 0.4
    c = np.sqrt(G * h)
    nombres = [0.3, 0.5, 0.7, 0.9]
    statique = F / (2 * np.pi * sigma**2) / (RHO * G)
    series = {}
    for n, fr in enumerate(nombres):
        u = fr * c
        if fichiers is None:
            x0 = -u * t / 2
            xs, ys, eta = champ(sigma, u, t, 2048.0, 1024.0, 2.0, x0, coupure, profondeur=h)
            xsrc = x0 + u * t
            m = (np.abs(xs[None, :] - xsrc) < 3 * sigma) & (np.abs(ys[:, None]) < 3 * sigma)
            series.setdefault("reference", []).append(np.abs(eta[m]).max() / statique)
            continue
        with open(fichiers[n], "rb") as f:
            tete = f.readline().split()
            nx, ny = int(tete[0]), int(tete[1])
            gx0, gy0, dx, xs_src = (float(v) for v in tete[2:6])
            w = np.frombuffer(f.read(), dtype="<f4").reshape(ny, nx).astype(float)
        x0 = xs_src - u * t
        gx = gx0 + dx * np.arange(nx)
        gy = gy0 + dx * np.arange(ny)
        GX, GY = np.meshgrid(gx, gy)
        xs, ys, eta = champ(sigma, u, t, 2048.0, 1024.0, 2.0, x0, coupure, profondeur=h)
        ref = bilineaire(xs, ys, eta, GX.ravel(), GY.ravel()).reshape(ny, nx)
        m = (np.abs(GX - xs_src) < 3 * sigma) & (np.abs(GY) < 3 * sigma)
        series.setdefault("W", []).append(np.abs(w[m]).max() / statique)
        series.setdefault("reference", []).append(np.abs(ref[m]).max() / statique)
    X = np.log(np.abs(1 - np.array(nombres) ** 2))
    for nom, a in series.items():
        a = np.array(a)
        p4 = np.polyfit(X, np.log(a), 1)[0]
        p3 = np.polyfit(X[:3], np.log(a[:3]), 1)[0]
        print(f"RES_S525 {nom} A_sur_statique {' '.join(f'{v:.4f}' for v in a)} prandtl_glauert "
              f"{' '.join(f'{1 / np.sqrt(1 - f * f):.4f}' for f in nombres)} pente_4={p4:.3f} pente_3={p3:.3f}", flush=True)


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
    elif sys.argv[1] == "delta":
        delta(sys.argv[2])
    elif sys.argv[1] == "profondeur":
        profondeur(sys.argv[2])
    elif sys.argv[1] == "plancher":
        plancher(sys.argv[2:] or None)
    elif sys.argv[1] == "calibration":
        calibration(sys.argv[2:])
    elif sys.argv[1] == "resonance":
        resonance(sys.argv[2:] or None)
    elif sys.argv[1] == "supercritique":
        supercritique(sys.argv[2:] or None)
    else:
        {"instrument": instrument, "instrument_fige": instrument_fige, "coque": coque}[sys.argv[1]]()
