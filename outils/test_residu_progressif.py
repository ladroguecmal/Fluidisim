"""Contre-épreuves indépendantes de l'oracle temporel S272."""
import math
import unittest
import residu_progressif as r

class Oracle(unittest.TestCase):
    def test_spatial_coefficients_against_midpoint_quadrature(self):
        count=32768
        for n in [0,1,3,4,5,17,64]:
            c,s=r.spatial(n)
            norm=(1 if n==0 else 2)/count
            qc=norm*math.fsum(math.cos(2*r.K*(i+.5)*r.L/count)*math.cos(n*math.pi*(i+.5)/count) for i in range(count))
            qs=norm*math.fsum(math.sin(2*r.K*(i+.5)*r.L/count)*math.cos(n*math.pi*(i+.5)/count) for i in range(count))
            self.assertLess(abs(c-qc),1e-7)
            self.assertLess(abs(s-qs),1e-7)

    def test_closed_solution_against_two_rk4_steps(self):
        a=.01
        for n in [1,4,17,63,64]:
            kn=n*math.pi/r.L;lam=kn*math.tanh(kn*r.H)
            d0,dc,ds,ks,kc=r.coefficients(n,a)
            def rhs(t,y,p):
                sn,cs=math.sin(r.NU*t),math.cos(r.NU*t)
                return lam*p+ks*cs+kc*sn, -r.G*y+d0+dc*cs+ds*sn
            errors=[]
            for dt in [.001,.0005]:
                y=p=0.;worst=peak=0.
                for j in range(round(2/dt)):
                    t=j*dt
                    b=rhs(t,y,p);c=rhs(t+dt/2,y+dt*b[0]/2,p+dt*b[1]/2)
                    d=rhs(t+dt/2,y+dt*c[0]/2,p+dt*c[1]/2)
                    e=rhs(t+dt,y+dt*d[0],p+dt*d[1])
                    y+=dt/6*(b[0]+2*c[0]+2*d[0]+e[0])
                    p+=dt/6*(b[1]+2*c[1]+2*d[1]+e[1])
                    exact=r.mode(n,t+dt,a)
                    worst=max(worst,abs(y-exact));peak=max(peak,abs(exact))
                errors.append(worst/peak if peak else worst)
            print(f'S272 oracle n={n} erreurs_RK4={errors}')
            self.assertLess(errors[0],1e-6);self.assertLessEqual(errors[1],errors[0])
            self.assertEqual(r.mode(n,0.,a),0.)

    def test_forcing_is_second_order_expansion_of_surface_conditions(self):
        # Conditions exactes évaluées à la vraie surface ; comparer leurs limites a².
        for x,t in [(.31,0.),(1.7,.43),(3.2,1.1)]:
            theta=r.K*x-r.OMEGA*t+r.PHASE
            sn,cs=math.sin(theta),math.cos(theta)
            for a in [1e-4,5e-5]:
                q=a*r.G*r.K/r.OMEGA;tau=math.tanh(r.K*r.H)
                zeta=a*cs;eta_x=-a*r.K*sn
                c=math.cosh(r.K*(zeta+r.H))/math.cosh(r.K*r.H)
                sh=math.sinh(r.K*(zeta+r.H))/math.cosh(r.K*r.H)
                u,w=q*c*cs,q*sh*sn
                exact_k=w-q*tau*sn-u*eta_x
                exact_d=r.G*a*(c-1)*cs-.5*(u*u+w*w)
                k2=a*q*r.K*math.sin(2*theta)
                d0=a*a*r.G*r.K*tau/2-q*q*(1+tau*tau)/4
                dc=a*a*r.G*r.K*tau/2-q*q*(1-tau*tau)/4
                d2=d0+dc*math.cos(2*theta)
                self.assertLess(abs(exact_k-k2)/(a*a),.002)
                self.assertLess(abs(exact_d-d2)/(a*a),.002)

if __name__=='__main__': unittest.main()
