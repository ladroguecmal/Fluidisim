# ADR-180 — Retour δ → W, et où va le volume qui n'est pas restitué

- **Statut : actée**, S312, 2026-09-20, **décision de l'utilisateur**, en réponse à la question
  que [SORTIE-DELTA-S311](../validation/SORTIE-DELTA-S311.md) §5 lui posait et qu'[ADR-179](ADR-179-tolerances-de-conservation-et-grandeur-restituee.md) §3
  laissait explicitement ouverte : *« le volume net sortant n'a pas de receveur dans W »*.
- **Complète ADR-179**, qu'elle ne remplace pas : D1 à D8 y restent en vigueur.
- **Poursuit le lot 2** d'[ADR-178](ADR-178-strategie-en-trois-systemes-physiques.md) D7.
- **Ne touche pas** au périmètre (ADR-127) ni au profil de coût (ADR-174 D3, non opposable
  pendant la construction physique, ADR-178 D4).

## 1. Ce que l'utilisateur a écrit

> « **Premier transfert : option 1, limitée aux tests.** Je vous autorise à commencer par le
> transfert de la composante représentable par W. Cependant, la perte du volume net ne doit pas
> devenir un comportement accepté du moteur final. Pour ce premier prototype, tout volume non
> restitué doit être **comptabilisé et publié séparément**. Cette perte doit rester explicitement
> identifiée comme une **limitation expérimentale**. Le prototype ne pourra pas être déclaré
> conforme à la conservation globale tant qu'un receveur approprié n'aura pas été construit. »

> « **Ne pas imposer artificiellement le volume net à W.** […] Il faut distinguer : la composante
> propagative représentable par W ; le volume net transféré entre les régions ; les composantes
> qui ne peuvent pas être représentées par les primitives actuelles de W. La troisième catégorie
> ne se limite pas nécessairement au volume net : **une perturbation de moyenne nulle peut
> également présenter une forme, une direction ou un spectre incompatibles** avec les primitives
> existantes. Ne présumez donc pas qu'en supprimant la moyenne, le reste devient automatiquement
> transférable. »

> « **Destination du volume net.** Notre architecture comporte déjà B, W, δ et V. Je souhaite que
> vous étudiiez **en priorité** la possibilité de conserver le volume net à travers **V** ou une
> **modification du niveau moyen de B**, selon le type d'environnement. Il ne faut pas créer une
> nouvelle primitive dans W avant d'avoir vérifié si cette responsabilité relève déjà d'une autre
> couche. Un volume sortant d'une piscine, d'un contenant ou d'une région locale ne doit pas
> disparaître simplement parce que W ne peut pas le représenter. Pour une masse d'eau ouverte, il
> faut déterminer comment le transfert est comptabilisé dans le **bilan global**, même lorsque son
> effet local sur le niveau moyen est négligeable. Dans l'attente de ce mécanisme, un **registre
> explicite** peut conserver le déficit du prototype. Il ne doit toutefois pas être présenté comme
> une restitution physique accomplie. »

> « **Poursuite du lot 2.** […] 1. Extraire la perturbation sortante à la surface de contrôle
> intérieure. 2. Séparer le volume net de la composante de moyenne nulle. 3. Déterminer quelle
> part du signal est réellement représentable par W. 4. Construire son transfert effectif et
> vérifier **amplitude, phase, direction et propagation**. 5. Publier distinctement le volume
> transféré, le volume en attente de restitution et les éventuelles pertes numériques.
> 6. Vérifier le bilan global **sans double comptage** entre B, W et δ. La référence doit rester
> le **signal physique sortant**, pas simplement l'intégrale de son volume. Le seuil T3 de 5 %
> doit être évalué sur le **transfert effectivement réalisé**, avec des critères adaptés à la
> grandeur testée. La réflexion artificielle doit continuer à être mesurée **indépendamment**. »

> « **Suite du projet.** Ne lancez pas de refonte générale de W ou de nouvelle campagne de rendu.
> Construisez d'abord le **transfert minimal physiquement cohérent**, en réutilisant autant que
> possible les interfaces existantes. Une fois le premier couplage réalisé, nous pourrons
> déterminer ce qui manque véritablement à W et quelles responsabilités doivent revenir à B ou V.
> L'objectif demeure un système complet de gestion des fluides : les différentes couches doivent
> collaborer sans créer ni détruire artificiellement de l'eau. »

## 2. Décisions

**D1 — L'option 1 est retenue pour le premier transfert, et elle est bornée aux tests.** Le
transfert porte la composante que W sait représenter. Le reste n'est pas transféré, et **aucun
banc, aucun document, aucun compteur** ne le présente comme restitué. Cette borne est une
propriété du prototype, pas une tolérance : elle s'exprime par l'interdiction du mot
« conforme », pas par un seuil.

