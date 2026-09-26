# ADR-202 — Le niveau de détail des contenants : V par défaut, effets factices au loin, δ à proximité

- **Statut : actée**, S376, 2026-09-26, **décision de l'utilisateur** (réponse à R27 et précision de conception).
- **Corrige** [ADR-200](ADR-200-la-dynamique-des-contenants-en-3d-volumetrique.md) D1, écrit trop large en S374 : *« dans un
  contenant qu'on voit ou avec lequel on interagit »*, le mouvement de l'eau devait être celui de δ. **Un contenant qu'on
  voit de loin n'est pas calculé par δ.** ADR-200 D2 (V garde la masse), D3 (APIC pour la lame et le jet) et D4 (l'ordre)
  restent ; D4 n'est plus pressé : δ sur GPU dans Godot, *« je ne pense pas qu'il faut le faire maintenant »* (R27).
- **Confirme**, sans les réécrire : sources `systeme_eau_architecture_globale` §2.1–2.2 (le détail local ne se déclenche
  que si l'interaction compte ; pluie intégrée par moyenne ou précalcul ; abris qui l'empêchent), ADR-010 §5–6 (pluie,
  `sky_exposure`, domaine δ demandé par un nœud), ADR-012 et ADR-013 (ordonnanceur, prédiction, activation), ADR-025 (la
  masse au nœud), ADR-197 D5 (la météo à la fin).

## 1. Les mots de l'utilisateur

> Pour les zones de cuves où le volume est quantifié approximativement en fonction de la quantité, le volume/l'eau est une
> simulation réaliste 3D volumétrique [seulement] […] si le joueur ou un élément perturbateur [est présent]. La piscine
> n'est pas dans une simulation coûteuse 3D volumétrique mais comme le principe de haute mer, puis se divise ensuite en
> fonction de la taille du volume/cuve en zone de simulation 3D volumétrique si le joueur ou […] un élément perturbateur.
> […] Comme pour les océans et mers, le système de prévisions et LOD de simulation doit être présent pour alléger les
> calculs inutiles.

## 2. Décisions

**D1 — Trois niveaux pour un contenant, comme pour la haute mer.** (a) **V seul**, toujours : le volume, les débits, la
masse autoritaire (ADR-010, ADR-199). (b) **Vu, sans perturbateur proche** : la surface plane à la cote de V, et des
**effets factices** bon marché — les rides de la pluie selon son intensité, réglées par la distance à l'observateur. Aucun
δ. (c) **Un joueur ou un élément perturbateur à proximité** — ou **prévu** (ADR-013) — : un domaine **δ 3D volumétrique**,
qui couvre le contenant entier s'il est petit, **une zone** autour de la perturbation s'il est grand ; V garde la masse
(ADR-025).

**D2 — La prévision et l'ordonnanceur valent pour les contenants.** Le domaine δ d'un contenant est un candidat de
l'ordonnanceur (ADR-012, porte A) comme ceux de la mer ; il est préparé **avant** l'arrivée prévue du perturbateur (ADR-013,
liste 9.2), et rendu à V quand plus rien ne le justifie.

**D3 — La pluie : des litres pour V, des rides factices pour l'image.** La météo, **calculée en amont**, donne pour chaque
contenant les litres qui le rejoignent (ADR-010 §5 : intensité × surface libre × exposition au ciel) ; un élément qui
bloque l'accès de l'eau au contenant la réduit ou l'annule. **Les impacts de pluie ne sont jamais simulés**, même quand le
contenant est en δ (le joueur qui se baigne sous la pluie) : trop coûteux ; ils restent un effet de rendu. La masse de pluie
entre par V, et δ la reçoit par le forçage (ADR-025).

**D4 — La météo n'est pas calculée en temps réel.** Elle est préparée en amont — *« par le serveur ? »* — et donne, dans le
temps, la chaleur et les précipitations par position ; elle ne gère pas forcément la position de chaque nuage ni chaque
éclair (*« à réfléchir »* : §3). Sa construction reste **à la fin** (ADR-197 D5).

