# ADR-005 — Zone de transition : éponge perturbative et transduction δ → W

- **Statut** : proposée
- **Session** : S01
- **Résout** : `zones_ouvertes §4` (partiellement défini — critique)
- **Dépend de** : ADR-001

---

## 1. Ce que la décomposition en couches a déjà éliminé

Le document source liste dix sous-problèmes pour la zone de transition. En régime perturbatif,
cinq disparaissent, non par astuce mais parce qu'ils n'existent plus :

| Sous-problème `§4` | Statut |
|---|---|
| conservation de phase | **sans objet** — la phase est portée par B, qui n'est jamais discrétisée |
| conversion onde analytique → état volumique | **sans objet** — B+W est un terme de forçage lu par le solveur, pas une condition d'entrée à convertir |
| interpolation spatiale entre régimes | **sans objet** — la somme est exacte partout, il n'y a pas deux champs à mélanger |
| interpolation temporelle | **sans objet** — δ vaut 0 à la naissance du domaine |
| réflexions numériques | **réduit** à un problème d'absorption d'un champ qui est déjà petit |

Restent trois problèmes réels, traités ci-dessous : l'absorption, la **sortie d'énergie**, et le
régime substitutif.

---

## 2. Absorption : couche éponge sur δ

Sur une bande de largeur `L_s` en bordure de domaine, δ et sa vitesse sont amortis :

```
σ(s) = σ_max · s²          s ∈ [0,1], 0 à l'intérieur, 1 au bord
δ  ← δ  · (1 − σ(s)·dt)
u_δ ← u_δ · (1 − σ(s)·dt)
```

Le profil quadratique est retenu (plutôt que linéaire) parce qu'il évite la réflexion parasite
créée par la discontinuité de dérivée à l'entrée de l'éponge.

**Réglage.** Le coefficient de réflexion d'une éponge vaut approximativement
`R ≈ exp(−2 ∫ σ/c ds)`. Avec un profil quadratique, `R < 1 %` est atteint pour
`σ_max ≈ 4·c/L_s` dès que `L_s ≥ λ_δ/2`, où `λ_δ` est la plus grande longueur d'onde que δ
transporte et `c` la célérité correspondante.

### 2.1 Le point décisif : λ_δ est borné par construction

C'est ici que la décomposition en couches se rembourse.

Si δ devait absorber le sillage transverse d'un bateau à 10 m/s (λ = 2πv²/g = 64 m), l'éponge
devrait faire **32 m de large**, soit davantage que le domaine utile. C'est exactement l'impasse
décrite en ADR-001 §1.2.

Comme toute longueur d'onde supérieure à `λ_cut` appartient à W et non à δ, l'éponge est
dimensionnée par `λ_cut`, pas par la physique de la scène.

Avec `λ_cut = 4 m` (valeur de départ proposée) :

- `L_s = 2 m`
- célérité en eau profonde `c = √(gλ/2π) = 2,5 m/s`
- `σ_max ≈ 5 s⁻¹`
- l'éponge représente ≈2 m sur un domaine de 20 m, soit ≈27 % du volume en 3D. Coût réel, mais
  borné et indépendant de la scène.

> **Note corrective (S16, dossier B2 §3.1 et §10.2).** Deux points sur ce paragraphe.
>
> **Le « ≈27 % » n'est pas reproductible** à partir de ce qui est donné : pour un domaine de 20 m
> avec 2 m d'éponge, la fraction de volume vaut **20 %** si deux faces opposées la portent, **36 %**
> si les quatre faces latérales la portent, **49 %** si les six faces la portent. Le paragraphe ne
> dit pas lesquelles. La grandeur n'est pas anecdotique : c'est le **coût d'entrée de tout domaine**.
>
> **Et `L_s = λ_cut/2` borne `λ_cut` par le haut, ce qui n'avait pas été calculé.** L'intérieur utile
> d'un domaine de largeur transverse `W` vaut `W − λ_cut`. Sur le domaine d'impact de référence,
> 6 × 6 m (SPEC-001 §2.4) : à `λ_cut = 4 m` il reste **33 % d'emprise linéaire, soit 11 % de surface
> au sol** ; à `λ_cut = 6 m`, **plus d'intérieur du tout**. La borne haute de `λ_cut` est donc dictée
> par le plus **petit** domaine, et elle est serrée — `λ_cut ≤ 3 m` pour qu'un domaine d'impact garde
> la moitié de son emprise, contre 4 m proposés ici. À trancher au banc B2.

`λ_cut` devient donc un **paramètre d'architecture central** : il fixe simultanément la frontière
W/δ, la largeur d'éponge, et le coût minimal d'un domaine. Il doit être choisi en premier lors du
benchmark B2/B3.

---

## 3. Sortie d'énergie : l'éponge est un transducteur, pas un absorbeur

Une éponge pure détruit l'énergie. Un domaine autour d'un impact important cesserait alors de
produire des vagues sortantes : l'onde s'arrêterait au bord du domaine. C'est le défaut
qu'aucune éponge classique ne traite, et que le document source appelle « conversion inverse
d'une perturbation simulée vers un état simplifié ».

**Mécanisme retenu.** La bordure du domaine mesure le flux d'énergie sortant, agrégé en secteurs.

```
Pour chaque secteur azimutal j (16 secteurs) :
    E_j += ∫_bordure_j  ρ g δ · (u_δ · n)  dA · dt
    f_j  = fréquence dominante estimée du signal δ dans ce secteur (compteur de passages à zéro)

Toutes les 0,25 s :
    si E_j > E_seuil :
        émettre un paquet W {
            origine    = barycentre du secteur j sur la bordure
            direction  = normale moyenne du secteur
            amplitude  = √(8·E_j / (ρ g · largeur_j))
            lambda     = 2π g / (2π f_j)²
            t_naissance= T_sim
        }
        E_j = 0
```

