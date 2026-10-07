# ADR-265 — Trente-quatrième revue de méthode (S646–S650)

- **Statut : actée**, S651, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-263](ADR-263-trente-troisieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S646 | la revue : le plan committé avec elle, au lieu d'avant — le rituel l'a refusé, la session l'a déclaré | aucun | la protection a servi |
| S647 | une commande de fin de session à trois documents en ligne (heredocs) rejetée par le shell à l'analyse ; refaite en fichier de script (ADR-245 D3 le disait déjà) | un aller-retour | rien de nouveau : la règle existe |
| S644–S650 | **les longs calculs bloquaient la session** : la maille fine (23 à 45 min) verrouille le binaire d'essai sous Windows ; aucune compilation possible pendant ce temps, six fois en sept sessions | des heures d'attente | **D1** |
| S648–S649 | la mesure du découpage de la planète faite pendant l'attente de S648, avant le plan de S649 ; déclarée, et ses critères de décision écrits avant d'être rejouée | aucun, déclaré | **D2** |
| S650 | un critère tenu à sa limite (0,150 m pour une borne de 0,15 m, au 10⁻⁷ près) — rapporté comme tel | — | rien à changer : ADR-256 D1 vaut pour les tolérances, une borne écrite avant reste la borne |
| S647–S650 | les deux lecteurs nouveaux (retournement, air enfermé) éprouvés sur des cas posés avant de juger (ADR-263 D2) ; réemployés tels quels en S650 | — | la protection a servi |

## 2. Décisions

**D1 — Un long calcul tourne sur une copie du binaire d'essai.** Avant un essai de plus de cinq minutes, la session copie le binaire
(`code/target/release/deps/water_core-*.exe`) dans son carnet et l'y lance (`<copie> <filtre> --ignored --nocapture`). La compilation
reste libre pendant le calcul ; la session avance sur autre chose. Éprouvé en S651 (le lecteur de S647, rejoué depuis la copie).

**D2 — Le travail fait pendant une attente appartient à la session suivante.** Il est permis s'il est exploratoire. La session qui s'en
sert le déclare dans son plan, écrit ses critères avant de le rejouer, et le rejoue.

## 3. La prochaine revue

S656.
