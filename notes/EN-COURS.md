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

Session : S497 — **terminée**. En autonomie, **6.3, un corps en marche produit son sillage** : l'émetteur de sillage (ADR-104) enchaîne
des tronçons déclarés par l'hôte ; ADR-103 : « une trajectoire déclarée, pas un flux de poses moteur ». Rien ne choisit les tronçons
depuis un corps qui bouge.

**Ce que la session fait.** `RigidBody::wake_leg` : le tronçon suivant de l'émetteur, visé du curseur vers la position **prédite** du
corps à la fin du tronçon (`x + v·Δ`), sous sa charge `m·g` ; le chemin de la source reste continu (l'émetteur l'exige) et se recale à
chaque tronçon. Un essai : la coque de la porte D (4 × 1,6 × 1 m, 3 200 kg, 31,4 kN) menée par le jeu sur un cercle de 20 m à 3 m/s,
tronçons de Δ = 1, 0,5, 0,25 s, contre la même trajectoire déclarée exacte (tronçons de 0,2 s sur le cercle).

**Ordre de grandeur, écrit avant (ADR-226 D3).** L'erreur de visée en fin de tronçon : `δ = ½·(U²/R)·Δ²` = 5,6 cm à 0,5 s ; elle se
recale à chaque tronçon (pas d'accumulation). Le terme concurrent, l'onde : `k·δ` avec `λ = 2πU²/g` ≈ 5,8 m → ≈ 6 % au plus, ≈ 2 % en
moyenne sur le tronçon ; en `Δ²`.

**Critères, écrits avant.** (1) le curseur suit le corps : à chaque fin de tronçon, l'écart au corps ≤ `½·(U²/R)·Δ²` × 1,2 (la formule
dans l'essai) ; (2) la charge publiée : `P₀ = m·g/(2πσ²)` à l'arrondi ; (3) le sillage émis contre le déclaré, aux points d'une grille
autour du cercle, à trois instants : ≤ 10 % de max|η| à Δ = 0,5 s (prévu 2 à 6 %), et un ordre ≥ 1,7 sur Δ = 1 / 0,5 / 0,25 s ; (4)
aucune admission refusée. 6.3 : le corps en marche couplé à la source.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `wake_leg` ; l'essai (1)–(4).
- [x] **P3** — preuve ; lot des registres (dû) ; rituel.

### Notes de reprise
- **P2** — `RigidBody::wake_leg`. Curseur/corps 0,2244 / 0,0560 / 0,0140 m (prédits 0,2250 / 0,0563 / 0,0141) ; P₀ = 1 249,048 Pa au bit ;
  sillage émis/déclaré 4,47 / **1,20** / 0,34 % (max|η| 21 cm) ; ordres 1,89 et 1,83 ; 0 refus. Impasses : le journal emprunte les
  émissions (préparées et acquittées d'abord, admises ensuite) ; un contrôleur ne se construit pas sur un journal vide. 655 essais.
- **P3** — preuve SILLAGE-EMIS-S497 ; **6.3 validée** ; lot : feuille de route (6 / 71 / 43, S495–S497), liste, index ; journal.
