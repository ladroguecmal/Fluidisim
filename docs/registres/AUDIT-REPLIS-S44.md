# Audit des valeurs de repli — S44

**Une valeur de repli placée après une mesure travaille sur un canal qui transporte aussi les
refus.** Ce registre les inventorie, suit ce que devient chaque valeur, et classe.

Action **S43-1**, qui reprend **S42-2**.

---

## 1. Pourquoi un inventaire plutôt qu'une troisième trouvaille

**Deux sessions de suite ont trouvé un repli de sévérité 1 en cherchant autre chose :**

| | repli | ce qu'il rendait | pourquoi c'était grave |
|---|---|---|---|
| **S42** | `NaN.min(10⁶)` | `10⁶` périodes | le **meilleur score** face à un minorant de 15 |
| **S43** | `else { 1.0 }` | l'ordre `1,0` | **l'ordre nominal du schéma**, dans les bornes de G10 |

Deux hasards font une méthode manquante. La recherche est mécanique, et son critère tient en une
question : ***que devient un refus qui passe là-dedans ?***

## 2. L'inventaire

**25 `unwrap_or` et 24 `min`/`max`** dans `code/`. Le classement ne porte pas sur la correction du
repli mais sur **ce que devient la valeur** :

| classe | ce que ça veut dire |
|---|---|
| **sain** | le repli est une valeur **définie**, pas un refus déguisé |
| **inoffensif ici** | un refus le traverse, mais il finit dans un **diagnostic imprimé** ou produit un **échec bruyant** |
| **fautif** | un refus le traverse et devient une **grandeur lue**, plausible ou meilleure |

### 2.1 Les treize `unwrap_or(NaN)` — sains, sous condition

Le refus **survit** : `NaN` échoue toute comparaison, donc un cas qui le reçoit devient rouge.

> **Mais ils ne sont pas sains par eux-mêmes.** `NaN` ne survit ni à `min`, ni à `max`, ni à une
> soustraction suivie d'un `max`. Ce sont **les trois défauts trouvés en S42, S43 et S44**. Un
> `unwrap_or(NaN)` n'est sain que si **rien en aval ne l'avale** — et c'est l'aval qu'il faut
> inspecter, jamais le repli seul.

### 2.2 Les huit `unwrap_or(0.0)`

| # | où | ce que rend le repli | classe |
|---|---|---|---|
| 1–2 | `delta.rs` — `u_paroi()`, `bords()` | pas de paroi mobile → **vitesse nulle** | **sain** — l'absence de paroi *est* une vitesse nulle, ce n'est pas un refus |
| 3 | `physics_shallow.rs` — `front_ritter` | front introuvable → **0**, c'est-à-dire *au barrage* | **inoffensif ici** |
| 4 | balayage CFL | idem, **imprimé** | inoffensif |
| 5 | balayage de seuils | idem, imprimé comme un écart de −100 % | inoffensif |
| 6 | `c04_ritter` → **assertion `C04-front`** | front à 0 contre une référence à 10,6 m → **écart de 100 %** | **inoffensif** — le cas échoue |
| 7–8 | affichage d'amplitude | diagnostic | sain |

**Aucun des huit n'est fautif**, et c'est contraire à la thèse déclarée. Le n° 6 méritait l'examen le
plus attentif — il est sur le chemin d'une assertion publiée — mais un front à `0` produit un
**échec**, pas un succès.

> **Il reste imparfait** : un échec à 100 % dit *« le front est au barrage »*, pas *« le front n'a
> pas été trouvé »*. La différence compte le jour où quelqu'un cherchera pourquoi.

### 2.3 Les `min`/`max` sur des grandeurs mesurées

| où | expression | classe |
|---|---|---|
| `physics.rs` — **déficit de demi-vie** | `(15 − mesure).max(0)/15` | **FAUTIF** |
| `physics.rs` — **déficit de R²** | `(0,9 − mesure).max(0)` | **FAUTIF** |
| `physics_shallow.rs` — saturation de demi-vie | `mesure.min(10⁶)` | corrigé en **S42** |
| `physics_dispersif.rs` — pas de temps | `(0,3/σ).min(4dt).max(0,5dt)` | sain — calcul interne, pas une mesure |
| `oracle.rs` — plancher de tolérance | `borne.max(10⁻¹²)` | sain — un plancher de test |
| `physics.rs` — abscisse de jauge | `(front − λ).min(L − λ)` | sain — bornage géométrique |

