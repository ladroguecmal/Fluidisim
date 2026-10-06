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

Session : S518 — **en cours**. En autonomie, **A329** (S517) : le recoupage d'une coque qui bouge coûte 25 ms par pas sur 786 000 mailles,
où 6.4 le disait sous 1 ms (dans un petit domaine). Le banc du sillage, instrumenté par étage (S518) : **recoupage du cœur 13,1 ms, envoi
à la carte 11,5 ms**, extraction 0,35 ms, l'hôte 0,07 ms (4 s, 400 pas).

**Ce que la session fait.** (a) Le cœur : les ouvertures d'avant le recoupage, aujourd'hui recopiées en entier dans les tampons de
sauvegarde (que le pas réemploie), deviennent des tableaux de la base, persistants, copiés dans la seule réunion de la boîte du recoupage
précédent et de la nouvelle (hors d'elle, avant = après) ; la vitesse des faces qui s'ouvrent, les colonnes solides, le transfert de S334 et
ses poids, `changed_faces` limités à la boîte ; la vérification de finitude et la boîte du solide en une seule passe. (b) La carte :
`set_motion_parts` prend les tableaux du cœur sans les concaténer, et ne compare à l'ombre que dans la réunion des deux dernières boîtes —
hors d'elle, rien n'a pu changer (le dépôt, le terme de paroi, le transfert et les faces ne sont non nuls que dans la boîte de leur pas) ; le
premier appel compare tout.

**Ordre de grandeur, calculé.** Ce qui reste entier : la passe sur les 836 000 nœuds (finitude et boîte), le remplissage à zéro du terme de
paroi (786 000 mailles) et des poids (245 000), les boucles de colonnes (49 000) — ≈ 1,9 M valeurs lues ou écrites une fois, contre
≈ 20 M aujourd'hui (trois copies et une boucle de 2,4 M faces, 786 000 mailles en colonnes, la concaténation et la comparaison de 6,7 M
valeurs). À ~1 ns la valeur : ≈ 2 ms. La boîte de la coque : ≈ 28 × 14 × 9 mailles — négligeable.

**Critères, écrits avant.** (1) **Au bit** : le recoupage en boîte rend les tableaux et la surface du recoupage entier (l'essai S508, plus
les vitesses des faces) ; la carte, avec l'envoi en boîte, rend la surface du banc du sillage au bit de l'envoi entier (empreinte des bits
de η). (2) **Le coût** : recoupage + extraction + envoi ≤ 3 ms par pas sur le banc du sillage (786 000 mailles), contre 24,9. (3) La suite
du cœur, les essais de la carte (`--lineaire-mobile`, `--lineaire-coque`) inchangés. A329 levée si (1) et (2) tiennent.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le cœur en boîte ; (1) côté cœur, (3).
- [ ] **P3** — la carte en boîte ; (1) côté carte, (2).
- [ ] **P4** — preuve ; A329 ; liste 6.4 ; rituel.

### Notes de reprise
- **P2 fini** — les ouvertures d'avant dans la base (`before_u/v/w`, +1 jeu de faces déclaré à l'hôte), copiées dans la réunion des deux
  boîtes ; faces qui s'ouvrent, colonnes solides, transfert S334, `changed_faces`, poids du transfert en boîte ; finitude + boîte en une
  passe (`solid_box_checked`). L'essai S508 étendu aux vitesses des faces : **au bit** sur 60 pas. Suite du cœur : 681 + 23, verte.
  Banc du sillage (4 s) : **recoupage du cœur 13,1 → 1,71 ms** ; l'envoi reste à 11,2 ms (P3).
