"""S392 — essais des nombres du film d'eau (`sol_mouille.py`) : réponses connues, réciprocité, limites."""
import math
import unittest

import sol_mouille as sm


class Film(unittest.TestCase):
    def test_normal_incidence_is_the_textbook_value(self):
        # ((n − 1)/(n + 1))², n = 1,333.
        self.assertAlmostEqual(sm.fresnel(1.0, 1.0, 1.333), (0.333 / 2.333) ** 2, places=12)

    def test_total_internal_reflection_beyond_the_critical_angle(self):
        critique = math.asin(1 / 1.333)
        self.assertEqual(sm.fresnel(math.cos(critique + 1e-3), 1.333, 1.0), 1.0)
        self.assertLess(sm.fresnel(math.cos(critique - 1e-3), 1.333, 1.0), 1.0)

    def test_two_independent_integrations_meet_reciprocity(self):
        f = sm.film(1.333)
        self.assertAlmostEqual(f["r_i"], f["reciprocite"], delta=1e-4)
        self.assertAlmostEqual(f["r_e"], 0.0664, delta=1e-4)
        self.assertAlmostEqual(f["r_i"], 0.4746, delta=1e-4)

    def test_without_contrast_nothing_is_reflected(self):
        self.assertAlmostEqual(sm.reflectance_diffuse(1.0, 1.0, pas=1000), 0.0, places=12)

    def test_white_does_not_darken_and_black_halves(self):
        r_i = sm.film()["r_i"]
        self.assertAlmostEqual(sm.albedo_mouille(1.0, r_i), 1.0, places=12)
        self.assertAlmostEqual(sm.albedo_mouille(1e-9, r_i) / 1e-9, 1.0 - r_i, places=6)

    def test_predictions_of_the_plan(self):
        self.assertAlmostEqual(sm.rapport(0.42, 1.0), 0.643, delta=5e-4)
        self.assertAlmostEqual(sm.rapport(0.20, 1.0), 0.569, delta=5e-4)


if __name__ == "__main__":
    unittest.main()
