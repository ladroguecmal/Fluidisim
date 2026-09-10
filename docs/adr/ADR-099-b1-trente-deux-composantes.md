
# ADR-099 — B1 : trente-deux composantes

- **Statut : actée**, S146, 2026-09-10, autonomie technique S71.
- **Premier banc exécuté du projet.** Onze sont définis depuis S02 ; aucun ne l'avait été.
- **Exécute** [PLAN-BENCHMARK](../validation/PLAN-BENCHMARK.md) §B1, recommandé par
  [BILAN-S69](../registres/BILAN-S69.md) §6.2 puis [BILAN-S145](../registres/BILAN-S145.md) §6.1.
- **Requalifie A187**, traite la part `Hs` de **S64-2**, ouvre **A212**.
- **Mesure :** [BANC-B1-S146](../validation/BANC-B1-S146.md).

## Décision

**`B` est configuré à 32 composantes** — `background::COMPOSANTES_B1`. Les trois critères
mesurables du protocole vont tous dans le même sens, ce qui est rare assez pour être dit :

| critère | 32 composantes | 256 composantes |
|---|---|---|
| coût par échantillon | **1 531 ns** | 12 763 ns — **×8,3** |
| dispersion de `Hs` (12 graines) | **1,09 pt** | 2,23 pt — **×2,0** |
| hauteur vue par un objet ≤ 30 m | référence | identique à 4 % près |

**Le nombre n'est pas un défaut imposé** : `SeaState::components` reste fourni par l'hôte.
Augmenter ce nombre est légitime pour une raison **perceptuelle** — la seule que ce banc n'a pas pu
mesurer. Ce ne l'est pas pour la justesse, qui se dégrade quand il croît.

## Ce que le banc a établi, et qui n'était pas attendu

**Il n'y a pas de biais sur `Hs`** : à toutes les densités, la moyenne des écarts tient dans
±0,42 %. Ce qui croît avec le nombre de composantes, c'est la **dispersion d'une réalisation à
l'autre** — écart-type 1,09 point à 32, 2,23 à 256.

**A187 change donc de nature.** Ses +6,612 % mesurés à 256 composantes n'étaient pas un défaut de
construction : c'était **une réalisation à trois écarts-types**, sur une graine unique. La cause
identifiée en S67 — la contribution croisée de composantes toutes contenues dans un cône de 30° —
explique précisément cela : leur nombre croît, leur indépendance non, et la variance de `Hs` d'un
tirage à l'autre augmente.

> **Augmenter le nombre de composantes ne rend pas la mer plus juste : il la rend moins
> prévisible.**

**Conséquence sur la tolérance.** A187 concluait qu'elle « ne peut pas descendre sous 7 % tant que
la cause est inconnue ». La cause est connue et quantifiée : **la tolérance tenable dépend de N**.
À 32 composantes, ±3 % couvre 2,7 σ ; à 256, 1,3 σ seulement. C'est un argument de plus pour un
nombre bas, et il ne vient pas du coût.

## Ce que cette décision ne porte pas

**Ni la forme du spectre.** `configure` répartit l'énergie uniformément dans la bande `[Tp/2, 2Tp]`,
et le code renvoyait cette grossièreté à B1. **Le renvoi était faux** : §B1 mesure le nombre et le
coût, jamais la répartition. C'est **A212**, et c'est exactement le mécanisme de L217 — un renvoi
non vérifié ferme la question au lieu de la laisser ouverte. Le commentaire est corrigé.

**Ni les volets perceptuels.** L'évaluation subjective en double aveugle sur trois états de mer et
la distance de perception d'une tuile FFT demandent des personnes ; la seconde demande en plus une
tuile FFT, qui n'existe pas — le fond est une somme de Gerstner. Ils restent **ouverts par
nécessité**, pas par choix.

**Ni le LOD spectral**, qui n'existe pas dans le code. Le protocole demandait le coût « avec LOD
actif et inactif » : ce volet n'a pas été mesuré parce qu'il n'y avait rien à activer. La courbe
`composantes effectives = f(taille)` que le banc produit est ce sur quoi un LOD futur s'appuiera —
et elle dit que pour tout objet du jeu, il n'y a rien à dégrader.

## Réception

275 tests inchangés, cinq ignorés ; aucun code de calcul modifié — la constante et la note sont
additives. Le banc est `code/water-core/examples/banc_b1.rs`, rejouable.

**Le rapport dit lui-même ce qu'il ne tranche pas**, et c'est délibéré : deux volets sur quatre
sont hors de portée. Une session qui lirait « B1 fait » sans cette réserve porterait un renvoi
faux de plus.
