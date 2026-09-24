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

Session : S352 — **terminée**. **ADR-190 D3 : la liste du projet fini, rangée par dépendance** — la première session
après la v1.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — S351 : porte A reçue au banc ; la v1 atteinte au sens d'ADR-174 D4, portes reçues séparément. Décision de
l'utilisateur ([ADR-190](../docs/adr/ADR-190-apres-la-v1-la-liste-entiere.md)) : désormais, la liste entière — 3 validés,
58 partiels, 59 absents.

**Ce que D3 demande.** Pour chacun des 117 points non validés : ce qu'il attend, ce qu'il débloque, son système ; puis
un ordre, pour que chaque session prenne un point dont les dépendances sont levées. **Lecture** : l'ordre — des
**fronts** successifs — va dans la feuille de route, seule porteuse de la trajectoire ; le détail point par point, trop
long pour elle, va dans un registre, comme le rangement par système de S309 (TROIS-SYSTEMES-S308 §8).

Critères, écrits avant le travail :
1. **Le registre** couvre les 117 points non validés, un par ligne, aucun oublié — un script compte les deux listes.
   Chaque ligne : système (A, B, C ou hors, repris de S308 §8), ce qui manque au périmètre final, ce qu'il attend —
   d'autres points, ou un fait extérieur au sens d'ADR-190 D5 —, ce qu'il débloque, son **front**.
2. **Les fronts** : 0 = rien d'autre à attendre qu'une session ; 1, 2… = après les points du front précédent ;
   E = un fait ou une action de l'utilisateur. Chaque point E nomme ce qu'on lui demandera.
3. **La feuille de route** porte les fronts, l'ordre proposé dans le front 0 et la place du lot 5 (ADR-184 D1 : une
   session sur deux) ; **la file** et **l'index** pointent le registre.
4. Aucun état de la liste ne change : c'est une carte, pas une capacité (maillons + 1, dit comme tel).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le registre, sections 1 à 6 de la liste (socle, B, W, δ, V, solides).
- [x] **P3** — le registre, sections 7 à 13 ; le compte des 117 points par script. *Fusion déclarée P2+P3 :
  un seul outil écrit et compte les treize sections.*
- [x] **P4** — les fronts dans la feuille de route ; file, index, liste (renvoi).
- [x] **P5** — rituel.

### Notes de reprise
- **P2+P3.** `outils/dependances_liste.py` : données écrites à la main (système, maintenant, attend, attente
  extérieure), fronts et « débloque » calculés, couverture et cycles contrôlés ; `--ecrire` régénère les tables de
  `docs/registres/DEPENDANCES-LISTE.md` ; `etat_projet.py --check` appelle `ecarts`, éprouvé sur trois défauts
  fabriqués (tables modifiées, point absent, dépendance inconnue). Un cycle trouvé et levé : 4.2 ↔ 4.9 — les
  interactions relèvent de 4.9. Résultat : **117 points ; front 0 : 34, 1 : 25, 2 : 9, 3 : 5, 4 : 2, 5 : 1, E : 41**
  (22 directs ; le réseau, 10.1, en commande 14). En aval transitif : 4.16 → 22 points, 2.7 → 20, 10.1 → 16.
  Systèmes refaits par énoncé : A 20, B 25, C 7, H 65 ouverts (S309 : B 29, H 64 sur 120).
