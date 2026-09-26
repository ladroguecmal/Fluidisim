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

Session : S372 — **en cours**. Verdict **R26** : *« je valide continue »* — la caméra à demi immergée et son ménisque
reçus. Par l'alternance d'ADR-191 D3, **la physique** ; à deux maillons (S370, S371), un lot qui **change l'état d'un
point** : **5.4 vannes et pompes**, *absent*, au front 0, dans V (ADR-010 §1 les nomme parmi les arêtes ; noyau reçu,
intact depuis S229). A320, la coque qui bouge ou la côte avancent sans changer d'état en une session.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Thèse.** Une **commande** par arête, `control_pm` (0 à 1 000, entier, état répliqué, 1 000 par défaut) : un orifice
commandé est une **vanne** (section × commande), un déversoir commandé une vanne de largeur, et une **pompe** est une
loi nouvelle — réseau **ouvert** : courbe parabolique `H(Q) = H0·(1 − (Q/Qmax)²)`, point de fonctionnement contre la
hauteur statique, clapet (pas de retour), prise dénoyée à sec, vitesse `n` par les lois de similitude (`Qmax·n`,
`H0·n²`). Le réseau fermé sous pression reste en v2 (ADR-010 §4). L'instantané passe en version 2 : la commande est un
état, pas une configuration (ADR-140).

**Critères, écrits avant.** (1) À commande 1 000, **tous les essais V existants inchangés au bit**. (2) Vanne à 500 : la
vidange de C12 dure **2 × 728 s à ±3 %** (intégrale analytique) ; à 0, **aucun millilitre** ne passe. (3) Pompe qui
vide A dans B à décharge libre : le temps jusqu'à la prise dénoyée contre l'intégrale analytique
`t = 2·√H0·A·(√u0 − √u1)/Qmax` **à ±1 %** ; volume final à un pas près. (4) Pompe noyée : l'équilibre à la hauteur de
barrage (`Δh = n²·H0`) **à ±1 %** ; vitesse ½ : débit à Δh nul = `½·Qmax` à ±1 %. (5) Masse conservée **exactement**,
pas déterministe au bit, refus atomiques. (6) Instantané v2 : une commande changée survit à la restauration, au bit.

### Plan

- [x] **P1** — jeton, plan seul ; R26 consigné.
- [x] **P2** — ADR-199 : vannes et pompes dans V (lois, commande, provenance, ce qui n'est pas fait).
- [x] **P3** — `control_pm` sur `Opening` ; la vanne ; critères 1 et 2.
- [x] **P4** — la pompe ; critères 3 à 5.
- [x] **P5** — l'instantané WVST v2 ; critère 6.
- [ ] **P6** — preuve `VANNES-POMPES-S372`, liste 5.4, file, feuille de route, index, ADR-010 et ADR-140 notes datées.
- [ ] **P7** — rituel.

### Notes de reprise

**P3 — fait.** `Opening::control_pm` (`CONTROL_FULL` = 1 000, `Default` écrit à la main), refus hors de 0..=1 000 ;
orifice et déversoir × `c/1 000`. **Critère 1** : empreinte des trajectoires de quatre montages (C12 2 000 pas, chaîne
600, mixte 200, déversoir 1 000) relevée sur `c04b2974` **avant** la modification, `0xa02de06bc52bfd2e` — inchangée ;
33 essais V passent. **Critère 2** : C12 à 1 000 / 500 / 250 : 727,4 / 1 455,1 / 2 910,6 s pour 728,3 / 1 456,5 /
2 913,1 (−0,12 / −0,10 / −0,08 %) ; fermée 10 000 pas : 0 ml, reste intact ; fermée 60 s puis rouverte : +600 pas
exactement ; déversoir à 250 : 0,249989. Littéraux `Opening` complétés dans les essais, l'exemple et `tests/`.

**P4 — fait.** `Flow::Pump { max_flow_mlps, shutoff_head_um, outlet_um }` : prise = position de l'arête, refoulement
libre ou noyé, clapet, à sec, similitude ; refus si `H0 ≤ 0` ou débit négatif ; empreinte de la base étendue (étiquette
2). **Critère 3** : prise dénoyée à **207,3 s** pour 207,25 (+0,025 %), A garde 99 772 ml (228 ml sous la prise, un pas).
**Critère 4** : barrage **249 999 / 750 001 ml** pour 250 000 / 750 000 ; similitude 4 999 et 2 499 ml/s pour 5 000 et
2 500 ; à mi-vitesse 3 m ne se montent plus. **Critère 5** : réseau d'avarie (mer, deux compartiments, vanne, deux
pompes, commandes changeantes, 6 000 pas) : masse exacte à chaque pas, bornes, deux exécutions au bit
(`0xc0ee093528e34036`). Empreinte à commande pleine inchangée. Suite du cœur : 534 + 20 + 2 + 1 réussis, 14 ignorés.

**P5 — fait.** WVST version 2 : troisième liste (commandes ≠ auteur), comptée aux octets 76..80 ; commande d'auteur dans
l'empreinte de la base ; configuration comparée sans la commande ; v1 refusée (`Version`). L'essai d'en-tête existant
attend désormais `WVST`. **Critère 6** : vanne à 300 et pompe à pleine vitesse capturées après 300 pas, restaurées
dans une destination sale : suite de 1 000 pas **identique au bit** (`0x71b3c56c74d1912b`) ; témoin d'omission
(commandes d'auteur) : diverge. Refus : commande égale à l'auteur, 1 001, −1, en-tête v1 ; sans écart, 88 octets comme
en v1. Suite du cœur : 536 + 20 + 2 + 1, 14 ignorés ; espace de travail entier : 0 avertissement.
