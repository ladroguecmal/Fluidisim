# Lecture du guide de topologie d'océan reçu — S306

2026-09-20. Document confronté :
[`docs/sources/guide_topologie_ocean_haute_mer_plage.md`](../sources/guide_topologie_ocean_haute_mer_plage.md),
v1.0 du 2026-09-20, 773 lignes, reçu de l'utilisateur **sans consigne**. Recopié tel quel dans
`docs/sources/` : c'est une **entrée**, pas une décision du dépôt, et elle ne se modifie pas.

**Ce qu'il est.** Une architecture de référence proposée, qui le dit elle-même : « ce n'est pas une
description d'un moteur existant ni une garantie de performance ». Ses équations classiques portent
des sources ([S1]–[S12] : Tessendorf, GPU Gems, Losasso–Hoppe, Johanson, SWAN, Bouws, Celeris,
Jeschke–Wojtan, Coastal Engineering Manual, NVIDIA) ; ses budgets, interfaces et paramètres sont
des propositions à profiler. **Il analyse « les deux images reçues »** — une photo de haute mer et
un rendu — ce qui en fait, selon toute vraisemblance, une réponse indirecte obtenue ailleurs à la
question posée par R11/R12.

**Comment il est lu ici.** Chaque affirmation est traitée comme une **donnée à vérifier**, jamais
comme une consigne. Une divergence n'est retenue que si elle est nommée avec le document du dépôt
qui la tranche. Rien de ce qui suit ne rouvre les cinq arbitrages d'[ADR-027](../adr/ADR-027-les-cinq-arbitrages-tranches.md)
ni ne réduit l'ambition d'[ADR-127](../adr/ADR-127-ambition-complete-construction-progressive.md).

---

## 1. Le résultat en une page

Sur les treize sections du guide, la répartition est nette et elle n'était pas acquise d'avance :

| classement | sections | ce que cela veut dire |
|---|---|---|
| **déjà acté et construit** | §2 (séparation des couches), §3 (convention, phases, temps), §4.1/4.3/4.4 (spectre, jacobien, bandes), §5.2–5.3 (projected grid et filtrage par empreinte), §8.3–8.4 (domaine local, propriété de surface), §9.2 (éponge) | le guide **décrit ce que le dépôt fait déjà**, parfois mot pour mot |
| **déjà acté, non construit** | §6 (côte), §7 (plage, déferlement, wet/dry), §12.1 en partie | connu et daté : portes E/F, A234, ADR-175 D5 |
| **neuf et actionnable tout de suite** | **§12.3** (test A/B à quatre sorties), **§1** (ordre de diagnostic), §5.3 (ratio `λ/Δ`), §4.3 (publier `min J`) | c'est ici que la session travaille |
| **neuf, à garder pour plus tard** | §6.4 (choix du solveur côtier), §4.1 (TMA), §7.1 (score de rupture calibré) | entrées pour la porte F, avec sources |
| **correction d'un chiffre du dépôt** | §7.1 sur `H/h ≈ 0,78` | note datée à porter ; voir §5 ci-dessous |
| **divergence réelle** | **aucune** | aucun point du guide ne contredit une décision actée |

**La conclusion la plus utile du guide n'est pas une technique, c'est un ordre de diagnostic.** Il
place les normales fines et l'environnement lumineux **avant** la topologie dans la liste des
causes d'un rendu qui ne ressemble pas à une photo. Le dépôt a fait l'inverse en S303–S304 : il a
mesuré la statistique de la surface et construit [ADR-176](../adr/ADR-176-asymetries-de-la-surface-rendue.md).
Ces deux démarches ne s'excluent pas, mais **le dépôt n'a jamais fait le test qui les départage**.
C'est ce que cette session construit.

---

## 2. Ce que le guide décrit et que le dépôt fait déjà

Ce classement est le plus important pour la suite : ce qui est déjà fait n'est pas à refaire, et
un guide qui redit une décision actée ne la rouvre pas (L137).

