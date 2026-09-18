"""S274 : lecture d'usage des traces du résidu progressif ; bibliothèque standard seulement.

Traduit l'écart entre le pas réel et l'oracle d'ordre deux S272 en grandeurs d'usage :
hauteur absolue, pente, déphasage de la vague entière (mode k en quadrature) et harmonique 2k.
Teste aussi la signature séculaire de l'ordre trois (dispersion d'amplitude de Stokes).
"""
import math
import sys
from residu_progressif import L, H, G, K, OMEGA, PHASE, read, field

SIGMA = math.tanh(K*H)
# Stokes, troisième ordre, profondeur finie : ω = ω0 [1 + (ka)² S] (Fenton 1985, première définition).
STOKES = (8+math.cosh(4*K*H)-2*SIGMA**2)/(16*math.sinh(K*H)**4)


def project(values, xs, t, harmonic):
    """Coefficients (c, s) de cos(mθ), sin(mθ), θ = kx − ωt + phase, sur une longueur d'onde entière."""
    n = len(values)
    c = 2/n*math.fsum(v*math.cos(harmonic*(K*x-OMEGA*t+PHASE)) for v, x in zip(values, xs))
    s = 2/n*math.fsum(v*math.sin(harmonic*(K*x-OMEGA*t+PHASE)) for v, x in zip(values, xs))
    return c, s


def slopes(values, dx):
    return [(values[i+1]-values[i-1])/(2*dx) for i in range(1, len(values)-1)]


def rms(values):
    values = list(values)
    return math.sqrt(math.fsum(v*v for v in values)/len(values))


def usage(path):
    dx, a, rows = read(path)
    xs = [(i+.5)*dx for i in range(round(L/dx))]
    gaps, refs, gslopes, rslopes = [], [], [], []
    phase_gap, phase_solver, h2 = [], [], []
    for t, y in rows:
        ref = field(xs, t, a)
        e = [b-c for b, c in zip(y, ref)]
        gaps += e; refs += ref
        gslopes += slopes(e, dx); rslopes += slopes(ref, dx)
        phase_gap.append((t, project(e, xs, t, 1)[1]/a))
        phase_solver.append((t, project(y, xs, t, 1)[1]/a))
        cy, sy = project(y, xs, t, 2); cr, sr = project(ref, xs, t, 2)
        h2.append((t, math.hypot(cy, sy), math.hypot(cr, sr), math.atan2(sy, cy)-math.atan2(sr, cr)))
    t_end, y_end = rows[-1]
    ref_end = field(xs, t_end, a)
    final_max = max(abs(b-c) for b, c in zip(y_end, ref_end))/max(abs(c) for c in ref_end)
    w2 = STOKES*(K*a)**2*OMEGA
    print(f'USAGE dx={dx} a={a} ecart_L2={math.sqrt(math.fsum(e*e for e in gaps)/math.fsum(r*r for r in refs)):.4f}'
          f' ecart_max_final_ADR120={final_max:.4f}')
    print(f'  hauteur : ecart_rms={rms(gaps):.3e} m max={max(map(abs, gaps)):.3e} m ;'
          f' eta_prime_rms={rms(refs):.3e} m ; ecart_rms/a={rms(gaps)/a:.2e}')
    print(f'  pente : ecart_rms={rms(gslopes):.3e} max={max(map(abs, gslopes)):.3e} ;'
          f' pente_eta_prime_rms={rms(rslopes):.3e} ; pente_houle ak={a*K:.3e}')
    tq, dq = phase_gap[-1]; ts, ds = phase_solver[-1]
    print(f'  dephasage (mode k en quadrature) a t={tq:.2f} s : ecart={dq:.3e} rad'
          f' ({dq/K*1e3:.3f} mm, {dq/OMEGA*1e3:.3f} ms) ; pas_reel={ds:.3e} rad ;'
          f' Stokes w2*t={w2*tq:.3e} rad')
    amp = [(r2 and y2/r2-1) for _, y2, r2, _ in h2[len(h2)//2:]]
    dph = [(p+math.pi) % (2*math.pi)-math.pi for *_, p in h2[len(h2)//2:]]
    print(f'  harmonique 2k (t>=1 s) : amplitude pas/oracle-1 moyen={sum(amp)/len(amp):+.4f},'
          f' dephasage moyen={sum(dph)/len(dph):+.4e} rad')
    return dx, a, rows, xs


def secular(full_path, half_path):
    """Signature de l'ordre trois : D = N(a) − N(a/2), N = η'/a², projeté sur sin θ."""
    dx, a, full = read(full_path)
    _, half_a, half = read(half_path)
    if half_a != a/2:
        raise ValueError('il faut a et a/2')
    xs = [(i+.5)*dx for i in range(round(L/dx))]
    predicted = STOKES*K*K*OMEGA*(a-half_a)
    share_num = share_den = 0.
    points = []
    for (t, y), (_, z) in zip(full, half):
        d = [p/a**2-q/half_a**2 for p, q in zip(y, z)]
        c, s = project(d, xs, t, 1)
        share_num += (c*c+s*s)*len(d)/2
        share_den += math.fsum(v*v for v in d)
        points.append((t, s))
    slope = math.fsum(t*s for t, s in points)/math.fsum(t*t for t, _ in points)
    linear = math.sqrt(math.fsum((s-slope*t)**2 for t, s in points)/math.fsum(s*s for _, s in points))
    print(f'SECULAIRE dx={dx} part_mode_k={share_num/share_den:.3f} pente_sin={slope:.4e} /m/s'
          f' Stokes={predicted:.4e} rapport={slope/predicted:.3f} ecart_a_la_droite={linear:.3f}')
    return slope/predicted


if __name__ == '__main__':
    if len(sys.argv) == 2:
        usage(sys.argv[1])
    elif len(sys.argv) == 4 and sys.argv[1] == '--seculaire':
        secular(sys.argv[2], sys.argv[3])
    else:
        print('usage : une trace, ou --seculaire a a/2', file=sys.stderr)
        sys.exit(2)
