# ADR-170 — Les trois poids de priorité sont bornés à [0,1], et le score est leur produit

Actée S278, 2026-09-18, autonomie S71. Précise ADR-012 §2 et ADR-013 §5 ; ne change aucune
décision antérieure.

## Constat

ADR-012 §2 pose la priorité d'un candidat comme `P = W_gameplay · W_perception · W_urgence`.
ADR-013 §5 pose le score d'activation comme « `s ∈ [0,1]`, composé selon ADR-012 §2, avec
hystérésis 0,60 / 0,40 ».

**Les deux ne se raccordent pas.** Rien dans ADR-012 ne borne les trois poids, et `W_urgence` est
explicitement `1/(temps avant que l'absence du domaine devienne visible)` — qui diverge quand ce
temps tend vers zéro. Le produit n'est donc pas dans `[0,1]`, et les seuils 0,60 / 0,40 d'ADR-013
n'ont pas de sens tant que le domaine de `P` n'est pas fixé. L'écriture de l'ordonnanceur (S278) a
buté sur cette ambiguïté au premier essai : elle ne se voyait pas tant que personne ne calculait.

## Décision

**Les trois poids sont dans `[0,1]` par contrat, et `s = P`.** Une soumission qui présente un poids
hors de `[0,1]`, non fini ou négatif est refusée, comme l'est déjà un `NaN`.

- `W_gameplay` : 0 quand l'eau n'a de conséquence sur personne, 1 quand elle en a une maximale sur
  un acteur, un objet joueur ou un objectif.
- `W_perception` : fraction d'écran × visibilité × facteur de regard — **déjà** dans `[0,1]` par
  construction, aucun changement.
- `W_urgence` : `min(1, horizon / temps_avant_visible)`, l'horizon étant déclaré par l'hôte. C'est
  la seule des trois qui demandait une normalisation, et c'est celle qui divergeait.

Le score d'activation est alors le produit lui-même, sans facteur d'échelle. Les seuils 0,60 et
0,40 d'ADR-013 §5 s'appliquent tels quels et gardent le sens qu'ils avaient : des fractions.

## Alternatives écartées

| Alternative | Motif de rejet |
|---|---|
| `s = P/(1+P)` | introduit une constante d'échelle invisible dans le profil, et rend les seuils d'ADR-013 dépendants d'elle |
| `s = P/P_max`, `P_max` déclaré au profil | `P_max` est une **capacité dérivée** dans un profil de ressources — exactement l'erreur de nature que la correction S05 (écart R04) a retirée pour `domaines_max` |
| Laisser `P` non borné et comparer les candidats entre eux | le tri par `P/C` fonctionnerait, mais l'activation ne fonctionne plus : un seul candidat présent serait toujours au-dessus ou toujours en dessous du seuil, selon l'échelle du jeu |

## Conséquences

Le tri par `P/C` d'ADR-012 §1 est inchangé : borner les poids ne change aucun ordre relatif.

Ce que la borne déplace, c'est la responsabilité de la normalisation de `W_urgence` : elle passe à
l'hôte, avec son horizon, et devient un nombre qu'on peut relire. Un horizon mal choisi se voit
alors dans les décisions plutôt que de se cacher dans une échelle implicite.

**Non calibré** : les seuils 0,60 / 0,40 restent les valeurs de départ d'ADR-013 §5, « toutes à
calibrer », et le banc B8 n'existe pas. Cet ADR fixe le **domaine** des poids, pas leur réglage.
