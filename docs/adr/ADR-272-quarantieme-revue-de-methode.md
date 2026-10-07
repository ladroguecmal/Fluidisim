# ADR-272 — Quarantième revue de méthode (S676–S680)

- **Statut : actée**, S681, 2026-10-08 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-270](ADR-270-trente-neuvieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S676 | la revue (ADR-270) | — | — |
| S677 | **le calcul du plan comptait `g` deux fois** (l'énergie en `g·a²/2`, puis encore `g`). `Hrms` au rivage, qui devait retrouver 0,511 m (S669), l'a montré avant l'écriture du plan | quelques minutes | couvert : ADR-239 D1 (une formule éprouvée par un calcul indépendant), ADR-253 D1 |
| S677 | le lot dû, fait après le rappel du rituel | — | le rituel le rattrape |
| S678 | **le remède du plan a manqué** (0,33 m/s) ; une seconde cause trouvée en route. **Le remède combiné tenait le montage du plan aux deux mailles** (2,5 et 3,1 mm/s), et l'aurait validé. Essayé sur d'autres plages, il manque sur deux sur six : le défaut dépend de la place de la ligne d'eau dans la maille | **aurait coûté** : un remède fragile validé sur un seul cas | **D1** |
| S679 | la conception ; l'obstacle des colonnes et du fond lisse nommé d'avance | — | — |
| S680 | l'interface simplifiée (le flux rendu par Saint-Venant, et non imposé) ; le bilan au bit | — | — |

## 2. Décisions

**D1 — Un remède à un défaut qui dépend de la place d'une interface dans la maille se juge sur trois places au moins.** Une ligne
d'eau, une surface, un rivage, une face coupée : le montage du plan n'en essaie qu'une. Le plan écrit au moins trois montages où
l'interface tombe ailleurs dans la maille (une autre pente, un autre niveau), aux deux mailles. Le remède n'est retenu que s'il tient
partout.

C'est un élargissement d'ADR-256 D2, qui vérifie un écart par deux variations : l'amplitude et la maille. La maille seule ne suffit
pas : en S678, les deux mailles du même montage passaient.

## 3. La prochaine revue

S686.
