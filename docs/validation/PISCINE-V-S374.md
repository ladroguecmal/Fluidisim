# La piscine de V dans Godot — S374

2026-09-26. Demande de l'utilisateur : *« commence à permettre de visualiser le système de piscine avec déversoir et
pompe »*. Couche V ([ADR-010](../adr/ADR-010-reseau-hydraulique-volumes-finis.md), [ADR-199](../adr/ADR-199-vannes-et-pompes-dans-v.md)),
rendue dans Godot ([ADR-192](../adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md)). **Pendant la session**, décision de
l'utilisateur : *« La dynamique de fluide doit se faire en 3D volumétrique »* — [ADR-200](../adr/ADR-200-la-dynamique-des-contenants-en-3d-volumetrique.md) ;
ce document est l'**état de départ** de ce chantier : V calcule la masse et les débits, Godot les montre ; aucune
dynamique n'est encore dessinée.

## Reproduire

- Commit de P4 de S374 ou plus récent ; Godot 4.4.1 (`<godot>`).
- **V** : `cargo run --manifest-path code/Cargo.toml --release --offline -p water-core --example piscine_v` — une seconde
  après la construction ; lignes `PISCINE_V_S374` : `critere=1 pas=3300 ecart_volume_max_ml=0 tenu`, `critere=2
  deversoir_ls=10.0717 pompe_ls=10.0926 analytique_ls=10.0924 … charge_m=0.01265 analytique_m=0.01266 … tenu` ; écrit
  `godot/donnees/piscine_v.json` (3 301 lignes, dérivé, non versionné).
- **Le contrôle du rendu** : `<godot> --path godot res://piscine.tscn --quit-after 3000 -- --controle-piscine` — quelques
  secondes, `CONTROLE_PISCINE_S374 critere=3 pire_m=3.6438e-08 tenu`.
- **Les images** : `VUES=ensemble,deversoir INSTANTS=2,40,200,260 <godot> --path godot res://piscine.tscn --quit-after
  5000 -- --captures` ; copies dans `viewer/captures/s374/`. La vue animée : sans `--`, touches 1 à 3, espace, Échap.
- **Toujours `--quit-after`** : une erreur d'analyse du script laisse Godot ouvert sur une scène vide.

## En une phrase

Une piscine à débordement — bassin de 8 × 4 m, déversoir de 4 m, bac tampon, pompe de 12 l/s — calculée par V au
millilitre, dont le régime établi tombe à 0,2 % du point de fonctionnement analytique, et que Godot rejoue en montrant
exactement les surfaces que V publie ; la dynamique de l'eau reste à faire, en 3D volumétrique (ADR-200).

## 1. La piscine dans V

`code/water-core/examples/piscine_v.rs`. Bassin 8 × 4 × 1,5 m (fond à 0), déversoir de 4 m sur le petit côté est, seuil à
1,40 m ; bac tampon 1 × 4 × 1,2 m contre la face extérieure du mur est, fond à −1,3 m ; pompe de 12 l/s, hauteur de
barrage 8 m, prise à 5 cm du fond du bac, refoulement libre à 1,7 m au-dessus du mur ouest. Départ : bassin 5 mm sous le
seuil, bac à 0,80 m ; la pompe tourne de 5 à 240 s ; 330 s. V publie à chaque pas les **surfaces**
(`Shapes::surface_plane`, I-01 : le consommateur ne reconstruit rien), les volumes, les débits d'arête, la commande.

**Critères, écrits avant.**

| critère | mesure | résultat |
|---|---|---|
| 1 — volume du circuit fermé | écart maximal sur 3 300 pas | **0 ml** |
| 2 — régime établi (moyenne de 180 à 240 s) contre le point fixe analytique (loi des trois demis, hauteur statique de la pompe, volume total) | déversoir / pompe / analytique | **10,0717 / 10,0926 / 10,0924 l/s** (−0,21 %, +0,001 %) |
| 2 | charge sur le seuil | **12,65 mm** pour 12,66 (−0,04 %) ; bac tampon à 0,659 m |

Le déversoir est encore 0,2 % sous la pompe à 240 s : le bassin monte toujours, de l'ordre du dixième de millimètre par
minute (constante de temps ≈ 27 s, `A/(dQ/dH)`).

## 2. Godot la rejoue

`godot/piscine.tscn`, `piscine.gd` : les pas de V interpolés à l'image ; trois vues, pause, indications (surface par
rapport au seuil, hauteur du bac, débits, état de la pompe). `bassin.gdshader` : l'optique de la mer ramenée au bac
(Fresnel exact, ciel de la scène, éclat du soleil, fond et parois par réfraction de Snell, colonne d'eau pure de
Maritorena), et des rides d'habillage. `paroi.gdshader` : parois éclairées par le modèle du fond de S359, dans les mêmes
unités que la lumière de l'eau.

**Critère 3, écrit avant** : la surface rendue de chaque bac contre celle que V publie, au dixième de millimètre —
six instants, sur un pas et entre deux : **3,6·10⁻⁸ m** au pire (l'arrondi f32 de la position).

Images : `viewer/captures/s374/piscine_{ensemble,deversoir}_{002,040,200,260}s.png` — la pompe arrêtée, le débordement qui
s'installe, le régime établi, la pompe arrêtée depuis 20 s. On y voit l'eau des deux bacs à la cote de V, le carrelage
à travers le bassin ; **ni lame, ni jet, ni mouvement** de l'eau.

## 3. Ce qui a été écarté, et pourquoi

P4 devait dessiner la lame du déversoir (hauteur critique `(q²/g)^⅓`, chute libre, épaisseur `q/v`) et le jet de la buse
(vitesse `Q/A`, section par continuité) — un habillage cinématique tiré des débits de V. **La décision de l'utilisateur
l'écarte** comme dynamique (ADR-200 D1) : le mouvement de l'eau des contenants se calcule par δ en 3D, V gardant la masse.
Le code préparé n'est pas entré dans le dépôt.

## 4. Limites

- Aucune dynamique : les surfaces sont planes à la cote de V ; les rides sont un habillage.
- Rejeu d'un scénario calculé d'avance : pas de commande en direct (ADR-200 D4).
- La lame et le jet ne sont pas dessinés : ils attendent APIC en 3D (ADR-200 D3).
- Un seul scénario ; géométrie en prismes (tables +Z).
