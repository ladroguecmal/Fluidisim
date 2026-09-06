# ADR-003 — Horloge de simulation et déterminisme du fond

- **Statut** : proposée
- **Session** : S01
- **Dépend de** : ADR-001, ADR-002
- **Comble** : angles morts A08, A28

---

## 1. Problème

ADR-001 fait reposer toute la cohérence multijoueur de B et W sur une seule affirmation :
*« la fonction est identique chez tous les clients »*. Cette affirmation n'est vraie que si trois
conditions sont réunies. Aucune n'est acquise par défaut, et les trois sont des sources classiques
de bugs découverts tard.

1. Tous les clients évaluent B au **même temps**.
2. Tous les clients évaluent B avec la **même arithmétique**.
3. Le temps utilisé a une **précision suffisante après plusieurs jours de session**.

---

## 2. Décision

### 2.1 Horloge

- `T_sim` : entier non signé 64 bits, en **microsecondes** depuis une époque fixe du monde.
  Portée : 584 000 ans. Autoritaire serveur.
- Toute évaluation de B et W prend `T_sim` en entrée, jamais un temps local, jamais un accumulateur
  de `deltaTime`.
- Le client estime l'offset serveur par un filtre à la Cristian/NTP simplifié, puis **n'applique
  jamais de saut** : la correction se fait par modulation du taux, bornée à ±0,1 % (soit 1 ms/s),
  sauf resynchronisation dure au chargement ou après une coupure > 5 s.

**Tolérance requise.** Une erreur d'horloge Δt produit une erreur de hauteur
`Δz ≈ A·ω·Δt`. Pour A = 1 m et T = 8 s (ω = 0,785 rad/s), Δt = 20 ms donne Δz = 1,6 cm.
Cible retenue : **|Δt| < 20 ms en régime établi**, ce qui est atteignable sans effort particulier.

### 2.2 Le piège du temps en float32

Un accumulateur `float32 t` incrémenté chaque frame, après 12 jours de session (10⁶ s), a un ulp
de **0,0625 s**. La houle se fige puis saute par paliers de 62 ms. Après 4 heures (1,4·10⁴ s),
l'ulp est déjà de 1 ms et le mouvement devient irrégulier à haute fréquence.

Règle : **le temps ne transite jamais en float32.** Le passage vers le GPU se fait par la phase
déjà repliée :

```
phase_i = fmod( ω_i · (T_sim · 1e-6), 2π )      // calculé en f64 côté CPU
```

Seul `phase_i` (∈ [0, 2π[) est envoyé en float32. Erreur bornée à 7,5·10⁻⁷ rad, définitivement.

### 2.3 Reproductibilité arithmétique de B

Pour que B soit bit-à-bit identique entre clients :

- La fonction `EvalBackground` est compilée en **sémantique IEEE-754 stricte** :
  `-ffp-contract=off`, pas de `-ffast-math`, pas de réassociation. La contraction FMA seule suffit
  à produire des écarts d'un ulp qui, sommés sur 128 composantes, deviennent visibles.
- L'ordre de sommation des composantes est **fixé par index**, jamais par ordre d'arrivée ni
  parallélisé sans réduction déterministe.
- Les paramètres spectraux sont dérivés d'un PRNG à **état entier** (PCG ou équivalent), semé par
  `(region_seed, component_index)`, jamais par un flux séquentiel — l'ordre d'évaluation ne doit
  pas influer.
- La table de composantes est **globale et immuable** (ADR-004 §3) : mêmes k, mêmes directions,
  mêmes phases initiales partout sur la planète.

**Portée du déterminisme exigé** : identité *numérique* entre clients pour B et W. δ n'est pas
concerné et n'a aucune exigence.

### 2.4 Vérification en continu

Un *hash de conformité* est calculé côté client à basse fréquence (1 Hz) :
`h = fnv1a( quantize(B(x_k, T_sim), 1 mm) for k in 16 points canoniques de la région )`, comparé
au hash serveur. Une divergence signale immédiatement une régression de compilation, un pilote
fautif ou un client modifié. Coût : 16 évaluations/s et 8 octets/s.

C'est le seul mécanisme réseau permanent lié à la houle.

---

## 3. Conséquences

- **Arrivée en cours de partie** : rien à transférer sinon `T_sim`, les descripteurs de région et
  la liste des événements W encore vivants. Une session rejointe est cohérente en un aller-retour.
- **Reconnexion** : identique. Aucun état d'océan à reconstruire.
- **Rejouabilité / replay / tests** : un scénario est entièrement défini par `(T_sim initial,
  descripteurs, journal d'événements)`. Cela rend le harnais de validation (voir
  `validation/PLAN-BENCHMARK.md`) trivial à construire — bénéfice majeur et non anticipé.

---

## 4. Ce qui reste ouvert

1. ~~**Dilatation ou accélération du temps**~~ — **tranché en S18 par
   [ADR-027](ADR-027-les-cinq-arbitrages-tranches.md) §2 : `T_sim` peut être arrêté ou avancé
   **globalement**, jamais mis à l'échelle **par joueur**.** Les trois besoins cités — voyage rapide,
   pause, mode photo — n'en demandent aucun une fois traités séparément : le voyage rapide
   est un déplacement dans l'espace, la pause en solo arrête l'horloge pour tout le monde, le mode
   photo fige le rendu. L'échelle globale reste disponible et ne coûte rien. Si un effet temporel par
   joueur devenait indispensable, il s'appliquerait à un plan d'eau **local déclaré non répliqué**,
   jamais à l'océan. *Position d'origine, conservée pour l'historique :*
   `T_sim` est le temps du monde et n'est **jamais** mis à l'échelle par joueur. Un joueur en
   accélération temporelle voit un océan qui avance à la vitesse du monde. Si le design impose le
   contraire, la cohérence multijoueur de B est perdue et il faut basculer l'océan concerné en
   couche locale non répliquée. **Décision de design, à trancher par l'équipe gameplay.**
2. Ordre de grandeur du nombre de composantes de B (64 / 128 / 256) → benchmark B1.
3. ~~Faut-il étendre le hash de conformité à W ?~~ **Clos.**
   → **S11** : la réponse est « oui » depuis S03 : SPEC-003 §2 place W répliqué dans le régime D1 avec le hash
   pour verdict, et depuis S10 l'invariant I-03 l'énonce lui-même. Répondu par supposition d'un
   document ultérieur, jamais constaté ici.
