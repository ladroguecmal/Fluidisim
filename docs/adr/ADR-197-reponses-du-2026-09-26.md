# ADR-197 — Réponses du 2026-09-26 : Godot moteur du jeu, tout corps flotte ou coule, pas d'hydrologie du terrain, météo et son à la fin, A289 déléguée

- **Statut : actée**, S369, 2026-09-26, **réponses de l'utilisateur** aux quatre questions posées à la fin de S368 —
  ce que la liste du projet fini attendait de lui (ADR-190 D5), du plus rentable au moins rentable.
- **Précise** [ADR-192](ADR-192-le-rendu-de-l-eau-dans-godot-4.md) (Godot n'est plus seulement le rendu de l'eau) et
  [ADR-190](ADR-190-apres-la-v1-la-liste-entiere.md) (un point sort du périmètre, D4 ; deux passent à la fin, D5).
- **Laisse entiers** [ADR-127](ADR-127-ambition-complete-construction-progressive.md) hors du seul point que D4 retire,
  [ADR-174](ADR-174-arbitrages-du-2026-09-19.md) et [ADR-027](ADR-027-les-cinq-arbitrages-tranches.md).

## 1. Les questions et les réponses, telles qu'écrites

Questions (fin de S368) : le réseau du jeu — un format ou un moteur existe-t-il ? ; Godot sera-t-il le moteur du jeu
entier, et puis-je télécharger godot-rust ? ; ce qui flotte et nage — acteurs, taille, forme, masse ; le terrain — un
modèle de sol, de nappes, un outil ? ; la météo et le son — d'où, quel moteur audio ? ; la voie d'A289 ; une seconde
machine cible et un vrai serveur ; le verdict R18 ; des photographies d'écume.

> 1 : Pas de réseau. Godot sera le moteur entier fais comme bon te semble. Tout flotte/coule des interactions physiques
> logique. Pas de terrain realiste avec hydrologie etc.... Pas de météo et son a faire à la fin.
> 2 : Le choix le plus favorable au realisme ainsi que les performances, simple. Pas encore.
> 3 : Rendu convaincant
> 4 : Plus tard

## 2. Décisions

**D1 — Le réseau : « Pas de réseau », consigné tel quel, sa portée demandée.** Ce qui est acquis : **aucun format ni
moteur réseau n'existe** pour le jeu. Ce qui ne l'est pas : si le jeu se passe de multijoueur. Les deux lectures
changent quatorze points (10.1 et son aval) ; la seconde serait une réduction d'ambition, qui ne se lit pas dans une
phrase ambiguë (ADR-127). La question est posée en une ligne ; d'ici là, rien n'est retiré et aucun travail de réseau
ne commence. Si le multijoueur reste, le format est **à nous** (ADR-028), sur le moteur de D2.

**D2 — Godot 4 est le moteur du jeu entier ; l'intégration est déléguée.** L'eau ne livre pas seulement son rendu à
Godot (ADR-192) : le cœur s'y branche, et le projet choisit comment. **godot-rust** (GDExtension) est autorisé dans son
principe par *« fais comme bon te semble »* ; le lot qui le télécharge nomme, à son ouverture, la caisse, sa version,
sa taille et sa source, comme toute dépendance (ADR-130). Le point 8.1 n'attend plus rien de l'utilisateur.

**D3 — Tout flotte ou coule selon la physique.** Pas de catalogue d'acteurs : navires, nageurs, objets sont des **corps
quelconques** — forme, masse, répartition — que l'eau porte, pousse, renverse ou laisse couler selon leurs grandeurs, par
des interactions physiques « logiques ». C'est le chemin de la porte D (corps rigide sur B, ADR-189), généralisé. Le
point 6.7 n'attend plus que 6.2.

**D4 — Pas de terrain réaliste à hydrologie : les eaux souterraines sortent du périmètre.** Décision explicite de
l'utilisateur, la seule forme qu'ADR-127 admette pour retirer : **5.11 (eaux souterraines)** ne se construit pas. La liste
le garde, marqué, pour mémoire ; il ne se rouvre que par l'utilisateur. Le terrain reste une **frontière** de l'eau —
fond, côtes, décor (2.7, 6.5) — sans nappes ni écoulement dans le sol ; l'absorption de la pluie par le sol (5.5) se
fera sans modèle de sol. L'outil de terrain (12.4) est celui du jeu, donc de Godot (D2).

**D5 — La météo et le son se font à la fin.** Lecture retenue — celle qui ne retire rien : les deux sont **à faire, en
dernier**. La source météo (2.8) sera construite par le projet à ce moment ; le son (7.8) passera par le moteur audio de
Godot (D2). Jusque-là, leurs points ne sont pas choisis par l'ordre des fronts.

**D6 — A289 : la voie est déléguée au projet, sous trois critères** — *le plus favorable au réalisme et aux
performances, simple*. Le projet choisit sur mesure, dans la session même, et consigne la voie reçue dans un ADR
propre.

**D7 — Une seconde machine cible, un serveur réel : pas encore.** 1.7, 9.10, 10.2, 10.3 et 11.5 gardent leur attente ;
la question ne se repose pas avant un fait nouveau de l'utilisateur.

**D8 — R18 : « Rendu convaincant ».** δ à 30 Hz interpolé au rendu, jugé en direct : reçu. Le point 8.7 n'attend plus de
l'utilisateur que le verdict de sa frontière.

**D9 — Les photographies d'écume : plus tard.** L'écume reste suspendue (S368, 7.1).

## 3. Ce que cette décision change ailleurs

- [Liste du projet fini](../LISTE-PROJET-FINI.md) : 5.11 marqué hors du périmètre ; 8.1, 8.7, 2.8, 7.8 annotés.
- Registre des dépendances (`outils/dependances_liste.py`) : attentes levées pour 6.7 et 8.1, R18 retiré de 8.7 ;
  2.8, 5.11, 7.8 et 10.1 réécrites ; la seconde cible et le serveur datés.
- [File active](../registres/QUESTIONS-OUVERTES.md#file-active) : ligne des décisions ; A289 déléguée.
- [Revue visuelle](../validation/REVUE-VISUELLE.md) §23 : verdict R18.

## 4. Ce qu'elle ne fait pas

Elle ne retire que 5.11. Elle ne décide pas du multijoueur (D1), ne télécharge rien (D2), ne choisit pas la voie
d'A289 sans mesure (D6), et ne rouvre ni ADR-027 ni le profil de temps d'ADR-174.

## Note datée du 2026-09-26 (S370) — D1 précisé

Question posée à la fin de S369 : « aucun format réseau n'existe encore, ou le jeu n'aura pas de multijoueur ? » Réponse
de l'utilisateur : *« Pas de réseau = pas encore »*. **Le multijoueur reste dans l'ambition** (ADR-127) ; rien n'est
retiré. Comme la seconde cible et le serveur (D7), le réseau attend un fait nouveau de l'utilisateur : 10.1 et son aval
gardent leur attente, et aucun travail de réseau ne commence d'ici là.
