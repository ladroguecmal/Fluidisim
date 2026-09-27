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

---

## Note corrective — B-S26 : le §2 est mesuré, et son réglage est remplacé

> *Reportée de la lignée B le 2026-09-06 (S35), renvois renumérotés. Une **seconde voie**, indépendante
> de celle-ci, desserre la même contrainte : voir [`ADR-043`](ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) §4 — et son §5, qui distingue les
> **trois** fonctions que ce §2 appelle « éponge ».*

Le §2 de cet ADR n'avait jamais été confronté à du code. Il l'a été en B-S26, par le cas C05
([ADR-042](ADR-042-l-eponge-mesuree-et-la-borne-de-lambda-cut-rouverte.md)), et trois de ses
affirmations doivent être lues avec ce qui suit.

**1. Le réglage `σ_max ≈ 4·c/L_s` ne donne pas `R < 1 %`.** Avec le profil quadratique du §2,
`∫₀^{L_s} σ_max·(x/L_s)² dx = σ_max·L_s/3`, donc `R ≈ exp(−8/3) = 6,95 %`. **Mesuré : 7,0 %.**
L'arithmétique et la mesure concordent à 1,3 %. Il faudrait `σ_max ≈ 6,9·c/L_s` pour tenir la
promesse ; **ADR-042 D1 retient `10·c/L_s`**, qui donne `1,5·10⁻³` avec de la marge.

**2. La formule `R ≈ exp(−2∫σ/c ds)` est excellente, puis s'effondre.** Jusqu'à `10·c/L_s` elle est
juste à quelques pour cent. Au-delà elle promet `2·10⁻⁶` là où on mesure `8,8·10⁻⁴` : la mesure
**sature vers `9·10⁻⁴`**, un plancher qui est la **réflexion à l'entrée de l'éponge** — l'impédance
que le modèle ne contient pas. **Amortir plus fort que `≈10·c/L_s` ne sert à rien**, ce que la
formule monotone ne peut pas exprimer.

**3. La condition `L_s ≥ λ_δ/2` n'est pas exercée en eau peu profonde.** À `σ_max` proportionnel à
`c/L_s`, `R` ne bouge pas quand l'éponge passe de `λ` à `λ/8`. Ce qui borne l'éponge par le bas est
**`σ_max·dt < 1`**, soit `L_s ≳ K·CFL·dx ≈ 5 mailles` — la **maille et le pas de temps**, pas la
longueur d'onde. **ADR-042 D2** remplace la règle ; **D4** rouvre en conséquence la borne haute de
`λ_cut` que `DOSSIER-B2 §3.1` en tirait.

**Ce qui reste vrai dans ce §2, et qui compte** : le §2.1 — *`λ_δ` est borné par construction, donc
l'éponge est dimensionnée par `λ_cut` et non par la physique de la scène* — n'est pas touché par la
mesure. Il l'est par D2, mais dans le sens qui l'arrange : si la largeur ne dépend plus de `λ` du
tout, l'argument de §2.1 devient superflu plutôt que faux.

**Et une réserve qui empêche de conclure** : le solveur de B-S26 est **non dispersif**. Une éponge
d'eau profonde doit absorber une **bande** de célérités, et c'est peut-être exactement ce dont
`L_s ≥ λ/2` protégeait. Voir ADR-042 §6.

---

## Note corrective — B-S27 : la largeur, et enfin quel `c`

> *Reportée de la lignée B le 2026-09-07 (S39), renvois renumérotés. **Elle rétracte la note de
> B-S26 qui la précède immédiatement**, et non ce §2.*

La note de B-S26 ci-dessus retirait la condition `L_s ≥ λ_δ/2`, la mesure ne trouvant pas sa trace.
**Cette note-là était fausse, et pour la raison la plus instructive** : elle avait été établie en eau
peu profonde, où l'amortissement ponctuel est sans réflexion par accident algébrique. En milieu
dispersif, mesuré en B-S27 ([ADR-046](ADR-046-l-eponge-en-eau-dispersive-retracte-ADR-042.md)) :

| `L_s/λ` | 0,125 | 0,25 | **0,5** | 1,0 | 2,0 |
|---|---|---|---|---|---|
| `R` | 0,669 | 0,515 | **0,227** | 0,0098 | 0,00144 |

**La condition `L_s ≥ λ_δ/2` du §2 est donc du bon genre — et de la mauvaise constante.** Elle donne
23 % pour un critère à 1 %. La règle devient **`L_s ≥ λ_δ`**, et **`L_s ≥ 2·λ_δ` dès que `δ` porte un
spectre** — le cas normal.

**Et l'ambiguïté sur `c` est tranchée.** Le §2 écrit « `c` la célérité correspondante » sans dire
laquelle ; en eau profonde, phase et groupe diffèrent d'un **facteur deux**. Au même point de
fonctionnement, `σ_max = 10·c_groupe/L_s` donne `R = 0,00144` et `σ_max = 10·c_phase/L_s` donne
`0,00269`. **C'est la vitesse de groupe**, celle qui transporte l'énergie.

**Ce que le §2 disait de juste, et que B-S26 avait pris pour faux** : la largeur est bien commandée par
la longueur d'onde. Le §2.1 — *`λ_δ` est borné par construction, donc l'éponge est dimensionnée par
`λ_cut` et non par la physique de la scène* — **reste l'argument central de cet ADR**, et il est
renforcé plutôt qu'affaibli : l'éponge coûte deux à quatre fois plus cher que ce paragraphe le
supposait, ce qui rend d'autant plus décisif le fait qu'elle soit bornée.

---

## Note du 2026-09-27 (S402) — le corps de cet ADR restauré

Le commit de S35 (`c2eb75ba`) a remplacé ce fichier entier par la note B-S26 qu'il devait lui ajouter — 202 lignes effacées, le
titre compris — ; celui de S39 (`16e48d60`) a remplacé à son tour cette note par la note B-S27. De S35 à S401, ADR-005 n'avait
plus ni titre ni §1 à §6 : I-12, ADR-006 §3.2 et ADR-012 §4 renvoyaient à un §5 absent. **Restauré en S402, sans rien réécrire** :
le texte de S16 (`c0df00f7`), puis la note B-S26 (S35), puis la note B-S27 (S39), chacun au bit. Un contrôle de
`outils/etat_projet.py` tient désormais que chaque ADR commence par son titre (METHODE, protections « en écrivant »).
