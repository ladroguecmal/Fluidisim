"""Contre-exemples des erreurs de classement et d'historique de l'ancien indicateur."""
import unittest
from datetime import datetime, timezone

from etat_projet import (EN_COURS_LINES, JOURNAL_ENTRY_LINES, MILESTONE_WORDS, QUEUE_ROW_WORDS,
                         activity, category, checklist, en_cours, encoding, first_sessions,
                         heartbeat, history, journal, layer, oversized, produced, reproduce)


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


class ControlesS321Tests(unittest.TestCase):
    """S321, ADR-187 D3, D5, D6 : chaque contrôle refuse le contre-exemple réel qui l'a fait naître."""

    def test_en_cours_refuse_les_archives_et_la_longueur_de_s320(self):
        vrai = "# Travail en cours\n\n## Session en cours\n\n## Archive — notes de S308 (lot du rendu)\n"
        (message,) = en_cours(vrai)
        self.assertIn("archive", message)
        self.assertTrue(en_cours("ligne\n" * (EN_COURS_LINES + 1)))
        self.assertEqual(en_cours("# Travail en cours\n\n## Session en cours\n\nAucune section d'archive.\n"), [])

    def test_encodage_abime_par_powershell_est_refuse_s301(self):
        abime = "Battement : r" + chr(0xC3) + chr(0xA9) + "cent"  # « é » relu en ANSI
        apostrophe = "l" + chr(0xE2) + chr(0x20AC) + chr(0x2122) + "eau"  # « ’ » relu en ANSI
        found = encoding({"REPRISE.md": abime, "notes/JOURNAL.md": apostrophe})
        self.assertEqual(len(found), 2)
        propre = "Résultat déjà réécrit — « l’eau », Âge, Été, δ ≈ 3·10⁻⁷, √(D/g)"
        self.assertEqual(encoding({"docs/x.md": propre}), [])

    def test_fichiers_produits_versionnes_sont_refuses(self):
        paths = ["outils/__pycache__/etat_projet.cpython-312.pyc", "code/target/release/x.exe",
                 "outils/etat_projet.py", "docs/validation/CIBLE-target.md"]
        self.assertEqual(produced(paths),
                         ["fichier produit versionné : outils/__pycache__/etat_projet.cpython-312.pyc",
                          "fichier produit versionné : code/target/release/x.exe"])

    LISTE = """## 4. Volumique

- [ ] **4.8 Sortie vers W** — *partiel* — transfert.
- [ ] **4.9 Fusion** — *absent*.
- [ ] **4.12 Cavité** — *partiel* — banc 2D ; *absent* avant S320.
- [x] **4.20 Validé** — reçu.

## Décompte

| section | points | validés | partiels | absents |
|---|---:|---:|---:|---:|
"""

    def test_decompte_de_la_liste_suit_ses_points_s316_s320(self):
        juste = self.LISTE + "| 4. Volumique | 4 | 1 | 2 | 1 |\n| **total** | **4** | **1** | **2** | **1** |\n"
        self.assertEqual(checklist(juste), [])
        perime = self.LISTE + "| 4. Volumique | 4 | 1 | 0 | 3 |\n| **total** | **4** | **1** | **0** | **3** |\n"
        found = checklist(perime)
        self.assertEqual(len(found), 2)
        self.assertIn("affiché (4, 1, 0, 3), compté (4, 1, 2, 1)", found[0])

    def test_preuve_nouvelle_sans_reproduire_est_refusee(self):
        first = {"docs/validation/B10-APIC-S320.md": 320, "docs/validation/NEUVE-S321.md": 321,
                 "docs/validation/AVEC-S321.md": 321, "notes/JOURNAL.md": 321}
        texts = {"docs/validation/NEUVE-S321.md": "# Neuve\n\nRésultat.\n",
                 "docs/validation/AVEC-S321.md": "# Avec\n\n## Reproduire\n\n`cargo run`\n"}
        (message,) = reproduce(first, texts)
        self.assertIn("NEUVE-S321", message)

    def test_entree_de_journal_bornee_a_partir_de_s321(self):
        ancienne = "## S320 — longue\n\n" + "texte\n" * 37
        neuve = "## S321 — courte\n\n" + "texte\n\n" * JOURNAL_ENTRY_LINES
        self.assertEqual(journal(ancienne + "\n" + neuve), [])
        (message,) = journal(neuve + "texte\n")
        self.assertIn("S321 : 21 lignes", message)

    def test_premiere_session_d_un_fichier(self):
        # `git log` rend le plus récent d'abord ; un commit sans numéro vaut 0 (ancien).
        commits = [dict(session=321, changes=[(1, 0, "b.md"), (1, 0, "a.md")]),
                   dict(session=None, changes=[(1, 0, "c.md")]),
                   dict(session=300, changes=[(1, 0, "a.md")])]
        self.assertEqual(first_sessions(commits, ["a.md", "b.md", "c.md", "neuf.md"], 321),
                         {"a.md": 300, "b.md": 321, "c.md": 0, "neuf.md": 321})


if __name__ == "__main__":
    unittest.main()
