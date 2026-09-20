# ADR-183 — L'essai oblique, la phase à distance, et le passage à l'ordre C

- **Statut : actée**, S315, 2026-09-20, **décision de l'utilisateur**, en réponse à
  [TRANSFERT-ORIENTE-S314](../validation/TRANSFERT-ORIENTE-S314.md) §8, qui nommait les deux
  vérifications que l'ordre B laissait dues.
- **Complète** [ADR-182](ADR-182-criteres-de-conservation-actes-et-ordre-b.md), qu'elle ne
  remplace pas : D7 à D9 y restent en vigueur, et l'essai 3 reste celui d'ADR-181 D9.
- **Ne touche pas** au périmètre (ADR-127), ni à l'ordre A → E, ni aux critères C1/C2/C3.

## 1. Ce que l'utilisateur a écrit

> « La primitive orientée constitue une avancée importante. […] Je valide la poursuite proposée,
> **sans considérer le transfert complet comme terminé**. »

> « **Terminer l'essai oblique à la frontière.** […] Je souhaite notamment vérifier : que
> l'énergie sortante est transmise dans la **direction attendue** ; que les **composantes
> tangentielles** sont correctement conservées ; que le transfert ne crée pas de composante
> **réfléchie ou transverse artificielle** ; que le résultat reste cohérent lorsque **l'angle
> d'incidence varie**. La primitive accepte déjà une direction oblique, mais il reste à démontrer
> que **le raccord l'exploite correctement**. »

> « **Vérifier la conservation de la phase pendant la propagation.** La régression de phase mesurée
> sur la ligne d'émission est encourageante, mais elle ne suffit pas à valider la propagation.
> Construisez un **oracle indépendant** pour vérifier la phase à distance, notamment à **dix
> longueurs d'onde** comme prévu. La comparaison devra tenir compte de la **dispersion**, du
> **temps de propagation** et de l'**élargissement naturel du paquet**. **Ne remplacez pas un
> défaut de phase par une simple correction d'amplitude ou un décalage temporel arbitraire.** »

> « **Passer ensuite à l'ordre C.** […] L'objectif n'est plus seulement de démontrer que W sait
> produire un paquet orienté. Il faut démontrer que ce paquet **représente correctement une
> perturbation réellement extraite de δ**. Évaluez conjointement l'amplitude, la direction, le
> spectre, la phase, la vitesse de propagation et la réflexion artificielle. **Distinguez les
> résultats propres à la primitive W, ceux du raccord et ceux de la simulation δ.** Le volume net
> reste intégralement dans `pending` tant que le receveur B/V n'est pas opérationnel. »

> « **A305 et A306.** Maintenez les deux anomalies ouvertes. A305 doit être isolée par les tests
> ciblés prévus sur la bande et l'éponge. Pour A306, examinez les **deux emplois** de l'estimateur
> de fréquence dans le harnais de validation. Il faut déterminer si des **réceptions antérieures**
> ont pu être affectées. Ces investigations peuvent avancer en parallèle. Elles ne doivent pas
> bloquer le développement indépendant de la primitive orientée. »

> « **Priorité générale.** […] Je souhaite que les prochaines sessions privilégient l'achèvement
> d'une **capacité fonctionnelle complète** : une perturbation produite par δ, correctement
> identifiée, transférée puis **propagée** dans W. Une fois ce trajet validé, nous pourrons
> poursuivre la restitution du volume net et les autres interactions physiques. »

## 2. Décisions

**D1 — L'essai oblique se fait à la frontière, pas seulement dans la primitive.** S314 a montré
que `WaveTrain` **accepte** une direction oblique et la porte en deux composantes. Cela ne dit
rien du **raccord**. Quatre vérifications, et ce sont des critères :

1. l'énergie sortante part dans la **direction attendue** ;
2. les **composantes tangentielles** sont conservées ;
3. le transfert ne crée **ni réflexion ni composante transverse artificielle** ;
4. le résultat **tient quand l'angle varie** — un seul angle ne prouve rien.

