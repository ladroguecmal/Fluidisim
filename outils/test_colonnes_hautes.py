"""Contre-épreuves du calcul de dispersion des colonnes hautes (S386) : ce que chaque variante doit redonner."""
import math
import unittest
import colonnes_hautes as c

N, DX = 28, 0.25


def kappa2(lam):
    return 4 / DX ** 2 * math.sin(math.pi / lam * DX) ** 2


class Dispersion(unittest.TestCase):
    def test_fine_scheme_reproduces_s295(self):
        # Les deux valeurs que `oblique_standing_wave_follows_its_dispersion_s295` imprime.
        (n1, e1), (n2, e2) = c.s295()
        self.assertEqual((n1, n2), (16, 32))
        self.assertAlmostEqual(e1, -1.435e-2, delta=5e-6)
        self.assertAlmostEqual(e2, -3.683e-3, delta=5e-7)

    def test_degenerate_restrictions_are_the_fine_scheme(self):
        for lam in (1.0, 4.0, 14.0):
            fine = c.column(N, DX, kappa2(lam), 0)[0]
            self.assertEqual(c.column(N, DX, kappa2(lam), 1, "G")[0], fine)
            self.assertAlmostEqual(c.column_nodes(N, DX, kappa2(lam), list(range(N)))[0], fine, places=12)
            self.assertAlmostEqual(c.stretched_s([DX] * N, kappa2(lam)) * 1, fine, places=12)

    def test_galerkin_never_lowers_the_effective_depth(self):
        # Ritz : restreindre l'espace des pressions surestime l'énergie, donc `s` et la fréquence.
        for lam in (1.0, 4.0, 14.0):
            fine = c.column(N, DX, kappa2(lam), 0)[0]
            self.assertGreaterEqual(c.column(N, DX, kappa2(lam), 16, "G")[0], fine * (1 - 1e-14))
            nodes = c.graded_nodes(N, 3, 1.25)
            self.assertGreaterEqual(c.column_nodes(N, DX, kappa2(lam), nodes)[0], fine * (1 - 1e-14))

    def test_reduced_operators_are_symmetric(self):
        k2 = kappa2(4.0)
        ops = [c.column(N, DX, k2, 16, "G")[2], c.column(N, DX, k2, 16, "Q")[2],
               c.column_nodes(N, DX, k2, c.graded_nodes(N, 3, 1.25))[1]]
        for a in ops:
            scale = max(abs(x) for row in a for x in row)
            asym = max(abs(a[i][j] - a[j][i]) for i in range(len(a)) for j in range(len(a)))
            self.assertLess(asym, 1e-12 * scale)

    def test_graded_nodes_reach_the_bottom_with_growing_gaps(self):
        nodes = c.graded_nodes(N, 3, 1.25)
        self.assertEqual((nodes[0], nodes[-1]), (0, N - 1))
        gaps = [b - a for a, b in zip(nodes, nodes[1:])]
        self.assertTrue(all(g >= 1 for g in gaps))
        self.assertEqual(gaps[-3:], [1, 1, 1][:len(gaps[-3:])])

    def test_stretched_layers_fill_the_depth(self):
        hs = c.stretched_layers(7.0, DX, 5, 1.2)
        self.assertAlmostEqual(sum(hs), 7.0, places=12)
        self.assertEqual(hs[-5:], [DX] * 5)


if __name__ == "__main__":
    unittest.main()
