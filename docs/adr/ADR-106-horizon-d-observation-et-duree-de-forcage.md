# ADR-106 — L'horizon d'observation n'est pas la durée de forçage

Statut : acté, S155, 2026-09-10, délégation technique. Complète ADR-071 et ADR-104 ;
aucune décision antérieure n'est modifiée, aucun ADR réécrit.

## Constat

ADR-071 borne à 16 secondes l'horizon du candidat modal, et le dit explicitement :
« Cette limite est un périmètre de travail **à calibrer par réception**, pas une durée physique
ni un TTL. » Soixante sessions plus tard, la calibration n'avait pas été faite, et la même
valeur borne aussi la fenêtre de contexte de `bound_pressure::Context::new`.

B2 mesure ses bilans à **60 secondes** (ADR-105, S153, S154). Le noyau de pression refuse
au-delà de 16. Le volet sillage de B2 est donc inaccessible, non par un défaut de physique mais
par une constante que personne n'avait mesurée.

La mesure de S155 ([HORIZON-MODAL-S155](../validation/HORIZON-MODAL-S155.md)) établit deux
choses, et la seconde n'était pas attendue.

**Il n'y a pas de mur à 16 secondes.** L'écart au noyau f64 croît continûment ; rien ne
distingue 16 s de 15 ou de 17. La valeur vaut 2^24 microsecondes — un nombre de bits, pas une
seconde.

**Mais l'âge n'est pas gratuit non plus**, contrairement à ce que la lecture du code suggérait :
après extinction, la rotation libre est entière et exacte, et l'on pouvait croire l'erreur plate.
Elle ne l'est pas. L'écart se décompose en deux termes indépendants :

| terme | origine | dépendance au temps | ordre mesuré |
|---|---|---|---|
| phase spatiale et amplitude | `from_distance` par axe, `magnitude` en f32 | **aucune** | 1e-7 à 5e-6 relatif |
| dérive de phase | **omega stocké en f32**, domega/omega de 6,6e-9 à 5,7e-8 | **linéaire**, \|domega\|·t | 8,8e-6 à 64 s (k=6) |

Neutraliser la seule pulsation — un oracle portant exactement le omega f32 du candidat — fait
tomber l'écart d'un facteur 58 pour k=(6,0), de 8,794e-6 à 1,505e-7, et le rend plat. La cause
du terme croissant est donc établie, et elle n'est ni l'horizon, ni les 24 bits du doublement de
`scale_integer`, ni la réduction de phase Q32 : **c'est le type de omega**.

## Décision

**1. Séparer les deux bornes que « 16 s » confondait.**

L'**horizon** est l'âge depuis la naissance auquel un mode accepte encore d'être échantillonné.
La **durée active** est le temps pendant lequel le forçage s'applique. Elles n'empruntent pas le
même chemin numérique — la seconde entre dans l'accumulation f32 de `scale_integer`, la première
seulement dans une rotation entière — et rien n'obligeait à les borner ensemble.

**2. Porter l'horizon à 64 secondes** (64 000 000 µs), dans `modal_pressure::ModalPressure::new`
et dans la fenêtre de contexte de `bound_pressure::Context::new`. Les deux ensemble ou aucune :
l'une sans l'autre ne débloque rien.

**3. Conserver la durée active à 16 secondes au plus.** Ce n'est pas une prudence de principe :
c'est que **B2 n'en a pas besoin**. ADR-104 découpe déjà le mouvement de l'hôte en tronçons,
chacun source distincte avec sa propre naissance. Un sillage de soixante secondes est fait de
tronçons courts qu'il faut pouvoir **observer** soixante secondes plus tard. Ce qui manquait
était l'horizon, jamais la durée.

**4. Écrire le budget de précision au lieu de la constante.** L'horizon de 64 s vaut avec le
budget mesuré ci-dessous ; c'est lui qui justifie la valeur, et c'est lui qu'il faudra rouvrir
si un consommateur demande mieux.

## Budget de précision reçu

Trente couples par point — cinq vecteurs d'onde, six rapports Doppler, 10 Pa, origine
(0,7 ; -0,3), durée active 4 s —, contre l'oracle f64 `PressureMode` selon la convention de S95
(le même f32 de g converti en f64 : on mesure l'implémentation, pas la quantification d'entrée).

| âge | écart élévation | relatif |
|---:|---:|---:|
| 16 s | 1,381e-7 m | 7,37e-6 |
| 32 s | 2,539e-7 m | 1,36e-5 |
| 60 s | 5,931e-7 m | 3,18e-5 |
| 64 s | 6,177e-7 m | 3,31e-5 |

**À 64 secondes, l'erreur relative reste sous 4e-5 sur les modes éprouvés.** L'énergie étant
quadratique en élévation, cela borne l'erreur d'énergie vers 7e-5 relatif — sous le seuil de
1e-4 E0 que B2 emploie depuis S153, mais **sans marge confortable**, et il faut le savoir avant
de lire un bilan de sillage à 60 s.

Les seuils de régression de S95 (2e-7 m, 2e-6 m/s) restent ceux de leur fixture et de sa durée
active. Ils sont **absolus** ; ils mêlent donc amplitude et précision, et c'est ce qui faisait
ressembler la croissance de l'amplitude à une perte de précision quand la durée active
augmentait. Un seuil relatif dirait mieux ce qu'on veut garantir ; ce n'est pas tranché ici.

## Ce que cette décision ne dit pas

- **Elle ne corrige pas la dérive.** Porter la pulsation à une précision supérieure à celle du
  f32 la réduirait d'environ deux ordres de grandeur, à coût nul en exécution puisque la
  conversion a lieu à la préparation. Ce n'est pas fait ici : cela change le noyau, donc le
  condensat de réception de S95, et cela demande sa propre réception. Enregistré en **A213**.
- **Elle n'étend pas la durée active**, bien que la mesure n'y ait rencontré aucun obstacle
  (2,8e-5 relatif à 64 s de forçage). ADR-074 et le format WPRS bornent aussi la durée ; les
  toucher est un autre chantier, et aucun consommateur ne le demande.
- **Elle ne certifie pas un sillage à 60 s.** Elle rend l'observation possible et en annonce le
  prix. Le bilan énergétique d'un sillage prolongé, son domaine de collecte et sa comparaison à
  un oracle restent à faire — c'est la première branche que S154 proposait et que S155 n'a pas
  prise.
- **Elle ne touche ni la conformité interplateforme (I-03), ni l'absence d'allocation (I-06)**,
  ni aucun invariant : seules deux bornes de domaine changent de valeur.

## Note du 2026-09-10 (S155), le jour même — une troisième borne, trouvée par un test

La décision ci-dessus nomme deux constantes. Il y en a **trois** : `pressure_source::Source::new`
valide la fenêtre `settings.start..end` en passant par `Context::from_recipe`, donc la fenêtre
d'admission WPRS a suivi les deux autres sans être citée. C'est bien l'effet voulu — une source
admise doit couvrir la durée qu'on peut observer — mais il n'avait pas été écrit, et c'est un
**test existant qui l'a signalé** en refusant de refuser.

Le bornage de fenêtre annoncé par ADR-074 §« durées positives dans la fenêtre <=16 s » est donc
remplacé ici par 64 s. ADR-074 n'est pas corrigé : il décrivait exactement l'état de son époque.