**D2 — L'oracle de phase à distance est indépendant, et « indépendant » exclut le train.** Une
somme de modes exacts propage exactement **par construction** : la comparer à sa propre formule ne
mesure rien. L'oracle doit être une **autre physique**. La comparaison doit tenir compte de la
**dispersion**, du **temps de propagation** et de l'**élargissement** — donc les trois se
calculent avant d'être lus, et non après.

**D3 — Un défaut de phase ne se rattrape ni par l'amplitude ni par un décalage temporel.**
L'interdiction est explicite et elle est structurelle : un banc qui offrirait le réglage finirait
par l'employer. C'est le pendant, pour la phase, de ce qu'ADR-182 D8 dit pour le spectre et la
direction.

**D4 — La distance de référence est dix longueurs d'onde.** Elle n'est pas un ordre de grandeur :
c'est la distance sur laquelle un écart de célérité de 1 % déplace la phase d'un dixième de tour,
donc celle où un désaccord cesse d'être invisible.

**D5 — L'ordre C évalue les six propriétés ensemble, et attribue chaque écart.** Amplitude,
direction, spectre, phase, vitesse, réflexion — et pour chacune, dire si elle relève de la
**primitive W**, du **raccord**, ou de la **simulation δ**. Un banc qui publierait six nombres
sans cette attribution ne remplirait pas la décision : c'est l'attribution qui dit où travailler.

**D6 — Le transfert δ → W est déclaré partiel, et le reste jusqu'à l'ordre C.** *« Le transfert
δ → W reste partiel jusqu'à validation des propriétés physiques conjointes. »* Aucun document du
dépôt ne le qualifie autrement.

**D7 — Le volume net reste intégralement dans `pending`** tant que le receveur B ou V n'est pas
opérationnel. ADR-180 D1, ADR-181 D1 et ADR-182 D9 restent entières.

**D8 — A305 et A306 restent ouvertes et avancent en parallèle.** A305 s'isole par les essais
ciblés sur la **bande** puis l'**éponge**. A306 demande d'examiner les **deux emplois** de
l'estimateur de fréquence dans le **harnais** et de déterminer si des **réceptions antérieures**
ont pu être affectées — c'est une question sur le passé du dépôt, pas seulement sur son présent.
Ni l'une ni l'autre ne bloque le développement de la primitive.

**D9 — La capacité visée est un trajet complet.** *« Une perturbation produite par δ, correctement
identifiée, transférée puis propagée dans W. »* Les sessions privilégient l'**achèvement** de ce
trajet sur l'ouverture d'un autre. Ni rendu photoréaliste, ni optimisation prématurée.

## 3. Ce que cette décision ne tranche pas

- **La forme de l'oracle indépendant** : D2 dit ce qu'il ne peut pas être, pas ce qu'il est.
- **Les angles d'incidence à essayer**, ni combien : D1 exige que le résultat tienne quand l'angle
  varie, sans fixer le balayage.
- **Le seuil auquel un écart de phase devient un défaut.** À dix longueurs d'onde, la dispersion
  numérique de δ en produit un qui se calcule ; distinguer ce qui est attendu de ce qui ne l'est
  pas est le travail de la session, et le seuil viendra après.
- **Le receveur du volume net** : ADR-181 D1 le désigne, sa construction est l'ordre D.

## 4. Ce qui devient faux si cette décision est mal lue

**« L'essai oblique est autorisé » ne veut pas dire « la primitive est validée pour l'oblique ».**
D1 porte sur le **raccord**, et la primitive n'en est qu'une moitié.

**« La régression de phase valait 0,9938 » ne dit rien de la propagation.** Elle est mesurée **sur
la ligne d'émission**, où le train est construit pour coïncider. C'est précisément pourquoi D2
existe.

**Et « le transfert fonctionne » reste faux tant que l'ordre C n'a pas eu lieu.** D6 le qualifie de
**partiel**, et aucune mesure isolée ne change ce qualificatif.