## 3. Les deux fautifs, et pourquoi ils sont exemplaires

`physics.rs` exprime les deux minorants de C03 sous forme de **déficit** :

```text
déficit de demi-vie = max(0, 15 − mesure) / 15      référence 0, tolérance 0
déficit de R²       = max(0, 0,9 − mesure)          référence 0, tolérance 0
```

**La formulation est excellente**, et le commentaire qui l'accompagne explique pourquoi : un minorant
s'exprime mal avec une tolérance relative — *avec `référence = 15` et 100 % de tolérance, une
demi-vie de 0,1 période « passerait »*. Le déficit est nul dès que le minorant est tenu, et croissant
avec ce qui manque.

**Le `max(0, …)` qui rend cette forme juste est exactement ce qui avale les refus.** Mesuré :

| entrée | ce que ça veut dire | avant S44 | après |
|---|---|---|---|
| `+∞` | le schéma **n'amortit pas** | déficit **0** — passe | **0** — passe, et c'est juste |
| `NaN` | il n'y avait **rien à mesurer** | déficit **0** — **PASSE** | **`NaN`** — échoue |

La correction est une fonction, `deficit(seuil, mesure, échelle)`, qui **sépare les deux cas que le
`max` confondait**. Aucun chiffre publié ne bouge : 20,69 et 24,40 périodes, `R²` 0,9920 et 0,9998.

## 4. Ce que l'inventaire apprend, et qui n'était pas dans la thèse

**La thèse était fausse sur la cible et juste sur le fond.** Elle visait les `unwrap_or(0.0)` — *zéro
est la meilleure valeur possible pour un écart* — et aucun des huit n'est fautif. Les deux fautifs
sont ailleurs, dans un `max(0, …)` dont **zéro est aussi la meilleure valeur possible**.

> **Le motif ne tient pas au repli mais à la grandeur** : dès qu'une mesure est un **déficit**, un
> **écart** ou une **erreur**, zéro est le succès parfait, et toute opération qui peut produire zéro
> à partir d'un refus le transforme en succès parfait. `unwrap_or(0.0)`, `max(0.0)`, `saturating_sub`,
> une différence de deux valeurs égales par défaut — la forme varie, la conséquence est la même.

Et une observation sur les trois défauts trouvés en trois sessions :

| | forme | valeur rendue | domaine |
|---|---|---|---|
| S42 | `min(10⁶)` | `10⁶` | **hors** du domaine plausible |
| S43 | `else { 1.0 }` | `1,0` | **dans** le domaine nominal |
| S44 | `max(0.0)` | `0` | **le meilleur point** du domaine |

**La gravité croît, et la visibilité décroît dans le même ordre.** Un `10⁶` finit par se faire
remarquer ; un `1,0` au milieu des ordres attendus, jamais ; un `0` sur un déficit est *le résultat
qu'on espère*.

## 5. La règle

> **Une valeur de repli se choisit hors du domaine des valeurs valides, ou n'existe pas.**
> Quand la grandeur est un écart, une erreur ou un déficit, **son domaine contient zéro et zéro en
> est le meilleur point** : aucune valeur de repli n'y est acceptable, et le refus doit aller dans
> le **type**.

Et son corollaire, qui est ce que cet audit a réellement coûté à trouver :

> **Un repli ne s'inspecte jamais seul.** `unwrap_or(NaN)` est irréprochable et a produit les trois
> défauts, parce que `min`, `max` et une soustraction suivie d'un `max` avalent tous le `NaN`.
> **C'est l'aval qu'il faut suivre**, jusqu'à l'assertion ou l'affichage.

## 6. Ce qui reste

