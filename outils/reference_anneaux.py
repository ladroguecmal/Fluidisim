"""S528 — la référence des anneaux d'impact de W en profondeur finie, indépendante de la somme de Bessel.

Le champ initial de W (une milliseconde après la naissance, sur ± 80 m — la portée de la somme à 128 modes —, prolongé par des zéros) est propagé par FFT avec la dispersion
exacte en profondeur finie, `η̂(k, t) = η̂(k, 0)·cos(ω t)`, `ω = √(g k tanh kh)` ; comparé à W à 5 et 10 s sur le disque de 40 m. Trois
grilles (256 m à 25 cm, 384 m à 25 cm, 256 m à 12,5 cm) ; et l'eau profonde (`ω = √(g k)`) sur le même champ initial, en témoin.

    python outils/reference_anneaux.py <fichier de anneaux_profondeur> <profondeur>
"""
import sys
import numpy as np

G = 9.81


def lire(chemin):
    grilles = []
    with open(chemin, "rb") as f:
        while True:
            tete = f.readline()
            if not tete:
                break
            nx, ny, x0, y0, dx, t = tete.split()
            nx, ny = int(nx), int(ny)
            eta = np.frombuffer(f.read(4 * nx * ny), dtype="<f4").reshape(ny, nx).astype(float)
            grilles.append((float(x0), float(dx), float(t), eta))
    return grilles


def propager(eta0, dx, t, h):
    n = eta0.shape[0]
    k1 = 2 * np.pi * np.fft.fftfreq(n, dx)
    KX, KY = np.meshgrid(k1, k1)
    k = np.hypot(KX, KY)
    w = np.sqrt(G * k) if h is None else np.sqrt(G * k * np.tanh(k * h))
    # Le champ initial est pris à 1 ms : on le ramène à t = 0 par le même propagateur (cos pair), l'erreur en 1 ms est sous 10⁻⁵.
    return np.fft.ifft2(np.fft.fft2(eta0) * np.cos(w * (t - 0.001))).real


def main():
    chemin, h = sys.argv[1], float(sys.argv[2])
    (xi0, dxi, _, ini), *suivis = lire(chemin)
    for (lx, dx) in [(256.0, 0.25), (384.0, 0.25), (256.0, 0.125)]:
        # Le champ initial (± 80 m, la portée de la somme à 128 modes) prolongé par des zéros sur la grille FFT, origine à l'indice n/2.
        pas = int(round(dx / dxi))
        sous = ini[::pas, ::pas]
        n = int(round(lx / dx))
        e0 = np.zeros((n, n))
        m = sous.shape[0]
        debut = n // 2 + int(round(xi0 / dx))
        e0[debut:debut + m, debut:debut + m] = sous
        for (x0, dxs, t, w) in suivis:
            ligne = []
            for nom, prof in [("finie", h), ("profonde", None)]:
                if nom == "profonde" and (lx, dx) != (256.0, 0.25):
                    continue
                ref = propager(e0, dx, t, prof)
                # Les points de W (pas dxs, depuis x0) sur la grille de la référence (origine au centre, indice n/2).
                n = ref.shape[0]
                idx = np.round((x0 + dxs * np.arange(w.shape[0])) / dx).astype(int) + n // 2
                r = ref[np.ix_(idx, idx)]
                X = x0 + dxs * np.arange(w.shape[0])
                XX, YY = np.meshgrid(X, X)
                disque = np.hypot(XX, YY) <= 39.0
                ecart = np.sqrt(((w - r)[disque] ** 2).sum() / (r[disque] ** 2).sum())
                ligne.append(f"{nom}={ecart:.4f}")
            print(f"ANNEAUX_S528 grille={lx:.0f}@{dx} t={t:.0f}s ecart_quadratique_relatif {' '.join(ligne)}", flush=True)


if __name__ == "__main__":
    main()
