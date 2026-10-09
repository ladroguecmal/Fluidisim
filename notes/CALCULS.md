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
| `20261005-004232-s482-v1-poches` | S482 | s482-v1-poches | `DUREE=60 APIC3D_POCHES=1 SORTIE=captures/s482 viewer/target/release/water-viewer.exe --v1-banc` | terminé, 2026-10-05 00:44 |
| `20261005-004435-s482-v1-temoin` | S482 | s482-v1-temoin | `DUREE=60 SORTIE=captures/s482t viewer/target/release/water-viewer.exe --v1-banc` | terminé, 2026-10-05 00:46 |
| `20261005-010641-s482-v1-poches-b` | S482 | s482-v1-poches-b | `DUREE=60 APIC3D_POCHES=1 SORTIE=captures/s482 viewer/target/release/water-viewer.exe --v1-banc` | terminé, 2026-10-05 01:08 |
| `20261005-010843-s482-v1-temoin-b` | S482 | s482-v1-temoin-b | `DUREE=60 SORTIE=captures/s482t viewer/target/release/water-viewer.exe --v1-banc` | terminé, 2026-10-05 01:10 |
| `20261005-203223-b10-poches-p24-paral` | S484 | b10-poches-p24-paral | `APIC3D_POCHES=1 APIC3D_APRES=1.5 FILS=16 code/target/release/examples/apic3d_b10.exe 2 24` | terminé, 2026-10-05 22:33 |
| `20261005-203441-remontee-r4` | S484 | remontee-r4 | `FILS=16 code/target/release/examples/apic3d_remontee.exe 4 0.04 1.5` | terminé, 2026-10-05 20:36 |
| `20261005-204739-remontee-r4-1ms` | S484 | remontee-r4-1ms | `FILS=16 PAS_US=1000 code/target/release/examples/apic3d_remontee.exe 4 0.04 1.5` | terminé, 2026-10-05 20:53 |
| `20261005-205939-remontee-r6-1ms` | S484 | remontee-r6-1ms | `FILS=16 PAS_US=1000 TRACE=1 code/target/release/examples/apic3d_remontee.exe 6 0.04 0.6` | interrompu (pas de fin.txt) |
| `20261006-002606-b10-gouttes-serie` | S488 | b10-gouttes-serie | interrompu (pas de fin.txt) |
import subprocess, os
for nd in (8, 12, 16):
    for g in (0, 1):
        env = dict(os.environ, FILS='16')
        if g:
            env['APIC3D_GOUTTES'] = '1'
        print(f'=== D/dx={nd} gouttes={g}', flush=True)
        r = subprocess.run([r'code/target/release/examples/apic3d_b10.exe', '2', str(nd)], env=env, capture_output=True, text=True, encoding='utf-8', errors='replace')
        for l in r.stdout.splitlines():
            if l.startswith('APIC3D_B10') and ('fr=' in l or 'GOUTTES' in l):
                print(l, flush=True)
        print('code', r.returncode, flush=True)
"` | lancé 2026-10-06 00:26 |
| `20261006-002902-b10-gouttes-serie2` | S488 | b10-gouttes-serie2 | terminé, 2026-10-06 00:47 |
import subprocess, os
for nd in (8, 12, 16):
    for g in (0, 1):
        env = dict(os.environ, FILS='16')
        if g:
            env['APIC3D_GOUTTES'] = '1'
        print(f'=== D/dx={nd} gouttes={g}', flush=True)
        r = subprocess.run([r'code/target/release/examples/apic3d_b10.exe', '2', str(nd)], env=env, capture_output=True, text=True, encoding='utf-8', errors='replace')
        for l in r.stdout.splitlines():
            if l.startswith('APIC3D_B10') and ('fr=' in l or 'GOUTTES' in l):
                print(l, flush=True)
        print('code', r.returncode, flush=True)
