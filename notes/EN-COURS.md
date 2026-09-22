# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S321 — **en cours**. **Demande de l'utilisateur** : *« reprends le projet, réalise une
analyse complète sur le code, les documents, méthodes de travail, réorganiser ou refaire des
principes des points améliorables »*.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : S320 close en reprise à chaud ; sa P5b est reportée ici (calcul en cours depuis 20:11).

**Ce que la session doit rendre possible.** Une méthode dont le coût suit le travail et non le
nombre de sessions ; une reprise qui se lit en peu ; des protections chargées au moment utile
plutôt que des leçons qu'on relit ; des réceptions qu'on peut reproduire. Consommateur : chaque
session suivante.

**Analyse faite avant ce plan**, en lecture seule : suites (578 + 36 réussis, 0 échec) ; historique
Git par session ; registres comptés et recoupés ; code lu, `clippy` et `rustfmt` passés ;
`simufluid` relu pour son propre retour d'expérience de méthode. Chiffres et constats : P2.

Critères, écrits avant le travail :
1. Le bilan chiffre chaque constat et nomme sa mesure ; ADR-187 dit, pour chaque principe, pourquoi
   et comment revenir en arrière.
2. `etat_projet.py --check` vérifie les contrôles nouveaux et passe ; ses essais passent, avec les
   contre-exemples réels du dépôt.
3. Lecture à froid ≤ 70 Ko ; `EN-COURS` ≤ 300 lignes, sans archive.
4. Zéro avertissement de construction (cœur, harnais, bancs, afficheur) ; version minimale vraie ;
   suites rejouées aux mêmes nombres d'essais, 0 échec.
5. Rien de la physique ne bouge : ni réception, ni seuil, ni porte, ni ambition ; AGENTS inchangé.

### Plan

- [x] **P1** — jeton, plan seul. *(Committé avec `[>]` par erreur ; coché en P2.)*
- [x] **P2** — bilan publié : `docs/registres/BILAN-GLOBAL-S321.md`.
- [x] **P3** — ADR-187 : les principes refondus, pourquoi, et comment les défaire.
- [x] **P4** — `EN-COURS` refondu : archives purgées (Git et les preuves les gardent), session en
  cours seule.
- [x] **P5** — METHODE refondue : principes, protections actives.
- [x] **P6** — REPRISE : lecture à froid bornée (§3), état court (§4), rituel en deux parties (§6),
  plafonds (§8).
- [x] **P7** — `etat_projet.py --check` étendu, avec essais : taille d'`EN-COURS`, encodage,
  fichiers produits versionnés, décompte de la liste, « Reproduire » des preuves nouvelles.
- [x] **P8a** — hygiène du dépôt : `.pyc` retirés et ignorés, `code/REPRISE.md` vide supprimé,
  README racine et du code, décomptes de la liste et de la feuille de route.
- [x] **P8b** — hygiène du code : `rust-version` vraie, avertissements à zéro, suites rejouées.
- [x] **P9** — index : carte par système en tête ; note datée à SPEC-003 ; file active.
- [>] **P10** — S320 P5b : §5 bis versé au retour du calcul — étape asynchrone, placée là où le
  calcul la permet.
- [ ] **P11** — rituel, appliqué sous sa forme nouvelle.

### Notes de reprise

**Reprise de S320 (20:11–20:25).** Arbre propre ; P5b relancée (`lot5_comparaison apic entree 0.0125 2
0.4`, sortie dans le répertoire temporaire de la session, hors dépôt) ; rituel de S320 terminé sans
attendre le calcul. Si la session est coupée avant P10 : **relancer la même commande** — ≈ 1 h à
1 h 45 à `D/dx` = 32 d'après les coûts de `D/dx` = 16 — puis verser §5 bis.

**P4.** Les notes archivées de S301 à S320 (1 560 lignes) sortent du fichier. Recoupées avant la
purge : leurs chiffres porteurs sont dans les preuves et le journal ; trois restent seulement dans
Git (le coût intermédiaire de 4,23 ms à 344 064 mailles de S302, la courbe `1 ; 1,4 ; 6` de S308,
dépassée par P7 de la même session, et l'indice de la bascule de S301, `n` = 215, maille
(19, 10, 32)) — retrouvables par la commande ci-dessus.

**P6.** Lecture à froid mesurée après la révision : **68 Ko** (AGENTS 10,3 ; REPRISE 12,8 ; dernière
entrée 2,8 ; invariants 11,7 ; ADR-001 §2 3,4 ; feuille §3 bis 8,6 ; décisions de la file 3,3 ; dix
lignes de porte 5,1 ; METHODE 10,3), contre 155 Ko avant.

**P7.** Six contrôles, chacun refusant son contre-exemple réel ; 16 essais de l'outil, 26 essais
Python au total. Au commit, `--check` échoue sur **douze** anomalies, toutes réelles et toutes pour
P8a : dix `.pyc` versionnés, et le décompte de la liste (section 4 et total : affiché 3 / 51 / 66,
compté 3 / 53 / 64). Piège rencontré : l'outil d'édition convertit les échappements Unicode en
caractères — le motif d'encodage s'est d'abord attrapé lui-même ; il est désormais écrit en
échappements construits par `chr(92)`, et l'outil ne se signale plus.

**P8b.** Zéro avertissement : `cargo test --no-run` et `cargo build` du code et de l'afficheur ;
suites rejouées **578 + 36 réussis, 0 échec**, 18 + 1 ignorés — mêmes nombres. Construit dans un
répertoire cible hors dépôt, parce que P5b tient `lot5_comparaison.exe` verrouillé (Windows).
Trois avertissements étaient de vrais défauts : `wake_plafond` imprimait un refus **vide** quand le
refus venait du deuxième ou du troisième profil (motif `(e, _, _) | …` inaccessible) ; `Mesure.masse`
de `lot5_comparaison` était calculée et jamais lue ; `delta3d_mobile` (S296) et `nl_surface_2d`
n'appellent pas `dispersion_error`, que leur module demande à tout banc (L277) — point de file.
Les campagnes du harnais que seuls les essais appellent sont déclarées au niveau du module.
