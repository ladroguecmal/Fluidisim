# S279 — la bande δ décidée par l'ordonnanceur

**Origine** : S278 avait écrit ce qui décide, et rien ne s'en servait — maillons à 1. La bande δ de
l'afficheur était câblée en dur depuis S275 : rien ne la décidait, ne la déplaçait ni ne l'éteignait.
Ce lot la branche.

## 1. Ce que la bande publie

À chaque image, `Layer::arbitrate` soumet ses trois poids à l'ordonnanceur, qui tranche ; δ n'avance
et ne s'affiche que s'il est retenu.

- **`W_perception` est calculé** : `Projection::screen_fraction` projette l'emprise de la bande
  (256 m × 200), la coupe au plan proche **avant** de diviser par la profondeur — sans quoi une
  emprise dont on tourne le dos paraîtrait immense — puis aux quatre bords du cadre, et rapporte
  son aire à celle du cadre.
- **`W_gameplay` et `W_urgence` sont déclarés au maximum.** Un afficheur n'a ni acteur, ni objectif,
  ni échéance à laquelle une absence deviendrait visible : ils n'ont aucune source. C'est donc la
  perception qui décide seule, ce qui est honnête tant qu'on ne prétend pas les avoir mesurés.
- **Le coût annoncé est celui du dernier pas payé** (ADR-012 §3), pas une constante.

Seuils : `0,45 / 0,35`, calibrés en [ADR-171](../adr/ADR-171-les-seuils-d-activation-appartiennent-au-profil.md).

## 2. Ce qui ne change pas — les douze empreintes de R10

`--delta --revue-delta`, après branchement : **les douze empreintes sont identiques** à celles
publiées en S275, pose haute comprise.

| pose | B seul / B+δ 4 ms / B+δ 16 ms |
|---|---|
| le long des crêtes | `2c2c7112d88de170` / `6655e844fb685e94` / `ab9ab268355170ef` |
| face à la houle | `a78b0282f39d0f4a` / `1d04366680e37613` / `0291b2d7d0986a73` |
| haute | `184d2c9d2cfab515` / `3c28db078621434b` / `ed8f64818ea6fb07` |
| rasante | `ab416e1f0c7240e3` / `fc448ff856113dee` / `f82b079ef7ed1ce9` |

**La pose haute n'est pas éteinte**, alors qu'ADR-171 la calibrait pour l'être. La raison est juste
et vaut d'être retenue : `--revue-delta` travaille à **temps figé** — les douze images sont rendues
au même âge de 20 s. Le domaine s'allume à la première pose, et aux suivantes son score tombe sous
`off` sans que rien ne s'éteigne, parce que ni la durée de vie minimale ni le délai d'extinction ne
s'écoulent quand le temps ne bouge pas. **Une décision demande du temps ; une image isolée n'en
donne aucun.**

L'identité au bit prouve donc que le branchement ne casse rien. Elle ne prouve pas qu'il décide.

## 3. Ce qui décide — le relevé dynamique, `--delta --delta-arbitrage`

937 images au pas de δ (16 ms), trois phases : la caméra regarde la bande, passe à la pose haute où
S275 a mesuré que δ ne change **aucun** pixel, puis revient.

| instant | part de cadre | bande |
|---|---|---|
| 0,000 s | 0,5456 | **allumée** |
| 6,016 s | 0,3185 | **éteinte** |
| 10,000 s | 0,5456 | **rallumée** |

Trois transitions, aucune de plus. La caméra s'éloigne à 5,000 s et l'extinction survient à
6,016 s : **1,016 s plus tard**, soit le délai d'extinction d'ADR-013 §5 plus un pas. Le retour, lui,
est immédiat — un allumage n'a pas de délai, seule la mort en a un.

C'est la première fois que le système éteint quelque chose de lui-même, et il l'éteint là où une
mesure indépendante avait établi que ça ne se voyait pas.

## 4. Le défaut que le branchement a révélé

Le premier essai déclarait un budget d'une image à 30 Hz, soit 33 ms. **La bande s'est éteinte au
bout de 0,352 s et n'est jamais revenue**, alors que rien à l'écran n'avait changé — la part de
cadre valait toujours 0,5456.

La cause est un cycle : le pire pas de δ mesuré en S276 vaut 45,8 ms ; un seul dépassement suffit à
exclure le domaine du budget ; **un domaine exclu n'exécute plus de pas, donc ne produit plus de
mesure de coût, donc conserve le coût qui l'a fait exclure, donc reste exclu.** L'exclusion par le
coût est **absorbante**.

Le budget de l'afficheur couvre désormais le pire pas connu (50 ms), ce qui referme le cas observé.
**Le défaut de fond n'est pas corrigé** : avec un budget plus serré — celui d'un jeu, 2 ms — il
reviendrait immédiatement, et aucun mécanisme n'en sort. Le traiter demande une décision qui n'est
pas prise : soit un coût annoncé qui décroît tant que le domaine ne tourne pas, soit la dégradation
d'ADR-012 §4 qui rétrécit le domaine au lieu de l'exclure. Voir L336.

## 5. Ce que ça ne dit pas

**Un seul candidat.** La bande δ est seule à soumissionner : le tri par `P/C`, la contention et la
famine ne sont pas exercés ici — ils l'étaient au banc de S278, sur un cas synthétique.

**Les poids déclarés restent déclarés.** Tant qu'aucun gameplay ne s'exprime, `s = W_perception`, et
la calibration d'ADR-171 ne vaut que pour ce cas.

**Rien ne déplace la bande.** L'ordonnanceur décide qu'elle vit ; son emprise, sa résolution et sa
position restent écrites en dur. Grille de référence, blocs épars, fusion et séparation (liste 1.5
et 1.6) ne sont pas commencés.

**Une extinction n'est pas gratuite pour l'onde injectée** : δ renaît au repos (I-12), donc l'onde
de S277 repart de zéro au rallumage. C'est le comportement voulu — un domaine perturbatif renaît à
δ = 0 — mais il faut le savoir avant de regarder.
