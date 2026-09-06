# ADR-021 — Autorité des grandeurs dérivées

- **Statut** : proposée
- **Session** : S05
- **Origine** : revue croisée, écarts R02, R03, R05, R08
- **Amende** : ADR-005 §3, ADR-008 §1, ADR-009 §3, ADR-012 §4, ADR-014 §5.2
- **Ajoute** : invariant I-15

---

## 1. Problème

Quatre écarts trouvés en revue croisée posent la même question sous quatre formes :

- une onde issue d'un domaine δ peut-elle devenir autoritaire (R03) ?
- l'aération, qui modifie la flottabilité, est-elle une exception à I-04 (R02) ?
- un paquet W élagué par la dégradation reste-t-il une donnée gameplay (R05) ?
- une poche d'air, qui fait flotter une coque retournée, relève de quelle autorité (R08) ?

Chacune avait reçu une réponse locale, dans l'ADR qui l'avait rencontrée. Les quatre réponses sont
compatibles, mais aucune ne s'appuie sur une règle — et l'une d'elles (R03) contredit un invariant.

---

## 2. Décision : une règle unique

> **I-15 — Une grandeur dérivée est autoritaire si et seulement si tous les participants peuvent la
> calculer à l'identique à partir de données répliquées.**

Ce n'est pas une nouvelle politique : c'est l'énoncé général dont I-04 était le cas particulier.
I-04 devient un corollaire — δ n'est jamais autoritaire *parce que* ses entrées ne sont pas
répliquées et son calcul pas reproductible.

L'intérêt de la formulation générale est qu'elle **répond d'avance** aux cas qui n'ont pas encore
été rencontrés, au lieu d'exiger un arbitrage à chaque nouveau phénomène. Et elle rend impossible
l'argument d'exception : on ne déroge pas à I-15, on constate qu'une grandeur satisfait ou non son
critère.

### Application aux quatre cas

| Grandeur | Entrées | Reproductible ? | Autorité |
|---|---|---|---|
| Houle B | descripteur régional + `T_sim` | oui, bit à bit | **oui** |
| Paquets `W_rep` | événements serveur + `T_sim` | oui | **oui** |
| Paquets `W_local` | transduction δ locale | non | non |
| Aération `A_rep` | B, `W_rep`, vent | oui | **oui** |
| Aération `A_local` | δ, `W_local` | non | non |
| Poche d'air | état V + pose du solide, tous deux répliqués | oui | **oui** |
| Champ δ | solveur non déterministe | non | non |

---

## 3. Conséquence sur l'origine des ondes répliquées *(R03)*

ADR-005 §3 posait que la transduction δ→W était la seule voie par laquelle δ influençait le monde
répliqué, et ADR-009 §3 organisait la validation serveur de cette émission. Or le serveur n'exécute
jamais δ (I-10) : il ne peut donc ni transduire, ni reproduire ce que le client a transduit.

**Décision.**

```
Onde répliquée  ←  émise par le SERVEUR, depuis la cause, au moment de la cause
Transduction δ  →  produit uniquement du W_local, cosmétique, non répliqué
```

Le serveur connaît la cause — il possède la physique des objets. Au moment de l'impact, il émet un
`WaveEvent` d'énergie `K·E_cause`. Le client peut émettre le même événement localement par
anticipation et le réconcilier par `id` (ADR-009 §7.2).

### 3.1 Deux bénéfices, dont un de sécurité

- **Le chemin d'énergie client → serveur disparaît.** L'angle mort A16 — un client modifié
  fabriquant un tsunami — n'est plus atténué par un plafond : il **n'existe plus**, faute de
  chemin. Le mécanisme de plafonnement d'ADR-009 §3 devient sans objet.
- **Le serveur n'a rien à valider**, donc rien à calculer. Sa charge par événement retombe à
  l'émission d'un enregistrement de 40 octets.

### 3.2 L'argument de fermeture

Il fallait vérifier qu'aucun phénomène de conséquence gameplay ne puisse naître exclusivement dans
δ. Il ne le peut pas, **par construction** : tout ce qui dépasse `λ_cut` appartient à W (ADR-001,
ADR-005 §2.1). δ ne contient, par définition, que du court, du local et du bref.

Cette fermeture n'est valide que tant que `λ_cut` sépare effectivement les deux couches. Si le banc
B2 conduisait à relever `λ_cut` au point que des phénomènes gameplay tombent dans δ, cette décision
devrait être rouverte. **À porter au protocole de B2 comme critère de recevabilité de `λ_cut`.**

---

## 4. Conséquence sur la dégradation *(R05)*