1. **Les six replis « inoffensifs ici »** le sont *ici* — parce que la référence n'est pas nulle et
   qu'un front à zéro produit un écart de 100 %. Le jour où l'un d'eux alimentera une grandeur dont
   zéro est le succès, il deviendra fautif sans que rien ne change dans son écriture.
2. **`front_mouille` devrait refuser plutôt que rendre zéro** — un front introuvable n'est pas un
   front au barrage. Action **S44-1**.
3. **Les treize `unwrap_or(NaN)` n'ont pas tous été suivis jusqu'à leur assertion.** Trois l'ont
   été, parce que trois défauts y menaient. Action **S44-2**.

## 7. Suivi exhaustif de l'aval — S45, 2026-09-07

Recensement sur `b521129` : **13 occurrences dans code/, dont 11 dans le harnais et
2 dans les tests du cœur**. Les numéros de ligne ci-dessous sont ceux de cette révision.

| # | Origine | Consommation finale et sort du refus |
|---|---|---|
| 1 | `delta.rs:1159`, front | affichage puis `mesure <= borne` : NaN fait échouer le test |
| 2 | `delta.rs:1184`, front | différence relative imprimée : NaN reste visible, aucune assertion |
| 3–4 | `oracle.rs:512,538`, front | `rapporter_sensibilite`, soustraction/abs/division : NaN imprimé |
| 5 | `physics.rs:75`, eta | Hs et homogénéité : sommes/variance/racine ou refus explicite, puis `Cas::passe` faux. **C02 : les passages absents suppriment les assertions. C10 : la comparaison du maximum ignore NaN et conserve zéro.** |
| 6 | `physics.rs:653`, front | mesure de C04, écart relatif puis comparaison : échec |
| 7 | `physics.rs:979`, front | erreur absolue → `Convergence::ordre` → `Observe(NaN)` ; asymptotique faux si trois ordres disponibles, donc non concluant ; sinon comparaison p > 0,8 fausse, échec. Aucun succès, mais statut Observe trompeur |
| 8 | `physics_dispersif.rs:84`, période | Cas de dispersion, écart relatif : échec |
| 9–10 | `physics_dispersif.rs:347,377`, réflexion | Cas B-S27-plancher et B-S27-R, référence zéro → abs → comparaison : échec |
| 11 | `physics_shallow.rs:967`, période au mur | différence relative imprimée seulement : NaN visible |
| 12 | `physics_shallow.rs:968`, période du mode | C03-T, écart relatif : échec |
| 13 | `physics_shallow.rs:1102`, ordre | C08-p : NaN donne tolérance zéro, puis échec ; C08-coherence : différence/abs, échec |

**Deux défauts candidats sur une même origine**, à reproduire en P3. Aucun `min/max`
supplémentaire n'est nécessaire : une branche conditionnelle peut ignorer une mesure, ou
supprimer l'assertion entière. Le chemin C02 couvre aussi l'essai à zéro S43-3.

### Résultat P3

Les deux tests échouent avant correction et passent après, avec leurs témoins :

- C02 sans excitation produisait **0 assertion sur 3** ; le champ entièrement hors référentiel
  suit la même branche. Les trois assertions sont désormais présentes et en échec. Les recherches
  de zéro refusent immédiatement un échantillon non fini, y compris pendant la bissection.
- C10 ignorait les refus pendant la recherche du maximum : une fenêtre entièrement invalide
  conservait η = 0 et passait les quatre assertions. Un seul point invalide invalide désormais
  les quatre mesures. Essais entièrement et partiellement hors référentiel ; témoins à Hs = 0 et 2 m.
- Le témoin monochromatique C02 conserve ses trois succès. L'eau plate est recevable pour la
  statique C10 et ne contient aucune période mesurable pour C02 : le refus dépend de la grandeur.

`cargo test --offline` : **98 succès, 2 tests ignorés** (38 cœur + 60 harnais exécutés).
`check` : deux scénarios, zéro échec, hashs inchangés. Le rapport `physics` nominal est comparé
avant/après sur les deux scénarios, hors durées : résultats inchangés, sortie 1 attendue pour
les échecs déjà présents (dont C04 ordre un). Aucun solveur ni seuil modifié.

