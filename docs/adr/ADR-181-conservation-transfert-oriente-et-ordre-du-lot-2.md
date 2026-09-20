# ADR-181 — Où va le volume net, comment se mesure un résidu, et dans quel ordre on continue

- **Statut : actée**, S313, 2026-09-20, **décision de l'utilisateur**, en réponse aux deux
  questions que [TRANSFERT-DELTA-W-S312](../validation/TRANSFERT-DELTA-W-S312.md) lui posait —
  le receveur du volume net (§1.5) et la normalisation de T1 (§5) — et aux trois manques que la
  même preuve chiffrait (§7).
- **Complète** [ADR-179](ADR-179-tolerances-de-conservation-et-grandeur-restituee.md) et
  [ADR-180](ADR-180-retour-delta-w-et-conservation-du-volume.md), qu'elle ne remplace pas : elle
  **révise D1 d'ADR-179** (la normalisation de T1) et **tranche §3 d'ADR-180** (le receveur).
- **Poursuit le lot 2** d'[ADR-178](ADR-178-strategie-en-trois-systemes-physiques.md) D7, et en
  fixe l'ordre interne.
- **Ne touche pas** au périmètre (ADR-127).

## 1. Ce que l'utilisateur a écrit

> « **Destination du volume net.** Je retiens une responsabilité distincte selon l'environnement :
> **V** pour les contenants et volumes finis, lorsque le transfert relève de leur bilan de masse ;
> **B** pour les masses d'eau ouvertes, avec l'introduction d'un **niveau moyen régional** capable
> de prendre en compte les variations de volume. **Attention : je ne souhaite pas qu'une quantité
> locale d'eau soit simplement répartie sur un océan infini par une modification arbitraire du
> niveau global.** La modification de B devra s'appuyer sur une **région ou un volume de contrôle
> identifiable**, avec une comptabilité cohérente des échanges. Les interfaces entre B, V et δ
> devront empêcher les créations et destructions involontaires de masse ainsi que les doubles
> comptages. W reste responsable de la propagation des perturbations représentables. Je ne
> souhaite pas lui ajouter artificiellement un mode de volume net si cette responsabilité
> appartient aux autres couches. Le registre `pending` doit être conservé tant que le receveur
> effectif n'est pas construit. »

> « **Révision de T1.** J'accepte de revoir la normalisation […] Je souhaite cependant éviter de
> remplacer le seuil actuel par une **tolérance absolue arbitraire**, qui pourrait masquer des
> erreurs sur les petites perturbations. Construisez un critère qui distingue explicitement : le
> **résidu absolu**, avec ses unités physiques ; le **résidu relatif** à une échelle pertinente du
> problème ; le **plancher d'arrondi attendu** pour la représentation numérique utilisée ; le
> **comportement du résidu cumulé** sur une durée donnée. La tolérance doit être justifiée par des
> mesures et, lorsque possible, par une **borne d'erreur numérique**. Vérifiez ce critère sur
> plusieurs **amplitudes, pas de temps et résolutions**, y compris sur des cas où une **erreur
> volontaire** est introduite afin de s'assurer que l'instrument reste capable de la détecter.
> **Je ne fixe pas de nouveau chiffre avant cette démonstration.** T2 reste également à confirmer
> sur les 10 secondes prévues. »

> « **Le transfert δ → W n'est pas encore validé.** […] Les deux premières mesures sont très
> satisfaisantes, mais elles ne suffisent pas à valider la transmission. Je souhaite conserver les
> critères d'amplitude et de réflexion, puis **étendre les critères de réception aux propriétés
> physiques réellement nécessaires**. La transmission doit reproduire une **perturbation sortante
> cohérente**, et pas seulement générer une onde possédant une amplitude équivalente. »

> « **Prochaine priorité : la primitive orientée de W.** […] Construisez une représentation
> capable de restituer les composantes propagatives sortantes compatibles avec W, en conservant
> autant que possible leur **direction**, leur **spectre** et leur **phase**. Commencez par un
> paquet d'ondes en eau profonde, dans le régime actuellement accepté par W. **N'essayez pas de
> faire passer artificiellement le cas de canal peu profond de S311** […] L'extension de W aux
> profondeurs finies constituera un développement distinct. […] Il faut distinguer l'énergie
> réellement transmise, l'énergie réfléchie, la dissipation et les composantes que W ne peut pas
> encore représenter. **Ne cherchez pas à corriger un écart de spectre ou de direction par un
> simple réglage d'amplitude.** »

