# C22 — couple 89600/179200 : la première fenêtre qui conclut, S59, 2026-09-08

Exécution de l'action **S57-1**. Critère **inchangé** : filtre ×30 sur l'écart mesuré des deux
oracles, cinq grilles et trois ordres exigés, test de stabilité avant `p > 0,8`. Aucun seuil,
aucun paramètre de montage et aucune tolérance n'ont été touchés.

## Le remède prescrit aurait invalidé la mesure

[`REFERENCE-C22-S56`](REFERENCE-C22-S56.md) §5 demandait, au-delà du quart d'heure, « un
découpage de calcul en tranches temporelles gardées en mémoire ». **Ce découpage n'est pas
neutre.** `avancer_jusqu_a(t_fin, cfl)` prend `dt = dt_cfl(cfl).min(t_fin − t)` : le dernier pas
est tronqué pour atterrir exactement sur `t_fin`, donc **chaque borne de tranche insère un pas
qui n'existe pas dans le calcul continu**. La séquence change, le champ avec elle.

Mesuré avant d'implémenter quoi que ce soit : à `n = 200`, `t = 1 s`, découper en quatre appels
donne un champ différent bit à bit du champ continu — test
`le_decoupage_temporel_n_est_pas_neutre`, qui conserve le chemin fautif comme témoin. Appliqué à
cette campagne, il aurait rendu les chiffres incomparables à ceux de S48, S49, S56 et S57,
c'est-à-dire exactement ce que le découpage devait permettre.

**Ce qui est retenu est le découpage d'observation.** `avancer_jusqu_a_observe` porte désormais
la seule boucle d'intégration, et `avancer_jusqu_a` n'en est qu'un appel avec un observateur
vide : l'identité est **structurelle**, pas une promesse à retester après chaque modification
(**L162**, **A176**). La campagne rend compte de son avancement une dizaine de fois par champ
coûteux, cadence déduite du premier `dt` — le pas varie d'un facteur 900 entre la plus petite
grille et le plus gros oracle, et une cadence en nombre de pas fixe ne conviendrait à aucun.

> Le second test a échoué à sa première écriture, et il avait raison : cadence de 50 pour une
> intégration de 35 pas, **l'observateur n'était jamais appelé** — et le champ était identique,
> donc l'assertion d'identité passait. Un témoin muet serait passé pour neutre. Le compte de
> rendus est désormais vérifié contre le nombre de pas réellement faits.

## Conditions et coût

Borne du mode `c22-shallow-fin` relevée de 76800 à **89600** — limite de campagne, sans sens
physique, premier multiple de 12800 au-dessus des 79 000 environ que S57 avait calculés. Release
locale, aucune suite de tests en concurrence, champs en mémoire et jamais sur disque (I-17).

| Poste | Mesure |
|---|---:|
| Oracle 89600 | 227,124 s |
| Oracle 179200 | 910,168 s |
| Huit grilles et mesures | 5,992 s |
| **Campagne** | **1143,284 s** *(19 min 03 s)* |

Projection de S57 : **1147,2 s**. Écart **−0,34 %**. Rapport des deux oracles : 4,0074 pour un
facteur de taille 2. Le poste « reste » vaut 5,992 s, contre 6,040 en S57 et 6,035 en S56.

## Écart des oracles

**Écart L1 89600/179200 : 2,118278666e-10**, seuil ×30 : **6,354835998e-9**.

| Couple | Écart L1 | Exposant local |
|---|---:|---:|
| 25600 / 51200 *(S48)* | 1,717298105e-9 | — |
| 51200 / 102400 *(S49, S56)* | 5,207593646e-10 | 1,72145 |
| 76800 / 153600 *(S57)* | 2,709078717e-10 | 1,61233 |
| **89600 / 179200** *(S59)* | **2,118278666e-10** | **1,59587** |

S57 avait prédit 2,1129e-10 avec l'exposant 1,61233 : **+0,26 %**. C'est la contre-épreuve de
**L175** — la même famille de modèles, dont deux membres s'étaient trompés de 4,6 % et 14,6 %
sur une extrapolation d'un facteur 1,5 en taille, tient à 0,26 % sur un facteur 1,167.

## Erreurs et admission

| nx | erreur / 89600 | erreur / 179200 | variation | retenue |
|---:|---:|---:|---:|:--:|
| 100 | 7,514861183e-5 | 7,514864191e-5 | 3,008002e-11 | oui |
| 200 | 2,380685710e-5 | 2,380692250e-5 | 6,539497e-11 | oui |
| 400 | 7,306654514e-6 | 7,306743365e-6 | 8,885086e-11 | oui |
| 800 | 1,982040951e-6 | 1,982139367e-6 | 9,841561e-11 | oui |
| 1600 | 5,048692450e-7 | 5,049692358e-7 | 9,999082e-11 | oui |
| 3200 | 1,253612834e-7 | 1,254589816e-7 | 9,769822e-11 | oui |
| 6400 | 3,124795503e-8 | 3,134587042e-8 | 9,791539e-11 | oui |
| **12800** | 7,680524371e-9 | **7,778410302e-9** | 9,788593e-11 | **oui** |