| guide | dépôt | état |
|---|---|---|
| §2 — séparer topologie physique, surface libre, maillage, shader | [ADR-001](../adr/ADR-001-decomposition-en-couches.md) sépare B / W / δ / V ; l'axe diffère mais la séparation « le maillage ne dicte pas la résolution physique » est I-13 | acté S01 |
| §2 — un heightfield ne peut pas se retourner | notre δ 3D publie **une hauteur par colonne** ; [ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) D5 nomme déjà la seconde représentation nécessaire au déferlement | acté, non construit |
| §3 — origine flottante, phase non modifiée par une translation | `WorldPos` en unités entières, phases en `PhaseQ32` (entiers), rebasage du sillage à la caméra (S212) | construit |
| §3 — toutes les couches au même temps de simulation | `SimTime` en microsecondes entières ; le temps ne transite jamais en f32 ([ADR-003](../adr/ADR-003-horloge-et-determinisme.md) §2.2, ulp f32 = 62,5 ms à 10⁶ s) | construit |
| §4.1 — JONSWAP à pic marqué, paramétré par scénario | `background_spectrum::bake_directional`, γ = 3,3, deux systèmes ([ADR-156](../adr/ADR-156-mer-multimodale-et-etalement.md)) | construit, reçu |
| §4.1 — `Hs ≈ 4√m0` n'est pas la hauteur d'une vague | SPEC-001 §3 ; `Hs` employée comme statistique partout | acté |
| §4.3 — surveiller le jacobien `J` du déplacement horizontal | `euler_slope` rend `det` « pour le repli » ; `covariance_transport` se rabat à `det < 0,1` ; `cwm_reference` publie le déterminant | construit — mais **jamais publié comme métrique**, voir §4 |
| §4.4 — ne pas compter deux fois une onde entre géométrie et shader | c'est exactement [ADR-155](../adr/ADR-155-queue-spectrale-en-pentes-par-pixel.md) (la queue commence où la bande résolue s'arrête) et [ADR-161](../adr/ADR-161-reflets-de-la-queue-non-resolue.md) | construit — avec une réserve mesurée, voir §4 |
| §5.2 — projected grid (Johanson [S4]) | **c'est notre maillage** : `grid_point` lance un rayon par sommet depuis la caméra vers `z = 0`, borné à l'horizon. Jamais acté par un ADR, voir §5 | construit de fait |
| §5.3 — filtrer les petites longueurs d'onde **avant** échantillonnage, selon l'empreinte projetée | `grid_spacing` calcule l'empreinte monde par sommet, `spectral_weight` coupe en fonction d'elle ([ADR-148](../adr/ADR-148-filtrage-spectral-image.md), [ADR-163](../adr/ADR-163-sommes-des-ondes-filtrees.md)) ; erreur mesurée ≤ 0,340 mm, retour caméra au bit ([COUPURE-S249](../validation/COUPURE-S249.md)) | construit, reçu |
| §5.3 — transférer la contribution visuelle du lointain vers une pente/BRDF filtrée | ADR-155 et ADR-161 : la queue non résolue vit en pentes par pixel et en reflets | construit |
| §5.4 — coutures, T-junctions, morphing | **sans objet** : une seule grille projetée, aucun raccord entre niveaux. Le risque que le guide décrit n'existe pas chez nous | sans objet |
| §8.3 — activer une simulation 3D locale sur critères | l'ordonnanceur décide qu'un domaine vit ([ADR-170](../adr/ADR-170-les-trois-poids-sont-bornes.md), [ADR-171](../adr/ADR-171-les-seuils-d-activation-appartiennent-au-profil.md)) ; les critères restants sont datés en file | partiel, daté |
| §8.4 — « couche résiduelle définie » : le solveur local fournit `delta_eta` sur un fond large connu | **c'est notre architecture exacte** : δ publie une perturbation, le rendu reconstruit `η = η_B + δ` (ADR-175 D7, [ADR-168](../adr/ADR-168-premier-rendu-de-delta.md)) | construit, reçu S302 |
| §9.2 — injecter par hauteur **et** vitesse, laisser sortir par éponge | [ADR-164](../adr/ADR-164-relaxation-hauteur-perturbative.md), [ADR-165](../adr/ADR-165-bande-du-fond-aux-frontieres.md), reçus S268–S270 | construit, reçu |
| §11 — une seule API `sample_water` côté jeu | requête de jeu sous CWM ([ADR-159](../adr/ADR-159-requete-de-jeu-sous-cwm.md)), admission ([ADR-142](../adr/ADR-142-composition-sur-l-union-des-emprises.md)) | construit |

**Deux de ces lignes méritent d'être lues deux fois.** Le guide propose, en §8.4, la « couche
résiduelle définie » comme l'une des deux façons correctes d'éviter la double surface : c'est mot
pour mot ce que le dépôt a construit et reçu en S302. Et en §5.2, il décrit le projected grid comme
« une variante, pas un prérequis » : c'est ce que nous employons depuis S211. Un document écrit
sans connaître le dépôt converge donc sur ses deux choix structurants de rendu.

