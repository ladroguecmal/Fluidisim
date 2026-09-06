# Audit des assertions — S29

Revue des **23 cas canoniques** et des assertions du code, selon un critère unique : *que peut voir
cette assertion, et que ne peut-elle pas voir ?*

**Origine.** Angle mort **A129**, trouvé en S28. C23 mesure une borne de pas de temps fausse —
`u_max` sous-estimé ×5,5, nombre de Courant réalisé à 2,48 — **sans que le solveur casse**. Un cas
dont l'assertion aurait été « le solveur diverge » serait passé, et aurait certifié l'absence d'un
défaut présent.

---

## 1. Le critère, et la catégorie qu'il a fait apparaître

> **Une assertion est recevable s'il existe une grandeur continue dont elle est le seuil.**

L'épreuve sur trois cas connus a validé le critère et révélé une troisième classe.

| Classe | Ce qui cloche | Remède |
|---|---|---|
| **A — recevable** | rien | — |
| **B — symptôme** | ne peut échouer que sur un accident | assertir sur la **grandeur gouvernée**, pas sur ses conséquences visibles |
| **C — vacuité** | satisfaite parce que le mécanisme testé est **absent** | un **témoin** qui doit faire échouer l'assertion |

**La classe C vient de l'épreuve sur C18**, « zéro allocation après initialisation ». Le réflexe est
de la ranger avec les assertions négatives ; c'est faux — il existe un compteur, il est lu, il
vaudrait autre chose si le défaut était là. Elle est **recevable**. Mais elle vaut aussi zéro **si
rien ne tourne**, et c'est une seconde façon d'être verte sans rien dire.

**Le remède de C existe déjà dans ce dépôt sans avoir été nommé.** `C01-jet` est un témoin : le
harnais signale comme **anomalie** le jour où il cesserait d'échouer. Ce qui manquait était de voir
que le dispositif répond à un problème général.

## 2. Le corpus — dix-huit cas sur vingt-trois sont exempts

| Cas | Assertion en cause | Classe | Ce qu'il faudrait assertir |
|---|---|---|---|
| **C07** | « amplitude **nettement supérieure** » en `Fr_h ≈ 1` | **B** | un rapport d'amplitude, avec un seuil |
| **C10** | « période **sensiblement plus longue** » avec masse ajoutée | **B** | le rapport `T_avec/T_sans`, contre `√(1 + m_a/m)` |
| **C11** | « **aucune divergence** sur 120 s » · « **aucun tremblement visible** » | **B** ×2 | l'amplitude de l'oscillation parasite, en fraction du rayon |
| **C15** | « **aucune plaque** ne se forme tant que `Hs > 0,15 m` » | **C** | l'épaisseur mesurée, **plus** un témoin à `Hs < 0,15` qui doit en produire |
| **C18** | « l'hôte serveur **compile et tourne** sans δ ni rendu » | **C** | passe tant que l'hôte serveur n'existe pas |

**C11 est le cas le plus atteint** : deux de ses trois assertions sont de classe B, et « visible »
n'a même pas d'observateur défini. C'est aussi, d'après sa propre note, *« le cas le plus petit et le
plus dur »* — celui qui doit être dans la batterie de base.

### 2.1 Et C20 est exemplaire

> *« Assertion sur la pente, pas sur la valeur absolue — une pente juste avec un décalage constant
> révèle un défaut de détection de contact, une pente fausse révèle un défaut de modèle. »*

C20 **distingue deux défauts par la forme de sa mesure**. Aucun seuil absolu ne le permettrait. C'est
le modèle à suivre pour réécrire les cinq cas ci-dessus, et il a été écrit en S12 sans que le
principe soit énoncé.

## 3. Le code — les témoins existaient sans être nommés

Trois assertions du véhicule protègent déjà contre la vacuité :

```rust
assert!(d.pas_effectues() > 100, "le solveur doit avoir travaillé");
assert!(u_fixe < 1e-5, "témoin : sans paroi mobile l'eau doit rester au repos");
assert!(mobile.max_abs_u() > 0.1, "la paroi doit mettre l'eau en mouvement");
```

