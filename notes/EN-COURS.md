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

---

## Session en cours

Session : S125 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S124-1 / A202 — mesurer construction, évaluation et mémoire à N=64/128/256,
puis trancher le choix du profil sans confondre domaine calculable et réception physique.

### Plan

- [x] **P1** — amorce, état réel, jeton et déclaration du plan seule.
- [x] **P2** — sonde reproductible : domaine commun et portée étendue, mise en régime,
      ordre alterné, médianes et dispersion, empreinte mémoire ; contrôle des sorties finies.
- [x] **P3** — rapport et ADR sur le profil retenu après mesure ; application nécessaire
      au code ou documentation explicite du maintien, sans migration silencieuse des fixtures.
- [ ] **P4** — vérification adaptée, rituel de fin complet, jeton rendu et commits propres.

### Notes de reprise

Départ 52e80a5 sur master. Deux worktrees propres au même commit ; c107bf ancien sans
avance ni modification ; lignée 5134cd archivée. Aucun worktree créé.
Cargo 1.97 disponible. S124 annonce neuf cas sur onze admis à N256 ; réception physique
limitée au scénario historique. Mesurer d'abord le candidat radial isolé ; cycle mixte
S118 ~49 ms cité comme contexte historique, pas comme mesure sur cette machine.

P2 : deux mesures release, sonde et COUT-PROFIL-IMPACT-S125. Coût commun N256/N64 ~4 ;
extension R128/N256 contre R16/N64 ~9. Les profils changent les bits ; WLIV encode déjà N.
Suite complète lancée, résultat final à recueillir en P4.

P3 : ADR-085 actée ; défaut N64 maintenu, N128/N256 explicites par service homogène.
Contrat documenté dans radial_impact et Prepared, aucun comportement ni codec modifié.