---

## 3. Ce que le guide couvre et que le dépôt n'a pas — et pourquoi ce n'est pas un retard

Les §6 et §7 — propagation côtière, réfraction, shoaling, déferlement, swash, wet/dry — occupent
un tiers du guide et sont **quasi intégralement absents** du dépôt. Ce n'est ni une surprise ni un
oubli :

- la bathymétrie est **un fond uniforme seulement**, et la référence non linéaire peu profonde
  manque : c'est la ligne **S116-2 / A234** de la file active, datée ;
- la porte en cours est **B** (δ sur les deux dimensions horizontales), et la côte relève de la
  porte **F** ([FEUILLE-DE-ROUTE §3 bis](../FEUILLE-DE-ROUTE.md)) ;
- [ADR-127](../adr/ADR-127-ambition-complete-construction-progressive.md) garantit que ces sujets
  restent **obligatoires** ; l'ordre des portes les place plus tard, il ne les retire pas.

Ce que le guide apporte réellement ici, c'est **de la matière sourcée pour quand ce moment viendra**,
et elle vaut d'être notée maintenant pour ne pas la reconstruire :

| apport | source | ce que cela tranchera |
|---|---|---|
| spectre **TMA** à profondeur finie, et sa limite explicite (« ne remplace pas la propagation spatiale ») | Bouws et al. 1985 [S6] | forme spectrale locale près du rivage, sans croire qu'elle fait la réfraction |
| bilan d'**action** `N = E/σ` et son défaut : il ne conserve pas les phases cohérentes | SWAN [S5] | le choix entre « advecter des modes et des phases » et « solveur local alimenté aux limites » — exactement la question que pose notre raccord B → δ |
| **Boussinesq** littoral GPU avec trait de côte mobile | Celeris [S7] | candidat réel pour le solveur côtier |
| **SWE classiques ne portent pas la dispersion intermédiaire** ; hybride bulk-flow + ondes dispersives | Jeschke & Wojtan 2023 [S8] | évite de choisir SWE par défaut pour la plage |
| wet/dry : positivité, *well-balanced*, bilan de masse | [S10] | critères de réception d'une plage, pas seulement un visuel |

**Rien de cela n'est à construire dans cette session ni dans la suivante.** C'est consigné pour
que le lot de la porte F ne reparte pas de zéro.

---

## 4. Ce qui est actionnable tout de suite — trois choses, et une seule est grosse

### 4.1 Le test A/B de §12.3 — jamais fait, et il départage des thèses concurrentes

Le guide demande quatre sorties **au même instant, même caméra, même exposition** : (i) hauteur
brute, (ii) normales géométriques sans détail, (iii) matériau d'eau **sans** normales fines,
(iv) rendu complet. Puis une règle de lecture : si (iii) ressemble déjà au défaut, c'est
l'environnement et le matériau ; si (i) manque de grandes masses, c'est le champ et le maillage ;
si seul (iv) dérive, c'est le filtrage des détails.

Le dépôt possède déjà **deux** de ces quatre sorties : `--no-tail` éteint la queue spectrale
(c'est (iii)), et le rendu complet est le défaut (c'est (iv)). Manquent les deux sorties
**géométriques**. C'est le travail de cette session.

Ce que ce test peut dire, et que rien dans le dépôt ne dit aujourd'hui : **si les stries de notre
mer viennent de la queue spectrale, du ciel, ou de la géométrie.** S303 et S304 ont répondu par la
statistique de la surface ; c'est une réponse, ce n'est pas la seule hypothèse.

### 4.2 Le ratio `λ/Δ` de §5.3 — nous sommes exactement à sa borne basse

Le guide dit que Nyquist (`λ > 2Δ`) est une **condition minimale**, pas un critère de qualité, et
propose `λ/Δ ≥ 4–8` comme point de départ. Notre filtre :

```
spectral_weight(k, h) = 1 − t²(3 − 2t),  t = clamp(2kh/π − 1, 0, 1)
```

vaut **1 tant que `k ≤ π/2h`**, c'est-à-dire `λ ≥ 4h`, et tombe à 0 en `λ = 2h` — Nyquist exact.
Nous sommes donc pleinement pondérés jusqu'à `λ/Δ = 4` et nous laissons une contribution
décroissante jusqu'à 2. **C'est la borne basse de la fourchette que le guide recommande.**

