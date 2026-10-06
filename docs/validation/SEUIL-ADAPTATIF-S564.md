# Le seuil adaptatif à l'échelle du contenant — S564 (liste 5.6)

*S564, 2026-10-06, en autonomie.* Intentions d'origine §2.2 : « les très petites variations peuvent être ignorées par un seuil adaptatif ;
une quantité significative dans un bidon peut être négligeable dans une piscine ».

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s564 -- --nocapture` ; suite du cœur : 730.

## 1. Ce qui est construit

`hydro_seuil.rs`, sous-module de V : `ecart_hauteur_um(nœud, volume_publié)` — l'écart de hauteur de surface le long de la verticale
locale, différence des plans de la géométrie pour les deux volumes ; `changement_significatif(…, seuil_um)`. **Le seuil est une hauteur** :
ce que voient le rendu et le joueur, propre à chaque contenant sans réglage. **La référence est le dernier état publié**, pas le pas
précédent : une fuite lente s'accumule et finit publiée. Rien n'est retiré de la masse (I-10) — le seuil décide de ce qu'on montre et
transmet, pas de ce qui existe.

## 2. Mesuré (seuil 500 µm)

| cas | référence (plan) | mesuré | verdict |
|---|---|---|---|
| bidon 0,2 × 0,1 m, +1 L | 50 000 µm | 50 000,0000 µm | significatif |
| piscine 10 × 5 m, +1 L | 20 µm | 20,0000 µm | ignoré |
| piscine, +25 L | 500 µm | 500,0000 µm | significatif (à égalité) |
| carène en V à 1 m³, +0,99 L | 494,8775 µm | 494,8775 µm | ignoré |
| carène en V, +1,01 L | 504,8726 µm | 504,8726 µm | significatif |
| piscine sous `g_eff = (1 ; 0 ; −9,759)`, +25 L | `500·cos θ` = 497,3955 µm | 497,3955 µm | ignoré |
| fuite de 1 L par pas dans la piscine, 100 pas | publiée tous les 25 pas | pas 25, 50, 75, 100 ; la masse exacte à chaque pas | — |

Critères (écrits avant) : (1) à 1 µm et le verdict — **tenus** (au dix-millième) ; (2) tous les 25 pas, la masse jamais touchée —
**tenu** ; (3) les refus (volume publié hors de la capacité, seuil négatif) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Le critère d'importance d'un changement de V existe et s'adapte seul au contenant, à sa forme et à la gravité. **Il n'a pas encore de
consommateur** : la réplication des volumes (10.x) et leur rendu le liront ; c'est là que le seuil choisi (500 µm ici) se règle sur le
perçu.
