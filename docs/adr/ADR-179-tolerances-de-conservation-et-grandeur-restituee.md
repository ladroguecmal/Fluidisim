# ADR-179 — Tolérances de conservation, et ce que « restituer » veut dire

- **Statut : actée**, S311, 2026-09-20, **décision de l'utilisateur**, en réponse aux trois
  tolérances proposées par [BILAN-MASSE-S310](../validation/BILAN-MASSE-S310.md) §6.
- **Complète** [ADR-178](ADR-178-strategie-en-trois-systemes-physiques.md) §3, qui laissait
  explicitement cette tolérance ouverte : « elle se proposera au premier bilan publié ».
- **Lance le lot 2** d'ADR-178 D7 — le retour δ → W.
- **Ne touche pas** au périmètre (ADR-127), ni au profil de coût (ADR-174 D3, non opposable
  pendant la construction physique, ADR-178 D4).

## 1. Ce que l'utilisateur a écrit

> « **T1 — Acceptée provisoirement.** Le résidu du bilan de masse doit rester inférieur ou égal à
> 10⁻⁶ de l'échelle définie pour le pas. »

> « **T2 — Acceptée comme objectif.** La dérive de volume d'un domaine fermé doit rester inférieure
> ou égale à 10⁻⁶ de l'amplitude de référence sur 10 secondes. Les résultats actuels sont
> encourageants, mais **la durée complète doit encore être éprouvée**. »

> « **T3 — Acceptée comme objectif initial, avec une précision indispensable.** […] Il faut définir
> précisément la grandeur à restituer. L'éponge absorbe actuellement des perturbations dont le
> **volume net signé** et la **quantité absolue** sont différents. Ces deux mesures ne doivent pas
> être confondues. Je ne souhaite pas que le moteur **crée artificiellement une nouvelle vague**
> pour compenser toute l'activité de l'éponge. Le transfert doit représenter la perturbation
> physique **sortante**, compatible avec W, **sans double comptage** avec le fond B/W entrant. Les
> composantes qui ne peuvent pas être représentées par W doivent être **identifiées**. Leur devenir
> ne doit pas être implicitement assimilé à une restitution réussie. »

> « Pour le premier test de T3, définissez donc un **cas contrôlé** comportant une perturbation
> sortante identifiable, une **grandeur de référence non nulle** et une **mesure de l'erreur
> correctement normalisée**. Le seuil de 5 % sera un critère de réception **de ce premier cas**, et
> non une validation universelle du couplage. Le seuil de réflexion de 1 % devra être mesuré
> **directement en 3D**. »

> « Ne revendiquez pas encore la conservation complète de l'énergie et de la quantité de mouvement
> tant que leurs bilans ne sont pas fermés. »

## 2. Décisions

**D1 — T1, provisoire.** Le résidu du bilan de masse reste ≤ **10⁻⁶** de l'**échelle du pas**, que
S310 définit comme `max(|delta|, |band_in|, |sponge_out|)`. *Provisoire* signifie révisable si une
seconde cible (A98) ou un portage sur la carte donne un autre plancher ; il ne se resserre pas sans
mesure, et ne se relâche pas sans arbitrage.

**D2 — T2, objectif à éprouver sur sa durée.** Dérive de volume d'un domaine fermé ≤ **10⁻⁶** de
l'amplitude de référence **sur 10 s**. S310 l'a mesurée sur **5 s** (1,48·10⁻¹⁰, quatre ordres de
marge) : **le critère n'est donc pas encore rendu**, et l'écrire tenu serait exactement l'erreur
que L347 nomme. La durée complète est due.

**D3 — La grandeur à restituer est le flux de perturbation sortant, pas l'activité de l'éponge.**
C'est la décision centrale, et elle sépare trois choses que S310 mesurait ensemble :

| | ce que c'est | ce qu'on en fait |
|---|---|---|
| **flux sortant** | ce que la perturbation emporte à travers une face extérieure, porté par la vitesse normale et la colonne mouillée | **c'est lui qu'on restitue** |
| **activité absolue de l'éponge** | `Σ|Δh|` sur la bande — 10,2 % du domaine par seconde (S310 §3) | **ne se restitue pas** : elle mélange l'onde sortante, l'onde entrante que l'éponge amortit aussi, et le rappel vers le repos |
| **net signé de l'éponge** | `ΣΔh` — quasi nul (−7,8·10⁻³ m³ en 6 s) | **ne se restitue pas** : un net nul ne veut pas dire qu'il ne s'est rien passé |