**S44-2 est close.** Le statut `Observe(NaN)` de C08 reste une dette de représentation,
sans succès indu sur ce chemin ; son traitement est porté par S45-1.

## 8. Le refus jusqu'au bilan C08 — S46, 2026-09-07

**S45-1 close.** `Convergence::ordre` refuse les entrées et résultats non finis, y compris
un rapport qui déborde à partir d'entrées finies. Les ordres négatifs finis restent observés
(ADR-032 §5). Les seuils d'arrondi et d'asymptoticité ne changent pas.

Deux essais échouaient avant correction : `Observe(NaN)` et une série dont les triplets refusés
étaient retirés avant le contrôle de stabilité. La seconde était déclarée stable : retirer
des trous construit une autre famille. `asymptotique` rend désormais `None` si un triplet de
la famille n'est pas exploitable ; il ne recolle plus les seuls ordres observés.

Le rapport principal utilisait deux classifications différentes. Elles sont remplacées par
`rapport_convergence::Bilan`, appelé pour chaque famille :

| Mesure et contexte | Classe dans le bilan |
|---|---|
| Cas régulier, stabilité établie, p > 0,8 | succès |
| Cas régulier, stabilité établie, p ≤ 0,8 | échec |
| Stabilité fausse ou non établie | sans verdict, cause affichée |
| Triplet indéterminé ou plancher d'arrondi | sans verdict, cause affichée |
| Cas singulier, même stabilisé | diagnostic sans verdict de validation |

**Chaque famille compte exactement une fois.** Le bilan imprime systématiquement succès,
échecs et sans-verdict, dont la somme donne le total. Un plancher n'est pas une preuve d'ordre ;
un refus n'est pas un ordre nul. Ce traitement applique l'énoncé amendé de C08 (S26), sans
modifier une décision d'architecture.

**Portée :** les quatre familles delta sur C04 et la famille régulière C22 dans le rapport
principal. Le montage hérité `physics_shallow::c08_convergence` conserve sa paire C08-p /
C08-coherence : il mesure Ritter sur trois grilles et applique toujours un seuil. Cette
différence avec l'énoncé amendé mérite son propre traitement (S46-1), pas une attribution
de validation globale à cette correction.

**Validation S46.** 101 tests réussis (38 cœur + 63 harnais), deux ignorés ; sept tests ciblés
incluant les refus, les témoins et dix familles de rapport. check : zéro échec, deux hashs
inchangés. Comparaison avant/après : 56 lignes de mesures, assertions et suites C08 identiques.
Le bilan principal passe à **5 grandeurs, 0 succès, 0 échec, 5 sans verdict** : le cas régulier
(p = 0,82, stabilité inconnue) n'était pas compté parmi les quatre sans verdict antérieurs.
La sortie physics reste 1, avec le même échec attendu de C04 ordre un.

## 9. La portée du C08 hérité — S47, 2026-09-07

**S46-1 close.** c08_convergence de shallow renvoie désormais DiagnosticRitter : un ordre
optionnel et un contrôle de cohérence, sans assertion de validation sur p. Le rapport garde
C08-p, ses six décimales et son refus explicite ; il affiche le contrôle séparément.
Le seuil 0,25 de ce contrôle est conservé : il a trouvé un défaut de terme de fond (ADR-040 §2).
Les douze autres assertions du second véhicule restent dans leur bilan ; un contrôle incohérent
ou non fini augmente toujours le compteur d'échecs et affecte le code de sortie.

Le type empêche de remettre tout le diagnostic dans la liste des Cas par un simple extend.
Le test vérifie p sous et au-dessus de l'ancien seuil, incohérence de 0,83, NaN et infinis.
Le test numérique conserve p = 0,999745 à 10^-6 près, issu de la mesure S36, et le témoin
historique à référence ponctuelle reste exécuté. Aucune grille, aucun schéma modifié.

Il manque encore un montage C22 régulier sur shallow pour valider sa convergence selon le
contrat courant. Cette absence est désormais visible et suivie par S47-1 ; elle n'est pas
remplacée par l'accord de deux estimateurs sur un cas singulier.