Ce n'est pas une erreur — la coupure a été reçue (erreur ≤ 0,340 mm, [COUPURE-S249](../validation/COUPURE-S249.md)) —
mais c'est un **paramètre jamais éprouvé contre un critère visuel**. Les composantes entre
`λ/Δ` = 2 et 4 sont précisément celles qui produisent le plus de bruit de pente à l'écran. Un
balayage du seuil est un essai d'une ligne.

### 4.3 `min(J)` n'est jamais publié

Le déterminant du jacobien est calculé partout où CWM déplace les sommets, et il sert déjà de
garde (`det < 0,1` → repli dans `covariance_transport`). Mais **aucun banc ne publie sa
distribution**. Le guide en fait un indicateur de premier rang (§4.3, §12.2). Deux lignes dans
l'instrument de S260 suffiraient à savoir si notre mer approche le repli — et, le cas échéant, à
expliquer autrement les crêtes que par la statistique.

### 4.4 Une réserve déjà mesurée que le guide éclaire

Le guide insiste (§4.4) : ne pas faire apparaître deux fois une onde dans la pente. Or S303 a
mesuré que notre `mss` est **14 % trop haute** par rapport à Cox–Munk — ligne inchangée dans la
file active. Un excès de variance de pente est exactement le symptôme d'un double comptage ou
d'une bande fine trop forte. **Le lien n'est pas démontré**, et il n'y a pas lieu de l'affirmer ;
mais il donne au test de §4.1 une prédiction vérifiable : si les stries viennent de la queue, la
`mss` mesurée sans queue doit tomber sous Cox–Munk, et non rester au-dessus.

---

## 5. Deux points à porter dans le dépôt

### 5.1 `H/h ≈ 0,78` — le guide corrige l'usage, pas la valeur

SPEC-001 §3 écrit : « Déferlement en eau peu profonde : `H/h ≈ 0,78` (McCowan) », et
[ADR-023](../adr/ADR-023-mecanismes-restes-a-specifier.md) §4 en dérive des « sites turbulents
permanents » sur les hauts-fonds (non construits). Le guide (§7.1, source **Coastal Engineering
Manual** [S9]) précise que ce rapport est **un repère issu du cas de la vague solitaire sur fond
horizontal**, et non une loi universelle ni un déclencheur suffisant : la pente du fond, la
cambrure incidente et le vent décident du type de déferlement et du seuil.

L'attribution du dépôt est juste (McCowan a bien traité la vague solitaire) ; c'est **l'emploi**
comme critère unique qui ne l'est pas. Une **note factuelle datée** à SPEC-001 §3 suffit — le
mécanisme n'étant pas construit, rien n'est à reprendre. Aucun ADR n'est réécrit (règle d'or).

### 5.2 Le maillage de rendu n'a jamais eu d'ADR

Le projected grid est employé depuis S211 et porte tout le rendu de B, y compris le filtrage par
empreinte dont dépendent ADR-148 et ADR-163. **Il n'est décidé nulle part.** Le guide le classe
parmi les huit décisions à acter avant production (§13.2 point 7), et il a raison : une stratégie
de maillage qui n'est écrite que dans un nuanceur est une décision qu'une session future prendra
sans le savoir. À porter comme point de file, pas comme travail de cette session.

Le guide signale aussi le risque propre à ce choix — « cas rasants, frustum, horizon et bords ».
Le dépôt l'a mesuré indépendamment : **A282 / S247, « angles rasants : coût tenu, échantillonnage
non »**. Deux sources indépendantes désignent la même faiblesse ; cela ne la résout pas, mais cela
la confirme.

---

## 6. Ce que cette lecture ne fait pas

1. **Elle n'acte rien du guide.** Un document reçu n'est pas une décision ; les points retenus
   deviennent des lignes de file ou des ADR par le chemin normal.
2. **Elle ne juge pas les images.** Le guide propose une lecture des deux images ; le dépôt a un
   superviseur humain pour cela depuis S254, et le **verdict R12 n'est toujours pas donné**.
3. **Elle ne vérifie pas les sources une à une.** [S1]–[S12] sont des références classiques et
   identifiables ; les chiffres du guide qui ne sont pas repris ici ne sont pas validés par le
   dépôt (I-14 : aucune valeur physique sans provenance).
4. **Elle ne construit pas la côte.** §6 et §7 restent de la matière pour la porte F.