**Fabriquer une vague pour compenser l'activité absolue créerait de l'énergie**, puisqu'une partie
de cette activité amortit une onde qui **entrait**. L'utilisateur l'écrit ; cet ADR en fait une
règle du dépôt.

**D4 — Pas de double comptage avec la bande entrante.** Ce que la bande B/W pousse dans δ
(`band_in`) n'a pas à ressortir : il est déjà dans W, par construction. Seule la part **produite
ou transformée par δ** se restitue. Un bilan qui restituerait `band_in` compterait deux fois la
même eau.

**D5 — Ce que W ne peut pas porter s'identifie et se déclare perdu.** Toute composante du flux
sortant qu'aucune primitive de W ne représente est **nommée, chiffrée, et comptée comme perte**,
jamais absorbée dans le terme « transmis ». C'est la différence entre un bilan et un habillage.

**D6 — T3 est la réception d'un cas, pas du couplage.** Le cas est **contrôlé** : une perturbation
sortante identifiable, une grandeur de référence **non nulle**, une erreur **normalisée par elle**
et déclarée avant la mesure. Seuils : **erreur de restitution ≤ 5 %**, **réflexion artificielle
< 1 % en énergie**, cette dernière mesurée **directement en 3D** — la mesure 2D de
[S269](../validation/REFLEXION-PAQUET-S269.md) (0,14–0,16 %) est un antécédent, pas une preuve.

**D7 — Aucune revendication d'énergie ni de quantité de mouvement** tant que leurs bilans ne sont
pas fermés ([BILAN-MASSE-S310](../validation/BILAN-MASSE-S310.md) §4). Elles restent des **états**.
Cela vaut aussi pour la réflexion « en énergie » de D6 : elle se mesure sur une **jauge**, par
comparaison d'un signal incident et d'un signal retour sur le même point, comme S269 le faisait —
ce qui ne demande pas de bilan fermé et n'en revendique pas un.

**D8 — L'ordre de preuve du lot 2** est celui que l'utilisateur donne, et chaque point est un
critère de réception, pas une étape de confort :

1. une perturbation sortante est **correctement identifiée** à la frontière de δ ;
2. son transfert vers W **ne crée pas de volume** ;
3. la perturbation transmise **se propage dans W** avec amplitude, phase et direction cohérentes ;
4. le raccord garde une réflexion artificielle **sous le seuil** ;
5. les bilans publiés distinguent **transfert, dissipation et résidu numérique**.

## 3. Ce que cette décision ne tranche pas

- **La primitive de W qui portera le transfert.** W a des impacts radiaux et des sillages ; rien ne
  dit qu'une de ces formes accepte un front sortant quelconque. C'est une question de construction,
  et son inventaire est le premier travail du lot (D5).
- **Le devenir des composantes non représentables.** Les identifier est obligatoire ; décider si on
  les accepte comme pertes, si on étend W, ou si on change la frontière, ne l'est pas encore.
- **Une tolérance sur l'énergie ou la quantité de mouvement**, qui porterait sur un nombre qui
  n'est pas un échange (D7).

## 4. Ce qui devient faux si cette décision est mal lue

**T3 à 5 % n'est pas un permis d'approximation.** C'est le seuil de réception d'un **premier** cas,
choisi parce qu'un premier retour ne sera pas exact et qu'un seuil inatteignable ne sert à rien.
Un second cas, ou une scène, demandera son propre seuil — et le premier ne le justifie pas.

**Et « restituer » ne veut pas dire « faire disparaître le déficit ».** Une implémentation qui
ferait coïncider les chiffres en injectant dans W ce que l'éponge a retiré satisferait le compteur
et violerait D3. Le compteur de S310 ne s'y oppose pas tout seul : c'est la définition de D3, et
l'identification de D5, qui l'interdisent.
