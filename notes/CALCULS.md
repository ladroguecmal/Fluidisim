# Calculs longs — le registre

*Tenu par `python outils/calcul.py` (S480) ; la colonne « état » se met à jour par `python outils/calcul.py etat`.* Un calcul de plus
de dix minutes se lance par l'outil : il survit à la conversation, sa sortie est dans `calculs/<id>/` (non versionné, dans le
dépôt). Ce qu'une session tire d'une sortie va dans sa preuve (`docs/validation/`) ; la sortie brute reste dans `calculs/`.

| id | session | nom | commande | état |
|---|---|---|---|---|
| `20261004-164944-s479-b10-bulle` | S480 | s479-b10-bulle | `garder b10_p24.txt bulle_b.txt` | sorties gardées |
| `20261004-164945-essai-survie` | S480 | essai-survie | `python -c "import time; print('début', flush=True); time.sleep(45); print('fini')"` | terminé, 2026-10-04 16:50 |
| `20261004-165114-essai-sortie` | S480 | essai-sortie | `python -c "import time; print('début', flush=True); time.sleep(8); print('fini')"` | terminé, 2026-10-04 16:51 |
| `20261004-165215-b10-poches-p24` | S480 | b10-poches-p24 | `APIC3D_POCHES=1 APIC3D_APRES=1.5 cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example apic3d_b10 -- 2 24` | en cours |
| `20261004-165232-essai-env` | S480 | essai-env | `ESSAI=42 python -c "import os; print(os.environ['ESSAI'])"` | terminé, 2026-10-04 16:52 |
| `20261004-221416-s481-bulle-suivi` | S481 | s481-bulle-suivi | `CAS=bulle MODE=suivi DUREE=0.15 viewer/target/release/water-viewer.exe --apic3d-poches` | échec (code 127), 2026-10-04 22:14 |
| `20261004-221515-s481-bulle-suivi` | S481 | s481-bulle-suivi | `CAS=bulle MODE=suivi DUREE=0.15 viewer/target/release/water-viewer.exe --apic3d-poches` | terminé, 2026-10-04 22:28 |
