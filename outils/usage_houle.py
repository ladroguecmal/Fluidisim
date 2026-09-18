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


def three_amplitudes(n2a, na, nh):
    """N = η'/a² aux amplitudes 2a, a, a/2 → (N*(2a,a), N*(a,a/2), c₂) ; c₂ exact si N = c₂+c₃a+c₄a²."""
    pair_high = [2*m-h for h, m in zip(n2a, na)]
    pair_low = [2*l-m for m, l in zip(na, nh)]
    c2 = [lo+(lo-hi)/3 for hi, lo in zip(pair_high, pair_low)]
    return pair_high, pair_low, c2


def normalized(path):
    dx, a, rows = read(path)
    return dx, a, [(t, [v/a**2 for v in y]) for t, y in rows]


def coefficient(paths, control=None):
    """Critères HOULE-USAGE-S274 §5 : coefficient d'ordre deux du pas réel, pas la houle complète."""
    runs = [normalized(p) for p in paths]
    (dx, a2, _), (dx1, a1, _), (dx3, a3, _) = runs
    if not (dx == dx1 == dx3 and a2 == 2*a1 and a3 == a1/2):
        raise ValueError('il faut 2a, a, a/2 sur la même maille')
    xs = [(i+.5)*dx for i in range(round(L/dx))]
    sums = dict(high=0., low=0., c2=0., qual=0., ref=0., den=0.)
    last = None
    for (t, n2a), (_, na), (_, nh) in zip(runs[0][2], runs[1][2], runs[2][2]):
        oracle = [v/a1**2 for v in field(xs, t, a1)]
        high, low, c2 = three_amplitudes(n2a, na, nh)
        for key, values in (('high', high), ('low', low), ('c2', c2)):
            sums[key] += math.fsum((b-c)**2 for b, c in zip(values, oracle))
        sums['qual'] += math.fsum((b-c)**2 for b, c in zip(high, low))
        sums['ref'] += math.fsum((b-c)**2 for b, c in zip(c2, low))
        sums['den'] += math.fsum(c*c for c in oracle)
        last = (c2, oracle)
    r = {k: math.sqrt(v/sums['den']) for k, v in sums.items() if k != 'den'}
    final_max = max(abs(b-c) for b, c in zip(*last))/max(abs(c) for c in last[1])
    print(f'COEFFICIENT dx={dx} N*(2a,a)={r["high"]:.5f} N*(a,a/2)={r["low"]:.5f} c2={r["c2"]:.5f}'
          f' qualification={r["qual"]:.5f} e_reference_extrap={r["ref"]:.5f} max_final_c2={final_max:.4f}')
    if control:
        (_, ca, rows_a), (_, ch, rows_h) = normalized(control[0]), normalized(control[1])
        if ca != a1 or ch != a3:
            raise ValueError('contrôle temporel : a et a/2 attendus')
        num = den = 0.
        for (t, na), (_, nh), (_, ma), (_, mh) in zip(runs[1][2], runs[2][2], rows_a, rows_h):
            fine = [2*l-m for m, l in zip(na, nh)]
            half = [2*l-m for m, l in zip(ma, mh)]
            oracle = [v/a1**2 for v in field(xs, t, a1)]
            num += math.fsum((b-c)**2 for b, c in zip(fine, half))
            den += math.fsum(c*c for c in oracle)
        r['temps'] = math.sqrt(num/den)
        print(f'  e_temporel N*(a,a/2) dt/dt2 = {r["temps"]:.5f}')
    return r


if __name__ == '__main__':
    if len(sys.argv) == 2:
        usage(sys.argv[1])
    elif len(sys.argv) == 4 and sys.argv[1] == '--seculaire':
        secular(sys.argv[2], sys.argv[3])
    elif len(sys.argv) in (5, 7) and sys.argv[1] == '--coefficient':
        coefficient(sys.argv[2:5], sys.argv[5:7] or None)
    else:
        print('usage : une trace ; --seculaire a a/2 ; --coefficient 2a a a/2 [a a/2 à dt/2]', file=sys.stderr)
        sys.exit(2)
