# ADR-054 — Construire W sans faux préalable

- **Statut : ACTÉE**, S71, 2026-09-08, sur délégation technique explicite de l’utilisateur.
- **Confirme** ADR-053 D1 et D3 : construction du système, discipline de connaissance conservée.
- **Remplace** ADR-053 D2/D4 et §3/§4 pour les motifs, l’ordre et la réversibilité technique.
- **Décision** : commencer par W ; B1 contribue au dimensionnement, sans bloquer son écriture.

## 1. Validation du choix

**Oui à la construction.** B et le harnais existent ; les comportements régionaux portés par W
manquent. Continuer exclusivement les audits ne produit ni sillage, ni propagation d’impact,
ni restauration d’événements. La conception doit désormais être confrontée à ces comportements.
Les pourcentages de S69 sont des estimations de maturité, sans dénominateur mesuré ; ils ne
constituent ni une preuve de couverture ni une prévision d’effort.

**Oui à W d’abord**, pour réunir propagation, événements autoritaires et persistance dans une
première tranche intégrable. Ce choix est une priorité d’ingénierie révisable, pas une nécessité
logique. Une contradiction décisive suffit à le rouvrir ; quatre arguments ne demandent pas
quatre réfutations. V est aussi constructible indépendamment de δ : C12 est explicitement un
cas V seul. C19 complet attend B, W **et V** ; construire W n’en valide que sa branche.

## 2. Préalables corrigés

| Assertion de S70 | Constat dans le dépôt | Conséquence |
|---|---|---|
| B1 entièrement exécutable | PLAN-BENCHMARK B1 exige LOD actif/inactif, double aveugle et répétition FFT ; le B actuel permet une mesure CPU partielle | Mesurer le coût lorsque nécessaire ; ne déclarer ni B1 clos ni N final sur cette seule mesure |
| A187 rend N physiquement faux | SPECTRE-DENSE-S67 explique l’écart par les covariances sur une fenêtre finie ; ADR-052 sépare ce diagnostic de la précision | Le nombre de composantes reste à choisir, sans réutiliser une cause réfutée |
| W nécessairement analytique | ADR-001 §2 et ADR-053 §5 laissent paquets et champ 2D ouverts | Choisir un premier candidat sans déclarer la comparaison gagnée |
| W → C02 → coupure finale | Une loi analytique exacte peut être une référence ; elle ne mesure pas l’erreur d’un autre solveur | Distinguer validation de W et décision de partage W/δ |
| B1 bloque WaveEvent | Le coût cumulé B+W contraint le budget ; il ne détermine ni identité, ni autorité, ni unités d’un événement | Commencer le contrat et la construction W immédiatement |

Le coût B+W doit être mesuré **ensemble** avant de recevoir une capacité de production. Aucune
valeur de N, de capacité W ou de λ_cut n’est fixée ici (I-14, I-16).

## 3. Lots de construction et réception

Les lots sont des résultats ; chaque session les découpe en étapes de moins de quinze minutes.
Le premier candidat sera à paquets analytiques sur CPU, en Rust sans dépendance moteur : il
permet d’exercer tôt le rejeu et l’autorité. Son domaine initial est un milieu uniforme ; la
réfraction bathymétrique et les autres candidats restent explicitement à construire pour B2.

| Lot | Produit dans le système | Preuve attendue avant réception |
|---|---|---|
| W1 — événement | Contrat sémantique et représentation versionnée de WaveEvent, construction et décodage contrôlés | Vecteurs d’octets de référence ; rejet des valeurs non finies/hors domaine et versions inconnues ; séparation serveur/local ; unités et bornes par kind |
| W2 — journal | Ingestion, déduplication, ordre stable, expiration et reconstruction d’un instantané W | Même état après livraison désordonnée, doublons et restauration ; conflit de contenu pour un même id signalé ; événement tardif et retrait exercés |
| W3 — propagation | Un impact rejouable, expansion en paquets et interrogation par l’interface commune B+W | Célérité de phase et transport de l’enveloppe distingués ; bilan d’énergie ; refus hors domaine déclaré ; évaluations indépendantes de l’ordre des requêtes |
| W4 — intégration | Sources de sillages, limites régionales, arrivée en cours de partie et budgets bornés | Scénarios B2 progressifs ; branche W de C19 ; saturation du pool sans perte autoritaire silencieuse ; coût et latence de B+W |
| B2 — sélection | Comparaison des candidats recevables, bathymétrie et articulation W/δ | Qualité à coût comparable, persistance et déterminisme croisé ; coupure recevable pour gameplay et navigation, ou conclusion explicitement partielle |

**W1 est la prochaine étape**, action S70-2. Les 50 octets de SPEC-006 §3.1 décrivent une somme
de champs, pas encore un protocole : vérifier encodage, alignement, endian, quantification et
évolution. Avant de figer les bornes, vérifier que les unités de `energy`, `displaced_l`, la
position locale et la durée couvrent les événements que le jeu doit représenter. Un débordement
ne doit pas devenir une saturation silencieuse. Tout changement de signature reçoit son ADR.

W2 doit distinguer fenêtre d’événements récemment publiés et journal nécessaire au rejeu :
un lecteur audio lent ne peut ni consommer l’historique d’un nouvel arrivant, ni conserver
indéfiniment une région. La politique de rétention doit être compatible avec I-06 et I-17.

W3 ne sera pas reçu sur la seule égalité entre deux appels à la même formule. Les références
physiques et les contrôles de reconstruction doivent exercer des erreurs distinctes. Les
cas nominaux ne remplacent ni les limites de validité ni les essais de refus.

## 4. Ambition et limites de validation

La cible AAA étendue impose que les coordonnées lointaines, les référentiels mobiles, les
événements tardifs, les capacités saturées et les restaurations fassent partie de la construction,
pas d’un durcissement final. Les exigences I-03 et I-05 ne deviennent pas vraies par déclaration :
une exécution sur cette machine ne certifie ni le déterminisme entre plateformes ni les budgets
du matériel cible. Ces preuves sont inscrites dans la réception, sans prétendre les posséder.

Invariants relus : I-01 à I-17 ; aucun amendé. En particulier, I-10/I-11 maintiennent le serveur
producteur de causes et excluent la transduction locale de l’autorité ; I-04 laisse δ cosmétique.
Un candidat W qui exigerait un nouvel état autoritaire côté serveur doit instruire cette
contradiction avant adoption. B2 ne peut pas contourner les invariants pour sélectionner un GPU.

La délégation du 2026-09-08 autorise les arbitrages techniques sans retour systématique à
l’utilisateur. Elle ne fournit pas les faits absents sur le moteur, le matériel cible ou le
protocole réseau existant. Les interfaces correspondantes restent des contrats d’intégration
à vérifier. Aucun développement n’est reporté à une équipe observatrice.

## 5. Portée de cette session

S71 tranche la demande de validation et livre cet ordre de construction. Elle n’implémente
aucun lot W et ne revendique aucun cas canonique supplémentaire. La priorité des prochaines
sessions est un produit W exécutable ; un nouvel audit doit répondre à un obstacle rencontré
dans ce produit, ou à un défaut bloquant déjà démontré.
