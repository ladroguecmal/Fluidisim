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

Session : S262 — en cours
Agent : Claude Opus 5, Claude Code ; fichiers, git, cargo, Python et GPU local disponibles.
Entrée (2026-09-17 07:44) : « Répare d'abord les défauts restants avant de peaufiner le visuel. »
Master propre 1e89da7, jeton libre, maillons 0. Le verdict R5 attend ; les finitions (vent de la
scène, transition vers la BRDF) passent après.

Défauts restants, publiés en S260–S261, dans l'ordre de traitement :

1. **Ligne d'horizon** : la grille projetée s'arrête à 1 500 m, ce que l'air clair révèle. Critère :
   sous `--ciel-clair`, grille prolongée jusqu'à l'horizon géométrique `√(2·R·h)`, R = 6 371 km,
   raccord à la couleur d'horizon ; sous la brume, grille et projection CPU **au bit** (1 500 m).
2. **Coût** : GPU eau 2,24–2,26 ms sous `--vagues --modulation --ciel-clair` en 1280×720, pour 2 ms
   (ADR-125). Critère : décomposer (cuisson du sillage, sommets, fragments, ciel) ; supprimer le
   travail dupliqué **sans changer l'image au-delà de l'arrondi** ; accord CPU/GPU conservé ; coût
   publié avec les techniques présentes et absentes (ADR-131). Si 2 ms ne sont pas atteints, le dire.
3. **A288, écart au jeu** : la surface rendue sous CWM s'écarte jusqu'à 0,365 m de la requête de
   jeu. Critère : requête eulérienne CWM dans le cœur (inversion de `x = α + D(α)`), dont l'élévation
   égale celle de la surface rendue à 1 mm près, avec convergence bornée et refus explicite.

### Plan

- [>] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — ligne d'horizon : distance lointaine en uniforme et dans `lod::Projection`, raccord ;
  brume au bit (R2–R4, `--spectral-verify`), rendu clair vérifié.
- [x] **P3** — coût : décomposition mesurée, fusion des boucles de bande par sommet, invariants de
  la queue précalculés ; accord CPU/GPU ; coût.
- [x] **P4** — ADR et protocole de la requête eulérienne CWM (A288).
- [ ] **P5** — cœur : requête, convergence, refus ; essais.
- [ ] **P6** — hôte : requête contre surface rendue ; écart publié.
- [ ] **P7** — rituel §6.

### Notes de reprise

P2 : `Projection::far`, `FrameData::far_distance` (brume 1 500 ; ciel clair √(2·6 371 km·h), 9,44 km à 7 m),
uniforme `impact.y`, raccord à la couleur d'horizon sur le dernier tiers. R2, R3, R4 et `--spectral-verify`
identiques au bit ; afficheur 16/1/0. Rendu clair : tirets de fin de grille disparus. **Incident** : le rendu
de contrôle `--revue=r5` a réécrit `captures/s261` (images R5 de S261, non versionnées ; reproductibles au
commit 1e89da7, empreintes publiées) ; nouveaux rendus sous `--revue=r6` → `captures/s262`.

P3 (coût) : décomposition 1280×720 référence avant : total 2,284, cuisson 1,272, queue par pixel ≈ 0,49, CWM
sommets + queue f⁻⁴ ≈ 0,33, nuages ≈ 0,05 ms. Faits : (a) bande de chaque mode de sillage précalculée par l'hôte
(seconde moitié du tampon, `spectral::band`, même `sqrt`) ; (b) `k` et direction unitaire de la queue
précalculés ; (c) sommet CWM à une seule boucle de bande (`band_cwm` rend hauteur et pente, `perturbations`) ;
(d) total de la grille = somme des 8 bandes ; (e) nuages à 2 octaves dans les reflets (habillage). Après :
1280×720 1,985–2,005 (référence), 1,978–1,988 (rasante) ; 960×540 1,54–1,60 ms ; cuisson 1,06–1,10.
Images : R2 0–3 octets sur 2,76 M (±1) après (a)–(c) ; scène complète 9–27 octets (≤ 8 niveaux, reflets) ;
arrondi. `--spectral-verify` : chemin grille à la 7e décimale, reste au bit ; `--tail-verify`, `--cwm-verify`
identiques ; `--multi --verify` 46 contrôles, max η 0,368 mm, LOD intérieur ≤ 0,39 mm (borne 2,5–3), coutures 4 µm.
