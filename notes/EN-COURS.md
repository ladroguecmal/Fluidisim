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

Session : S280 — en cours
Agent : Claude Opus 5, Claude Code desktop ; fichiers, git, cargo, Python, GPU local.
Entrée : « Continue », jeton libre, master 5a3500e. La file porte en tête le défaut ouvert de
S279 : **l'exclusion par le coût est absorbante** (L336) — un domaine qu'un pic fait sortir du
budget n'exécute plus de pas, donc ne produit plus de mesure, donc conserve le coût qui l'a fait
sortir. S279 l'a refermé en élargissant le budget de l'afficheur ; le défaut est entier.
Objectif : **qu'un pic ne condamne plus un domaine**, et que le budget reste inviolé.

### Ce que l'analyse a écarté avant d'écrire

- **Distribuer le reliquat au lieu d'exclure** (ADR-012 §1 point 5, le solveur s'arrête dans son
  budget) : juste en général, **inapplicable à δ**. Le pas couplé est tout ou rien — un pas partiel
  avancerait l'histoire de moins que `FRAME_US` en croyant l'avoir faite (`Live::advance` compte
  ses pas), et la surface mobile n'admet pas de demi-pas (ADR-152).
- **Admettre d'office un domaine affamé** : viole la seule propriété qu'ADR-012 §1 demande de
  défendre avant toutes les autres. Écarté sans discussion.
- **Faire décroître l'estimation dans l'ordonnanceur** : ce n'est pas son travail. ADR-012 §3 confie
  la mesure du coût au solveur, qui la réinjecte ; l'ordonnanceur ne doit pas inventer un chiffre
  que personne n'a mesuré.

**Ce qui reste, et qui est le vrai défaut** : l'estimation se faisait sur **le dernier pas**, alors
qu'ADR-012 §3 dit en toutes lettres que la cible de mesure est un **centile**, jamais une valeur
isolée. Un pic à 45,8 ms pour une médiane à 22 condamnait le domaine. La correction appartient à
l'hôte, pas au cœur.

### Plan

- [ ] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — estimation robuste dans l'hôte : médiane glissante des derniers pas payés, et retour
  vers l'estimation nominale quand plus rien n'est payé — un domaine qui ne tourne plus ne sait
  plus ce qu'il coûte, et le dire est plus honnête que de garder son pire chiffre. Essais.
- [ ] **P3** — réception : `--delta-budget=<ms>` pour éprouver le cas serré ; la bande survit à
  33 ms là où S279 la voyait mourir à 0,352 s ; relevé dynamique de S279 inchangé ; douze
  empreintes de R10 inchangées.
- [ ] **P4** — rituel §6.

### Notes de reprise

Chiffres de S279 à retrouver : bande morte à **0,352 s** sans retour avec un budget de 33 ms ;
relevé à 50 ms — allumée 0 s, éteinte **6,016 s**, rallumée **10,0 s**, trois transitions. Douze
empreintes R10 dans ORDONNANCEUR-S279. Coût de δ : médiane 23,1 ms, p95 29,2, **maximum 45,8**
(COUT-DIRECT-S276).
