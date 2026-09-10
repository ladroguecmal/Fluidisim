# S138 — Audit des renvois : ce que S137 avait manqué

2026-09-10. Suite de S137-1. Aucune décision nouvelle ; deux notes correctives, un angle mort.

## 1. Ce que l'audit mécanique établit : le corpus est cohérent

Recherche automatique sur les 93 ADR, les 217 leçons, les 204 angles morts, les six
spécifications et tous les documents de `docs/` et `notes/`.

| vérification | résultat |
|---|---|
| numéros d'ADR | 93, de 1 à 93 — aucun trou, aucun doublon |
| numéros de leçons | 217, de 1 à 217 — aucun trou, aucun doublon |
| numéros d'angles | 204 définis — aucun cité sans entrée |
| renvois vers un ADR inexistant | aucun |
| renvois vers une leçon ou un angle inexistant | aucun |
| renvois `SPEC-00x §y` vers une section absente | **aucun** |

Les dix-sept « absences » signalées au premier passage étaient toutes des artefacts de mes
motifs de recherche : SPEC-003 vit dans `docs/validation/` et non `docs/specs/` ; « §5 bis » ne
correspondait pas à mon expression ; et les « §10.3 » désignent les **points numérotés** d'une
section, pas des sous-titres. Vérifier chaque signalement avant de le rapporter était le premier
travail, et il a supprimé la totalité des résultats automatiques.

**Un audit qui ne trouve rien est un résultat**, à condition d'avoir cherché pour de bon. Celui-ci
n'a rien trouvé sur les identifiants — c'est ce qu'on veut savoir d'un corpus de cette taille.

## 2. Ce que la lecture trouve : le correctif de S137 était incomplet

S137 a écrit que « trois ADR » portaient le renvoi erroné « à calibrer B2 ». **Ils sont six.**

| décision | ce qui est renvoyé à B2 | verdict |
|---|---|---|
| ADR-058 §12 | paramètres de **source** (`w`, forme du spectre) | **erroné** — corrigé ici |
| ADR-058 §21 | limite de pente `max_slope` | **douteux** — voir A205 |
| ADR-059 §30 | choix de quadrature de W | plausible — B2 choisit la technologie de W |
| ADR-060 | forme et bande spectrales | erroné — corrigé en S137 |
| ADR-062 §50 | limite de pente `max_slope` | **douteux** — voir A205 |
| ADR-083, ADR-092 | `α`, `η` | erroné — corrigés en S137 |
| ADR-085 §17 | **`α`** et sources physiques | **erroné** — corrigé ici |

ADR-085 a été écrite en S125 par un autre agent, et portait déjà l'erreur — recopiée d'ADR-060
comme les autres. S137 ne l'a pas vue parce qu'elle n'a pas cherché : elle a corrigé les trois
décisions qu'elle avait sous les yeux.

**D'où L218** : quand une erreur est trouvée par hasard, la première question n'est pas comment
la corriger mais **combien de fois elle figure**. Une recherche de texte coûte quelques secondes
et transforme une correction ponctuelle en correction réelle. Annoncer « trois ADR » sans avoir
compté, c'est publier un décompte faux — ce que le dépôt sait déjà payer (S07, S10).

Deux notes correctives datées sont posées, dans ADR-058 et ADR-085.

## 3. Ce qui reste douteux, et n'est pas tranché

Deux décisions renvoient `max_slope` à B2. **Aucun banc ne fixe une limite de pente** : B2 choisit
la technologie de W et `λ_cut` ; B4 juge la décomposition additive `|δ|/Hs`, pas la linéarité
d'une onde. Le renvoi désigne un banc qui ne répondra pas.

Mais la correction n'est pas de le rediriger vers un autre banc, et c'est pourquoi cette session
ne le fait pas. **SPEC-001 §4 donne déjà la cambrure limite de Stokes**, `H/λ ≈ 1/7`, d'où une
pente de déferlement `πH/λ ≈ 0,449` — quatre fois et demie le seuil de 0,1 employé partout depuis
S77. La limite physique est donc dérivable, comme `α` l'était en S136.

Ce qui manque n'est pas une mesure du monde : c'est le **rapport entre la borne L1 du modèle** —
`Σ|a_k|·k`, majoration conservative — **et la pente réelle du champ**. Ce rapport se mesure dans
le modèle. Enregistré en **A205**, et c'est la suite.

## 4. Ce qui n'est pas revendiqué

L'audit a porté sur les identifiants, les sections de spécification et les renvois vers les
bancs. Il n'a **pas** vérifié les renvois « traité en Sxx » ni « résolu par ADR-0xx », qui
demandent d'ouvrir la cible et de juger si elle traite bien la chose — c'est ce qui a coûté à
S137, et un audit exhaustif de ce type dépasse une session.

`max_slope` n'est pas redirigé, et le rapport entre borne L1 et pente réelle n'est pas mesuré :
la session le nomme, elle ne le fait pas.

Aucun code n'a changé : 269 tests réussis, cinq ignorés, inchangés.

## 5. Suite

**S138-1, S139 :** mesurer le rapport entre la borne L1 du modèle et la pente réelle du champ,
et voir si `max_slope` se dérive comme `α` s'est dérivé. Si oui, un troisième paramètre sort de
l'arbitraire ; si non, on saura au moins quel banc doit le fixer, ce qu'aucun document ne dit
aujourd'hui.

Restent ouverts : l'extension de fenêtre, la profondeur finie de pression (S116-2), le bilan
mixte, la durabilité disque, les deux calibrations de B10, et l'audit des renvois de type
« traité en Sxx » que cette session n'a pas mené.

93 ADR, 205 angles, 17 invariants, 6 spécifications, 23 cas.