**D2 — Trois catégories, publiées séparément, à chaque pas.** L'utilisateur en nomme trois, et
elles ne se recouvrent pas :

| catégorie | ce que c'est | ce qu'on en fait |
|---|---|---|
| **transféré** | la part du flux sortant qu'une primitive de W porte effectivement | remise à W, et **vérifiée** (D5) |
| **en attente de restitution** | le volume net, et toute composante qu'aucune primitive ne porte | **registrée**, jamais appelée restitution |
| **perte numérique** | le résidu du schéma et des sommes | publiée à part, sous T1 (ADR-179 D1) |

Un compteur qui additionnerait les deux premières lignes annulerait la décision. Le registre est
un **passif**, pas un produit.

**D3 — La troisième catégorie ne se réduit pas au volume net.** Une composante de **moyenne
nulle** peut être non représentable par sa **forme**, sa **direction**, son **spectre**, sa
**phase** ou son **régime**. S311 n'avait identifié que le volume net ; l'utilisateur interdit
d'en conclure que le reste passe. **L'inventaire de ce que W ne porte pas s'établit composante
par composante, et par la mesure sur les primitives** — pas par lecture du contrat de
`WaveEvent` : un champ présent dans un encodage ne prouve pas qu'un champ construit l'honore.

**D4 — Le receveur du volume net se cherche dans les couches existantes, et dans cet ordre.**
V, ou le **niveau moyen de B**, selon le type d'environnement ; **aucune primitive nouvelle dans
W** tant que cette vérification n'a pas été faite et publiée. C'est une décision d'architecture,
pas de confort : la couche qui porte une moyenne n'est pas nécessairement celle qui porte les
perturbations, et le supposer est exactement l'erreur que D3 interdit ailleurs.

**D5 — Le transfert se vérifie sur la grandeur qu'il transporte.** Amplitude, phase, direction
et propagation. Le seuil T3 de 5 % d'ADR-179 D6 porte sur le **transfert effectivement réalisé**,
avec un critère adapté à la grandeur testée — et non sur l'intégrale du volume, que le premier
transfert ne porte justement pas. **La référence reste le signal physique sortant.**

**D6 — La réflexion artificielle reste mesurée séparément.** Elle ne fusionne pas avec l'erreur
de transfert : un raccord peut transmettre juste et réfléchir trop, et l'inverse. Le protocole de
S311 — jauge sur la ligne de contrôle, fenêtres déduites de la géométrie, mesure en 3D — reste
celui-là.

**D7 — Le bilan global se vérifie sans double comptage entre B, W et δ.** Ce que la bande B/W
pousse dans δ ne ressort pas au titre du transfert (ADR-179 D4) ; ce que le transfert remet à W
ne se recompte pas comme entrant au pas suivant. Un bilan qui ne sait pas distinguer les deux ne
vaut rien.

**D8 — Périmètre de la suite immédiate.** Aucune refonte générale de W, aucune nouvelle campagne
de rendu. **Le transfert minimal physiquement cohérent d'abord**, par les interfaces existantes.
Ce qui manque véritablement à W, et le partage des responsabilités entre B et V, se décide
**après** le premier couplage, sur ce qu'il aura mesuré. L'objectif reste le système complet :
les couches collaborent sans créer ni détruire d'eau.

## 3. Ce que cette décision ne tranche pas

- **Quelle couche portera effectivement le volume net.** D4 impose l'ordre de l'étude, pas son
  résultat. Le registre est une **attente**, pas un choix.
- **Le mécanisme de comptabilisation en masse d'eau ouverte.** L'utilisateur demande de le
  déterminer ; il ne dit pas lequel. « L'effet local est négligeable » n'est pas « il n'y a rien
  à compter ».
- **La primitive de W qui portera le transfert**, ni si les primitives existantes suffisent.
  ADR-179 §3 le laissait déjà ouvert, et la mesure seule peut le fermer.
- **Une tolérance sur l'énergie ou la quantité de mouvement** : ADR-179 D7 reste entière.

## 4. Ce qui devient faux si cette décision est mal lue

**« Option 1 » n'est pas « le volume net est perdu ».** C'est *« le volume net n'est pas
transféré par ce prototype, il est registré, et un receveur reste à construire »*. Un document
qui écrirait « perte acceptée » ferait dire à la décision le contraire de sa première phrase.

**« Publié séparément » n'est pas « mentionné en note ».** C'est un terme de sortie du banc, à
côté du transféré, au même rang, à chaque pas.

**Et « représentable par W » ne se lit pas dans un format.** `WaveEvent` porte des champs
`direction_turns`, `anisotropy` et `displaced_l` ; qu'ils existent dans un encodage ne dit rien
de ce qu'un champ construit en fait. D3 demande la mesure, et elle seule.
