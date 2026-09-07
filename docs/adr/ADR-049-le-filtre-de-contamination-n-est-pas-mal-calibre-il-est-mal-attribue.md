# ADR-049 — Le filtre de contamination n'est pas mal calibré, il est mal attribué

- **Statut** : proposée
- **Session** : S60
- **Tranche** : action **S57-2**, angle mort **A179** *(sévérité 2)*
- **Corrige** : rien n'est réécrit. **A179 est requalifié** — son constat est exact, sa conclusion
  implicite ne l'est pas.
- **Confirme** : le filtre ×30 dans son rôle, **sans aucun changement de valeur ni de forme**
- **Produit** : la mesure d'invariance à l'oracle, publiée y compris hors filtre ; l'essai de refus
  à oracle grossier ; la contre-épreuve rétrospective de la campagne 51200/102400
- **Clôt** : **S57-2**. **Ouvre** : **S60-1**, et **A182**

---

## 1. La question, et pourquoi elle était piégée

**A179** avait mesuré que le filtre d'admission de C22 — *l'erreur d'une grille doit valoir au
moins trente fois l'écart L1 entre les deux oracles* — est piloté par le biais du **plus grossier**
des deux oracles, celui dont aucune erreur publiée ne dépend. L'action **S57-2** demandait de
décider si le critère devait comparer l'erreur au **biais de l'oracle de mesure** plutôt qu'à cet
écart.

La question arrivait au pire moment : **S59 venait d'obtenir le premier succès de C22**, à
22,40 % de marge d'admission. Rouvrir un critère juste après avoir obtenu un résultat, c'est le
geste que ce dépôt refuse ailleurs — *« la session qui rendra C04 vert devra changer de schéma,
pas de seuil »* ([`ADR-031`](ADR-031-le-front-de-mouillage-elimine-l-ordre-un.md)). **Rien n'a
donc été assoupli, et la sortie de cette session est de ne pas remplacer le critère.** Ce qui suit
dit pourquoi, et ce que les mesures ont trouvé en chemin.

## 2. Ce que les mesures disent

### 2.1 L'invariance à l'oracle ne peut pas remplacer le filtre

L'ordre est estimé par différences successives — `p = log₂(|e_h − e_{h/2}| / |e_{h/2} − e_{h/4}|)`
— où **un biais additif uniforme s'annule exactement**. D'où la piste : calculer l'ordre contre
chacun des deux oracles et exiger qu'ils coïncident. C'est direct, sans modèle, et cela porte sur
la grandeur publiée.

**L'essai de refus la disqualifie.** Oracle volontairement grossier, 3200/6400 — contamination
flagrante, l'erreur de la grille 1600 est sous-estimée de **5,2 %** et deux grilles sur cinq sont
refusées par le filtre :

| | invariance mesurée | erreur réelle sur l'ordre |
|---|---:|---:|
| oracle 3200 / 6400 | 5,087e-3 | 1,616e-4 |
| oracle 51200 / 102400 | 2,342e-6 | 7,590e-7 |
| oracle 89600 / 179200 | 1,050e-6 | *(référence)* |

