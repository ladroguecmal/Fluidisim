# La structure du projet — S480

*S480, 2026-10-04, décision de l'utilisateur ([ADR-221](../adr/ADR-221-la-structure-du-projet.md)).* La session a été coupée après
P1 (battement 12:03) et reprise à chaud à 16:37 : P2 était écrite et cochée, non committée — complétée.

## Reproduire

- `python outils/rituel.py debut` — l'amorce, le jeton et ce qu'il commande, le tableau de bord, les calculs.
- `python outils/tableau_de_bord.py --check`, `python outils/decisions.py --check`, `python outils/anomalies.py --check` — chacun à 0 ;
  `python outils/etat_projet.py --check` les appelle tous les trois.
- `python outils/calcul.py lancer essai -- python -c "import time; time.sleep(45); print('fini')"`, fermer la conversation, puis
  `python outils/calcul.py etat` : « terminé ».
- `python outils/rituel.py fin --session S480 --suivante "…"` sur un plan non coché : deux `MANQUE`, aucun fichier modifié.

## 1. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| (1) une session froide trouve en deux pages le pourquoi, les décisions en vigueur et la suite | [BOUSSOLE](../../BOUSSOLE.md) : le pourquoi (les intentions d'origine, DyingStar), la fin (la liste à 100 %), l'architecture en quatre lignes, seize décisions en une ligne, ce qui attend l'utilisateur, la méthode, les pièges ; la suite renvoie au jeton | tenu |
| (2) les trois registres régénérés à l'identique, `--check` à 0 | régénérés deux fois, aucune différence ; une ligne ajoutée à la main est détectée par `etat_projet --check` | tenu |
| (3) le journal archivé sans perte | 479 entrées avant, 99 + 100 + 100 + 100 + 70 archivées + 10 vivantes après ; le texte des entrées identique à l'espacement près ; liens relatifs recalés d'un niveau, 0 lien mort | tenu |
| (4) un calcul lancé par `calcul.py` survit à la conversation et se retrouve par le registre | un calcul de 45 s, son shell fermé aussitôt, retrouvé « terminé » ; B10 à 24 mailles lancé ainsi (≈ 2 h 30) | tenu |
| (5) le rituel de fin de S480 fait par `rituel.py` | voir le commit de P5 | tenu |

## 2. Ce que la structure a trouvé dès son premier passage

- **A322 levée en S442 n'avait jamais été écrite au registre des angles morts** : la vue des anomalies ouvertes la donnait ouverte.
  Note datée ajoutée ; le registre généré la donne close.
- **Les sorties de S479** (`b10_p24.txt`, `bulle_b.txt`) n'existaient que dans le dossier temporaire d'une conversation : mises à
  l'abri dans `calculs/`. `b10_p24.txt` est vide — le calcul est mort avec la conversation ; `bulle_b.txt` donne, dans la grande cuve
  (80 × 80 × 68, dx = 2 cm, R = 8 cm), **47,84 Hz contre 42,22** de Minnaert (rapport 1,133, six périodes). La petite cuve donnait
  × 1,021 ; la correction de cuve par les images va dans l'autre sens (× 0,96 attendu) : l'écart est à attribuer par K2-2.
- Sur 220 ADR, 49 portent encore « proposée » — les premières sessions, avant que l'usage n'écrive « actée » ; la colonne « nommé par »
  du registre dit lesquelles ont été appliquées depuis.

## 3. Les lectures des outils, et leurs limites

- `decisions.py` lit le statut par mots dans l'en-tête ; aucun ADR n'écrit « remplacée par » dans son propre en-tête (un ADR ne se
  réécrit pas) : la suite d'un ADR se lit dans les plus récents qui le nomment.
- `anomalies.py` ne lit que les entrées à statut (39, depuis ≈ S280) ; les anciennes, au tableau général, ne sont pas reprises. Une
  note en italique qui nomme une autre anomalie ne tranche pas (en S436, « A324 corrigée » est écrit sous A320). A303 a été donné
  deux fois (S309, S312) : la seconde est « A303 bis ».
- `calcul.py` : sous Windows, un processus détaché n'a pas de console ; la commande reçoit des poignées explicites, sans quoi sa sortie
  se perd.
