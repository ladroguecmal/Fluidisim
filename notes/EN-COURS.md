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

Session : S450 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite proposée : **la surface continue**
([ADR-211](../docs/adr/ADR-211-les-trucages-retenus.md) D2 : l'utilisateur juge C10 sur une surface sans interstice, dont seuls les
jets se détachent).

**Ce que la session trouve en entrant.** La carte de la bande (`apic3d_carte`) n'a pas de rendu : elle ne sert qu'aux calculs et à
leurs bancs. Mais le champ `φ` d'`Apic3` est **déjà le champ unique** : `z − η` dans la zone des colonnes, la reconstruction des
particules dans la bande, `z − fond` sous le fond (`columns_label`). Une isosurface de ce champ seul est continue par construction —
l'interstice viendrait d'un rendu qui dessinerait colonnes et bande séparément.

**Ce que la session fait — une première image, hors ligne** (le rendu en direct sur la carte viendra après le verdict). Le banc
`surface_continue` (cœur, sans carte) : B10 en quart (une sphère entre dans l'eau, `Fr` = 2, `D/dx` = 8), la zone des colonnes et la
bascule (S408) ; aux instants choisis, l'isosurface `φ = 0` par **tétraèdres marchants** (sans table d'ambiguïté), le quart reflété en
entier ; un rendu logiciel (perspective, tampon de profondeur, ombrage de Lambert et reflet du ciel), la sphère en gris ; un PPM par
instant (`captures/s450/`, ADR-124), son aperçu PNG par `outils/apercu_ppm.py`.

**Critères, écrits avant.** (1) le maillage n'a **aucune arête ouverte à l'intérieur** du domaine échantillonné ; (2) **au raccord**
bande | colonnes, la hauteur de la surface lue sur `φ` de part et d'autre d'une face de frontière ne saute pas de plus d'**un quart de
maille**, sur tout le calcul ; (3) quatre images, montrées à l'utilisateur, qui juge la continuité (revue) ; (4) le banc sous deux
minutes.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — le banc `surface_continue` : isosurface, rendu, PPM ; mesures (1), (2), (4).
- [ ] **P3** — les images à l'utilisateur ; preuve ; rituel (allégé).

### Notes de reprise
