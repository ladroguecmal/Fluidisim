# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S558 — **en cours**. En autonomie, **4.18 — l'énergie du chemin coupé** : S557 a établi l'invariant du pas linéaire sur fond
plat ; « manquent l'énergie du pas couplé et du chemin coupé ». Le chemin coupé (fond quelconque, mailles en partie solides) est celui de
toute scène réelle.

**La dérivation, depuis le code** (`divergence_cut`, `correct_cut`, `apply_cut`). La divergence pèse chaque face par son ouverture `a_f` ;
la correction s'applique, sans poids, aux seules faces ouvertes entre deux mailles fluides (le couvercle à sa demi-maille) ; les faces
ouvertes sur du solide ne sont jamais corrigées et restent au repos. La sommation par parties de S557 tient alors **dans le produit
scalaire pondéré par les ouvertures** : `⟨v, G p⟩_a = Σ a_f·v_f·(G p)_f·dx³ = Σ p_c·w_c·dA` pour `D_a v = 0`. L'invariant devient
`Q_a = ½ρ·Σ a_f·ω_f·u_f²·dx³ + ½ρg·Σ (η^n − z₀)·(η^{n+1} − z₀)·dA` (`ω` = ½ au couvercle, entièrement ouvert par contrat), égal à E₀ au
départ. Le même calcul sans le poids `a_f` (le témoin) n'a aucune raison d'être conservé.

**Le montage.** La cuve de S557 (16 × 8 × 6 mailles de 25 cm, z₀ = 1,5 m), un fond en pente de 0,2 à 0,7 m avec une bosse de 0,3 m
(centrée en 2,5 ; 1,2 m), le haut du fond sous 1,0 m : la couche du couvercle reste entièrement mouillée. La bosse de 2 cm de S557,
12 000 pas de 10 ms. Le plancher : celui de S557, ≈ 10⁻⁶ de E₀ (calculé alors ; l'énergie de la bosse ne dépend pas du fond).

**Critères, écrits avant.** (1) `|Q_a/E₀ − 1|` < 10⁻⁴ à chaque pas (rapport au plancher : 100). (2) La hausse de `Q_a` d'un pas à
l'autre, au pire, sous 10⁻⁵ de E₀. (3) Le montage coupe vraiment : des ouvertures strictement entre 0 et 1 existent (comptées). Le témoin
sans poids, publié sans critère.

### Plan

- [x] **P1** — jeton ; la dérivation ; plan.
- [ ] **P2** — l'essai ; (1)–(3).
- [ ] **P3** — preuve ; liste 4.18 ; rituel.

### Notes de reprise
