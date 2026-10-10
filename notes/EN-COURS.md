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

Session : S762 — **terminée**. En autonomie ; session longue. ADR-289 D3.2, le banc de la 3D corrigée (ADR-294) : **le corps qui flotte**.
**La question** : une sphère libre se pose-t-elle à son tirant d'eau exact, celui d'Archimède, et l'écart diminue-t-il avec la maille ?

**La référence** (exacte) : une sphère de rayon `R`, de densité relative `s`, s'enfonce d'une calotte `h` telle que
`(h/R)²·(3 − h/R) = 4s` : `h/R` = 0,6527 (s = 0,25), 1 (0,5), 1,3473 (0,75). Le tirant mesuré : le niveau de l'eau loin de la sphère
(lu par φ, `|x − x_c| > 0,3 m`) moins le bas de la sphère. Ainsi l'eau que la sphère déplace dans la cuve fermée ne fausse rien.

**Le montage** : une cuve de 1,2 × 0,4 m, 0,4 m d'eau ; la sphère de `R` = 0,1 m au centre, posée 2 cm au-dessus de son équilibre, libre
(`set_body_mass`, la masse ajoutée implicite de S655) ; 5 s ; la moyenne et la vitesse maximale sur la dernière seconde.

**Les essais** : (A) la 3D d'ADR-294 à 2,5 cm, s = 0,25, 0,5, 0,75 ; (B) la 3D sans projection, s = 0,5, 2,5 cm ; (C) la 3D d'ADR-294 à
5 cm ; (D) à 1,25 cm, s = 0,5.

**Les critères, écrits avant** (ADR-293 D3 : la lecture du niveau moyennée sur des dizaines de colonnes, son quantum est sous le mm) :
1. le tirant d'eau à **1 cm** (5 % du diamètre) de l'exact, aux trois densités, à 2,5 cm ;
2. l'écart diminue quand la maille diminue (C → A → D, s = 0,5) ;
3. la sphère posée : la vitesse verticale sous 1 cm/s sur la dernière seconde ;
4. la masse de l'eau exacte (le nombre de particules constant).

**Contrôles du plan** (ADR-276, ADR-287, ADR-290, ADR-293, ADR-295)

- **témoin** : la flottaison de S653–S655 (5 cm, s = 0,5 : le centre 1,3 cm sous le niveau, niveau nominal) ; Archimède.
- **instrument** : le niveau par φ loin de la sphère (`volume_surface_s708`), le centre du corps (`body()`).
- **calcul** : (A) et (B) 4 × 5 min ; (C) 2 min ; (D) 40 min.
- **ADR** : ADR-289 D3.2 ; ADR-287 D1 (un essai canonique) ; ADR-293 D3 ; ADR-295 D1 (sans objet : aucune règle nouvelle).
- **pièges** :
  - la projection de densité près du corps : sa densité nominale ne compte que le fond, pas la sphère ; les mailles voisines du corps
    peuvent paraître creuses et attirer les particules. (B) le départage ;
  - le niveau monte quand la sphère s'enfonce (la cuve est fermée) : le tirant se lit contre le niveau mesuré, jamais contre le nominal ;
  - la sphère dérive vers une paroi : sa place horizontale est rapportée.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'essai ; (A), (B), (C), (D).
- [x] **P3** — preuve ; fermeture.

### Notes de reprise
- **P2 fini en partie** (928 s ; D arrêté, la session close à la demande de l'utilisateur, qui continue sur un autre compte) :
  - (A) 2,5 cm : le tirant +0,3 / −4,3 / −2,0 mm (s = 0,25 / 0,5 / 0,75) ; la vitesse 1,2 à 4,6 cm/s ;
  - (B) sans projection : −4,2 mm ; 2,5 cm/s ;
  - (C) 5 cm : +11,5 mm ;
  - (D) 1,25 cm : **non mesuré**.

  (1) tenu ; (2) en partie ; (3) manqué, mal posé (la cuve fermée, le schéma qui conserve l'énergie) ; (4) tenu.
