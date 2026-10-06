# Le consommateur des impacts prédits, confirmés ou rejetés — S510 (liste 9.5, validée)

*S510, 2026-10-06, en autonomie.* Le journal des impacts ([ADR-056](../adr/ADR-056-cause-et-journal-impact.md)) tient les causes —
prédites par le client, confirmées ou rejetées par le serveur — et rend `Change::Retract`, la prédiction à annuler. Rien ne le consommait :
la composition ne lit que les confirmés.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s510 -- --nocapture` — quatre essais ; suite du cœur :
  669 essais.

## 1. La construction

`code/water-core/src/wave_consumer.rs` — pour l'**image** (le jeu ne lit que les confirmés, I-04), dans des emplacements prêtés par l'hôte
(I-06), sans écriture sur un refus :

- une prédiction s'affiche dès son admission (en fondu si son impact est déjà né) ;
- une confirmation au **même effet visible** (tout l'impact sauf son identifiant et son origine) prend sa place sans rien changer ;
- une confirmation **corrigée** fond enchaîné de la prédiction au confirmé sur 0,5 s (`smoothstep`) ;
- un **rejet** éteint la prédiction en fondu sur 0,5 s (ou la retire, si son impact n'est pas encore né) ;
- chaque impact garde sa naissance : **aucun retour du temps**. L'image est `Σ wᵢ·ηᵢ(t)` ; l'hôte multiplie chaque impact par son poids.

## 2. Mesuré (un impact de 100 J, λ = 4 m, au point situé à 5 m, 60 images/s)

| cas | mesuré |
|---|---|
| confirmation au même effet (0,5 s après la prédiction) | l'image **au bit** de celle du seul confirmé, sur les 150 images |
| rejet à 1 s : saut d'une image dû au retrait | **1,56 %** de l'amplitude au point (5,9 mm) ; nul après le fondu ; la couche retirée |
| témoin : retrait sec au même instant | 8,6 % — la valeur de `η` à cet instant ; au plus 100 % sur une crête |
| confirmation 0,5 m plus loin, à 1 s : saut dû au fondu enchaîné | **2,28 %** ; après le fondu, l'image au bit du seul confirmé |
| un emplacement, une seconde cause | refusée (`Full`), rien d'écrit ; prédiction tardive et rejet sans affichage : sans effet |

La prévision du plan pour le témoin (« ≈ 100 % ») supposait un rejet sur une crête ; le saut d'un retrait sec vaut `|η|` à l'instant du
rejet. La borne du fondu, elle, ne dépend pas de l'instant : `1,5·Δt/τ` = 5 %.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) confirmation au même effet : l'image au bit avant et après, au bit du seul confirmé | au bit | tenu |
| (2) rejet : ≤ 5 % ; nul après `τ` | 1,56 % ; nul | tenu |
| (3) confirmation corrigée : ≤ 5 % ; après `τ` au bit du confirmé | 2,28 % ; au bit | tenu |
| (4) l'âge suit l'horloge ; capacité bornée, refus sans écriture | tenu | tenu |

**9.5 validée** — événement prédit, confirmé ou rétracté, sans retour arrière du temps. Hors du point : le transport réseau des causes
(10.1, 3.7). La liste : **9 points validés sur 120**.
