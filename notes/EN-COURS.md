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

Session : S519 — **en cours**. En autonomie : **le lot des registres** (dû à S519), puis **C07 en eau profonde** (3.2) — le cas canonique
« angle de Kelvin à ±2° » n'a jamais été exécuté : le sillage de W (source de pression gaussienne mobile, `wake_source`) n'a jamais été
mesuré contre sa théorie. La leçon de S517 : trois instruments d'angle, aucun éprouvé avant d'être appliqué.

**Ce que la session fait.** (a) **Une référence indépendante de W** (numpy) : la réponse linéaire exacte en temps de l'eau profonde à une
pression gaussienne `p₀·exp(−r²/2σ²)` (la charge de `wake_source`, `p₀ = F/2πσ²`) partie du repos à `t = 0` et menée à `U` constante — par
mode, `η̂(k,T) = −(k/ρ)·p̂(k)·∫₀ᵀ sin ω(T−s)/ω · e^{−i kₓ U s} ds`, en forme fermée, sur une grille FFT assez grande pour que le transitoire
n'y revienne pas. (b) **L'instrument d'angle éprouvé sur la référence d'abord** : la moyenne de |η| le long des rayons issus de la source
(celui de S517), sur la plage de distances où le sillage est établi ; `σ` et `U` choisis sur la seule référence pour qu'il y lise Kelvin
à 1° (un sillage dominé par les cuspides) — si aucun couple raisonnable ne le permet, l'instrument est changé avant de toucher W. (c) **W
mesuré** aux mêmes paramètres : recette 256 × 256 à coupure 6 (domaine honnête d'ADR-132 : 89 m, 26,2 s), `T` = 24 s.

**Ordre de grandeur, calculé.** À `U` = 2 / 2,5 / 3 m/s : `λ₀ = 2πU²/g` = 2,56 / 4,0 / 5,76 m ; le sillage établi jusqu'à `U·T/2` = 24 / 30
/ 36 m derrière la source à `T` = 24 s ; `Fr_σ = U/√(gσ)` = 0,45 / 0,56 / 0,68 à σ = 2 m. 65 536 nœuds par point échantillonné.

**Critères, écrits avant.** (1) **La référence et l'instrument** : sur la référence, l'instrument lit 19,47° à 1° (sinon pas de mesure de
W). (2) **Le champ de W** contre la référence, dans la zone établie (de 2 λ₀ derrière la source jusqu'à `U·T/2` − 2 λ₀, dans le coin de
Kelvin élargi à 30°) : écart quadratique relatif ≤ 10 %. (3) **C07 profond** : l'instrument sur W lit 19,47° à 2°. L'eau peu profonde de
C07 (`arcsin(1/Fr_h)`) reste hors de portée — W est en eau profonde (aucune `tanh` dans la pression de W) ; 3.2 le dira.

### Plan

- [ ] **P1** — jeton ; le lot des registres (feuille de route S517–S518) ; plan.
- [ ] **P2** — la référence et l'instrument éprouvé ; (1).
- [ ] **P3** — W mesuré ; (2), (3).
- [ ] **P4** — preuve ; liste 3.2 ; CAS-CANONIQUES (C07) ; rituel (`--lot`).

### Notes de reprise