ADR-012 §4 rang 6 autorisait « réduire le nombre de paquets W ». Un paquet `W_rep` porte une donnée
gameplay : l'élaguer chez un joueur et pas chez un autre produit deux mondes différents.

**Décision.** L'élagage porte sur :

1. tous les paquets `W_local`, sans restriction ;
2. les paquets `W_rep` dont l'amplitude est passée **sous le seuil de pertinence gameplay** — ils
   ne portent plus rien à répliquer et leur suppression est déterministe, donc identique chez tous.

Un paquet `W_rep` au-dessus du seuil n'est jamais élagué, quel que soit le profil de qualité. Si
le budget ne suffit plus, la dégradation passe au rang suivant ; si aucun rang ne suffit, c'est une
saturation à documenter (SPEC-003 §9.2), pas à masquer.

**Corollaire mesurable** : le nombre de paquets `W_rep` au-dessus du seuil est identique sur tous
les clients. C'est une assertion du harnais, à ajouter au cas C18.

---

## 5. Conséquence sur l'aération *(R02)*

Le champ `A` d'ADR-014 §5 se scinde à la source :

```
A = A_rep(B, W_rep, vent)  +  A_local(δ, W_local)
```

- `A_rep` alimente la flottabilité autoritaire — un nageur ne flotte pas dans l'eau blanche d'un
  déferlement, et c'est vrai pour tous les joueurs simultanément.
- `A_local` n'agit que sur la pose de rendu et le ressenti, plafonné comme la contribution de δ
  (ADR-008 §1).

**Il n'y a aucune exception à I-04**, et la formulation d'ADR-014 §5.2 est corrigée en ce sens.
La distinction est gratuite : les deux sources sont déjà séparées dans le pipeline de production du
champ.

---

## 6. Conséquence sur les poches d'air *(R08)*

Une poche d'air (ADR-015 §3) est déterminée par l'état du nœud V qui la contient et par la pose du
solide, tous deux répliqués, via une équation d'état déterministe. Elle satisfait I-15 : **elle est
autoritaire**, et rejoint la ligne V de la table d'ADR-008 §1.

Conséquence pratique : le point de non-retour d'une coque retournée qui coule (ADR-015 §3) est le
même pour tous les joueurs. C'est nécessaire — c'est un événement de gameplay, pas un effet.

---

## 7. Ce que cette décision coûte

- **ADR-009 §3 perd son mécanisme de plafonnement**, remplacé par une émission serveur. Le
  paramètre `K` (fraction d'énergie transmise à la surface) survit, mais côté serveur, comme
  paramètre d'équilibrage et non comme garde-fou anti-triche.
- **La transduction δ→W perd son statut de mécanisme d'autorité** et devient un simple procédé de
  continuité visuelle. Son intérêt est intact — sans elle, une onde s'arrêterait au bord d'un
  domaine — mais son enjeu baisse, ce qui autorise à la dégrader librement sous contrainte de
  budget (voir la note corrective d'ADR-012 §4, écart R06).
- **Le serveur doit connaître les causes assez finement** pour émettre des événements crédibles :
  masse, vitesse, angle d'entrée. Il les possède déjà pour la physique des solides ; il faut
  simplement que le point d'émission soit branché là et non dans le système d'eau du client.

## 8. Ce qui reste ouvert

1. Seuil de pertinence gameplay d'un paquet `W_rep` — en amplitude ou en énergie ? Proposition :
   amplitude, comparée à une fraction de `Hs` local. À calibrer avec B8.
2. Table `E_cause` par type d'objet et valeur de `K` — inchangée depuis ADR-009 §7.1, mais elle
   change de propriétaire : c'est désormais une donnée d'équilibrage gameplay.
3. Recevabilité de `λ_cut` au regard de l'argument de fermeture §3.2 — intégré au protocole B2
   *(ajout S05, et second fondement ajouté en S15)*. **Le mode d'emploi est dans
   [`DOSSIER-B2`](../validation/DOSSIER-B2.md) §6**, qui énumère les phénomènes gameplay ondulatoires
   et leur longueur caractéristique : le plus court est le **sillage à 5 m/s, 16 m**, de sorte que le
   critère de fermeture laisse de la marge jusqu'à `λ_cut ≈ 6 m`. Ce n'est donc pas lui qui borne
   `λ_cut` — c'est l'éponge du domaine d'impact (ADR-005 §5, note S16).
   → **S11** : le critère porte désormais **deux** conséquences et non une. SPEC-006 §5.6 s'appuie sur le
   même argument de fermeture pour justifier que le signal de traversabilité ignore δ. Un
   relèvement de `λ_cut` remettrait donc en cause l'autorité des ondes répliquées **et** la validité
   du signal de navigation. Un banc qui n'en vérifierait qu'une laisserait passer l'autre.
