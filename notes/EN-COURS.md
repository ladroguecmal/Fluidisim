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

Session : S216 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **A255** — le majorant du sillage pèse désormais 88 % du budget de pente et sa famille
n'a aucune loi. Comprendre d'où vient son pessimisme avant de chercher à le mesurer.

### Entrée

Ligne `Session suivante` de S215. Maillons 0 à l'amorce (W avancée dans `code/*/src` **et** ADR-133
actée). A255 est une ligne de la file active J1 et conditionne la mutualisation de J1-bis.

### État réel constaté à l'amorce

`master` et trois copies de travail propres au même commit `bd4030b`. Jeton `libre`, battement
13:09. Machine : AMD Ryzen AI 7 350, RTX 5070 Laptop (DX12). Aucune dépendance nouvelle prévue.

### Ce que la lecture du code change au sujet, avant toute mesure

La suite S216 telle que S215 l'a écrite disait : « refaire sur la famille du sillage la campagne qui
a traité l'impact ». **La lecture de `slope_envelope_tight` déplace la question.**

```rust
for s in self.slots { bound += |k_s| * |eta_s|; }
```

C'est une somme **scalaire** de contributions dont les vecteurs d'onde pointent dans des
**directions différentes** — le demi-spectre couvre un demi-disque. Or la pente est un **vecteur** :
sa norme est celle de la somme vectorielle, pas la somme des normes. Ce majorant ne peut donc être
atteint que si tous les modes s'alignent *en phase* **et** *en direction*, et la seconde condition
ne dépend ni du temps ni du point : **elle est fausse par construction de la recette**.

Il y a donc **deux pessimismes distincts**, et les confondre ferait chercher une loi mesurée là où
une inégalité exacte suffit :

1. **Directionnel — statique, prouvable, sans calibration.** Pour toute direction `θ`, la pente
   projetée vaut au plus `Σ |a_i| |k_i| |cos(θ − θ_i)|`. Le maximum de cette quantité sur `θ` est un
   majorant **exact** de la norme de la pente, plus serré que la somme scalaire dès que les
   directions sont étalées. Pour un étalement uniforme sur un demi-disque, le rapport vaut `π/2`
   ≈ 1,571. **Le pessimisme mesuré pendant le forçage vaut 1,39 à 1,90** : le même ordre.
2. **De phase — dynamique.** Ce qui reste une fois le directionnel retiré : la décohérence de
   L290, qui croît avec l'âge (4,77 à 39 s).

**Conséquence sur l'ordre des travaux.** Le premier ne demande **aucune mesure, aucune table,
aucune garde, aucun domaine de validité** : il est exact par construction, et les directions `θ_i`
vivent sur la grille angulaire **fixe** de la recette, donc le maximum se calcule exactement et non
par balayage. Le chercher en second serait une faute de méthode : on calibrerait une table sur un
écart dont une partie s'annule gratuitement.

### Thèse et critères, déclarés avant toute mesure

