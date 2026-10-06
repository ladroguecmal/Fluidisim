# Le sillage de la coque dans δ contre la théorie de sa coque — S520 (liste 4.13)

*S520, 2026-10-06, en autonomie.* S519 a éprouvé un instrument d'angle (le bord d'Airy) sur la théorie et sur W. S520 l'applique à la coque
de la porte D dans δ, sur la carte, à 4–6 λ₀ — un domaine de 1,49 M mailles, abordable depuis S518.

## Reproduire

- `python outils/reference_sillage.py coque` — la théorie de la coque (pression `ρ g d` sur son empreinte de 4 × 1,6 m), trois grilles.
- `python outils/calcul.py lancer sillage-delta SILLAGE_NX=416 SILLAGE_NY=224 DUREE=30 VITESSE=3 RAMPE=3 SORTIE=calculs/delta_s520.bin --
  viewer/target/release/water-viewer.exe --lineaire-sillage` (≈ 75 s), puis `python outils/reference_sillage.py delta calculs/delta_s520.bin`.

## 1. La théorie de la coque

La réponse linéaire exacte en temps (S519) à la pression hydrostatique de la coque sur son empreinte, coupée au nombre d'onde de Nyquist de
δ. **Le bord d'Airy y lit 16,40 / 16,40 / 16,36°** sur trois grilles à 3 m/s (par λ₀ : 16,1–16,6°, stable). Balayée en vitesse : 19,1°
à 1,5 m/s, 14,4° à 2, 21,7° à 2,5, 16,4° à 3, 19,0° à 4 — l'angle lu sur une coque de 4 m varie avec L/λ₀. Le critère (1), « 19,47° à
1° sur la théorie », est donc **manqué** à 3 m/s ; un critère nouveau a été écrit et committé **avant** de lancer δ : (2') δ à 1° de la
théorie de sa coque (16,40°).

## 2. La coque dans δ

104 × 56 × 4 m (416 × 224 × 16 mailles de 25 cm), 30 s à 3 m/s (rampe de 3 s), 3 000 pas. **Un défaut levé en route** : à 1,49 M mailles
les noyaux de la carte indexés par face demandaient 71 504 groupes, au-delà de la limite de 65 535 de wgpu (l'exécution s'arrêtait) ; la
prédiction, la correction, les faces du mouvement et la dispersion passent en deux dimensions au-delà de la limite (en deçà, une rangée :
le banc du sillage de S518 rend la même empreinte de η, `e41630abd739b189` ; S503, S504 inchangés), les autres noyaux le vérifient.

| | mesuré |
|---|---|
| élévation maximale (sur l'étrave) / à mi-parcours | 0,957 / 0,715 m — bornée, finie partout |
| coût par pas : recoupage + extraction + envoi / pas complet | 3,7 ms / **23,3 ms** (1,49 M mailles, 60 cycles) |
| le bord d'Airy, fenêtre 4–6 λ₀ | **20,56°** (la théorie de la coque : 16,40° ; Kelvin : 19,47°) |
| par fenêtre d'1 λ₀, de 2 à 7 λ₀ | 23,0 / 22,7 / 21,2 / 20,0 / 15,0° — instable |
| moyenne de \|η\| le long des rayons, δ / théorie | 13° : 0,067 / 0,190 ; 15° : 0,054 / 0,236 ; 17° : 0,062 / 0,111 ; 21° : 0,039 / 0,130 ; 23° : 0,022 / 0,085 ; 25° : 0,012 / 0,043 ; 27° : 0,006 / 0,019 |

## 3. Les critères

| critère | | |
|---|---|---|
| (1) sur la théorie de la coque, l'instrument lit 19,47° à 1° | 16,40° | **manqué** |
| (2') le bord d'Airy sur δ à 1° de la théorie de sa coque | 20,56° contre 16,40° | **manqué** |
| (3) borné, fini ; le coût publié | 0,96 m ; 23,3 ms par pas | tenu |

## 4. Ce que cela dit — et ce que cela ne dit pas

Le diagnostic, après coup et sans nouveau critère : **sur la théorie de la coque, le bord d'Airy lit un creux d'interférence** (le profil
tombe de 0,236 à 15° à 0,111 à 17°, puis remonte à 0,164 à 19°) et non la décroissance extérieure ; sa stabilité par λ₀ ne prouvait pas
qu'il lisait ce qu'on voulait. Éprouvé sur une source gaussienne (un profil à un lobe), il ne l'est pas sur une coque (un profil à
plusieurs lobes). Sur la décroissance extérieure (21–27°), δ et la théorie ont la même forme (rapport 3,2–3,9), mais **δ est 3,5 fois
moins ample** et ses lobes intérieurs diffèrent. Ni le modèle de référence (une pression sur l'empreinte n'est pas un corps qui perce la
surface) ni δ ne sont désignés par cette mesure : **A330**. 4.13 garde « le sillage mesuré » parmi ce qui manque ; la carte tient
désormais 1,49 M mailles et 30 s.
