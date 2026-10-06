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

Session : S505 — **en cours**. En autonomie, **6.4, C23 sur le système** : C23 (CAS-CANONIQUES, ADR-035) — le pas borné par la vitesse
**gouvernante**, celle du fluide relative à la paroi sur une face coupée, plus la célérité — n'a été éprouvé que sur le véhicule 1D (S28).
Le δ 3D ne borne pas son pas : l'hôte le choisit, le cœur ne garde que `dt²·g/dx ≤ 1`, et une coque peut franchir plus d'une maille par pas.

**Ce que la session fait.** (a) Mesurer d'abord : la coque de la porte D en translation à 5 m/s dans la référence, des pas qui lui font
franchir 0,25 à 4 mailles par pas, contre un calcul au pas fin ; (b) une borne en amont dans le cœur (`Volume3::courant_bound`) et le
compteur du Courant réalisé, d'une même vitesse gouvernante (ADR-035 §3) ; (c) C23 rejoué : eau au repos, coque menée de 0,5 à 20 m/s.

**Ordre de grandeur, écrit avant.** `c = √(g·h)` = 4,43 m/s pour 2 m d'eau ; à `ν` = 0,45, la borne absolue (`c` seule) laisse la paroi
franchir `u_p·dt/dx` > 1 dès `u_p > c·(1/ν − 1)` = 5,4 m/s — une coque de jeu (bateau à 10 m/s) y est. Le terme concurrent de l'erreur :
l'élévation que la coque produit (centimètres) ; une paroi qui saute une maille entière ouvre et ferme des faces sans l'état intermédiaire
— prévision : l'écart au calcul fin croît au-delà d'une maille par pas.

**Critères, écrits avant.** (1) la mesure publiée : l'écart au calcul fin selon les mailles franchies par pas, et s'il rompt au-delà de 1 ;
(2) borne et compteur d'une même vitesse gouvernante ; sous la borne gouvernante, le Courant réalisé vaut `ν` à toutes les vitesses de
paroi (au millième) ; sous la borne absolue, il dépasse 1 au-delà du seuil analytique ; (3) sous la borne gouvernante, l'écart au calcul
fin reste du même ordre à toutes les vitesses.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la mesure (a).
- [ ] **P3** — la borne et le compteur ; C23 rejoué (b), (c).
- [ ] **P4** — preuve ; liste 6.4 ; rituel.

### Notes de reprise
- **P2** — `c23_coque 5` (calcul détaché) : élévation de référence 0,77 m (départ impulsif à 5 m/s) ; écart au calcul fin (k = 0,125) :
  k = 0,25 → 4,7 %, 0,5 → 7,4 %, **1 → 100 %**, 2 → 152 %, 3 → 194 %. La rupture est à une maille franchie par pas. Conséquence : la
  vitesse gouvernante d'ADR-035 (fluide relatif à la paroi) ne suffit pas sur une grille coupée — le fluide suit la coque ; il faut aussi la
  vitesse de la paroi sur la grille (son déplacement par pas). Une précision d'ADR-035 : ADR-229.
