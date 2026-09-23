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

Session : S332 — **en cours**. **Lot 4 : le corps dans δ** ; chemin de la v1
([ADR-189](../docs/adr/ADR-189-la-v1-d-abord.md)).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« continue, jusqu'à la v1 »* ; suite déclarée par S331.

**Ce que la session doit rendre possible.** Le corps du jeu (S331) **déplace sa paroi dans δ** : à chaque pas,
sa pose donne la distance signée de sa coque aux nœuds, sa vitesse de corps rigide — translation **et
rotation** — donne celle de la paroi ; la coque **perce le couvercle** du mode linéaire, dont les faces
qu'elle couvre se ferment. δ rend au corps une force, qui ne nourrit qu'un **décalage visuel borné**
(ADR-008 §1 : ≤ 8 cm) — jamais la trajectoire de jeu (I-04). Consommateur : le bateau de la porte D.

Critères, écrits avant le code :
1. **Rotation** : une sphère qui tourne sur elle-même ne pousse pas l'eau — vitesse maximale sous 1 % de
   `Ω·R` après un pas ; un pavé qui tourne en pousse. Translation seule : le chemin de S330 **au bit**
   (les valeurs de `C_m` de §9 se retrouvent).
2. **Coque qui perce** : le pas linéaire l'accepte ; un lac au repos autour d'elle reste au repos au bit.
3. **Volume** : cube de C10 qui pilonne en perçant le couvercle — `Σ(η − reste − z₀)·dx²` suit le volume
   immergé du solide dans le domaine à 10⁻⁹ m³ près.
4. **Masse ajoutée du cube flottant**, départ impulsif en pilonnement, trois mailles : convergente ; publiée
   contre le disque de même aire en fluide illimité, `(8/3)ρR³` (la référence de C10), et sa moitié
   `(4/3)ρR³` (un corps qui flotte, surface à pression nulle).
5. **I-04** : la trajectoire du corps avec δ attaché, **identique au bit** à celle sans δ ; le décalage
   visuel reste sous 8 cm et n'est pas nul.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — mouvement rigide de la paroi ; critère 1.
- [x] **P3** — la coque perce le couvercle ; critères 2 et 3.
- [x] **P4** — banc : masse ajoutée du cube flottant, trois mailles (critère 4).
- [x] **P5** — le corps du jeu pilote sa paroi, décalage visuel borné ; critère 5.
- [ ] **P6** — preuve : section datée de [CORPS-RIGIDE-S331](../docs/validation/CORPS-RIGIDE-S331.md), avec
  « Reproduire » ; file, liste.
- [ ] **P7** — rituel.

### Notes de reprise
**P2 (23:35).** `set_solid_rigid(nœuds, V, Ω, c)` : la part couverte de chaque face avance à `V + Ω × (x − c)`
prise au centre de la face ; faces qui naissent à cette vitesse ; `set_solid` = sans rotation, au bit —
les `C_m` de S330 se retrouvent à l'identique. **Critère 1, verdict partagé** : sphère qui tourne sur
elle-même, vitesse maximale **1,08 %** de `Ω·R` à 6 mailles par rayon — *manqué* —, **0,73 %** à 12 —
*tenu* ; décroissance d'ordre **0,56** seulement. Cause lue : la vitesse de paroi au centre de la face,
non au centroïde de sa part couverte — une petite part couverte au coin d'une face s'écarte de `Ω·dx/2`,
et une petite maille l'amplifie. Remède nommé, non fait : le centroïde de la part couverte. Pavé qui
tourne : 0,42 m/s, il pousse l'eau.
**P3 (23:38).** `configure_with_floating_solid` : le solide peut occuper la couche du couvercle ; le pas
linéaire ne refuse plus qu'un **fond** qui l'atteindrait. L'opérateur pondérait déjà la condition du
couvercle par l'ouverture : une face couverte devient paroi, une face en partie couverte garde la
condition sur sa part libre ; la part couverte avance à la vitesse de la coque. Cube de C10 à son tirant,
24³ : **lac au repos au bit**, 100 pas ; pilonnement imposé 2 cm à 2 Hz, 100 pas : la surface suit le
volume de coque plongé à **2,7·10⁻¹⁰ m³**. Sans l'autorisation, la configuration refuse. Cœur : 490.
**P4 (23:39).** `delta3d_fond_coupe --masse-ajoutee-flottant` : cube de C10 à son tirant, 2,4 × 2,4 m sur 1,2 m
d'eau, départ impulsif en pilonnement ; 5 s. Masse ajoutée **33,20 / 41,26 / 41,66 kg**, limite extrapolée
**41,69 kg** — convergente, **critère 4 tenu**. Contre les deux disques : `(8/3)ρR³` = 61,36 kg, la référence
de C10, la **surestime de 47 %** ; `(4/3)ρR³` = 30,68 kg la sous-estime. 41,7 kg = 0,67 de la masse du cube :
le rapport des périodes deviendrait √(1 + 41,7/62,5) = **1,291**, encore dans la tolérance de C10 (1,414 ±
15 %). δ peut donner au jeu son coefficient de masse ajoutée, mesuré hors ligne — un paramètre, pas une force
de δ au pas : I-04 tient.
**P5 (23:42).** `rigid_body.rs` : `oriented_box_distance` — la coque d'un corps aux nœuds de δ — et
`RenderOffset`, le ressort borné d'ADR-008 §1 (8 cm ; la part en rotation, ≤ 3°, manque). Cube de C10
lâché la base à la surface, 300 pas : sa coque pilotée dans δ par `set_solid_rigid`, la force de δ sur la
coque n'anime que le ressort. **Trajectoire de jeu identique au bit** à celle du corps seul ; décalage
maximal **6,7 cm** ; coque descendue à 0,488 m. Critère 5 tenu. Cœur : 491 réussis.
