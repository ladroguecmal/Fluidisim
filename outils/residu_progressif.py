"""Oracle modal indépendant S272 ; bibliothèque standard uniquement."""
import math
from pathlib import Path
import sys

L, H, G, K, PHASE = 4., 1., 9.81, math.pi/2, 0.37
OMEGA = math.sqrt(G*K*math.tanh(K*H))
NU = 2*OMEGA


def spatial(n):
    kn=n*math.pi/L
    c=1. if n==4 else 0.
    s=0. if n%2==0 else 2/L*(1/(2*K+kn)+1/(2*K-kn))
    return c,s


def coefficients(n, amplitude):
    c,s=spatial(n)
    q=amplitude*G*K/OMEGA
    tau=math.tanh(K*H)
    d0=amplitude**2*G*K*tau/2-q*q*(1+tau*tau)/4
    dc=amplitude**2*G*K*tau/2-q*q*(1-tau*tau)/4
    ak=amplitude*q*K
    ca,sa=math.cos(2*PHASE),math.sin(2*PHASE)
    return (d0 if n==0 else 0.,dc*(c*ca-s*sa),dc*(c*sa+s*ca),
            ak*(s*ca+c*sa),ak*(s*sa-c*ca))


def mode(n,t,amplitude):
    if n==0:
        return 0.  # K moyen nul sur la longueur d'onde entière ; D0 ne change que phi0.
    kn=n*math.pi/L
    lam=kn*math.tanh(kn*H)
    big=math.sqrt(G*lam)
    d0,dc,ds,ks,kc=coefficients(n,amplitude)
    ac=lam*dc+NU*kc
    bs=lam*ds-NU*ks
    denominator=big*big-NU*NU
    return (lam*d0/(big*big)*(1-math.cos(big*t))
            +ac/denominator*(math.cos(NU*t)-math.cos(big*t))
            +bs/denominator*(math.sin(NU*t)-NU/big*math.sin(big*t))
            +ks/big*math.sin(big*t))


def field(xs,t,amplitude,modes=256):
    coeff=[mode(n,t,amplitude) for n in range(1,modes+1)]
    return [math.fsum(y*math.cos((n+1)*math.pi*x/L) for n,y in enumerate(coeff)) for x in xs]


def read(path):
    text=Path(path).read_text(encoding='utf-8-sig')
    if 'FIN iterations_max=' not in text or 'REFUS' in text:
        raise ValueError('calcul incomplet ou refusé')
    header=next(row for row in text.splitlines() if row.startswith('CAS '))
    values=dict(item.split('=') for item in header.split()[1:])
    dx,a=float(values['dx']),float(values['a'])
    rows=[]
    for line in text.splitlines():
        if line.startswith('TRACE '):
            tokens=line.split()
            rows.append((float(tokens[1][2:]),list(map(float,tokens[2:]))))
    if len(rows)!=40 or any(abs(t-(i+1)*0.05)>1e-9 or len(y)!=round(L/dx)
                           or not all(math.isfinite(z) for z in y) for i,(t,y) in enumerate(rows)):
        raise ValueError('trace incomplète ou non finie')
    return dx,a,rows


def measure(path):
    dx,a,rows=read(path)
    xs=[(i+0.5)*dx for i in range(round(L/dx))]
    err,den,guard=0.,0.,0.
    for t,y in rows:
        ref=field(xs,t,a,256);low=field(xs,t,a,128)
        err+=math.fsum((b-c)**2 for b,c in zip(y,ref))
        den+=math.fsum(c*c for c in ref)
        guard+=math.fsum((b-c)**2 for b,c in zip(low,ref))
    result,convergence=math.sqrt(err/den),math.sqrt(guard/den)
    print(f'MESURE dx={dx} a={a} erreur={result:.9g} garde_oracle={convergence:.9g}')
    if convergence>0.001:
        raise ValueError('oracle modal non convergé')
    if result>0.02:
        raise ValueError('écart résiduel >2 % : non reçu')
    return result


def sensitivity(reference_path,other_path):
    dx,a,base=read(reference_path)
    other_dx,other_a,other=read(other_path)
    if dx!=other_dx: raise ValueError('mailles différentes pour la sensibilité')
    xs=[(i+.5)*dx for i in range(round(L/dx))]
    err=den=0.
    for (t,y),(t2,z) in zip(base,other):
        if t!=t2: raise ValueError('instants différents')
        ref=field(xs,t,a)
        err+=math.fsum((b-c*(a/other_a)**2)**2 for b,c in zip(y,z))
        den+=math.fsum(v*v for v in ref)
    result=math.sqrt(err/den)
    print(f'SENSIBILITE erreur_normalisee={result:.9g}')
    return result


if __name__=='__main__':
    try:
        if len(sys.argv)==2: measure(sys.argv[1])
        elif len(sys.argv)==4 and sys.argv[1]=='--sensibilite': sensitivity(sys.argv[2],sys.argv[3])
        else: raise ValueError('une trace, ou --sensibilite reference autre')
    except (ValueError,OSError) as e:
        print(f'NON_RECU : {e}',file=sys.stderr)
        sys.exit(1)
