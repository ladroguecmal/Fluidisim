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

Session : S323 — **en cours**. **Lot 5, le raccord particules ↔ colonnes**
([ADR-186](../docs/adr/ADR-186-apic-seconde-representation.md) §3), précédé du compteur de volume
géométrique d'APIC qu'A313 exige avant lui.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« Continue »*, après S322 ; alternance d'ADR-184 et règle des deux maillons (à 2) : la
session doit faire avancer une capacité.

**Ce que la session doit rendre possible.** Faire passer une région d'eau d'une représentation à
l'autre — des particules d'APIC aux colonnes d'une fonction hauteur, et retour — **en conservant son
volume**, et savoir le mesurer. C'est le geste qui fera consommer B10 par δ ; il fait avancer 4.16
(surface non graphe) et 4.20 (changement de représentation en cours de simulation). Consommateur :
le raccord dynamique, puis la porte D et C20.

**Le compteur (A313).** Volume géométrique = aire où `φ < 0`, `φ` la surface reconstruite des
particules, par carrés marchants sur la grille des centres — **l'interface même que voit le fluide
fantôme** —, bandes contre les parois comprises, corps et air enfermé exclus par construction.

**Les primitives.** *Particules → colonnes* : dans une colonne à un seul segment d'eau posé sur le
fond, `η` = volume géométrique de la colonne / `dx`. *Colonnes → particules* : ensemencer sous `η` au
quart de maille, le reste de chaque colonne reporté à la suivante pour que la masse totale tienne.

Critères, écrits avant le code :
1. Compteur **exact** sur une interface plane (à l'arrondi), **d'ordre deux** sur un disque — le
   rapport d'erreur vaut ≈ 4 quand la maille est divisée par deux.
2. Publiés : sa dérive au repos et en ballottement sur 10 s ; le tassement du corps lent, **avec et
   sans séparation**, lu directement comme l'écart entre volume géométrique et masse.
3. Aller-retour colonnes → particules → colonnes sur un état réel : masse totale à **une particule
   près** (`dx²/4`), volume géométrique par colonne à **0,2 maille** de hauteur près.
4. Rien dans le cœur ; aucune réception antérieure touchée (ballottement de S318 au bit).

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — le compteur, et son épreuve sur des distances exactes (plan, disque, trois mailles).
- [ ] **P3** — le compteur sur repos, ballottement, corps lent avec et sans séparation, B10 à `Fr` = 2.
- [ ] **P4** — les deux primitives, et l'aller-retour sur un état réel.
- [ ] **P5** — preuve : section datée de [B10-APIC-S320](../docs/validation/B10-APIC-S320.md), avec
  « Reproduire » ; A313, file, liste 4.16 et 4.20 si la mesure le permet.
- [ ] **P6** — S320 P5b : §5 bis au retour du calcul lancé à 20:11 — asynchrone.
- [ ] **P7** — rituel.

### Notes de reprise