"` | lancé 2026-10-06 00:29 |
| `20261006-012537-eveil` | S491 | eveil | `python outils/eveil.py 14` | interrompu (pas de fin.txt) |
| `20261006-043934-roulis-s502` | S502 | roulis-s502 | `code/target/release/examples/rayonnement_coque.exe --mode roulis --omega 2.0,2.5,3.0` | terminé, 2026-10-06 04:44 |
| `20261006-043936-tangage-s502` | S502 | tangage-s502 | `code/target/release/examples/rayonnement_coque.exe --mode tangage --omega 3.5,4.0,4.5` | terminé, 2026-10-06 04:43 |
| `20261006-044722-lacet-s502` | S502 | lacet-s502 | `code/target/release/examples/rayonnement_coque.exe --mode lacet --omega 2.0,3.0,4.0` | terminé, 2026-10-06 04:52 |
| `20261006-044723-cavalement-s502` | S502 | cavalement-s502 | `code/target/release/examples/rayonnement_coque.exe --mode cavalement --omega 2.0,3.0,4.0` | terminé, 2026-10-06 04:52 |
| `20261006-044724-embardee-s502` | S502 | embardee-s502 | `code/target/release/examples/rayonnement_coque.exe --mode embardee --omega 2.0,3.0,4.0` | terminé, 2026-10-06 04:52 |
| `20261006-074013-eveil` | S504 | eveil | `python outils/eveil.py 14` | terminé, 2026-10-06 21:40 |
| `20261006-074606-c23-mesure` | S505 | c23-mesure | `code/target/release/examples/c23_coque.exe 5` | terminé, 2026-10-06 07:46 |
| `20261006-074926-c23-gouvernant` | S505 | c23-gouvernant | `code/target/release/examples/c23_coque.exe gouvernant` | terminé, 2026-10-06 07:51 |
| `20261006-075141-c23-rampe` | S505 | c23-rampe | `code/target/release/examples/c23_coque.exe 5` | terminé, 2026-10-06 07:52 |
| `20261006-075142-c23-gouvernant-rampe` | S505 | c23-gouvernant-rampe | `code/target/release/examples/c23_coque.exe gouvernant` | terminé, 2026-10-06 07:54 |
| `20261006-075505-c23-localise-0.5` | S505 | c23-localise-0.5 | `code/target/release/examples/c23_coque.exe localise 0.5` | terminé, 2026-10-06 07:56 |
| `20261006-075506-c23-localise-5` | S505 | c23-localise-5 | `code/target/release/examples/c23_coque.exe localise 5` | terminé, 2026-10-06 07:56 |
| `20261006-075709-c23-souscritique` | S505 | c23-souscritique | `code/target/release/examples/c23_coque.exe 2 0.0625,0.125,0.25,0.5,1` | terminé, 2026-10-06 07:57 |
| `20261006-075710-c23-gouvernant-souscritique` | S505 | c23-gouvernant-souscritique | `code/target/release/examples/c23_coque.exe gouvernant 0.5,1,2,3` | terminé, 2026-10-06 07:58 |
| `20261006-081101-a328-0.5` | S507 | a328-0.5 | `code/target/release/examples/c23_coque.exe localise 0.5` | terminé, 2026-10-06 08:12 |
| `20261006-081102-a328-2` | S507 | a328-2 | `code/target/release/examples/c23_coque.exe localise 2` | terminé, 2026-10-06 08:12 |
| `20261006-081300-a328-milieu-0.5` | S507 | a328-milieu-0.5 | `code/target/release/examples/c23_coque.exe localise 0.5` | terminé, 2026-10-06 08:14 |
| `20261006-081300-a328-milieu-2` | S507 | a328-milieu-2 | `code/target/release/examples/c23_coque.exe localise 2` | terminé, 2026-10-06 08:14 |
| `20261006-081311-a328-milieu2-0.5` | S507 | a328-milieu2-0.5 | `MILIEU=1 code/target/release/examples/c23_coque.exe localise 0.5` | terminé, 2026-10-06 08:14 |
| `20261006-081312-a328-milieu2-2` | S507 | a328-milieu2-2 | `MILIEU=1 code/target/release/examples/c23_coque.exe localise 2` | terminé, 2026-10-06 08:14 |
| `20261006-081538-a328-bosse` | S507 | a328-bosse | `BOSSE=1 code/target/release/examples/c23_coque.exe localise 0` | terminé, 2026-10-06 08:15 |
| `20261006-081612-a328-bosse2` | S507 | a328-bosse2 | `BOSSE=1 code/target/release/examples/c23_coque.exe localise 0` | terminé, 2026-10-06 08:17 |
| `20261006-105843-ref-sillage-s519` | S519 | ref-sillage-s519 | `python outils/reference_sillage.py instrument` | terminé, 2026-10-06 10:59 |
| `20261006-110837-sillage-delta-s520` | S520 | sillage-delta-s520 | `SILLAGE_NX=416 SILLAGE_NY=224 DUREE=30 VITESSE=3 RAMPE=3 SORTIE=calculs/delta_s520.bin D_MIN=23.06 D_MAX=34.59 viewer/target/release/water-viewer.exe --lineaire-sillage` | échec (code 101), 2026-10-06 11:08 |
| `20261006-152710-sillage-delta-s520b` | S520 | sillage-delta-s520b | `SILLAGE_NX=416 SILLAGE_NY=224 DUREE=30 VITESSE=3 RAMPE=3 SORTIE=calculs/delta_s520.bin D_MIN=23.06 D_MAX=34.59 viewer/target/release/water-viewer.exe --lineaire-sillage` | échec (code 101), 2026-10-06 15:27 |
| `20261006-152831-sillage-delta-s520c` | S520 | sillage-delta-s520c | `SILLAGE_NX=416 SILLAGE_NY=224 DUREE=30 VITESSE=3 RAMPE=3 SORTIE=calculs/delta_s520.bin D_MIN=23.06 D_MAX=34.59 viewer/target/release/water-viewer.exe --lineaire-sillage` | terminé, 2026-10-06 15:29 |
| `20261006-155157-w-profondeur-s522` | S522 | w-profondeur-s522 | `code/target/release/examples/c07_profondeur.exe calculs/w_profondeur_s522.bin` | terminé, 2026-10-06 15:52 |
| `20261006-155244-ref-profondeur-s522` | S522 | ref-profondeur-s522 | `PYTHONIOENCODING=utf-8 python outils/reference_sillage.py profondeur calculs/w_profondeur_s522.bin` | terminé, 2026-10-06 15:52 |
| `20261006-155722-ref-super-s523` | S523 | ref-super-s523 | `PYTHONIOENCODING=utf-8 python outils/reference_sillage.py supercritique` | terminé, 2026-10-06 15:57 |
| `20261006-155934-ref-super2-s523` | S523 | ref-super2-s523 | `PYTHONIOENCODING=utf-8 python outils/reference_sillage.py supercritique` | terminé, 2026-10-06 15:59 |
| `20261006-164625-conv25-s529` | S529 | conv25-s529 | `SILLAGE_DX=0.25 SILLAGE_NX=224 SILLAGE_NY=160 SILLAGE_NZ=12 DT_US=10000 DUREE=15 SORTIE=calculs/conv_s529_25.bin viewer/target/release/water-viewer.exe --lineaire-sillage` | terminé, 2026-10-06 16:46 |
| `20261006-164642-conv12-s529` | S529 | conv12-s529 | `SILLAGE_DX=0.125 SILLAGE_NX=448 SILLAGE_NY=320 SILLAGE_NZ=24 DT_US=5000 DUREE=15 SORTIE=calculs/conv_s529_12.bin viewer/target/release/water-viewer.exe --lineaire-sillage` | échec (code 101), 2026-10-06 16:46 |
| `20261006-164801-conv12b-s529` | S529 | conv12b-s529 | `SILLAGE_DX=0.125 SILLAGE_NX=448 SILLAGE_NY=320 SILLAGE_NZ=24 DT_US=5000 DUREE=15 SORTIE=calculs/conv_s529_12.bin viewer/target/release/water-viewer.exe --lineaire-sillage` | terminé, 2026-10-06 16:51 |
| `20261006-202153-eveil` | S547 | eveil | `python outils/eveil.py 14` | terminé, 2026-10-06 21:40 |
| `20261007-092553-eveil` |  | eveil | `python outils/eveil.py 14` | terminé, 2026-10-07 23:25 |
| `20261009-075126-eveil` |  | eveil | `python outils/eveil.py 14` | terminé, 2026-10-09 21:52 |
