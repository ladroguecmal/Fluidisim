# ADR-182 — Les trois critères de conservation, actés sous conditions, et l'ouverture de l'ordre B

- **Statut : actée**, S314, 2026-09-20, **décision de l'utilisateur**, en réponse aux trois seuils
  proposés par [PLANCHER-BILAN-S313](../validation/PLANCHER-BILAN-S313.md) §6.
- **Remplace D1 d'[ADR-179](ADR-179-tolerances-de-conservation-et-grandeur-restituee.md)**, que
  [ADR-181](ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md) D5 avait laissée sans
  chiffre. T2 (D2 d'ADR-179) est **confirmée inchangée**.
- **Ouvre l'ordre B** d'ADR-181 D10.
- **Ne touche pas** au périmètre (ADR-127) ni à l'ordre A → E.

## 1. Ce que l'utilisateur a écrit

> « **C1 — Accepté provisoirement : rapport au plancher ≤ 10.** Le plancher empirique proposé,
> `u32 × activité / √N`, constitue une référence adaptée aux régimes mesurés. **Conservez toutefois
> une distinction explicite entre une loi observée expérimentalement et une borne numérique
> démontrée.** Le seuil doit être réévalué si le schéma, la précision, la méthode de sommation ou
> le régime physique change significativement. »

> « **C2 — Accepté provisoirement : forme du résidu cumulé ≤ 5, à partir de 200 pas.** […]
> Cependant, le cas ouvert présente actuellement une valeur de 13,67. **Il doit rester signalé
> comme non conforme à C2.** Je ne souhaite ni augmenter arbitrairement le seuil pour faire passer
> ce cas, ni attribuer prématurément l'anomalie à une fuite physique. »

> « **C3 — Accepté : T2 inchangée.** […] Je retiens le seuil existant de 10⁻⁶, sans le modifier.
> Les trois contrôles sont **complémentaires et doivent être utilisés ensemble** sur les cas
> auxquels ils s'appliquent. »

> « **Portée de la validation.** La détection d'une fuite de 10⁻¹³ m³ par pas est un résultat
> intéressant **du banc actuel**. **Ne la généralisez pas** automatiquement à toutes les tailles
> de domaine, amplitudes et configurations numériques. Les critères devront conserver leurs
> **unités**, leur **domaine de validité** et une **normalisation explicite**. La réussite de C1
> et C3 ne doit pas masquer un échec de C2, et inversement. »

> « **A305** […] Conservez A305 comme anomalie **identifiée, non encore expliquée**. Je souhaite
> que sa résolution fasse l'objet d'un **diagnostic ciblé**, en distinguant la bande B/W de
> l'éponge. La faible amplitude de la dérive ne justifie pas de l'ignorer, mais elle ne doit pas
> non plus bloquer tout le développement de la primitive orientée de W. Les travaux indépendants
> peuvent avancer en parallèle. […] Si un **terme systématique légitime** est identifié, le critère
> devra distinguer son **effet attendu** d'un défaut de conservation. Toute correction de
> l'instrument devra être **justifiée et éprouvée avec des défauts injectés**. »

> « **Ordre B.** […] L'objectif est de corriger les limitations observées en S312 : 50 % de
> l'énergie transmise dans la mauvaise direction, une représentation spectrale insuffisante, une
> phase non conservée. **Commencez par une onde progressive unique en eau profonde, puis un paquet
> spectral et une propagation oblique.** Je souhaite que les cinq essais prévus soient réalisés,
> avec des mesures **indépendantes** de l'amplitude, de la direction, du spectre, de la phase et de
> la réflexion. **Le transfert ne devra pas être déclaré validé au seul motif que son amplitude et
> sa réflexion respectent déjà leurs seuils.** Le volume net reste comptabilisé dans `pending`
> jusqu'à la construction de son receveur effectif. **Ne modifiez pas artificiellement W pour lui
> attribuer la responsabilité du niveau moyen.** »

> « **Orientation générale.** Continuez le lot 2 sans ouvrir de nouveau chantier de photoréalisme
> ou d'optimisation prématurée. […] Chaque développement doit produire une **capacité
> identifiable**, accompagnée des tests nécessaires à sa validation. »

## 2. Décisions

**D1 — C1 est acté à 10, provisoirement, et la nature de son plancher est à dire à chaque fois.**
`u₃₂ · activité / √N` est une **loi observée**, vérifiée sur quatre décades d'amplitude, un facteur
16 en `dt` et un facteur 16 en `N` — **ce n'est pas une borne démontrée**. Tout document qui
l'emploie dit lequel des deux il invoque. Le seuil se réévalue si le **schéma**, la **précision**,
la **méthode de sommation** ou le **régime physique** change significativement — quatre
déclencheurs, et ils sont nominatifs.

**D2 — C2 est acté à 5 à partir de 200 pas, et le cas ouvert reste non conforme.** Il ne se
rattrape ni en relevant le seuil, ni en décrétant que son biais est physique. **Les deux
échappatoires sont nommées et fermées**, ce qui est plus fort que d'exiger une explication : le
dépôt doit porter une non-conformité ouverte aussi longtemps qu'elle n'est pas comprise.

**D3 — C3 est acté : T2 inchangée, 10⁻⁶ de l'amplitude de référence sur 10 s.** Le seuil de
D2 d'ADR-179 n'est pas touché ; il est simplement **tenu** désormais.

**D4 — Les trois s'appliquent ensemble, et aucun n'excuse un autre.** *« La réussite de C1 et C3
ne doit pas masquer un échec de C2, et inversement. »* Un banc qui publie deux critères sur trois
ne publie pas une conservation. Cela vaut aussi pour la portée mesurée en S313 : C1 et C2 ferment
un pas, C3 surveille une durée, et une fuite d'état passe les deux premiers.

**D5 — La sensibilité de 10⁻¹³ m³ par pas appartient à son banc.** Elle ne se transporte pas à une
autre taille de domaine, une autre amplitude, une autre configuration numérique. Tout critère
publié porte ses **unités**, son **domaine de validité** et sa **normalisation explicite** — trois
mentions obligatoires, pas trois recommandations.

**D6 — A305 reste ouverte, et son diagnostic est ciblé.** La bande B/W et l'éponge se distinguent
l'une de l'autre, à configurations **comparables**. Si un **terme systématique légitime** est
trouvé, le critère devra séparer son **effet attendu** d'un défaut — et toute correction de
l'instrument sera **justifiée et éprouvée avec des défauts injectés**, comme S313 l'a fait. La
faible amplitude ne justifie pas de l'ignorer ; elle ne justifie pas non plus de bloquer l'ordre B.
**Les deux travaux avancent en parallèle.**

**D7 — L'ordre B est autorisé, et son objet est nommé** : les trois limitations mesurées en S312 —
**50 % de l'énergie à contresens**, **spectre insuffisant**, **phase non conservée**. Progression
imposée : **onde progressive unique en eau profonde**, puis **paquet spectral**, puis
**propagation oblique**. Les cinq essais d'ADR-181 D9 sont dus, avec des mesures **indépendantes**
des cinq grandeurs.

**D8 — Le transfert ne se déclare pas validé sur deux critères tenus.** L'amplitude (1,24·10⁻⁵) et
la réflexion (2,84·10⁻⁷) sont déjà sous leurs seuils depuis S312 : elles ne valent pas réception.
C'est la reprise explicite de D8 d'ADR-181, et elle est ici opposable à tout banc de l'ordre B.

**D9 — Le volume net reste dans `pending`**, et **W ne reçoit pas la responsabilité du niveau
moyen**. La primitive orientée porte des perturbations propagatives ; si elle portait une moyenne,
elle cesserait d'être une primitive de W (S312 §1.3). ADR-180 D1, D2 et ADR-181 D1, D2 restent
entières.

**D10 — Chaque développement produit une capacité identifiable et ses essais.** Ni photoréalisme,
ni optimisation prématurée. C'est D12 d'ADR-181 reformulée par l'usage : le livrable d'une session
est une capacité, pas une campagne.

## 3. Ce que cette décision ne tranche pas

- **La forme de la primitive orientée** : famille de modes, anisotropie d'un champ existant, ou
  source de pression. D7 fixe la progression et les essais, pas la construction.
- **La cause d'A305**, explicitement : D6 impose un protocole, pas un coupable.
- **Le receveur du volume net** — ADR-181 D1 le désigne (V ou un niveau moyen régional de B) ;
  sa construction est l'ordre D, non ouvert.
- **Ce que deviendraient C1 et C2 si un terme systématique légitime était trouvé** : D6 dit que le
  critère devra l'en distinguer, pas comment.

## 4. Ce qui devient faux si cette décision est mal lue

**« C1 est acté » ne veut pas dire « le plancher est démontré ».** D1 l'interdit en toutes lettres.
Un document qui écrirait « borne » là où la mesure dit « loi observée » ferait exactement l'erreur
que S313 reproche à S312 — attribuer sans loi —, une génération plus tard et dans l'autre sens.

**« Les trois critères sont actés » ne veut pas dire « la conservation est acquise ».** Le cas
ouvert **échoue C2**, aujourd'hui, et D2 exige qu'il reste signalé comme tel.

**Et « l'ordre B est autorisé » ne veut pas dire « le transfert va être validé ».** D8 pose que
deux critères tenus ne font pas une réception, et les cinq essais existent précisément pour que le
verdict porte sur les six propriétés.
