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

Session : S484 — **en cours**. En autonomie, **K2-3 — les grosses bulles libres** ([conception](../docs/registres/CAMPAGNE-K2-S478.md)) :
la remontée d'une bulle d'air résolue, dans la référence APIC 3D avec poches (S479, S481 ; `FILS=16`, S483).

**Ce que la session fait.** Un exemple `apic3d_remontee` : un **quart de cuve** (la bulle centrée sur le coin ; les parois d'APIC
reflètent — plans de symétrie : une cuve deux fois plus large pour un quart des mailles), une bulle de rayon `R` lâchée près du fond ;
le centre de la poche suivi à chaque pas ; la vitesse terminale par une droite sur la partie établie de la remontée. La référence publiée :
**Davies et Taylor (1950)**, calotte sphérique, `U = 0,711·√(g·d_e)` (Clift, Grace et Weber 1978), valable pour `Eo = ρ·g·d_e²/σ > 40`
(ici ≈ 900 : la tension de surface, absente du modèle, n'y joue pas) ; la correction de paroi de Collins (1967), négligeable sous
`d_e/D = 0,125`. Deux résolutions (`R/dx` = 4 et 6) pour la convergence.

**Entrées, et comment elles se vérifient.** La cuve, la bulle et la profondeur sont imprimées par l'exemple ; le volume de la poche au
départ contre celui de la sphère (un quart) ; la masse exacte à chaque pas (le nombre de particules).

**Critères, écrits avant.** (1) la vitesse terminale à **15 %** de Davies–Taylor à `R/dx` = 6 ; (2) l'écart entre `R/dx` = 4 et 6 dit (la
convergence), sans seuil ; (3) la bulle remonte selon `−g` (la dérive latérale du centre sous 0,1 R) ; masse exacte. Si (1) ne tient
pas, l'écart est attribué (la résolution, le volume suivi, la paroi), pas maquillé.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `apic3d_remontee` ; un essai court.
- [>] **P3** — les deux résolutions (par `calcul.py`) ; (1) à (3).
- [ ] **P4** — preuve ; liste 7.4 ; rituel.

### Notes de reprise
- **P2** — `code/water-core/examples/apic3d_remontee.rs` : quart de cuve (8 R de demi-largeur, 22 R d'eau), bulle au coin à 2,5 R du
  fond, pas stable ≤ 5 ms, la plus grande poche suivie ; la droite sur `z ≥ z0 + 2R`. Essai court (R/dx = 3, 13 pas) : la poche suivie,
  masse exacte ; le centre d'un quart de bulle est à 3R/8 des axes (la dérive se lit par rapport au départ). R/dx = 4 lancé à 20:34.
- **P3a** — R/dx = 4 au pas stable (≈ 3,5 à 5 ms) : la bulle cale puis s'effondre (« plus de poche » à 0,166 s). **Trouvé** : le centre
  d'une poche était divisé par le volume entier (mailles d'eau voisines comprises) — tiré vers l'origine d'un facteur ≈ 0,8 depuis S479
  (la « remontée » de POCHES-AIR-S479 aussi) ; corrigé dans la référence et la carte (la part d'air des seules mailles d'air). Au pas de
  1 ms : la bulle part de 10 cm et monte (15 cm à 0,12 s, ≈ 0,5 m/s). Le critère (3) de dérive latérale est mal posé (une calotte qui
  s'aplatit écarte le centre du quart sans dériver). R/dx = 4 à 1 ms lancé (1,5 s).
