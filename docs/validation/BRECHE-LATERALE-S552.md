# Un navire gîte par sa brèche — S552 (liste 6.6)

*S552, 2026-10-06, en autonomie.* S548 (une barge envahie) et S549 (la carène libre) ensemble : une citerne latérale s'envahit par une
brèche, la coque gîte vers elle et s'enfonce — l'eau entre sous une pesanteur que la gîte incline, et pèse là où elle se tient.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s552 -- --nocapture` ; suite du cœur : 711.

## 1. Le montage (aucun code neuf)

La barge de S548 (20 × 8 × 4 m, 246 t, proxy 4 × 32 × 16) ; une citerne latérale de V à tribord (20 × 0,5 × 4 m, forme volumique), une
brèche de 0,1 m² à son fond ; la mer vue du navire, une forme volumique de 400 × 400 × 20 m centrée sur la verticale de la brèche, replacée
à chaque pas sur la surface du monde ; V sous la pesanteur du navire `Rᵀ·(0, 0, −g)` ; l'eau de la citerne pèse en son centre mouillé
(S549).

## 2. La référence, indépendante (ADR-239 D1)

La flottabilité perdue en section, intégrée numériquement hors de l'essai : la section intacte (`y` ∈ [−4 ; 3,5] m) sous la flottaison
inclinée, la force et le moment résolus par bisection — gîte **8,088°**, tirant au centre (le long de l'axe du navire) **1,6355 m**, eau dans
la citerne **21,68 m³**. Le même calcul montre qu'une citerne de 2 m n'aurait aucun équilibre avant 40°.

## 3. Mesuré

| | mesuré | référence |
|---|---|---|
| gîte | **8,101°** | 8,088° (0,16 %) |
| tirant au centre | **1,6192 m** (profondeur verticale de la quille) | 1,6355 m le long de l'axe (0,997 %) ; la verticale, `1,6355·cos θ` = 1,6193 m (0,01 %) |
| eau dans la citerne | **21,689 m³** | 21,68 m³ (0,04 %) |
| masse de V | exacte à chaque pas | — |

## 4. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) la gîte à 3 % | 0,16 % | tenu |
| (2) le tirant au centre à 1 % ; l'eau à 2 % | 0,997 % (un écart de définition, `cos θ` : 0,01 % à définition égale) ; 0,04 % | tenu |
| (3) la masse de V exacte | exacte | tenu |

## 5. Ce que cela dit

Le corps rigide, V sous une pesanteur inclinée par la gîte et le centre mouillé couplés : la coque gîte et s'enfonce jusqu'à l'équilibre de
la théorie navale, la citerne pleine jusqu'à la flottaison. **6.6 avance.** Manquent la dynamique de l'eau qui court dans un compartiment,
plusieurs compartiments et leurs cloisons, le chavirement au-delà du pont mouillé, la poche d'air porteuse, un navire réel et les brèches en
jeu.
