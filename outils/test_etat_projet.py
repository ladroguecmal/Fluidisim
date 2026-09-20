"""Contre-exemples des erreurs de classement et d'historique de l'ancien indicateur."""
import unittest
from datetime import datetime, timezone

from etat_projet import (MILESTONE_WORDS, QUEUE_ROW_WORDS, activity, category, heartbeat,
                         history, layer, oversized)


class InventoryTests(unittest.TestCase):
    def test_harness_viewer_and_nested_sources_are_separate(self):
        self.assertEqual(category("code/water-harness/src/physics.rs"), "harnais_src")
        self.assertEqual(category("viewer/src/water.wgsl"), "afficheur_src")
        self.assertEqual(category("code/water-core/src/radial_impact/table.rs"), "coeur_src")

    def test_non_session_commit_does_not_inherit_the_previous_session(self):
        raw = ("\x1ea\tS227 P1 — plan\n2\t1\tnotes/EN-COURS.md\n"
               "\x1eb\tMaintenance\n99\t0\tcode/water-core/src/hydro_network.rs\n"
               "\x1ec\tS226 P6 — fin\n3\t0\tviewer/src/main.rs\n")
        self.assertEqual(activity(history(raw), 227), {"S220–S229": {"markdown": 2}})

    def test_eras_continue_after_the_old_hardcoded_limit(self):
        raw = "\x1ez\tS301 P2 — suite\n5\t0\tviewer/src/main.rs\n"
        self.assertEqual(activity(history(raw), 290), {"S300–S309": {"afficheur_src": 5}})

    def test_layer_is_explicitly_only_a_filename_hint(self):
        self.assertEqual(layer("code/water-core/src/hydro_network.rs"), "V")
        self.assertIsNone(layer("code/water-core/src/tests_hydro_network.rs"))
        self.assertIsNone(layer("code/water-core/src/host.rs"))

    def test_state_documents_have_word_ceilings_s294(self):
        short = "| **A1 / court** | état | déclencheur |"
        long = "| **A2 / long** | " + "mot " * QUEUE_ROW_WORDS + "| x |"
        roadmap = ("## 2. Les jalons\n\n### J1 — court\n\ntexte\n\n### J2 — long\n\n"
                   + "mot " * (MILESTONE_WORDS + 1) + "\n\n## 3. Suite\n\n" + "hors " * 999)
        found = oversized(short + "\n" + long, roadmap)
        self.assertEqual(len(found), 2)
        self.assertIn("A2 / long", found[0])
        self.assertIn("J2 — long", found[1])
        self.assertEqual(oversized(short, "## 2. Les jalons\n\n### J1\n\ncourt\n"), [])


class HeartbeatTests(unittest.TestCase):
    """S309 — le contre-exemple est l'erreur réelle de S308, pas un cas inventé."""

    MAINTENANT = datetime(2026, 9, 20, 11, 48, tzinfo=timezone.utc)  # 13:48 +02:00

    def test_battement_a_l_heure_ou_dans_le_passe_passe(self):
        self.assertEqual(heartbeat("Battement : 2026-09-20 13:48 +02:00\n", self.MAINTENANT), [])
        self.assertEqual(heartbeat("Battement : 2026-09-20 09:00 +02:00\n", self.MAINTENANT), [])

    def test_battement_extrapole_de_s308_est_refuse(self):
        (message,) = heartbeat("Battement : 2026-09-20 15:14 +02:00\n", self.MAINTENANT)
        self.assertIn("dans le futur de 86 min", message)

    def test_deux_minutes_de_tolerance_pour_l_ecart_de_lecture(self):
        self.assertEqual(heartbeat("Battement : 2026-09-20 13:50 +02:00\n", self.MAINTENANT), [])
        self.assertTrue(heartbeat("Battement : 2026-09-20 13:52 +02:00\n", self.MAINTENANT))

    def test_ligne_absente_ou_illisible_est_une_anomalie(self):
        self.assertTrue(heartbeat("JETON : libre\n", self.MAINTENANT))
        self.assertTrue(heartbeat("Battement : hier soir\n", self.MAINTENANT))


if __name__ == "__main__":
    unittest.main()
