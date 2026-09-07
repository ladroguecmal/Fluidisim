# ADR-050 — Le filtre de contamination est une condition géométrique, et il présuppose l'ordre qu'il sert à mesurer

- **Statut** : proposée
- **Session** : S61
- **Tranche** : action **S60-1**, **dissoute** — le régime qu'elle prescrivait n'existe pas
- **Corrige** : rien n'est réécrit. [`ADR-049`](ADR-049-le-filtre-de-contamination-n-est-pas-mal-calibre-il-est-mal-attribue.md)
  §3 D4 reçoit une note corrective datée : l'expérience qu'il nommait est impossible, non coûteuse.
- **Confirme** : `ADR-049` D1 — le filtre reste conservé, sans aucune modification
- **Produit** : [`GEOMETRIE-DU-FILTRE-S61`](../validation/GEOMETRIE-DU-FILTRE-S61.md) ; la règle de
  dimensionnement `o ≈ 6·n` ; le tableau de coût des campagnes futures
- **Clôt** : **S60-1**, par dissolution. **Ouvre** : **A183**, action **S61-1**

---

## 1. Ce que S60 avait prescrit, et pourquoi c'était sans objet

`ADR-049` D4 refusait de remplacer le filtre ×30 tant qu'un régime n'aurait pas été observé :
celui où **l'erreur d'une grille passe sous l'écart des deux oracles**. Sans ce point, tout
critère fondé sur l'invariance reposerait sur une hypothèse d'uniformité jamais éprouvée à sa
limite. L'action **S60-1** nommait l'expérience et l'annonçait à « moins de sept minutes ».

**Le régime n'existe pas.** Les deux quantités comparées ont la **même origine** — l'erreur du
schéma — et leur rapport est borné en dessous par la géométrie de l'emboîtement. Pour un schéma
d'ordre `p`, avec `k = oracle / grille` :

```text
ratio = e(n) / ‖o1 − o2‖ ≈ k^p / (1 − 2^-p)
```

L'emboîtement impose `k ≥ 2` — la grille la plus fine possible est la moitié de l'oracle — donc
`ratio ≥ 2^p/(1 − 2^-p)`, soit **5,3 à l'ordre deux**. Mesuré sur treize couples issus de cinq
campagnes : `ratio ≈ 2,011·k^1,902·o^-0,058`, écart maximal 23,5 %, et **4,75 au point `k = 2`**.

La dépendance à la taille de l'oracle est négligeable ; aucune campagne, si longue soit-elle, ne
descend sous 1. **S60-1 est dissoute** — statut légitime de `REPRISE.md` §8 : la question ne
reçoit pas de réponse, elle cesse de se poser.

## 2. Ce que le filtre exige réellement

### D1 — Le filtre ×30 équivaut à « l'oracle est six fois plus fin que la grille la plus fine »

| Oracle | `k` requis | Grille la plus fine admissible |
|---:|---:|---:|
| 25600 | 5,63 | 4 547 |
| 51200 | 5,75 | 8 906 |
| 89600 | 5,85 | 15 323 |
| 179200 | 5,97 | 30 009 |

La condition est **géométrique** et se lit sur un rapport d'entiers, **avant tout calcul**.
L'historique de C22 s'y range sans exception : `k` valait 2 en S48, 4 en S49 et S56, 6 en S57,
**7 en S59** — et l'admission bascule exactement là. **Quatre campagnes et près d'une heure de
calcul ont mesuré ce que cette suite donnait.**

### D2 — Le seuil d'admission d'une mesure d'ordre est une fonction de l'ordre

```text
la grille n est admise  ⟺  (o/n)^p ≥ 30·(1 − 2^-p)
```

À l'ordre 2, `k ≥ 4,7`. À l'ordre 1, `k ≥ 15`. À l'ordre 3, `k ≥ 3,2`. **Dimensionner la campagne
suppose de connaître la réponse qu'elle cherche**, et se tromper d'hypothèse ne produit aucune
erreur visible : cela produit un **« sans verdict »**. C'est l'histoire de C22 de S48 à S57, et
c'est **A183**.

Ce n'est pas un défaut du filtre : c'est une propriété de tout critère d'admission bâti sur une
comparaison d'erreurs entre grilles. Elle doit être **écrite**, parce qu'elle change la manière de
conduire une campagne — **on ne choisit pas un oracle, on choisit un `k`, et c'est une hypothèse
d'ordre qu'on assume.**

### D3 — Le filtre est conservé, et sa description est complétée sans être réécrite

Il reste décrit comme un indicateur empirique quant à ce qu'il **borne** : l'écart de deux oracles
du même schéma ne majore pas leur erreur commune, et rien ici ne change cela. Ce qui est ajouté
porte sur ce qu'il **exige**, et c'est exact et calculable : `k ≥ 6` environ.

## 3. Ce qui remplace l'expérience dissoute

L'hypothèse que S60-1 voulait éprouver — *la contamination reste-t-elle additive et uniforme ?* —
n'avait pas besoin d'un régime extrême. Elle se lit sur la colonne « variation » du rapport, qui
mesure la contamination grille par grille. En S59, de `nx = 800` à `nx = 12800` — un facteur 16 en
grille, 256 en erreur — elle est **plate à 2 % près**, et le même plateau apparaît en S56, S57 et
jusque dans l'essai à oracle 3200.

**La propriété est donc établie sur tout le domaine accessible.** Ce qui reste à décider n'est plus
l'uniformité, c'est **quel `k` on assume** — et c'est une décision de campagne, pas de critère.

## 4. Ce que cette décision ne dit pas

- **Elle ne rouvre aucun verdict.** Les refus de S48 à S57 restent des refus, le succès de S59
  reste ce qu'il est, et le filtre n'a pas bougé d'un chiffre.
- **Elle ne dit pas que l'oracle est un bon juge.** Il reste du même schéma ; **A114** et
  `ADR-043` §3 sont intacts. La géométrie décrit le critère d'admission, pas la valeur de la
  référence.
- **Elle ne dit pas que les campagnes de S48 à S57 étaient inutiles.** Ce sont elles qui
  fournissent les treize points. Ce qui est en cause est le **dispositif**, qui ne disait pas
  d'avance ce qu'il pouvait admettre — d'où l'action **S61-1**.