> « **Ordre de poursuite.** A. Corriger et valider l'instrument T1. B. Construire la transmission
> orientée de la composante représentable par W. C. Vérifier conjointement amplitude, phase,
> direction, spectre et réflexion. D. Développer la comptabilité et la restitution effective du
> volume net à travers B ou V, suivant l'environnement. E. Tester le couplage complet sur des
> configurations progressivement plus complexes. Ces travaux peuvent être **préparés en
> parallèle** lorsque leurs interfaces sont indépendantes. […] Le solveur de référence peut rester
> coûteux pendant cette phase. **Continuez néanmoins à mesurer les performances** pour éviter des
> choix architecturaux difficilement optimisables. »

> « **Objectif global.** Notre priorité n'est toujours pas la finition photoréaliste de l'océan.
> Nous construisons un moteur de fluides complet pour un jeu vidéo. […] Je souhaite que chaque
> session fasse progresser ces capacités, **sans multiplier les campagnes de mesures qui ne
> répondent pas à une question technique identifiable**. […] ne déclarez pas le couplage terminé
> tant que les échanges physiques essentiels ne sont pas effectivement assurés. »

## 2. Décisions

**D1 — Le receveur du volume net est V ou B, selon l'environnement, et jamais W.** V pour un
contenant ou un volume fini, quand le transfert relève de son bilan de masse. B pour une masse
d'eau ouverte, par un **niveau moyen régional**. Cela ferme §3 d'ADR-180, qui posait la question
sans la trancher, et confirme ce que S312 avait mesuré : W n'a de mode `k = 0` dans aucune de ses
productions, et lui en ajouter un ferait de B sous un autre nom.

**D2 — Un niveau moyen régional s'adosse à une région identifiable, pas à un océan.** C'est la
précision la plus lourde de cette décision, et elle interdit la solution qui paraissait la plus
simple : *« je ne souhaite pas qu'une quantité locale d'eau soit simplement répartie sur un océan
infini par une modification arbitraire du niveau global »*. Un niveau moyen de B doit donc porter
**une région ou un volume de contrôle identifiable** et une comptabilité des échanges à sa
frontière. Un scalaire global qui absorberait n'importe quel apport serait un compteur qu'on
satisfait — la même faute qu'ADR-180 D2 nomme pour le registre.

**D3 — Les interfaces B / V / δ empêchent la création, la destruction et le double comptage.**
C'est une exigence de **construction**, pas de vérification *a posteriori* : une interface qui
laisse passer une création et la signale ensuite n'est pas conforme à cette décision.

**D4 — Le registre `pending` est conservé tant que le receveur n'existe pas**, et un volume
comptabilisé mais non restitué n'est jamais présenté comme une conservation physique accomplie.
ADR-180 D1 et D2 restent entières.

**D5 — T1 devient un critère à quatre grandeurs, et aucune tolérance ne précède sa
démonstration.** Les quatre se publient **séparément**, chacune avec son unité :

| grandeur | ce qu'elle dit |
|---|---|
| **résidu absolu** | en m³, ce que le pas ne referme pas |
| **résidu relatif** | rapporté à une **échelle pertinente du problème**, à justifier |
| **plancher d'arrondi attendu** | ce que la représentation numérique impose, **dérivé** |
| **résidu cumulé** | son comportement sur une **durée** — dérive ou marche aléatoire |

Une **tolérance absolue arbitraire est refusée** au même titre qu'un relatif mal normalisé : elle
masquerait les erreurs sur les petites perturbations. La tolérance se justifie par des **mesures**
et, lorsque possible, par une **borne d'erreur numérique**, et se vérifie sur plusieurs
**amplitudes**, **pas de temps** et **résolutions**. **Aucun chiffre n'est fixé avant cette
démonstration** — ce qui rend caduque la formulation de D1 d'ADR-179 sans lui substituer autre
chose pour l'instant.

**D6 — L'instrument se prouve sur un défaut qu'on lui donne à trouver.** Un critère de
conservation qui n'a jamais vu de fuite ne sait pas qu'il en verrait une. Une **erreur volontaire**
est introduite, et la **plus petite erreur détectée** est publiée : c'est la sensibilité réelle de
l'instrument, et elle borne par le bas toute tolérance qu'on voudra écrire.

