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

Session : S623 — **en cours**. En autonomie (ADR-247 : la physique des partiels). **Le lot** (dû ; feuille de route S620–S622), puis
**4.11 — la restauration depuis une graine** (ADR-022 §3, I-17) : la moitié laissée par S609. ADR-013 §4 : un domaine substitutif né faux met
une minute à s'établir (60,65 s mesurées en S610) — d'où la graine cuite hors ligne.

**Ce que la session fait.** Un module `graine.rs` : `SeedState` (genre, paramètres de cuisson `hs`, `tp`, `θ`, phase de marée, liquide ; la
grille ; la hauteur totale et la vitesse en f16 ; l'identifiant = l'empreinte du contenu) ; `condense` — **réservé à un hôte de cuisson** :
compilé seulement sous `cfg(test)` ou la fonctionnalité `cuisson` (L19 : l'interdit inexprimable — un hôte de jeu n'a pas la fonction) ;
`restaurer(paramètres, tolérance, volume du nœud en ml)` — refusé au-delà de la tolérance de paramètres, la forme renormalisée sur la masse du
nœud V (autoritaire) ; `choisir` — la graine la plus proche dans la tolérance, jamais une interpolation de champs (I-09) ;
`Domaine1D::depuis_etat`. Ne fait pas : le 2D, la graine côtière de 12.3 branchée, la mesure de la tolérance (banc B4).

**Références, calculées avant** (`s623_ref.py`, numpy). Le domaine de S609/S610 né au repos sous B, avancé 120 s ; sa graine (2 + η et u en
f16) ; le nœud tient **399998810 ml** par mètre de largeur ; la graine en porte 400.0048828125 m³ ; restaurée et renormalisée :
**399.9988100000 m³**. Restauré avec B décalé de 120 s : écart initial à B **0.002478421581 m** (sous 5 mm) — **établi en 0.05 s**, un
pas, contre 60,65 s depuis le repos (S610).

**Quantum** : le ml (10⁻⁶ m³) ; le f16 (2·10⁻³ m à 2 m). **Critères, écrits avant.** (1) le volume restauré à moins d'1 ml de celui du nœud ;
(2) l'écart initial à B égal à la référence à 10⁻⁹ m, sous 5 mm, et l'établissement en un pas ; (3) la tolérance : `hs` à ±10 %, `tp` à
±0,5 s, `θ` à ±0,1 rad, la même phase — dedans, restauré ; dehors (chacun à son tour), refusé ; (4) `choisir` : `hs` = 0,21 m parmi des
graines à 0,1, 0,2, 0,4, 0,8 m → celle de 0,2 ; `hs` = 0,3 m → aucune ; (5) l'identifiant : le même contenu, la même empreinte ; un seul f16
changé, une autre ; (6) refus : un volume de nœud non positif, des tableaux de taille fausse.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [ ] **P2** — `graine.rs`, `depuis_etat` et leurs essais ; (1)–(6).
- [ ] **P3** — preuve ; liste 4.11 ; rituel (`--lot`).

### Notes de reprise
