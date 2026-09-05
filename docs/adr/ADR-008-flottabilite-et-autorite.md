# ADR-008 — Flottabilité, forces sur les solides et frontière d'autorité

- **Statut** : proposée
- **Session** : S01
- **Résout** : `zones_ouvertes §22` (ouvert — décision expérimentale majeure), `§14` partiellement
- **Dépend de** : ADR-001, ADR-003, ADR-007

---

## 1. Décision

> **Toute force capable de modifier une issue de gameplay est calculée à partir de B + W, ou de V.
> Jamais à partir de δ.**

C'est l'invariant I-04. Il n'est pas un compromis de performance : c'est la seule formulation qui
survive simultanément au non-déterminisme du GPU (ADR-001 §1.3) et à la latence de lecture
(ADR-007 §4.1).

`zones_ouvertes §22` posait la flottabilité comme une décision expérimentale à trancher par
benchmark. Elle ne l'est pas au niveau de l'architecture : le benchmark ne peut porter que sur le
*raffinement* du modèle, pas sur sa source de données.

### Répartition

| Effet | Source | Autorité | Réplication |
|---|---|---|---|
| Poussée d'Archimède, assiette, roulis, tangage | B + W | serveur / propriétaire | aucune (recalculée identiquement) |
| Poussée d'une vague sur un joueur, déferlante, bore | W | serveur | événement source |
| Flottabilité dans une citerne, une piscine, une cale | V | serveur | état V |
| **Poussée d'une poche d'air piégée** *(ajouté S05, écart R08)* | V + pose du solide | serveur | état V |
| **Perte de portance en eau aérée** *(ajouté S05, écart R02)* | `A_rep` (B, W_rep, vent) | serveur | aucune (recalculée identiquement) |
| Secousse haute fréquence, gerbe qui frappe la coque, tremblement | δ, `A_local` | **aucune** | aucune |

La contribution de δ s'applique sur la **transformation de rendu**, jamais sur la transformation
physique : un ressort borné (translation ≤ 8 cm, rotation ≤ 3°) entre la pose physique et la pose
affichée. Un client qui ne simule pas δ voit le même bateau au même endroit, avec un peu moins de
vie. Aucune divergence possible, par construction.

---

## 2. Modèle de flottabilité retenu

Intégration de pression sur un **proxy de flottabilité** (jeu de points échantillons attachés à
une enveloppe convexe simplifiée), pas sur le maillage de rendu.

```
Pour chaque point d'échantillon i (volume associé V_i, normale n_i) :
    η_i  = EvalWater(x_i, B|W).height
    d_i  = η_i − z_i                          // immersion signée
    f_i  = ρ_liquide · g_eff · V_i · sat(d_i / h_i)      // sat : lissage sur l'épaisseur h_i
    v_rel= v_solide(x_i) − v_eau(x_i)          // v_eau vient aussi de B+W (vitesse orbitale)
    f_i += −C_n (v_rel·n_i) n_i |v_rel|  −  C_t (v_rel − (v_rel·n_i)n_i) |v_rel|
```

Trois éléments à ne pas omettre :

- **`sat()`** — sans lissage de l'entrée/sortie d'eau d'un échantillon, la force est discontinue et
  le solide vibre à la fréquence d'échantillonnage. Épaisseur de lissage `h_i ≈ dx_proxy`.
- **`v_eau` orbital** — la vitesse orbitale de la houle (et non seulement la hauteur) est ce qui
  fait *avancer* un bateau dans une vague et *rouler* une bouée. L'omettre donne une eau qui
  soulève sans entraîner : défaut classique, immédiatement perceptible.
- **Masse ajoutée** — jamais mentionnée dans les documents sources, et pourtant décisive. Un corps
  qui accélère dans un fluide entraîne une masse d'eau ; pour un mouvement vertical de coque, la
  masse ajoutée est **de l'ordre de la masse déplacée elle-même**. Sans elle, la réponse est trop
  vive, le pilonnement trop raide, et l'intégrateur devient marginalement stable.
  Implémentation minimale : tenseur de masse ajoutée diagonal constant par archétype de coque
  (`m_a` en pilonnement, cavalement, embardée + inerties), appliqué comme masse effective.

## 3. Contrainte de stabilité — le petit objet est le vrai problème

