# Vannes et pompes dans V — S372

2026-09-26. Couche V ([ADR-010](../adr/ADR-010-reseau-hydraulique-volumes-finis.md)), décisions
[ADR-199](../adr/ADR-199-vannes-et-pompes-dans-v.md). Liste **5.4**, *absente* jusqu'ici, au front 0. Session de physique
de l'alternance d'ADR-191 D3, choisie à deux maillons parce qu'elle change l'état d'un point. Référence CPU du cœur,
arithmétique entière, machine de référence.

## Reproduire

- Commit de P5 de S372 (`b38c598b`) ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib tests_controles -- --nocapture`
  — une trentaine de secondes de construction, 0,01 s d'essais ; treize essais, lignes attendues :
  - `TRAJECTOIRES_V_S372 empreinte=0xa02de06bc52bfd2e` ;
  - `VANNE_S372 commande=500 vidange=1455.1 s attendu=1456.5 s` (et 1 000 : 727,4 ; 250 : 2 910,6), `reouverture : 7274
    pas contre 7874 pas`, `deversoir rapport=0.249989` ;
  - `POMPE_S372 a_sec t=207.3 s attendu=207.25 s`, `barrage A=249999 ml B=750001 ml`, `similitude plein=4999 ml/s
    moitie=2499 ml/s`, `avarie … empreinte=0xc0ee093528e34036` ;
  - `INSTANTANE_S372 taille=160 octets suite=0x71b3c56c74d1912b restauree=0x71b3c56c74d1912b`.
- La suite du cœur : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core` — 536 + 20 + 2 + 1
  réussis, 14 ignorés.

## En une phrase

Chaque ouverture du graphe V porte désormais une commande entière — une vanne se ferme, une pompe de cale se lance —, la
pompe suit une courbe parabolique contre la hauteur statique, et tout reste exact au millilitre, reproductible au bit et
sauvegardable ; à commande pleine, V est celui d'avant, au bit.

## 1. La commande, et ce qui ne bouge pas

`Opening::control_pm`, de 0 à 1 000, **1 000 par défaut** (ADR-199 D1) : un **état**, posé par l'hôte entre deux pas
depuis un événement de jeu répliqué ; hors de 0..=1 000, le pas est refusé sans rien écrire.

**Critère 1, écrit avant** : à commande pleine, les trajectoires existantes **inchangées au bit**. L'essai hache les
volumes et restes de chaque pas de quatre montages du noyau — C12 (2 000 pas), une chaîne de trois cuves (600), un réseau
mixte qui rejette hors réseau (200), un déversoir (1 000) ; **l'empreinte a été relevée sur le code de S371, avant toute
modification du pas** : `0xa02de06bc52bfd2e`, inchangée après la vanne, puis après la pompe. Les 33 essais V d'avant
passent.

## 2. La vanne

Orifice : section × `c/1 000` sous Torricelli ; déversoir : largeur × `c/1 000` (ADR-199 D2). **Critère 2** : la
vidange de C12 est inversement proportionnelle à la section, `t = (A/(C_d·a·c))·√(2h₀/g)`.

| commande | vidange | analytique | écart |
|---:|---:|---:|---:|
| 1 000 | 727,4 s | 728,3 s | −0,12 % |
| 500 | 1 455,1 s | 1 456,5 s | −0,10 % |
| 250 | 2 910,6 s | 2 913,1 s | −0,08 % |

Fermée : **0 ml** en 10 000 pas, et le reste de l'arête intact. Fermée 60 s après 100 s, puis rouverte : la vidange finit
**600 pas exactement** après celle de la vanne toujours ouverte — la commande n'a pas de mémoire cachée. Déversoir à 250 :
rapport des débits **0,249989**.

## 3. La pompe

`Flow::Pump { max_flow_mlps, shutoff_head_um, outlet_um }` (ADR-199 D3) : `Q = Qmax·√(n² − Δh/H0)`, zéro sinon (clapet),
zéro si la surface amont est sous la prise (à sec), `n = c/1 000`.

| critère | montage | analytique | mesuré |
|---|---|---|---|
| 3 | A (1 m², 1 m) vers B, refoulement libre à 3 m, prise à 0,1 m, 5 l/s, `H0` 10 m ; `t = 2·A·√H0·(√u0 − √u1)/Qmax`, `u = H0 − z_ref + h` | prise dénoyée à **207,25 s** | **207,3 s** (+0,025 %) ; A garde 99 772 ml, 228 ml sous la prise (un pas) |
| 4 | refoulement noyé au fond de B (fond à 2 m), `H0` 2,5 m : équilibre `2 + h_B − h_A = H0` | A **250 000**, B **750 000** ml | **249 999 / 750 001** |
| 4 | hauteur statique nulle, 10 pas | 5 000 et **2 500** ml/s (`n` = ½) | **4 999 et 2 499** |
| 4 | 3 m à monter, `H0` 10 m | pleine vitesse : débite ; mi-vitesse (`n²·H0` = 2,5 m) : rien | idem |
| — | prise au-dessus de la surface | rien | 0 ml en 1 000 pas |

**Critère 5 — un réseau d'avarie** : la mer (100 m²), un compartiment inondé par une brèche, une vanne vers le
compartiment voisin, une pompe de cale qui rejette par-dessus bord, une autre qui refoule dans une citerne sur le pont ;
les commandes changent pendant 6 000 pas (vanne ouverte à 700 après 150 s, pompe de cale de 40 % à 100 % et retour,
brèche fermée à 500 s). **Masse exacte à chaque pas** (rejet compté), volumes bornés, deux exécutions **identiques au
bit** ; chaque arête a servi. Refus atomiques d'une pompe sans hauteur de barrage ou de débit négatif.

## 4. L'instantané WVST version 2

Troisième liste d'écarts : les commandes différentes de l'auteur, comptées dans les quatre octets que la version 1
réservait ; la commande d'auteur entre dans l'empreinte de la base, la configuration comparée l'exclut ; **la version 1
est refusée** (`Version`), sans migration — aucune sauvegarde n'existe hors des essais (ADR-199 D5). **Critère 6** : une
vanne mise à 300 et une pompe lancée à pleine vitesse, capturées après 300 pas (160 octets), restaurées dans une
destination sale : la suite de 1 000 pas est **identique au bit** à celle du graphe jamais sauvegardé ; le **témoin
d'omission** — la même restauration, commandes d'auteur — diverge. Refus : commande égale à celle de l'auteur (non
canonique), 1 001, −1, en-tête version 1 ; aucune destination touchée. Sans écart de commande, la taille est celle de la
version 1. L'essai d'en-tête existant attend `WVST\x02`.

## 5. Limites

- **Réseau ouvert seulement** : le réseau fermé sous pression reste la v2 d'ADR-010 §4 (liste 5.8).
- Le coefficient de débit ne varie pas avec l'ouverture : **à calibrer** par arête. Vanne proportionnelle seulement.
- Pompe : ni pertes de charge, ni temps de montée, ni puissance, ni énergie, ni cavitation ; la courbe parabolique est
  une approximation déclarée (ADR-199 D3). Débit maximal et hauteur de barrage : données d'auteur.
- La commande saute d'un pas à l'autre : une manœuvre progressive est à la charge de l'hôte.
- Prismes et tables +Z dans les essais ; la géométrie orientée (ADR-139) passe par le même calcul de surface, sans essai
  propre à la pompe sous gravité inclinée.
- Aucun consommateur : ni hôte, ni serveur V, ni scène ; déterminisme vérifié sur cette machine seulement (I-03 entre
  plateformes non reçu).
