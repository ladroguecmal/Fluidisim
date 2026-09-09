# S131 — Sortir de la saturation, et ce que cela coûte vraiment

2026-09-10. [ADR-087](../adr/ADR-087-sortie-de-saturation-annoncee.md) actée. S130-1 réalisée.

## 1. Inventaire : le chemin existait, avec un piège

ADR-086 a reçu la saturation comme état terminal du contrôleur. La sortie passe par `copy_into`
vers un stockage plus grand, puis `retry`. Trois faits, lus avant de décider :

- **`copy_into` prend `&self`**, et le contrôleur expose son journal en lecture seule depuis
  ADR-086. L'élargissement et la reprise se font donc **pendant que le contrôleur sert encore**.
  Seule la reconstruction impose de le libérer.
- **`copy_into` copie l'attente** avec la publication, sans admission implicite.
- **`copy_into` ne refuse que si le stockage est plus petit que la publication.** Un stockage de
  taille exactement `count` passe la copie et laisse l'attente irrésolue : `retry` y rend `Full`
  de nouveau.

Le troisième point est le piège : **un élargissement peut réussir sans sortir de la
saturation**, et l'hôte ne l'apprend qu'après avoir payé la copie *et* la reconstruction, pour
rien. C'est ce qui manquait, et c'est ce que la décision comble.

## 2. Décision

[ADR-087](../adr/ADR-087-sortie-de-saturation-annoncee.md) : `Journal::required_capacity()` rend
le nombre d'emplacements nécessaires pour que la copie **et** la reprise aboutissent — la
publication, plus l'attente s'il y en a une. La garantie est une équivalence, pas une prudence.

## 3. Réception

**L'équivalence, par balayage sur les tailles de stockage.** Pour chaque taille inférieure à la
capacité annoncée, le test vérifie *laquelle* des deux étapes échoue — `Capacity` pour la copie
quand le stockage est plus petit que la publication, `Full` pour la reprise quand il est juste
assez grand pour elle. À la capacité annoncée, les deux aboutissent.

**Le contrôleur ne cesse pas de servir pendant les tentatives.** Après chaque échec
d'élargissement, le test rééchantillonne le champ en quatre points et le compare en bits à ce
qu'il valait avant : identique. Une sortie de saturation ratée ne coûte pas le service.

**Le champ d'après est celui du journal élargi.** Après reconstruction, l'échantillon diffère de
l'ancien — les deux sources y sont — et il coïncide en bits avec une préparation directe du
journal élargi. Le contrôleur change de nouveau d'instant : la saturation est derrière.

## 4. Ce que le cycle coûte

Médianes sur 21 mesures, après mise en régime (A195), aux deux recettes de la campagne :

| | 224×128 | 256×128 |
|---|---|---|
| élargissement + reprise, **service maintenu** | 0,1 µs | 0,1 µs |
| reconstruction, **sans champ** | 12,21 ms | 13,41 ms |

**L'élargissement est gratuit** — une copie de références de sources — et il ne coûte pas le
service, puisque `copy_into` n'emprunte le journal qu'en lecture. Toute la dépense est dans la
reconstruction, et elle vaut exactement une préparation : 12,6 ms et 14,5 ms mesurés en S119 et
S120 pour les mêmes recettes.

Deux conséquences pour l'hôte :

- **L'ordre importe, et il est mesurable.** Élargir puis libérer laisse l'hôte sans champ le
  temps d'une préparation. Libérer puis élargir l'en priverait pendant tout le cycle, pour rien.
- **La fenêtre vaut environ trois quarts de trame** à 60 Hz, et elle est incompressible : il
  n'existe pas de chemin qui republie sans recalculer. C'est un argument de plus pour
  dimensionner le pool afin de ne jamais saturer — cette sortie est un secours, pas une manœuvre
  de routine.

## 5. Ce qui n'est pas revendiqué

La fenêtre sans champ n'est pas supprimée, seulement bornée et mesurée. Rien n'est alloué : le
stockage élargi vient de l'hôte, comme tous les pools depuis I-06. `required_capacity` dit ce
qu'il faut **maintenant**, pas ce qu'il faudra.

Le journal ne retient toujours qu'une source en attente ; ADR-075 n'est pas rouvert. Aucune
source publiée n'est retirable. La transaction mixte reste hors de portée, comme en S130.

## 6. Vérification

167 core + 93 harnais = **260 tests réussis, cinq ignorés** ; le test de sortie de saturation
passe aussi en release. Hachages de la campagne `cycle_mixed` identiques à ceux de S118.

*(La première version de la mesure était placée avant le bloc de mise en régime : sa médiane
tenait, mais son maximum montait à 35 ms. Déplacée après, comme A195 l'impose depuis S119.)*

## 7. Suite

**S131-1, S132 :** la fenêtre sans champ est incompressible parce que la reconstruction repart
de zéro. Or le journal élargi contient les **mêmes** sources que l'ancien, plus une : les
coefficients de l'ancien champ restent valides pour la part commune. Un chemin incrémental —
ajouter la contribution de la source admise au champ publié plutôt que tout recalculer —
supprimerait la fenêtre et accélérerait aussi l'admission ordinaire d'ADR-086. À mesurer avant
de décider : la superposition modale est linéaire (S112), donc le gain semble atteignable, mais
l'identité en bits avec la voie directe ne l'est peut-être pas.

Restent ouverts : la transaction mixte, l'extension de fenêtre, la profondeur finie de pression
(S116-2), le bilan mixte, la durabilité disque, le générateur physique d'ADR-055.

87 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas.