**D5 — Le débordement vu par le joueur est une vraie simulation.** La lame d'un déversoir (piscine à débordement ou autre)
est simulée en 3D volumétrique : si elle touche un obstacle, elle y réagit réellement, comme si le joueur venait la
déranger. Elle relève d'APIC (ADR-200 D3). La portée de « vu » est une zone d'ombre (§3, question 3).

**D6 — Le remplissage suit la filtration.** Selon le système de filtration — pompe, skimmer, trop-plein, déversoir : les
arêtes de V (ADR-199) —, la piscine se remplit sous la pluie ou garde son niveau ; c'est V qui le calcule.

## 3. Zones d'ombre — posées à l'utilisateur le 2026-09-26, avec une proposition chacune

1. **Qui calcule la météo, et comment V reçoit la même pluie partout.** V est déterministe et répliqué (I-03, I-10) : les
   litres de pluie doivent être les mêmes pour tous. *Proposition* : une chronologie météo **générée par le serveur** (ou
   déterministe depuis une graine et un calendrier, comme B depuis `T_sim`), publiée par régions en paramètres lents
   (comme `HydroSample`) ; V intègre la pluie comme une source d'arête, identique partout.
2. **Les éléments bloquants qui changent en jeu.** `sky_exposure` est précalculée hors ligne (ADR-010 §5) ; une bâche posée,
   un toit détruit la changent en cours de partie. *Proposition* : exposition précalculée pour le décor, plus une liste
   répliquée d'occultants mobiles, recalculée à leur changement. *À savoir* : le joueur peut-il couvrir ou découvrir un
   contenant ?
3. **La lame du débordement, vue de loin.** « Le joueur voit le débordement comme une véritable simulation » : toujours,
   dès qu'elle est visible, ou seulement à proximité, avec une lame légère (animée depuis le débit de V) au loin ?
   *Proposition* : vraie simulation dans une portée proportionnelle à la taille apparente de la lame ; au-delà, la lame
   légère, sans réaction aux obstacles.
4. **Contenant entier ou zone : le seuil.** *Proposition* : le contenant entier en δ si sa plus grande dimension tient dans
   un domaine (l'ordre de 10 à 20 m à 5–10 cm de maille) ; au-delà, une fenêtre autour de la perturbation, comme la mer.
5. **Nuages et éclairs.** *Proposition* : la météo autoritaire ne porte que des champs régionaux lents (précipitations,
   température, vent) ; nuages et éclairs en sont dérivés localement, pour l'image seulement (comme δ, I-04) — **sauf** si un
   éclair a une conséquence de jeu (dégâts, feu) : alors un événement du serveur. *À savoir* : la météo a-t-elle des
   conséquences de jeu au-delà de l'eau ?
6. **La chaleur et l'eau.** « La position de la chaleur » : l'évaporation qui fait baisser une piscine un jour chaud, le gel
   d'un bassin (ADR-017) ? *Proposition* : oui, par V (un puits d'évaporation selon température et vent), à la fin avec la
   météo.
7. **Les effets factices au loin : jusqu'où ?** Les rides de la pluie, et aussi un vent qui ride un bassin, les feuilles, un
   objet qui tombe vu de loin ? *Proposition* : tout ce qui n'a pas de conséquence et se voit de loin est factice et réglé
   par la distance ; tout perturbateur **physique** (objet, joueur, lame qui heurte) déclenche δ.

## 4. Ce qui ne change pas

V autoritaire et répliqué ; le serveur n'exécute que V (I-10) ; δ n'a jamais d'autorité (I-04) ; la mer inchangée ; rien
n'est retiré du périmètre ; la météo et le son restent à la fin (ADR-197 D5).

> **Note du 2026-09-26 (S377).** Les sept zones d'ombre du §3 ont reçu leurs réponses :
> [ADR-203](ADR-203-reponses-aux-zones-d-ombre-d-adr-202.md) — météo aussi poussée que l'eau, son autorité déléguée sous
> « réaliste et performant » ; couvertures (bâche entière ou demi) posées et retirées en temps réel ; lame et obstacles
> factices de loin ; contenants divisés en domaines, sans seuil unique ; ordre des systèmes suivants (météo, topologie
> d'un territoire, feu, neige) ; chaleur et gel par V ; le factice pour tout ce qui est sans conséquence et lointain.

