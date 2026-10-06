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

Session : S534 — **en cours**. En autonomie, **2.6 C1 — le champ de courant 2D régional** (ADR-011 §1 : une grille précalculée hors ligne,
en lecture seule, « une lecture de texture » ; embouchures, détroits, littoral, courants d'auteur). C0 et C2 existent depuis S513.

**Ce que la session fait.** `current_field::CurrentField` — une grille de vitesses de surface (f32), origine et pas, échantillonnée
bilinéairement (bornée à la grille), son gradient par maille et l'accélération advective `(u·∇)u`. `RegionalCurrentWater` enveloppe une
requête (B, B + W, ou `CurrentWater`) : la vitesse augmentée du champ (au profil C2 de décroissance donnée), **la pente que le champ
implique** — `g∇η = −(u·∇)u`, l'équilibre d'un courant permanent : c'est elle qui fournit au corps la force centripète, par la poussée du
proxy — et l'accélération `(u·∇)u` (la masse ajoutée). La hauteur n'est pas relevée (5 cm sur 10 m dans l'essai : sans effet sur un corps
noyé). C1 n'advecte pas les vagues (la réfraction par le courant n'est pas portée) ; sans champ, rien ne change.

**Ordre de grandeur, calculé.** Rotation solide Ω = 0,1 rad/s à 10 m : 1 m/s, accélération centripète 0,1 m/s², pente 0,0102, période
62,8 s ; le pas symplectique (`Ω·dt` = 10⁻³) fait osciller le rayon de ± 0,025 % sur deux tours, sans dérive.

**Critères, écrits avant.** (1) Un champ uniforme rend la requête de C0 (vitesse) à 10⁻¹² ; un champ linéaire est échantillonné et dérivé
exactement (10⁻¹²). (2) **Un corps neutre** (une sphère d'un point, masse ajoutée ½ρV, noyé à 2 m) lâché à la vitesse de l'eau dans une
rotation solide (une grille de 41 × 41 au pas de 1 m) : son rayon à 0,5 % près sur deux tours ; sa position après une période à 1 % du
rayon de son départ. (3) Le témoin : sans la pente du champ, le même corps s'écarte de plus de 10 % en deux tours (la force qui le tient
est bien celle-là).

### Plan

- [ ] **P1** — jeton, plan seul.
- [ ] **P2** — le champ, la requête, les essais ; (1)–(3).
- [ ] **P3** — preuve ; liste 2.6 ; rituel.

### Notes de reprise
