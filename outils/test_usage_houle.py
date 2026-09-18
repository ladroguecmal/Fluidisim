"""Contre-épreuves de l'instrument S274, sans trace."""
import math
import unittest
import usage_houle as u


class Instrument(unittest.TestCase):
    def test_three_amplitudes_recover_second_order_coefficient(self):
        a = .01
        c2, c3, c4 = [.3, -1.2], [40., -7.], [900., 250.]
        n = lambda amp: [x+y*amp+z*amp*amp for x, y, z in zip(c2, c3, c4)]
        high, low, got = u.three_amplitudes(n(2*a), n(a), n(a/2))
        for g, want in zip(got, c2):
            self.assertAlmostEqual(g, want, places=9)
        for lo, want, z in zip(low, c2, c4):
            self.assertAlmostEqual(lo, want-z*a*a/2, places=9)

    def test_projection_on_a_full_wavelength(self):
        dx = .03125; xs = [(i+.5)*dx for i in range(round(u.L/dx))]
        t = .7; th = [u.K*x-u.OMEGA*t+u.PHASE for x in xs]
        values = [.2*math.cos(v)+.05*math.sin(v)+.03*math.cos(2*v) for v in th]
        c, s = u.project(values, xs, t, 1)
        self.assertAlmostEqual(c, .2, places=12); self.assertAlmostEqual(s, .05, places=12)
        c2, s2 = u.project(values, xs, t, 2)
        self.assertAlmostEqual(c2, .03, places=12); self.assertAlmostEqual(s2, 0., places=12)

    def test_stokes_coefficient_limits(self):
        # Profondeur infinie : ω₂/ω = (ka)²/2 ; ici kh = π/2.
        deep = (8+math.cosh(4*20.)-2)/(16*math.sinh(20.)**4)
        self.assertAlmostEqual(deep, .5, places=6)
        self.assertAlmostEqual(u.STOKES, .6107, places=3)


if __name__ == '__main__':
    unittest.main()
