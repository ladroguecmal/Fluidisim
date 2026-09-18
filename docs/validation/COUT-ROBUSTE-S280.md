# S280 — un pic ne condamne plus un domaine

**Origine** : le défaut ouvert de [S279](ORDONNANCEUR-S279.md) §4, écrit en [L336](../../notes/LECONS.md).
L'ordonnanceur estimait le coût d'un domaine par celui de son **dernier pas**. Un seul pas à
45,8 ms — pour une médiane de 22 — suffisait à le faire sortir du budget ; et un domaine sorti
n'exécute plus de pas, donc ne produit plus de mesure, donc conserve le coût qui l'a fait sortir.
La bande δ s'éteignait au bout de 0,352 s et ne revenait jamais, alors que rien à l'écran n'avait
changé. S279 avait refermé le cas en élargissant le budget ; le défaut restait entier.

## 1. Ce qui a été écarté avant d'écrire

- **Distribuer le reliquat au lieu d'exclure.** C'est ce qu'ADR-012 §1 point 5 prévoit — le solveur
  s'arrête dans le budget qu'il reçoit — et c'est **inapplicable à δ** : le pas couplé est tout ou
  rien. Un pas partiel avancerait l'histoire de moins que `FRAME_US` en croyant l'avoir faite, et
  la surface mobile n'admet pas de demi-pas (ADR-152).
- **Admettre d'office un domaine affamé.** Cela viole la seule propriété qu'ADR-012 §1 demande de
  défendre avant toutes les autres : la somme des budgets ne dépasse jamais le profil.
- **Faire décroître l'estimation dans l'ordonnanceur.** Ce n'est pas son travail. ADR-012 §3 confie
  la mesure au solveur, qui la réinjecte ; l'ordonnanceur ne doit pas inventer un chiffre que
  personne n'a mesuré.

**Ce qui restait était le vrai défaut**, et ADR-012 §3 le disait déjà en toutes lettres : « la cible
de mesure est le 99ᵉ centile de la contribution par frame, jamais la moyenne ». Une valeur isolée
n'est pas une estimation. La correction appartient à l'hôte, pas au cœur.

## 2. La correction

L'hôte garde les **huit derniers pas payés** et réinjecte leur **médiane**, prise du côté prudent —
celui qui protège le budget. Un pic isolé ne commande plus.

Le compte d'échantillons **décroît d'une unité par pas non payé**. Un domaine qui ne tourne plus ne
sait plus ce qu'il coûte, et le dire est plus honnête que de garder son pire chiffre : vidé, il
retombe sur son estimation nominale et retente. C'est ce qui empêche la condamnation.

Le cœur ne change pas, sinon d'un `set_profile` qui permet à un banc d'éprouver un budget serré
sans rien réallouer.

## 3. Ce que ça donne — `--delta --delta-arbitrage --delta-budget=<ms>`

937 images, trois phases de caméra, le scénario de S279.

| budget | pas **vivants** | pas **payés** | estimation maximale | ce qui se passe |
|---|---|---|---|---|
| 20 ms | 689 / 937 | **0** | 22,0 ms | vivant, **jamais servi** : le budget est réellement insuffisant |
| 33 ms | 689 / 937 | **689** | 29,7 ms | tout est servi — **S279 mourait ici à 0,352 s** |
| 50 ms | 689 / 937 | **689** | 28,4 ms | idem, profil de l'afficheur |

Trois choses à lire dans ce tableau.

**Le budget de 33 ms ne tue plus.** C'est exactement celui qui condamnait la bande en S279. Les 689
pas payés sont les 689 pas vivants : aucun refus budgétaire, alors que des pas individuels
dépassent 33 ms — l'estimation, elle, ne dépasse jamais 29,7.

**La décision ne dépend pas du budget.** 689 pas vivants dans les trois cas, aux mêmes instants.
Le score dit qui a le droit de vivre, le budget dit seulement qui tourne — et c'est la séparation
qu'ADR-012 §1 demande.

**À 20 ms, le domaine est affamé, pas absorbé.** Il reste vivant, son estimation reste à 22 ms —
son coût nominal, honnête — et il redeviendrait servable dès que le budget le permettrait. C'est
la différence avec S279, où le chiffre conservé était un pic que plus rien ne pouvait corriger.

Les douze empreintes de la revue R10 sont inchangées, et le relevé dynamique de S279 est identique :
allumée à 0 s, éteinte à 6,016 s, rallumée à 10,0 s, trois transitions.

## 4. Ce que ça ne règle pas

**La famine reste**, et c'est une autre question : un domaine trop cher pour le budget ne tourne
pas, et aucune dégradation ne vient le rétrécir. Les sept rangs d'ADR-012 §4 ne sont pas écrits ;
le rang 1 — rétrécir l'emprise des domaines non focaux — est la réponse prévue, et il demande la
forme des domaines, qui n'existe pas.

**Huit échantillons est un choix, pas une mesure.** Assez pour qu'un pic ne commande pas, assez peu
pour qu'un renchérissement réel se voie en un cinquième de seconde à la cadence de δ. Aucun banc ne
l'a éprouvé, et le banc B8 n'existe toujours pas.

**Le régime oscillant n'a pas été observé.** Si la médiane dépassait durablement le budget, le
domaine alternerait entre oubli, retentative et exclusion — comportement borné mais dégradé. Le cas
à 20 ms ne l'atteint pas, puisqu'il n'est jamais servi et que son estimation reste au nominal.