*(triplet 400–800–1600 ; « erreur réelle » = écart à la valeur obtenue avec l'oracle le plus fin)*

**5,1e-3 sur un ordre de 1,85, c'est 0,3 % : aucun seuil raisonnable ne l'aurait refusé**, alors
que la contamination des erreurs, elle, est massive. Deux oracles emboîtés d'un facteur deux
partagent l'essentiel de leur erreur : leurs ordres sont également faux, et leur écart reste petit.
C'est **A114** appliqué au critère lui-même, et
[`ADR-043`](ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) §3 le disait déjà des oracles.

Le test `l_invariance_ne_voit_qu_un_biais_heterogene` cerne exactement le domaine : un biais
uniforme **mille fois** l'erreur de la grille la plus fine ne déplace pas l'ordre d'un milliardième ;
un biais hétérogène **cent fois plus petit** le déplace de plus de `1e-3`. **L'invariance ne mesure
pas la contamination, elle mesure son hétérogénéité.**

### 2.2 Mais le filtre a bloqué trois sessions un verdict déjà atteignable

Contre-épreuve rétrospective : la campagne 51200/102400 — celle que le filtre a fait refuser en
S56 — rejouée, avec les ordres relevés **hors filtre**.

| Triplet du verdict, 3200–6400–12800 | Ordre |
|---|---:|
| 51200 / 102400 *(S56, grille refusée)* | **1,997566515** |
| 89600 / 179200 *(S59, grille admise, verdict publié)* | **1,997599436** |
| **écart** | **3,29e-5** |

Les trois ordres de la fenêtre coïncident de même à `7,2e-5` près, et le test de stabilité
n'aurait pas vu la différence. **L'ordre que S59 publie était mesurable en S56**, pour 371,972 s.
Entre les deux campagnes : **1987,7 s de calcul, 33 min 08 s, et deux sessions entières** — pour
déplacer un ordre de trois centièmes de millième.

## 3. La décision

### D1 — Le filtre ×30 est conservé, sans aucune modification

Ni sa valeur, ni sa forme, ni son application. Il n'a jamais admis à tort, aucun verdict publié
ne change, et **aucune des mesures ci-dessus ne montre qu'il est trop strict pour ce qu'il
protège** : la fiabilité des **erreurs** que le rapport imprime. À oracle 3200, l'erreur de la
grille 1600 est fausse de 5,2 % — le filtre la refuse, et il a raison.

### D2 — A179 est requalifié : le défaut n'est pas le calibrage, c'est l'attribution

A179 disait *« le filtre est piloté par la qualité de l'oracle dont on ne se sert pas »*, et c'est
exact. Sa conclusion implicite — *donc il est mal calibré* — ne l'est pas. **Le filtre est bien
calibré pour les erreurs et mal attribué au verdict d'ordre** : on lui fait commander une grandeur
qu'il ne gouverne pas, parce qu'une grille refusée coupe la famille et supprime les triplets qui
la contiennent.

C22 publie **deux choses de nature différente** — des erreurs et un ordre — sous **un seul**
critère d'admission. C'est cela qui a coûté trois sessions. C'est **A182**.

### D3 — L'invariance à l'oracle est publiée comme diagnostic, et disqualifiée comme critère

Elle est affichée à chaque fenêtre, **y compris sur les triplets que le filtre refuse**, ce qui
rend visible ce qui ne l'était pas : un ordre écarté du verdict n'est plus un ordre invisible.
Elle **n'entre dans aucun verdict**, et §2.1 dit pourquoi elle ne le peut pas.

Sur les trois oracles mesurés, elle **majore** l'erreur d'oracle sur l'ordre d'un facteur 3 à 31.
C'est la propriété d'un bon indicateur — conservateur, et dans l'unité de la grandeur publiée —
mais trois points ne font pas une loi, et le facteur varie d'un ordre de grandeur.

### D4 — Le remplacement du critère reste **ouvert par décision**, et l'expérience qui manque est nommée

> **Note corrective — 2026-09-08, S61.** *L'expérience nommée ci-dessous n'existe pas.* Le régime
> `erreur < écart des oracles` demande un rapport oracle/grille `k ≈ 1`, alors que l'emboîtement
> impose `k ≥ 2` : les deux quantités ont la même origine, et leur rapport est borné en dessous
> par `2^p/(1 − 2^-p)`, soit **5,3 à l'ordre deux** — 4,75 mesuré. Aucune taille d'oracle n'y
> change rien, la dépendance mesurée étant en `o^-0,058`. **S60-1 est dissoute**, et l'hypothèse
> d'uniformité qu'elle devait éprouver est établie autrement, par la platitude de la colonne
> « variation » sur un facteur 256 en erreur. Voir
> [`ADR-050`](ADR-050-le-filtre-de-contamination-est-une-condition-geometrique.md) et
> [`GEOMETRIE-DU-FILTRE-S61`](../validation/GEOMETRIE-DU-FILTRE-S61.md). Le reste de cette
> décision — D1, D2, D3, et le refus de remplacer le critère — **tient sans changement**.

Aucune mesure de ce dépôt n'a atteint le régime décisif : celui où **l'erreur d'une grille passe
sous l'écart des oracles**. Même à oracle 3200, l'erreur de la grille 1600 vaut encore 4,8 fois
cet écart. Tant que ce régime n'est pas observé, **rien ne dit que la contamination y reste
additive et uniforme** — et c'est précisément l'hypothèse sur laquelle reposerait tout critère
fondé sur l'invariance.

L'expérience est écrite dans **S60-1** : mesurer une grille dont l'erreur est franchement
inférieure à l'écart des deux oracles — une grille 25600 contre les oracles 51200/102400, où
l'erreur attendue vaut environ 3,7 fois l'écart, puis 51200 elle-même si le montage le permet.
**Sans ce point, changer le critère serait extrapoler hors du domaine mesuré**, exactement ce que
**L175** apprend à ne pas faire.

## 4. Ce que cette décision ne dit pas

- **Elle ne valide pas le verdict de S59 davantage.** L'oracle reste du même schéma ; ce qui est
  établi reste que le schéma converge vers sa propre limite (**A114**).
- **Elle ne dit pas que les trois sessions ont été perdues.** Elles ont produit la mesure qui
  permet aujourd'hui de le dire — rétrospectivement, et personne ne pouvait le savoir en S56. Ce
  qui est en cause est le **dispositif**, pas la conduite des sessions.
- **Elle ne rouvre aucun résultat publié.** Les ordres de S48, S49, S56 et S57 restent ce qu'ils
  étaient ; les campagnes refusées restent refusées.