La raideur de flottabilité vaut `k = ρ · g · A_flottaison`. La pulsation propre en pilonnement est
`ω = √(k/(m + m_a))`. Un intégrateur semi-implicite reste sain tant que `ω·dt ≲ 0,3`.

| Objet | A (m²) | m (kg) | ω (rad/s) | ω·dt à 30 Hz |
|---|---|---|---|---|
| Navire 60 m | 600 | 1,2·10⁶ | 2,2 | 0,07 — sain |
| Barque | 6 | 400 | 8,6 | 0,29 — limite |
| Caisse flottante | 1 | 50 | 9,9 | 0,33 — instable |
| Balle de ping-pong | 1,3·10⁻³ | 2,7·10⁻³ | 67,6 | **2,25 — divergence** |

**La flottabilité d'un petit objet léger est numériquement plus difficile que celle d'un
porte-conteneurs.** C'est contre-intuitif et c'est la source la plus fréquente d'objets qui
tremblent ou décollent à la surface.

Règle retenue : à la création d'un corps flottant, on calcule `ω·dt`.

- `ω·dt ≤ 0,3` → intégration normale au pas de simulation.
- `0,3 < ω·dt ≤ 1,0` → sous-cyclage local de la flottabilité (2 à 4 sous-pas), coût négligeable.
- `ω·dt > 1,0` → **bascule en mode contraint** : l'objet ne subit plus de force de flottabilité,
  il est projeté sur la surface (`z = η`, orientation alignée sur la normale, vitesse horizontale
  amortie vers la vitesse orbitale). Visuellement correct, exactement stable, sans coût.

Le mode contraint couvre naturellement les débris, bouteilles, caisses légères, cadavres, glaçons —
c'est-à-dire l'écrasante majorité des objets flottants d'une scène.

## 4. Ballottement en référentiel accéléré

Un compartiment liquide à bord d'un solide mobile (ADR-002 §2.2) déplace son centre de masse. Le
couplage retour est réel et peut faire chavirer.

Décision : rétroaction activée seulement si `m_liquide / m_porteur > 0,05`. Modèle : pendule
équivalent à un degré de liberté par axe horizontal, dont la période propre est celle du premier
mode de ballottement `T = 2π/√(g·(π/L)·tanh(π h/L))` pour un réservoir de longueur L et de
profondeur h. Pas de solveur volumétrique : le mode fondamental suffit, et il est déterministe.

Exemple : cuve de 8 m, remplie à 1,5 m → `T ≈ 3,5 s`. Une houle de 3,5 s entre en résonance avec
la cuve — cas de gameplay intéressant et physiquement juste, obtenu pour un coût nul.

## 5. Ce qui reste ouvert

1. Nombre et placement des points d'échantillon par archétype (20 à 60) → benchmark B6.
2. Valeurs de `C_n`, `C_t` et du tenseur de masse ajoutée par archétype → calibration.
3. ~~Cas du corps qui sort entièrement de l'eau puis retombe : la force d'impact d'entrée
   (*slamming*).~~ **Spécifié en S12 → [ADR-023](ADR-023-mecanismes-restes-a-specifier.md) §2.**
   La grandeur retenue n'est pas la pression `∝ ρv²A` envisagée ici mais l'**impulsion de masse
   ajoutée** `J = Δ(½πρc²)·v_rel` : la pression de pic diverge quand le relèvement de carène tend
   vers zéro — on ne la connaît donc pas — alors que l'impulsion est un bilan de quantité de
   mouvement. Elle réutilise le tenseur de masse ajoutée imposé au §2 ci-dessus, et elle est
   **autoritaire** au titre d'I-15, ce qui permet qu'un claquement casse quelque chose.
4. ~~Nageur / joueur en surface : modèle distinct, probablement cinématique contraint.~~
   **Résolu en S12 → [ADR-023](ADR-023-mecanismes-restes-a-specifier.md) §3, et il n'y avait pas de
   modèle à écrire.** Le mode contraint du §3 ci-dessus reçoit une **seconde condition d'entrée** :
   *un corps contrôlé, en surface, y bascule quel que soit son `ω·dt`*. Le critère existant est un
   critère de **stabilité numérique**, qu'un nageur passe sans difficulté (`ω·dt ≈ 0,14`) alors que
   c'est bien ce mode qu'il lui faut — pour le contrôle et la caméra, non pour la stabilité.
   Ce qui reste à l'équipe personnage est l'animation, la caméra et la vitesse de nage soutenue.