Chacune échoue si le mécanisme testé est absent. **Le réflexe était bon ; il n'était pas
systématique, et rien ne l'exigeait.**

## 4. La faute était chez moi, et elle portait une conclusion

`stabilite_par_courant`, écrite en S27, classait une exécution en `Stable / Diverge / NonFini`. Elle
a répondu **« OK partout »** de `ν = 0,45` à `0,99`, et ADR-035 §4 en a tiré la ligne *« aucune
divergence, aucun `NaN` »*.

**Classe B.** Et la mesure de S29 le démontre : à `ν = 1,05`, le schéma **amplifie le mode de maille
d'un facteur 7,5 en cent pas** — amplitude finale 0,0075 m pour un seuil de divergence à 0,06 m.
**L'ancien critère aurait répondu `Stable`.**

**Ce qui sauve la conclusion d'ADR-035 est ailleurs** : la mesure de justesse — l'erreur de période,
grandeur continue de 0,0008 % à 0,0136 % — est de classe A, et c'est elle qui porte le verdict. La
ligne de stabilité était décorative.

### 4.1 La mesure de remplacement, et ce qui la valide

Le facteur d'amplification `|G|` du mode de maille (`λ = 2·dx`), la grandeur que von Neumann
gouverne :

| `ν` | `|G|` par pas | amplitude finale/initiale |
|---|---|---|
| 0,45 | 0,9595 | 2,91e−4 |
| 0,90 | 0,9291 | 6,91e−4 |
| 0,99 | 0,9355 | 2,48e−3 |
| **1,05** | **1,0204** | **7,53e0** |
| 1,50 | 1,7903 | 5,50e20 |

> **La transition est exactement à `ν = 1`.** La mesure n'a pas servi à établir cette borne : elle
> la **retrouve**. C'est ce qui la valide — une mesure de stabilité incapable de retrouver la
> frontière connue ne dirait rien des frontières inconnues. Le contrôle est en test permanent
> (`la_mesure_retrouve_la_frontiere_theorique`).

`Stabilite` et `stabilite_par_courant` sont **retirées**, avec une note en place. Conservées, elles
seraient un faux positif en attente.

## 5. La faute a une variante, et j'y suis retombé pendant l'audit

Le premier balayage d'amplification incluait `ν = 1,05` et a rendu **exactement le résultat de
`ν = 0,99`**. `avec_cfl` bornait silencieusement à `[0,05 ; 0,99]`.

**L'instrument était incapable de produire le résultat qu'il cherchait, et rien ne le disait.** Ce
n'est pas une assertion qui ne peut pas échouer, c'est un **réglage qui ne peut pas atteindre le
régime testé** — la même faute d'un cran plus haut, et invisible par la même mécanique.

> **Corollaire à retenir : vérifier qu'un montage peut atteindre le régime où l'assertion échoue.**
> Une assertion recevable sur un montage qui ne peut pas la mettre en défaut retombe en classe C.

## 6. Ce qui reste à faire

1. **Réécrire les cinq assertions fautives** (§2) — C07, C10, C11, C15, C18. Aucune n'est
   exécutable aujourd'hui : elles attendent des couches qui n'existent pas. **Les corriger
   maintenant coûte peu ; les corriger après qu'un candidat B3 les aura passées coûtera une
   campagne.** Action **S29-1**.
2. **Rendre le témoin systématique.** Chaque cas dont l'assertion peut être satisfaite par l'absence
   du mécanisme doit porter un témoin qui la met en défaut. Action **S29-2**.
3. **Un contrôle d'atteignabilité** pour chaque balayage : vérifier que les bornes du montage
   permettent d'atteindre le régime visé (§5). Action **S29-3**.
4. **Les 47 assertions n'ont pas toutes été exécutées** — la plupart attendent leur couche. Cet
   audit porte sur leur **forme**, pas sur leur résultat, et la forme est ce qui peut être corrigé
   avant que la couche existe.
