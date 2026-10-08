# ADR-276 — Quarante-troisième revue de méthode (S691–S695)

- **Statut : actée**, S696, 2026-10-08 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-274](ADR-274-quarante-deuxieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S691 | la revue (ADR-274) | — | — |
| S692 | la conception du LOD (ADR-275) | — | — |
| S693 | **une valeur par défaut cachée dans une aide reprise** : `Plage::nouvelle` (S650) centrait l'onde à la distance canonique (3,488 m), alors que la 3D et la référence la centraient à 3,4 m. C'est une seconde source dans un montage qui devait n'en avoir qu'une (ADR-273 D2) | deux essais de 10 à 12 min | **D1** |
| S693 | **des relances chevauchées** : un essai arrêté, un autre lancé sur le même binaire, deux processus vivants | quelques minutes | `outils/essai.py` refuse un essai déjà vivant (S695) |
| S693 → S695 | **un témoin qui faisait varier deux causes.** Déplacer le raccord du large changeait à la fois ce que Saint-Venant porte et la part de l'onde qui traverse le raccord. La tendance a été attribuée au porteur et écrite dans la preuve de S693 ; S694 a construit un porteur dispersif en partie sur cette prémisse. S695 a montré que c'était le raccord | une attribution fausse écrite ; une session sur une prémisse en partie fausse (SGN reste utile : la référence de A234) | **D2** |
| S694 | une valeur de tête dans le texte du plan (« 2,08 m », le script : 2,40 m) | aucun | ADR-243 D1 le dit déjà |

## 2. Décisions

**D1 — Un état initial se construit dans l'essai, depuis une seule fonction, pour tous les solveurs du montage.** Une aide d'une
session antérieure (`Plage`, `montage_s647`, `relais_libre_s653`) n'est reprise qu'après avoir relu chacune de ses valeurs contre la
référence : la position de l'onde, le repère vertical, la durée. La ligne « pièges » du plan les énumère. ADR-273 D2 disait « une
seule source » ; S693 montre qu'une aide peut en cacher une seconde.

**D2 — Un témoin ne fait varier qu'une cause.**
- Pour chaque variation, le plan écrit tout ce qu'elle change. Si deux causes nommées bougent ensemble, le témoin ne les départage pas,
  et la preuve ne lui attribue rien.
- Une attribution écrite dans une preuve se rejuge avant qu'une session ne s'appuie sur elle.

C'est le complément d'ADR-259 D1 (un témoin qui ôte une cause) et d'ADR-256 D2 (deux variations).

## 3. La prochaine revue

S701.