**D7 — T2 est due sur 10 s**, la durée demandée. ADR-179 D2 l'avait déjà dit ; elle n'a toujours
été mesurée que sur 5 s (S310).

**D8 — Le transfert δ → W n'est pas validé, et les critères de réception s'étendent.** Amplitude
et réflexion sont **conservées** — elles sont tenues (1,24·10⁻⁵ et 2,84·10⁻⁷, S312) — mais elles
ne suffisent pas. La transmission doit reproduire une perturbation sortante **cohérente** :
direction, spectre et phase entrent dans la réception. Corollaire explicite : **un écart de
spectre ou de direction ne se corrige pas par un réglage d'amplitude**, et un bilan doit séparer
l'énergie **transmise**, **réfléchie**, **dissipée**, et les composantes **non représentables**.

**D9 — Priorité suivante : la primitive orientée de W**, sur un **paquet d'ondes en eau
profonde**, dans le régime que W accepte déjà. Cinq essais ciblés, qui sont des critères et non
des étapes de confort :

1. une onde **unidirectionnelle** simple ;
2. un **paquet** composé de plusieurs longueurs d'onde ;
3. une propagation **oblique** à la frontière ;
4. la **cohérence de phase** entre le signal sortant de δ et celui introduit dans W ;
5. transmission et réflexion sur **plusieurs résolutions**.

**Le canal peu profond de S311 ne se force pas.** Son écart de célérité de 44 % (S312 §2) mesure
précisément un régime que W ne prend pas en charge ; l'extension de W aux **profondeurs finies**
est un **développement distinct**, à ouvrir séparément.

**D10 — L'ordre du lot 2 est A → E**, et la préparation peut être parallèle quand les interfaces
sont indépendantes : A l'instrument T1, B la transmission orientée, C la vérification conjointe,
D la comptabilité et la restitution du volume net, E le couplage complet sur des configurations
progressivement plus complexes.

**D11 — Le coût reste mesuré sans être opposable.** Le solveur de référence peut rester coûteux
pendant cette phase (ADR-178 D4 reste vraie), **mais les performances continuent d'être
mesurées** — pour éviter un choix d'architecture difficilement optimisable, pas pour arbitrer
maintenant.

**D12 — Une session fait progresser une capacité, ou répond à une question technique
identifiable.** *« Sans multiplier les campagnes de mesures qui ne répondent pas à une question
technique identifiable. »* C'est la règle des deux maillons de REPRISE §6, énoncée par
l'utilisateur et désormais opposable : une campagne se justifie par la décision qu'elle débloque.

## 3. Ce que cette décision ne tranche pas

- **La forme du niveau moyen régional** : constante par région, champ lent, entrée de
  l'ordonnanceur, ou état porté par V. D2 dit ce qu'il ne peut pas être ; pas ce qu'il est.
- **La frontière d'une « région »** pour B — géométrique, par domaine δ, ou par cellule de
  monde. C'est le premier travail de l'ordre D.
- **Les chiffres de T1**, explicitement : D5 les interdit avant la démonstration.
- **La forme de la primitive orientée** de W : anisotropie du champ existant, nouvelle famille de
  modes, ou source de pression. D9 fixe les essais, pas la construction.
- **L'extension de W aux profondeurs finies**, reconnue nécessaire et renvoyée à un lot distinct.

## 4. Ce qui devient faux si cette décision est mal lue

**« B reçoit le volume net » n'autorise pas un niveau global.** C'est l'inverse : D2 existe pour
interdire exactement cette lecture, qui est la plus tentante parce que la plus simple à écrire.

**« T1 est révisée » ne veut pas dire « T1 est assouplie ».** Aucun seuil n'est relevé ; le seuil
est **retiré** en attendant une démonstration, et un absolu arbitraire est refusé au même titre
qu'un relatif mal normalisé. Une session qui écrirait une tolérance commode avant les balayages
et la borne violerait D5.

**Et « amplitude et réflexion tenues » ne veut pas dire « transfert reçu ».** D8 le dit en toutes
lettres : deux critères tenus sur six propriétés ne font pas une transmission. Le dépôt ne
déclare pas le couplage terminé (D12).
