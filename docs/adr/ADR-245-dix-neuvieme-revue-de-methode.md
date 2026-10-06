# ADR-245 — Dix-neuvième revue de méthode (S571–S575)

- **Statut : actée**, S576, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-244](ADR-244-dix-huitieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un état arrondi pris comme départ du pas suivant** (S575 : le pilote de C15 repartait chaque heure de l'épaisseur de glace quantifiée ; la troncature, jusqu'à un quantum par pas, s'accumulait — 2,4 mm en 720 pas, un critère manqué). V le savait depuis ADR-010 §4 (le report de reste des arêtes) ; aucune protection ne le disait hors de V | une mesure refaite | **protection nouvelle** (D1) |
| **Deux expressions d'un même seuil** (S573 : la praticabilité d'un bateau `d − tirant − marge > 0`, sa prévision `d ≥ tirant + marge` ; en f32 elles divergent de 4,5·10⁻⁸ et se contredisaient à la borne) | un défaut réel, trouvé par l'essai de borne | **protection élargie** (D2) |
| **Des scripts encore écrits par heredoc** (S562, S566, S574), contre la lettre d'ADR-240 D1 ; entre apostrophes droites (`'EOF'`), sans dommage. Ce qui a cassé en S520 et S555, c'était une chaîne entre apostrophes en ligne de commande | aucun | **la règle corrigée** (D3) : une règle violée sans dommage est mal écrite |
| La vérification de route passée d'abord par les notes (S574, ADR-244 D1) ; le script du plan qui vérifie une table avant d'écrire (S574) ; les nombres du plan écrits par le script (S572–S575) | — | **rien à changer** |

## 2. Décisions

**D1 — Un état quantifié ne sert jamais de départ au pas suivant.** Millilitres, quanta, valeurs publiées en f32 ou en `half` : l'état
exact (un cumul, un reste) se garde à part, et le quantifié se dérive de lui à chaque pas (généralise ADR-010 §4).

**D2 — Un seuil calculé vit dans une seule fonction, que tous ses usages lisent** (une décision, une prévision, une publication) — sans
quoi deux arrondis le font diverger à sa borne (élargit L137).

**D3 — ADR-240 D1, corrigé** : un heredoc entre apostrophes droites (`<<'EOF'`) est permis pour écrire un fichier ; ce qui reste interdit,
c'est une chaîne entre apostrophes en ligne de commande (`-c '…'`, `printf '…'`) qui contient du français — l'apostrophe la ferme.

## 3. La prochaine revue

S581.
