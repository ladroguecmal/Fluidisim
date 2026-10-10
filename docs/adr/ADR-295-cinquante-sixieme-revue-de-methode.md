# ADR-295 — Cinquante-sixième revue de méthode (S756–S760)

- **Statut : actée**, S761, 2026-10-10 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-293](ADR-293-cinquante-cinquieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S757–S760 | le bilan avant le remède (ADR-290 D1) a choisi entre deux hypothèses avant tout essai (S759) ; la référence extérieure avant la décision (ADR-293 D1) a précédé ADR-294 | — | elles ont tenu |
| S758 | **une règle de vitesse réfutée en une session** : `|v|² ← |v|² − 2g·Δz` à chaque déplacement de la projection. Écrite cas par cas, elle se jugeait d'avance : les déplacements vont vers le bas autant que vers le haut, et la règle accélère ceux qui descendent | une session | **D1** |
| S759 | **une borne d'un seul côté, appliquée à chaque pas** (la projection ne rend que la perte du pas) : un cliquet, l'oscillation amortie de 2,7 % par période ; le compte cumulé l'a levé | un essai | **D2** |
| S759 | `fermer.py` laissait `BOUSSOLE.md` hors du commit de fermeture | un commit de rattrapage | corrigé en S760 (ses chemins) |
| S760 | **un script de plan en échec, suivi d'un `git add -A` sur une autre ligne** : un état étranger commis sous le titre du plan (corrigé avant la poussée) | un commit à reprendre | **D3** |

## 2. Décisions

**D1 — Une règle de correction s'écrit par ses cas signés avant le calcul** : ce qu'elle fait au repos, dans un sens, dans l'autre (vers le
haut et vers le bas, un gain et une perte). Un cas qui contredit l'hypothèse écarte la règle sans calcul. C'est ADR-290 D3 (la cible
vérifiée statiquement), étendue des cibles aux règles.

**D2 — Une correction de conservation tient un compte cumulé, jamais une borne pas à pas.** Une borne d'un seul côté, appliquée à chaque
pas, oublie le reste quand elle n'est pas atteinte et rogne quand elle l'est : un cliquet, qui dissipe ou injecte même quand le bilan
moyen est juste.

**D3 — Un commit d'étape passe par `outils/etape.py`** (`--script`, `--message`, `--pousser`) : le script de l'étape, puis le commit,
rien si le script échoue. C'est le pendant de `fermer.py` pour les étapes.

## 3. La prochaine revue

S766.
