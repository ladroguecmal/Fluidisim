# ADR-243 — Dix-septième revue de méthode (S561–S565)

- **Statut : actée**, S566, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-242](ADR-242-seizieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Une valeur du plan écrite de tête** (S562 : « la charge à 1 % vers 190 s », calculée ensuite à 220 s). ADR-237 D1 dit « un ordre de grandeur s'écrit après l'avoir calculé » ; il a été calculé — après avoir été écrit. S564 a fait autrement : son script de plan calculait les nombres et les écrivait lui-même | aucun (la durée restait suffisante) ; mais la règle a échoué une fois encore | **protection élargie** (D1) |
| Un heredoc pour ajouter du code (S562), contre la lettre d'ADR-240 D1 ; protégé par des apostrophes droites, il n'a rien cassé | — | la règle reste telle quelle (simple à suivre) ; **rien à changer** |
| Le commit gardé par le code du rituel (ADR-242 D2), S561–S565 | — | appliqué cinq fois ; **rien à changer** |
| Les références par des méthodes indépendantes (bissection, Hardy Cross, forme fermée), S563–S565 | — | justes ; **rien à changer** |
| Le périmètre de 5.7 non réduit sans l'utilisateur (S563 : la part hors de V reste écrite comme manque) | — | **rien à changer** |

## 2. Décision

**D1 — Les nombres d'un plan sont écrits par le script qui les calcule** (une chaîne formatée depuis la valeur calculée), jamais recopiés
ni tapés : un nombre de tête ne peut alors plus entrer au plan (élargit ADR-237 D1, L388).

## 3. La prochaine revue

S571.