1. **Décomposer avant de mesurer une loi.** Trois grandeurs à chaque âge : somme scalaire
   (`slope_envelope_tight`, ce que le budget consomme aujourd'hui), **majorant directionnel**, et
   **pente réelle** échantillonnée finement. Deux rapports : `scalaire/directionnel` — la part
   statique — et `directionnel/réel` — ce qui reste à expliquer.
2. **Un majorant reste un majorant** : le directionnel n'est jamais dépassé par la pente réelle,
   à aucun âge ni aucun point. Un seul dépassement invalide le raisonnement, pas la mesure.
3. **La part statique est-elle une propriété de la recette ?** Si `scalaire/directionnel` ne dépend
   que de `angular` et du profil spectral — et non de l'âge, ni des tronçons, ni de σ —, alors elle
   se calcule, et rien n'a besoin d'être calibré.
4. **Ce qui reste a-t-il une échelle ?** Seulement ensuite, et en séparant **forçage** (le majorant
   croît pendant que la source émet) et **après extinction**.
5. Aucun seuil de réussite présumé ; publication avec techniques, domaine et — si un temps est
   mesuré — rang de passage (L289).

**Prédiction écrite pour être contredite** : la part statique vaut environ 1,5 et **explique
l'essentiel du pessimisme pendant le forçage** (mesuré 1,39 à 1,90) ; après extinction le résidu
`directionnel/réel` croît quand même, jusqu'à 3 environ à 39 s. Si elle tient, le remède est une
inégalité exacte et non une table. Si le résidu est déjà grand pendant le forçage, c'est que la
décohérence est immédiate et la part statique marginale — et le sujet redevient celui de S215.

### Plan

- [x] **P1** — jeton, entrée, relecture du majorant, thèse, critères et plan seuls.
- [x] **P2** — décomposer : somme scalaire, majorant directionnel (balayage fin hors ligne) et pente réelle, aux âges de la fixture ; les deux rapports.
- [ ] **P3** — *fusionné avec P4 après P2, un seul balayage les porte* : la part statique est-elle une propriété de la recette (`angular`, `radial`, `cutoff`, σ, tronçons) ? combien de directions distinctes le demi-spectre porte-t-il ? et le discriminant d'emprise — à âge fixé, agrandir l'emprise fait-il monter le maximum réel ?
- [x] ~~**P4**~~ — fusionné dans P3.
- [ ] **P5** — décider et construire ce que le verdict autorise : enveloppe directionnelle exacte dans le cœur si la part statique le mérite (maximum exact sur la grille angulaire, sans balayage), sinon dire pourquoi.
- [ ] **P6** — recevoir : jamais dépassée, plus serrée, coût de préparation ; budget recomposé sur la scène J1.
- [ ] **P7** — document de réception (en-tête ADR-131) ; suite complète `code/`.
- [ ] **P8** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide : le travail commence en P2)*

P2 : `code/water-core/examples/enveloppe_sillage_s216.rs`. Contrôle de lecture d'abord : la somme
scalaire reconstruite depuis `render_components` reproduit `slope_envelope()` à **1e-7** — ce que
l'exemple manipule est bien ce que le budget consomme.

| âge (s) | phase | scalaire | directionnel | réel | part statique | résidu | total |
|---:|---|---:|---:|---:|---:|---:|---:|
| 0,5 | forçage | 0,038037 | 0,024464 | 0,019385 | **1,5548** | 1,2620 | 1,9621 |
| 2 | forçage | 0,107494 | 0,077957 | 0,064453 | 1,3789 | 1,2095 | 1,6678 |
| 4 | forçage | 0,135072 | 0,107316 | 0,097430 | 1,2586 | **1,1015** | 1,3864 |
| 8 | forçage | 0,144921 | 0,116545 | 0,085966 | 1,2435 | 1,3557 | 1,6858 |
| 16 | forçage | 0,164995 | 0,135757 | 0,086934 | 1,2154 | 1,5616 | 1,8979 |
| 18 | après | 0,156797 | 0,131175 | 0,069110 | 1,1953 | 1,8981 | 2,2688 |
| 24 | après | 0,157044 | 0,131058 | 0,043181 | 1,1983 | 3,0351 | 3,6369 |
| 39 | après | 0,157981 | 0,131885 | 0,033152 | **1,1979** | **3,9782** | 4,7654 |

**Prédiction partiellement contredite.** J'avais écrit « part statique ≈ 1,5, et elle explique
l'essentiel du pessimisme pendant le forçage ». Elle vaut 1,55 à la naissance mais **retombe à
1,20 et s'y fixe** ; elle explique l'essentiel à 4 s (1,26 sur 1,39) et **moins de la moitié** à
16 s (1,22 sur 1,90). Le demi-spectre du sillage n'est donc **pas** étalé uniformément sur un
demi-disque — sinon le rapport vaudrait π/2 ≈ 1,571 : le sillage de Kelvin concentre son énergie
dans un cône, et c'est cette concentration que le 1,20 mesure.

**Deux enseignements qui séparent les deux remèdes.**
- La part statique est **stable après extinction** (1,195 à 1,198 de 18 à 39 s) : c'est une
  propriété de la **recette et de la trajectoire**, pas de l'âge. Un gain de 20 %, exact, sans
  calibration ni domaine de validité — à prendre si son calcul est bon marché.
- Le **résidu** porte tout le reste et croît sans borne visible : 1,10 à 4 s, 1,56 à 16 s,
  **3,98 à 39 s**. C'est la décohérence de L290, et elle demande une loi mesurée comme ADR-133 —
  donc sa propre campagne, pas une inégalité.
