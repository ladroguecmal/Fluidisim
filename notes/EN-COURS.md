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

Session : S331 — **en cours**. **Lot 4 : le corps rigide sur B + W**, jugé sur C10 ; chemin de la v1
([ADR-189](../docs/adr/ADR-189-la-v1-d-abord.md)).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« continue, jusqu'à la v1 »* ; suite déclarée par S330.

**Ce que la session doit rendre possible.** Un corps que **le jeu** fait flotter — I-04 : toute force de
jeu vient de B + W, jamais de δ (ADR-008). `body.rs` n'a qu'un modèle statique. Il faut un corps rigide à
six degrés de liberté — quaternion, inertie principale —, une poussée par **proxy de points volumiques**
(ADR-008 §2 : immersion saturée sur l'épaisseur de chaque point, exacte pour une ligne d'eau plane), une
masse ajoutée, un intégrateur symplectique déterministe, et l'eau reçue par une interface de requête —
calme pour C10, B + W pour la porte D. Consommateur : le bateau de la porte D, que δ verra par `set_solid`.

**Un fait à écrire avant la mesure.** Le cube de C10 à 500 kg/m³ en mer a une hauteur métacentrique
`GM = d/2 + a²/(12d) − a/2 = −4,3 cm` : **il est instable en roulis** — un cube de cette densité flotte
incliné. L'essai de pilonnement reste bref (l'arrondi y croît en `e^{3,2 t}`) ; la rotation s'éprouve sur
un pavé plat, stable.

Critères, écrits avant le code (eau de mer, ρ = 1025 kg/m³) :
1. **Proxy** : volume immergé du cube droit = `A·d` exact à 10⁻¹², pour tout tirant.
2. **Tirant** (C10) : moyenne du pilonnement sur des périodes entières = `(ρ_c/ρ)·H` = 0,24390 m **± 1 %**.
3. **Période sans masse ajoutée** (C10) : `2π√(ρ_c·H/(ρ·g))` = 0,9907 s **± 5 %**.
4. **Masse ajoutée** (C10) : disque équivalent, `m_a = (8/3)·ρ·(A/π)^{3/2}` ; rapport des périodes
   **1,414 ± 15 %**, et à 1 % de `√(1 + m_a/m)`.
5. **Roulis** : pavé 1 × 1 × 0,3 m à 500 kg/m³, lâché à 5° — période à **± 5 %** de `2π√(I/(m·g·GM))`.
6. **Déterminisme** : deux trajectoires identiques au bit ; aucune allocation dans le pas.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `rigid_body.rs` : corps, proxy, requête d'eau, intégrateur ; critères 1 et 6.
- [x] **P3** — C10 : tirant, période, masse ajoutée (critères 2 à 4).
- [x] **P4** — roulis du pavé (critère 5).
- [ ] **P5** — preuve : `docs/validation/CORPS-RIGIDE-S331.md`, avec « Reproduire » ; C10 exécuté,
  file, liste 6.1.
- [ ] **P6** — rituel.

### Notes de reprise
**P2 + P3 (23:26), fusion déclarée** : les essais de C10 sont trois fonctions du fichier d'essais du
module. `rigid_body.rs` : `RigidBody` (six degrés de liberté, quaternion, masse ajoutée diagonale),
`ProxyPoint`, `WaterQuery` / `CalmWater`, `forces`, `step` symplectique. **Proxy exact** : `A·d` à 10⁻¹².
**C10** : tirant **0,243958** m pour 0,243902 (0,02 %) ; période **0,990724** s pour 0,990726 (2·10⁻⁶) ;
disque équivalent `m_a` = 61,359 kg, rapport **1,40775** — modèle 1,40774, C10 1,414 ± 15 %. Deux
trajectoires à six degrés de liberté, traînée comprise, **identiques au bit**. Cœur : 486 réussis.
**P4 (23:27).** Pavé 1 × 1 × 0,3 m à 500 kg/m³, proxy 16 × 16 × 8, lâché à son tirant incliné de 5° :
roulis **0,86178 s** pour 0,86142 (0,04 %, GM 0,4926 m) ; amplitude gardée à 5,000° sans traînée.
