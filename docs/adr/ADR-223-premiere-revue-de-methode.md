# ADR-223 — Première revue de méthode (S481–S485)

- **Statut : actée**, S486, 2026-10-05 ; décisions prises ici ([ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 : toutes les cinq
  sessions, la session relit ses frictions et corrige elle-même la méthode).
- **Précise** [METHODE](../../notes/METHODE.md) (trois protections), la [boussole](../../BOUSSOLE.md) (ses pièges) et le banc d'essais du
  cœur (un essai). Ne touche ni au périmètre ni à l'ordre des campagnes.

## 1. Les frictions relues, et leur suite

| friction (journal) | coût | suite |
|---|---|---|
| Deux calculs longs tués ensemble avec la session (S481, 7 h) | un calcul à refaire | **fait en S481** : `calcul.py` lance par WMI, hors de la session (ADR-222 D1) |
| Une vérification suspendue 12 h par la veille de la machine (S483) | une demi-journée d'attente apparente | **rien à changer** dans le dépôt : à la reprise, lire l'horloge et l'état des processus avant de conclure à un blocage (la protection « calcul long » de METHODE le dit déjà) |
| Le quart de cuve de S484, pris pour une symétrie sans l'avoir éprouvé : deux sessions sur un montage faux (× 0,57 contre × 0,90) | une session et demie | **protection nouvelle** (D1) |
| Le centre des poches faux de S479 à S484 (× 0,8), une grandeur de diagnostic jamais éprouvée à sa naissance | des chiffres faux publiés (POCHES-AIR-S479) | **essai nouveau** (D2) : `air_pocket_centroid_is_the_bubble_centre_s486` |
| Le rejeu d'un écoulement qui se déstabilise, comparé en trajectoire sur 0,4 s (S485) — les deux calculs divergent après 0,05 s | un critère manqué, mal posé | la protection L371 (l'incertitude vraie est la sensibilité à une perturbation minime) le disait ; **précisée** (D3) |
| Des textes écrits par Python dans un heredoc : `\n`, `\U` interprétés, six éditions cassées en trois sessions | du temps, deux fichiers à réparer | **piège nouveau** (D4) |
| FXC (DirectX 12) refuse l'écriture indexée dans un tableau local de structure, et un `switch` qui retourne dans chaque branche | deux compilations ratées | **piège nouveau** (D4) |
| Le coût multiplié par quatre vu à la main (S481) | une session (S482) | **fait en S483** : le banc de non-régression du rituel |
| Une poche d'une maille, un air partagé avec des fragments, un rappel trop lent : trois défauts de la référence de S479, trouvés par des scènes, pas par ses essais | deux sessions | la règle qui manquait est D1 bis : un modèle neuf s'éprouve **dans une scène de jeu** avant d'être déclaré tenu |

## 2. Décisions

**D1 — Un montage simplifié s'éprouve contre le montage entier avant de servir.** Une symétrie (quart, demi), une paroi prise pour un
plan de symétrie, un domaine réduit : à maille grossière, une fois, la grandeur mesurée comparée entre les deux ; l'écart est écrit dans la
preuve. **D1 bis** — un modèle neuf du cœur (une poche, un fond, un corps) n'est « tenu » qu'après une scène de jeu (`--v1` ou équivalent),
pas seulement ses cas analytiques : les trois défauts des poches sont sortis des scènes.

**D2 — Une grandeur de diagnostic s'éprouve à sa naissance** sur un cas de réponse connue (le centre d'une sphère, le volume d'un cube),
par un essai du cœur — la protection « un instrument s'éprouve sur un cas de réponse connue » s'applique aux sorties, pas seulement aux
bancs.

**D3 — Deux calculs d'un écoulement qui se déstabilise se comparent en statistiques au-delà de leur temps de divergence** (une vitesse
moyenne, une fréquence), pas en trajectoire ; le temps de divergence se mesure (le témoin de L371) avant d'écrire le critère.

**D4 — Pièges nouveaux** (boussole) : un texte qui contient des barres obliques inverses s'écrit par l'outil d'édition ou depuis un fichier,
jamais dans une chaîne Python d'un heredoc ; sous DirectX 12 (FXC), pas d'écriture indexée dans un tableau local de structure ni de
`switch` dont chaque branche retourne.

## 3. La prochaine revue

S491 (cinq sessions après S486).