> **Correction (S05, écart R03).** Ce paragraphe affirmait que la transduction était « la seule
> voie par laquelle δ peut influencer le monde répliqué », et renvoyait à une validation serveur.
> C'était contradictoire avec I-10 : le serveur n'exécute jamais δ, donc il ne peut ni transduire
> ni reproduire ce qu'un client a transduit.
>
> **La transduction ne produit que du `W_local`, cosmétique et non répliqué.** Les ondes répliquées
> sont émises par le serveur, depuis leurs causes. Voir ADR-021 §3. Le mécanisme décrit ci-dessus
> est inchangé ; seul son statut change — il assure la continuité visuelle, pas l'autorité.

L'entrée est en revanche gratuite et sans mécanisme : W est déjà dans le terme de forçage que le
solveur δ lit à chaque pas. **La transition est asymétrique**, et c'est correct — le document
source la supposait symétrique, ce qui l'aurait rendue beaucoup plus coûteuse.

---

## 4. Régime substitutif : relaxation, pas éponge

Quand le domaine possède le champ total (déferlement, volume fini, intérieur), il faut une vraie
frontière génératrice.

**Entrée (côté large)** — zone de relaxation :

```
φ ← α(s)·φ_cible + (1−α(s))·φ_calculé          α : 1 au bord → 0 à l'intérieur
φ_cible = surface et vitesses issues de W (champ de hauteur 2D déjà réfracté)
```

Largeur requise : ≈1 longueur d'onde incidente. Pour une houle de 8 s levée à 3 m de fond
(λ ≈ T·√(gh) = 8·5,4 = 43 m), cela fait **43 m**. C'est cher, et c'est la vraie raison pour
laquelle les zones de déferlement doivent être rares, planifiées et attachées à la géographie —
pas créées dynamiquement autour de chaque joueur.

**Atténuation pratique** : la zone de relaxation n'a pas besoin de la résolution du rouleau. Elle
peut tourner à `dx` grossier (0,5 m) et n'être raffinée que dans la bande de déferlement.

### 4.1 Où commence la zone de déferlement — chiffre exploitable

Le critère de déferlement en eau peu profonde est `H/h ≈ 0,78` (McCowan). Pour une houle de
hauteur H arrivant sur une plage de pente `p` :

```
h_déferlement ≈ H / 0,78
distance au rivage ≈ h_déferlement / p
```

| Houle H | Pente 1:20 | Pente 1:50 | Pente 1:100 |
|---|---|---|---|
| 0,5 m | 13 m | 32 m | 64 m |
| 1 m | 26 m | 64 m | 128 m |
| 2 m | 51 m | 128 m | 256 m |
| 4 m | 103 m | 256 m | 513 m |

La largeur de la zone à simuler en substitutif est donc **entièrement dérivable de la bathymétrie
et de l'état de mer**, avant l'exécution. Elle n'a pas à être découverte dynamiquement. C'est un
gain de prévisibilité important pour l'ordonnanceur (ADR-012) et pour le précalcul côtier
(`zones_ouvertes §27`).

Sur une plage à faible pente et forte houle, la zone dépasse 250 m : elle sera **toujours** hors
budget en volumétrique intégral. Ces cas doivent être résolus dans W (déferlement 2D + habillage
de rouleau procédural), avec du volumétrique uniquement dans un rayon de quelques mètres autour du
joueur. À acter comme contrainte de design de niveau.

---

## 5. Cycle de vie d'un domaine — coût visuel nul

| Événement | Traitement | Coût visuel |
|---|---|---|
| Création | δ initialisé à 0 partout | **nul** : la surface reste B+W |
| Croissance | nouveaux blocs à δ=0 | **nul** |
| Rétrécissement | transduction δ→W puis amortissement sur τ ≈ 0,3 s | négligeable |
| Destruction | idem, τ ≈ 0,5–1,5 s selon l'énergie résiduelle | négligeable |
| Bascule perturbatif → substitutif | `total := B+W+δ` | **continu par construction** |
| Bascule substitutif → perturbatif | `δ := total − (B+W)` | **continu par construction** |

C'est le second gain majeur de la décomposition : l'apparition et la disparition des domaines,
que le document source identifiait comme un risque de rupture visible, deviennent des non-
événements.

---

## 6. Ce qui reste ouvert

1. Valeur de `λ_cut` → **à décider en premier**, benchmark B2/B3.
2. Nombre de secteurs de transduction (16 proposé) et `E_seuil`.
   → **S11** : les 16 secteurs ont désormais un **second consommateur** : SPEC-006 §4.3 les réutilise pour
   l'occlusion acoustique par aération, explicitement pour ne pas inventer une seconde
   discrétisation. Le nombre se tranche donc pour les deux usages à la fois.
3. Estimation de la fréquence dominante par secteur : passages à zéro vs corrélation — la méthode
   par passages à zéro est bruitée à faible amplitude, prévoir un plancher.
4. La transduction doit-elle conserver l'énergie exactement, ou volontairement en perdre 10–20 %
   pour éviter l'accumulation de paquets parasites ? Piste : perte volontaire, à mesurer.
   → **S11** : **à moitié répondu, et ailleurs.** La correction S05 d'ADR-012 §4 rang 1 déclare répondre à ce
   point : « la transduction ne perd rien en régime normal, et tout sous pression ». C'est une
   réponse au **régime dégradé**. La question posée ici porte sur le régime **normal**, et elle
   reste entière.
