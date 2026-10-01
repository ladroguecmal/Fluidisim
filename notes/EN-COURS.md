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

Session : S430 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : **C7d-1**
([preuve](../docs/validation/APIC-CARTE-S416.md) §21.1 : la vitesse propre de δ ne demande rien sous une houle calme, mais prend 0,94
de la fenêtre sous la houle raide — grande sans déformation).

**Ce que la session fait.** (1) **La déformation propre de δ** comme critère du fond : `floor_deformation` (s⁻¹), la norme de
Frobenius du gradient de la vitesse **relative à B** (`u − U_B` ramenée aux centres des mailles, puis les différences centrées de
`vorticity` — le gradient discret de B s'en retranche, non seulement l'analytique) ; sans fond B, le gradient de la vitesse totale.
Éteint par défaut (S429 au bit). Un essai : une houle posée sur la grille ne déforme rien relativement à elle-même ; un cisaillement
enfoui ajouté est pris. (2) **Mesurer** sur la vague de Chen, avec les critères de S429 — et, pour situer, la norme du gradient de B
lui-même : `√2·kaω`, ≈ 5 s⁻¹ en surface à `ε` = 0,55, ≈ 0,8 s⁻¹ à 0,1.

**Critères, écrits avant** (vague de Chen, 40 mailles par λ, `ny` 4, maintien 0,3 s, fond 4, `fond_b=1`, **déformation 1 et 2
s⁻¹**, sans seuil de vitesse) : (a) `ε` = 0,55 — part de la fenêtre en particules au retournement **sous 0,5**, retournement à l'instant
de la forme seule (à un pas) ; (b) `ε` = 0,1 — **rien dans la fenêtre** après le premier pas, sur 2,5 τ ; (c) sans la clé, S429 au
chiffre près ; (d) aucun retour rapide de plus que la forme seule ; (e) suite du cœur, zéro avertissement. Si (a) manque encore, la
session publie où la déformation propre se loge (au retournement : les colonnes prises, leur distance à la crête) — sans changer de
critère en cours de mesure.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `floor_deformation` relative à B ; essai du cœur.
- [ ] **P3** — mesures sur la vague de Chen ; critères.
- [ ] **P4** — suite ; preuve §21.2 ; registres.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2** — `Apic3::deformation` (la norme de Frobenius du gradient de `u − U_B` aux centres, mêmes différences que `vorticity` ; sans B, de la vitesse totale) et `ColumnsSwitch::floor_deformation` (s⁻¹), éteint par défaut ; la clé `deformation` du banc. Essai `the_own_deformation_of_delta_is_read_relative_to_b_s430` : une houle de gradient 2,9 s⁻¹ en surface, seuil 1 — le gradient total prend les 32 colonnes, relatif à B aucune ; un cisaillement enfoui ajouté, pris lui seul (10 à 21).
