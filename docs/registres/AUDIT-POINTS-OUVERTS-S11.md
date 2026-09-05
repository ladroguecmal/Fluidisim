# Audit des points ouverts — S11

Passage en revue des **110 points** inscrits dans les listes « ce qui reste ouvert » des 26
documents qui en portent. C'est un axe d'audit que personne n'avait parcouru : la revue croisée S05
a confronté les ADR entre eux, la revue S08 les SPEC entre elles, et **ni l'une ni l'autre n'a
regardé les points reportés**.

La leçon **L40**, écrite en S10, dit pourquoi : un audit vérifie ce qui est *affirmé*, et un point
reporté se lit comme une lacune connue et suivie — c'est-à-dire comme quelque chose dont on sait
déjà qu'il n'est pas résolu. Personne ne va vérifier qu'une question ouverte **a encore un objet**.
Le cas qui a déclenché cet audit avait traversé neuf sessions et deux revues croisées : ADR-007 §5.3
réclamait un format pour un mécanisme qu'ADR-013 §6, écrit la même session, avait dissous.

---

## Méthode

Trois questions par point, toujours dans cet ordre :

1. **A-t-il encore un objet ?** Une décision ultérieure l'a-t-elle dissous ?
2. **Sa formulation tient-elle encore ?** Chiffres périmés, renvois cassés, prémisse changée.
3. **Qui attend, et quoi ?** Une mesure, une réunion, une décision humaine, du code.

Six verdicts, dont un seul demande une action lourde :

| Verdict | Signification | Action |
|---|---|---|
| **A — dissous** | le point n'a plus d'objet ; une décision ultérieure l'a supprimé | le clore et dire par quoi |
| **B — clos ailleurs** | la question a reçu sa réponse dans un autre document, sans que le point soit marqué | le clore et renvoyer |
| **C — formulation périmée** | la question subsiste, l'énoncé est faux ou obsolète | note corrective |
| **D — dupliqué** | le même point vit dans deux documents ou plus | désigner le porteur, renvoyer depuis l'autre |
| **E — valide** | rien à faire | — |
| **F — pas une question** | une observation ou une décision classée par erreur en point ouvert | requalifier |

---

## Inventaire

| Document | Points | Document | Points |
|---|---|---|---|
| SPEC-006 | 8 | ADR-009 · 010 · 011 · 013 | 4 chacun |
| SPEC-004 | 6 | ADR-016 · 018 · 019 | 4 chacun |
| SPEC-005 · ADR-014 · 015 · 017 · 022 | 5 chacun | SPEC-003 | 4 |
| ADR-001 · 004 · 005 · 006 · 007 · 008 | 4 chacun | ADR-002 · 003 · 012 · 020 · 021 | 3 chacun |

**Total : 110 points, 26 documents.**

Quatre documents n'en portent aucun, et c'est normal : SPEC-001 et SPEC-002 sont des fiches de
référence chiffrée, `CAS-CANONIQUES` et `PLAN-BENCHMARK` sont eux-mêmes des listes de travail. Les
registres non plus — un registre consigne, il ne reporte pas.

**Corrélation à noter** : les documents les plus chargés sont les plus récents (SPEC-006, huit
points, écrit en S09) et les plus transversaux (SPEC-004, six). Ce n'est pas un défaut de ces
documents — un document qui n'ouvre aucune question est suspect (`METHODE.md`) — mais cela dit où
la dette de suivi s'accumule le plus vite.
