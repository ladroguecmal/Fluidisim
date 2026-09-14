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

Session : S234 — en cours
Agent : Claude Code, Opus 5 (fichiers, git, cargo, Python et GPU local disponibles)
Entrée : « Reprends le projet », master propre à98430a1, trois copies propres au même commit
(avancées par S233 à 22:12), jeton libre.

**Objectif.** Premier LOD spatial **intégré à l'hôte GPU** : la densité du maillage d'eau suit
la charge que le contenu exige, et non plus seulement le pas de 2 px. Qualité entre sommets,
coutures entre niveaux et coût complet publiés face aux 2 ms (ADR-125, ADR-131 D3).
**Thèse, calculée avant construction (P1, script hors dépôt).** Pose S212 : 86,2 % des
129 600 sommets dans l'emprise du sillage ; 67 % espacés de moins de 0,25 m, soit plus de
8 échantillons par λ_min = 2,09 m. Couper les modes au-delà de Nyquist ne retire que ≈2 % du
travail (7,5 % de sommets sous-échantillonnés, en fond de scène) : c'est une correction de
repliement, pas un levier de coût. Le levier est la **densité près de la caméra**.
**Critère de qualité, à provenance.** Erreur d'interpolation linéaire bornée par
`h²/8 · Σ|a|·k²` (B, impact et sillage publiés), confrontée à la tolérance de 3 mm déjà reçue
aux sommets (`Gpu::verify`, S211). Pentes : erreur publiée contre la grille 2 px, sans seuil
inventé. Référence : cœur aux points **intérieurs** des triangles, pas seulement aux sommets.
**Arrêt.** LOD intégré, vérification sommets + intérieurs + coutures (aucune fissure), coût
fixe/balayé et cadence mesurés avec techniques présentes/absentes. Si la borne de P2 ne
permet aucune réduction de charge, ce constat est le résultat : ne pas assouplir la
tolérance pour obtenir un gain, et reporter le lot vers la technique suivante.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [ ] **P2** — lectures ciblées (ADR-129/131/132, HOTE-GPU-S212) ; calculer `Σ|a|k²` et
  `Σ|a|k³` réels des trois couches sur la fixture ; charge réductible par pose ; choisir la
  forme (rangées seules, ou bandes de colonnes cousues) et déclarer le protocole.
- [ ] **P3** — planificateur CPU du maillage (rangées et bandes), sans allocation par image,
  index cousus sans jonction en T ; tests unitaires de couverture et de couture.
- [ ] **P4** — shader et passe consomment le plan ; `--verify` inchangé aux sommets ; nouvelle
  vérification aux intérieurs et aux coutures contre le cœur, LOD contre grille 2 px.
- [ ] **P5** — coût : `BENCH`, cadence fixe et balayée, avec et sans LOD ; document de
  validation, en-tête ADR-131 D3.
- [ ] **P6** — rituel §6, file, feuille de route, jeton ; copies à synchroniser.

### Notes de reprise

Base : S233 413 tests réussis, 5 ignorés ; S225 passe eau 4,1585 ms fixe, 2,8479 balayée.
P1 : script `grid_geom.py` (bloc-notes de session) reproduit `ocean_vertex` sans hauteur.
Pose S201 : h>λ_min/2 sur 7,5 % des sommets de l'emprise, travail Nyquist/plein 0,978 ;
pose 30 m / tangage −0,5 : 60 % dans l'emprise, 0 % sous-échantillonné. Histogramme des pas
(0,25 m) : 75 463 sommets sous 0,25 m sur 111 715 dans l'emprise.