**La grille 12800 est admise**, à **1,2240 fois** le seuil — marge **+22,40 %**, contre +22,5 %
prédits par S57. Elle avait été refusée à 0,494 fois le seuil en S56, puis à 0,9556 en S57.

## Verdict

| Fenêtre | Grilles | Ordres | Verdict |
|---|:--:|---|---|
| 100–1600 | 5/5 | 1,637650 · 1,631736 · 1,849838 | non concluant, régime asymptotique non atteint |
| 200–3200 | 5/5 | 1,631736 · 1,849838 · 1,960625 | non concluant, régime asymptotique non atteint |
| 400–6400 | 5/5 | 1,849838 · 1,960625 · 2,011671 | non concluant, régime asymptotique non atteint |
| **800–12800** | **5/5** | **1,960625 · 2,011671 · 1,997599** | **OK — p = 1,96, stabilisé** |

Bilan : **quatre grandeurs, un succès, zéro échec, trois sans verdict** ; sortie 0. C'est le
**premier succès de validation de C22** — quatre campagnes, S48, S49, S56 et S57, s'étaient
conclues sans aucun.

## Ce que ce succès dit, et ce qu'il ne dit pas

**Il ne dit pas que le solveur résout Saint-Venant à l'ordre deux.** La référence est un
**oracle numérique du même schéma**, plus fin : ce qui est établi est que le schéma converge à
l'ordre 2 environ **vers sa propre limite de raffinement**, avec un régime asymptotique atteint
sur 800–12800. Un défaut de modèle partagé par toutes les résolutions est invisible à ce
dispositif — c'est ce que posent
[`ADR-032`](../adr/ADR-032-c08-n-est-pas-executable-tel-qu-enonce.md) et
[`ADR-043`](../adr/ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) §3, et **A114** reste
entier : avec un oracle, le triplet le plus fin est le moins fiable.

Trois réserves, à porter avec le chiffre :

1. **Le filtre est empirique.** L'écart de deux oracles du même schéma n'est pas une borne
   prouvée de leur erreur commune. Le facteur 30 est une convention de marge, pas une preuve.
2. **La marge d'admission est de 22 %**, sur une grandeur dont l'exposant de décroissance baisse
   à chaque campagne — 1,721 puis 1,612 puis 1,596. Le succès tient à une grille admise de peu.
3. **Les trois ordres ne sont pas monotones** : 1,9606 · 2,0117 · 1,9976. Ils oscillent autour de
   deux plutôt qu'ils n'y convergent par en dessous, ce que le test de stabilité accepte et ce
   qu'une contamination résiduelle explique aussi bien qu'un ordre exactement deux.

## Apport au dossier A179 — sans toucher au critère

Le déplacement des erreurs entre S57 et S59, sur les mêmes grilles contre deux oracles
différents, est **encore un plateau additif** : 1,1839e-11 en moyenne dès `nx = 800`. La loi de
biais ajustée en S57 — décroissance en `n^-1,879` — prédisait **1,2301e-11** : elle tient à
**−3,8 %** sur un troisième couple qui n'a pas servi à l'ajuster.

Sous cette loi, le biais résiduel de l'oracle 179200 vaut environ **3,66e-11**. L'erreur de la
grille 12800 le dépasse d'un facteur **212**, quand l'indicateur qui commande le filtre n'en vaut
que **5,8** fois. **A179 est confirmé, et non traité** : changer le critère est l'objet de
**S57-2**, par ADR, et le mélanger à cette mesure aurait rendu le succès inexploitable — on
n'aurait pas su s'il venait de la référence plus fine ou d'un critère assoupli.

> **Et la mesure apporte à A179 un fait qu'on n'attendait pas.** Les ordres calculés contre
> l'oracle 89600 et contre l'oracle 179200 coïncident à **1e-5 près** sur les six triplets — par
> exemple 1,997604569 contre 1,997599436. La raison est structurelle : l'estimateur de Richardson
> travaille sur des **différences successives d'erreurs**, où une contamination additive uniforme
> s'annule. **La grandeur mesurée est donc presque insensible à l'oracle, alors que le droit de
> la publier en dépend entièrement.** Quatre campagnes et une heure de calcul ont été dépensées
> sur un filtre qui gouverne l'admission d'une valeur qu'il ne déplace pas.

## Vérification

127 tests réussis (40 cœur + 87 harnais), deux ignorés — deux ajoutés sur le découpage. Refus du
mode vérifiés : `0`, `12800`, `32000`, `102400` *(multiple de 12800 : c'est la borne de campagne
qui le refuse, pas l'emboîtement)*, `usize::MAX`, et `89601` en ligne de commande. Release
compilée ; les deux scénarios `check` restent verts, hashs `0x3e2c06a7b00e73e3` et
`0x1a8b0629a9f51b6e` inchangés. Aucun test lancé pendant la campagne.

## Suite

**S57-1 est close.** La fenêtre 800–12800 conclut ; les trois fenêtres plus grossières restent
non concluantes, et c'est leur état attendu — elles n'atteignent pas le régime asymptotique.

Un couple 102400/204800 coûterait environ **24,9 min** et n'est pas nécessaire : il ne
renforcerait que la marge d'admission d'une grille déjà admise, sans changer un ordre qui ne
dépend pas de l'oracle. **La question utile n'est plus la taille de la référence, c'est le
critère** — S57-2.
