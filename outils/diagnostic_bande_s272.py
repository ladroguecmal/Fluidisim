"""Diagnostic indépendant de la quadrature de bande ADR-152, pas un reçu du pas réel."""
import math
from residu_progressif import L,H,G,K,OMEGA,PHASE


def compare(dx,a):
    nx=round(L/dx);nz=round(1.5/dx);q=a*G*K/OMEGA
    heights=[a*math.cos(K*(i+.5)*dx+PHASE) for i in range(nx)]
    rectangle=[];linear=[];exact=[]
    for i in range(nx+1):
        x=i*dx;cs=math.cos(K*x+PHASE)
        zeta=heights[0] if i==0 else heights[-1] if i==nx else (heights[i-1]+heights[i])/2
        # ADR-165 prescrit la hauteur analytique aux faces externes, eta'=0.
        if i in (0,nx): zeta=a*cs
        r=l=0.
        for layer in range(nz):
            zc=(layer+.5)*dx-H
            lo=layer*dx-H;hi=(layer+1)*dx-H
            end=max(lo,min(hi,zeta));start=max(lo,min(hi,0.))
            u=q*math.cosh(K*(zc+H))/math.cosh(K*H)*cs
            uz=q*K*math.sinh(K*(zc+H))/math.cosh(K*H)*cs
            r+=u*(end-start)
            l+=u*(end-start)+uz/2*((end-zc)**2-(start-zc)**2)
        rectangle.append(r);linear.append(l)
        exact.append(q*cs/K*(math.sinh(K*(H+zeta))-math.sinh(K*H))/math.cosh(K*H))
    norm=sum(v*v for v in exact)
    errs=[math.sqrt(sum((b-c)**2 for b,c in zip(values,exact))/norm) for values in [rectangle,linear]]
    print(f'BANDE dx={dx} a={a} rectangle={errs[0]:.9g} reconstruction_lineaire={errs[1]:.9g}')
    return errs


if __name__=='__main__':
    for dx in [.125,.0625,.03125]:
        for a in [.01,.005]: compare(dx,a)
