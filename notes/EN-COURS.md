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

Session : S162 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : instruire A217 : vérifier le blocage de la référence non linéaire dispersive,
puis établir ce qu'une expérience bornée peut dire de l'additivité en eau profonde.

### Plan

- [x] **P1** — état réel, outils, jeton et plan committé seul.
- [x] **P2** — lectures de reprise ; inventaire des références et formulation précise du manque.
      Synchroniser les trois copies propres sans avance vers le jeton courant, sans les supprimer.
- [ ] **P3** — confronter une voie analytique ou numérique minimale à la question A217 ;
      mesurer si une référence recevable existe, sinon publier le blocage argumenté et le prochain lot.
      Distinguer une correction liée d'ordre deux d'une référence intégrale évolutive.
- [ ] **P4** — verdict et propagation aux points ouverts ; contrôles adaptés, aucun seuil inventé.
- [ ] **P5** — rituel de fin REPRISE §6, journal, index, décomptes, suite et jeton rendu.

### Notes de reprise

Départ : master 832762f, arbre propre. Trois copies à 824ee62 (S159), toutes propres,
jetons libres ; aucune branche vivante plus avancée. Branche 5134cd archivée conservée.
Cargo 1.97.0 disponible. Aucun worktree créé. Suite héritée : 299 tests/cinq ignorés.
A217 prioritaire ; BILAN-S145 porté via la poursuite B4, après B1 et clôture S63-1.
Piège : un solveur linéaire donnerait une additivité exacte par construction ; un développement
faiblement non linéaire peut réfuter une universalité, pas recevoir B4 complet.
P2 : inventaire et distinction résidu/superposition publiés dans ADDITIVITE-PROFONDE-S162.
Trois copies synchronisées au commit P1 occupé. Aucun retrait.
