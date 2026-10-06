# ADR-231 — Sixième revue de méthode (S507–S510)

- **Statut : actée**, S511, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-230](ADR-230-cinquieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un correctif de script échoué, puis le script d'origine exécuté quand même** : deux fois en S509 (des guillemets imbriqués, puis une ancre absente) — la commande suivante partait sur une ligne nouvelle, sans attendre le succès de la précédente ; une version fautive appliquée, rattrapée à la main | deux reprises | **protection nouvelle** (D1) — une erreur répétée dans la même session |
| **Le rappel du lot dû manqué** : le rituel l'imprime en tête de sa sortie, la lecture n'en prenait que la fin (S506 → fait en S507) | un lot en retard | **fait** (D2) : le rappel répété en dernière ligne |
| Une variable préfixée dans le shell non transmise au calcul détaché (S507 ; `calcul.py lancer nom VAR=val -- …` existe) | deux calculs inutiles | **fait** : la boussole le dit |
| Une prévision de témoin fausse (« ≈ 100 % » pour un retrait sec, qui vaut `\|η\|` à l'instant, S510) | aucun | publiée corrigée dans la preuve ; **rien à ajouter** |
| Un critère manqué (1 ms, S508), tenu à la session suivante par un instrument (le profil par étage) | une session | la méthode a marché : mesurer, publier le manqué, reprendre ; **rien à changer** |
| A328 ouverte sur une référence trop proche (S505), levée en S507 | une session | ADR-230 D1 l'a fait voir ; **rien à ajouter** |

## 2. Décisions

**D1 — Un enchaînement de commandes s'arrête au premier échec** : `&&` entre les étapes dépendantes (jamais une ligne nouvelle ni `;`), et
un correctif de script qui échoue ne laisse pas partir le script qu'il corrigeait — on corrige le fichier visé à la main (outil d'édition)
plutôt que de corriger le correctif.

**D2 — Ce que le rituel demande s'imprime en dernière ligne** (`outils/rituel.py` : le rappel du lot dû, répété à la fin).

## 3. La prochaine revue

S516.
