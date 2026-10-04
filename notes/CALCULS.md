# Calculs longs — le registre

*Tenu par `python outils/calcul.py` (S480) ; la colonne « état » se met à jour par `python outils/calcul.py etat`.* Un calcul de plus
de dix minutes se lance par l'outil : il survit à la conversation, sa sortie est dans `calculs/<id>/` (non versionné, dans le
dépôt). Ce qu'une session tire d'une sortie va dans sa preuve (`docs/validation/`) ; la sortie brute reste dans `calculs/`.

| id | session | nom | commande | état |
|---|---|---|---|---|
| `20261004-164944-s479-b10-bulle` | S480 | s479-b10-bulle | `garder b10_p24.txt bulle_b.txt` | sorties gardées |
| `20261004-164945-essai-survie` | S480 | essai-survie | `python -c "import time; print('début', flush=True); time.sleep(45); print('fini')"` | terminé, 2026-10-04 16:50 |
| `20261004-165114-essai-sortie` | S480 | essai-sortie | `python -c "import time; print('début', flush=True); time.sleep(8); print('fini')"` | terminé, 2026-10-04 16:51 |
