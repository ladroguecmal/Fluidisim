# ADR-203 — Réponses aux zones d'ombre d'ADR-202 : la météo, les bâches, les effets factices

- **Statut : actée**, S377, 2026-09-26, **décisions de l'utilisateur**, réponses point par point aux sept questions
  d'[ADR-202](ADR-202-niveau-de-detail-des-contenants.md) §3.
- **Précise** ADR-202 (qui n'est pas réécrit), [ADR-010](ADR-010-reseau-hydraulique-volumes-finis.md) §5 (la pluie et
  `sky_exposure`), [ADR-197](ADR-197-reponses-du-2026-09-26.md) D5 (la météo à la fin).

## 1. Les réponses, et ce qu'elles décident

**D1 — La météo sera aussi poussée que l'eau** (question 1). *« Je ne sais pas ce qui est le mieux, mais la météo sera
aussi poussée que l'eau […] donc un système réaliste et performant. »* Le choix de son autorité — générée par le serveur ou
déterministe depuis une graine — est **délégué au projet**, sous deux critères, réalisme et performances, **au moment où
la météo sera conçue**. Contrainte déjà acquise, qui vaudra alors : V étant déterministe et répliqué (I-03, I-10), les
litres de pluie qu'il reçoit sont les mêmes pour tous. La proposition d'ADR-202 §3.1 reste la piste, non la décision.

**D2 — Les couvertures changent en temps réel, partiellement** (question 2). *« Le calcul peut être fait à l'avance par le
système de prévision des évènements mais devra aussi pouvoir réagir en temps réel, car le joueur a la possibilité de poser
une demi-bâche ou complète et la retirer à tout moment. »* L'exposition au ciel d'un contenant est donc **dynamique et
fractionnaire** : la part précalculée du décor (ADR-010 §5), plus des **occultants mobiles répliqués** — bâche entière,
demi-bâche —, posés et retirés à tout instant, pris en compte au pas de V suivant ; la prévision (ADR-013) anticipe ce qui
est prévisible, le reste se recalcule à l'événement. Fait de jeu acquis : **le joueur peut couvrir et découvrir un
contenant, entièrement ou en partie.**

**D3 — La lame du débordement, de loin : factice** (question 3). La proposition est retenue : vraie simulation dans une
portée proportionnelle à sa taille apparente ; au-delà, une lame légère, animée depuis le débit de V. Et si, **de loin**,
le joueur perçoit un obstacle dans l'eau qui tombe, sa réaction est **simulée avec peu de ressources, factice** — pas de δ
ni d'APIC pour ce qu'on voit de loin.

**D4 — Les systèmes d'économie se répondent ; les contenants se divisent en domaines** (question 4). Pas de seuil unique
« contenant entier ou zone » : comme pour la mer, un contenant est **divisé en domaines**, dont les parties reçoivent une
simulation plus ou moins volumique selon les éléments présents et la présence des joueurs. L'ordonnanceur (ADR-012) et la
prévision (ADR-013) servent la mer et les contenants ensemble, sous un même budget.

**D5 — La météo, un système complet, après l'eau ; l'ordre des systèmes suivants** (question 5). La météo sera très
complexe, comme l'eau, avec des interactions avec l'environnement (comme l'eau en aura, par exemple un barrage cassé). Les
nuages et les éclairs relèvent de sa conception, pas de celle de l'eau. **L'ordre des systèmes après la fin du projet de
l'eau** : la **météo**, puis la **topologie d'un territoire**, puis le **feu**, et d'autres systèmes comme la **neige**.
Ce projet-ci n'en construit aucun ; il leur laisse les entrées dont ils auront besoin (la pluie, la chaleur, le vent, par
régions et dans le temps). ADR-197 D4 (pas de terrain réaliste à hydrologie : 5.11 hors du périmètre de l'eau) n'est pas
rouvert : la topologie d'un territoire sera un système à part, et c'est à sa conception qu'on dira ce qu'il demande à l'eau.

**D6 — La chaleur agit sur l'eau des contenants** (question 6) : l'évaporation fait baisser une piscine par temps chaud,
le froid gèle un bassin (ADR-017) — par V, un puits d'évaporation selon la température et le vent, et la phase ; construits
avec les entrées de la météo.

**D7 — Tout ce qui n'a pas de conséquence et se voit de loin est factice** (question 7) : les rides de la pluie, le vent sur
un bassin, un objet qui tombe vu de loin ; tout perturbateur **physique** proche — objet, joueur, lame qui heurte — déclenche
δ.

## 2. Ce que cela change dans la liste

- **5.5** (pluie selon l'exposition au ciel) : l'exposition est dynamique et fractionnaire (D2).
- **2.8** (précalcul côtier et météo) : la météo est un système complet, le premier après l'eau (D5) ; l'eau en consomme les
  entrées, ne la construit pas.
- **7.6** (glace et vapeur) : l'évaporation et le gel des contenants par V (D6).
- Rien n'est retiré du périmètre ; les systèmes de D5 viennent **après** celui de l'eau, ils n'y sont pas ajoutés.
