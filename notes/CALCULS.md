# Calculs longs — le registre

*Tenu par `python outils/calcul.py` (S480) ; la colonne « état » se met à jour par `python outils/calcul.py etat`.* Un calcul de plus
de dix minutes se lance par l'outil : il survit à la conversation, sa sortie est dans `calculs/<id>/` (non versionné, dans le
dépôt). Ce qu'une session tire d'une sortie va dans sa preuve (`docs/validation/`) ; la sortie brute reste dans `calculs/`.

| id | session | nom | commande | état |
|---|---|---|---|---|
| `20261004-164944-s479-b10-bulle` | S480 | s479-b10-bulle | `garder b10_p24.txt bulle_b.txt` | sorties gardées |
| `20261004-164945-essai-survie` | S480 | essai-survie | `python -c "import time; print('début', flush=True); time.sleep(45); print('fini')"` | terminé, 2026-10-04 16:50 |
| `20261004-165114-essai-sortie` | S480 | essai-sortie | `python -c "import time; print('début', flush=True); time.sleep(8); print('fini')"` | terminé, 2026-10-04 16:51 |
| `20261004-165215-b10-poches-p24` | S480 | b10-poches-p24 | `APIC3D_POCHES=1 APIC3D_APRES=1.5 cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example apic3d_b10 -- 2 24` | échec (code 3221225786 — tué par Ctrl+C / fin de session), 2026-10-04 23:46 |
| `20261004-165232-essai-env` | S480 | essai-env | `ESSAI=42 python -c "import os; print(os.environ['ESSAI'])"` | terminé, 2026-10-04 16:52 |
| `20261004-221416-s481-bulle-suivi` | S481 | s481-bulle-suivi | `CAS=bulle MODE=suivi DUREE=0.15 viewer/target/release/water-viewer.exe --apic3d-poches` | échec (code 127), 2026-10-04 22:14 |
| `20261004-221515-s481-bulle-suivi` | S481 | s481-bulle-suivi | `CAS=bulle MODE=suivi DUREE=0.15 viewer/target/release/water-viewer.exe --apic3d-poches` | terminé, 2026-10-04 22:28 |
| `20261004-222939-s481-v1-poches` | S481 | s481-v1-poches | `APIC3D_POCHES=1 DUREE=60 SORTIE=captures/s481 viewer/target/release/water-viewer.exe --v1-banc` | échec (code 1), 2026-10-04 22:31 |
| `20261004-223338-s481-v1-poches-2` | S481 | s481-v1-poches-2 | `APIC3D_POCHES=1 DUREE=60 SORTIE=captures/s481 viewer/target/release/water-viewer.exe --v1-banc` | échec (code 1), 2026-10-04 22:35 |
| `20261004-224708-s481-v1-poches-min8` | S481 | s481-v1-poches-min8 | `APIC3D_POCHES=1 DUREE=60 SORTIE=captures/s481 viewer/target/release/water-viewer.exe --v1-banc` | terminé, 2026-10-04 22:51 |
| `20261004-225159-s481-v1-temoin` | S481 | s481-v1-temoin | `DUREE=60 SORTIE=captures/s481t viewer/target/release/water-viewer.exe --v1-banc` | terminé, 2026-10-04 22:53 |
| `20261004-225411-s481-b10-p16-poches` | S481 | s481-b10-p16-poches | `APIC3D_POCHES=1 ND=16 viewer/target/release/water-viewer.exe --apic3d-carte-b10` | échec (code 3221225786 — tué par Ctrl+C / fin de session), 2026-10-04 23:46 |
| `20261004-234747-essai-detache` | S481 | essai-detache | `python -c "import time; time.sleep(5); print('ok')"` | terminé, 2026-10-04 23:47 |
| `20261004-234817-s481-b10-p16-poches-2` | S481 | s481-b10-p16-poches-2 | `APIC3D_POCHES=1 ND=16 viewer/target/release/water-viewer.exe --apic3d-carte-b10` | interrompu (pas de fin.txt) |
| `20261004-234907-essai-wmi` | S481 | essai-wmi | `ESSAI=7 python -c "import os,time; time.sleep(3); print('ok', os.environ['ESSAI'])"` | terminé, 2026-10-04 23:49 |
| `20261004-234949-essai-wmi2` | S481 | essai-wmi2 | `ESSAI=8 python -c "import os,time; time.sleep(3); print('ok', os.environ['ESSAI'])"` | terminé, 2026-10-04 23:49 |
| `20261004-235007-s481-b10-p16-poches-3` | S481 | s481-b10-p16-poches-3 | `APIC3D_POCHES=1 ND=16 viewer/target/release/water-viewer.exe --apic3d-carte-b10` | terminé, 2026-10-05 00:07 |
| `20261005-003206-s482-bulle-suivi-mg` | S482 | s482-bulle-suivi-mg | `CAS=bulle MODE=suivi DUREE=0.15 viewer/target/release/water-viewer.exe --apic3d-poches` | terminé, 2026-10-05 00:42 |