**Validation S47 :** 102 succès (38 cœur + 64 harnais), deux ignorés. check sans échec,
hashs inchangés. 57 lignes de mesures/assertions/suites identiques avant/après hors les deux
lignes requalifiées ; p = 0,999745 et écart des estimateurs = 0,001459 conservés.
physics garde la sortie 1 attendue (C04 ordre un). Second véhicule : 12 assertions,
1 contrôle de cohérence sans échec et 1 diagnostic sans verdict de validation.

## 10. Le front absent reste absent — S50, 2026-09-07

**S44-1 close. Correction du diagnostic de départ :** front_mouille renvoyait déjà
Option dans le cœur. Les appelants de physics_shallow remplaçaient None par zéro.
Le recensement réel sur 7a33d95 donne **cinq sites** : quatre positions et un indice de
profil ; la mention « six replis » du §6 et de la fiche S44-1 était inexacte.

| Chemin | Traitement S50 | Aval vérifié |
|---|---|---|
| front_ritter / raffinement | Option conservée ; conversion au diagnostic seulement | mention front introuvable, NaN et écart NaN imprimés |
| balayage CFL | absence conservée jusqu'au diagnostic | aucun front zéro fabriqué ; volume toujours imprimé |
| balayage des seuils | absence conservée jusqu'au diagnostic | soustraction, abs et division conservent NaN |
| position / C04-front | Option transmise à cas_front | grandeur front introuvable, mesure NaN ; ecart_rel reste NaN, passe est faux |
| profil autour du front | Option de liste d'indices | absence annoncée, aucune cellule de remplacement ; borne haute limitée au domaine |

Les autres appels du détecteur dans oracle.rs conservaient déjà le refus en NaN ;
ils ne sont pas modifiés. Les deux replis d'amplitude ne concernent pas un front.
Aucun seuil changé, aucun calcul de flux ou de hauteur modifié, aucun ADR réécrit.

Deux tests nouveaux : bassin sec et seuil non atteint, témoin humide au front connu ;
refus jusqu'à l'assertion avec référence 10,6 puis zéro, et témoins valides correspondants.
Le témoin Some(0) face à zéro passe ; None face à zéro échoue. Le profil absent est None.
Ce cas couvre le risque anticipé par S44 : changer la référence ne peut plus rendre
le remplacement zéro favorable. C'est une application de L166, sans nouvel angle mort.

**Validation S50 :** 107 tests réussis (38 cœur + 69 harnais), deux ignorés. Sept tests
ciblés passent, dont la reproduction du front historique. Rapport physics avant/après :
seules quatre lignes de durées diffèrent ; mesures, assertions et profils nominaux inchangés.
C04-front shallow : 10,562500 m contre 10,640868 m, écart 0,736 %. Sortie physics 1
attendue pour C04 ordre un. check : zéro échec, hashs 0x3e2c06a7b00e73e3 et
0x1a8b0629a9f51b6e inchangés. Aucun nouveau résultat de validation physique revendiqué.

## 11. La formule d'ordre restait dupliquée — S51, 2026-09-07

Correction de portée du §8 : le refus non fini de S46 protégeait Convergence::ordre,
mais ordre_grossier_estime avait une seconde formule et rendait encore Some(NaN).
Le test S51 le reproduit. Les deux chemins partagent désormais ordre_richardson ; leurs
planchers et catégories restent distincts. Inventaire, appelants et limites dans
[ AUDIT-MESURES-S51 ](../validation/AUDIT-MESURES-S51.md). A176.

## 12. Les grilles retirées avant le contrôle — S53, 2026-09-07

Complément de portée au §8 : conserver les triplets refusés dans Convergence ne protégeait
pas contre une grille supprimée par le montage C22. Trois défauts reproduits et corrigés :
modulo zéro, allocation refusée effacée, stabilité acceptée sur tailles irrégulières.
Voir [GRILLES-C22-S53](../validation/GRILLES-C22-S53.md), A177. Le montage conserve les
refus ; le calcul vérifie les doublements ; le filtre sain ne recolle plus de trou.
Les résultats nominaux du rapport restent inchangés.
