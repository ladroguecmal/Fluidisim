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

Session : S579 — **en cours**. En autonomie, **2.2 — la marée dans la surface de B** : S577–S578 calculent le niveau ; il doit entrer
dans l'échantillon que B publie (`WaterSample`), que la composition B + W (ADR-062) et ses consommateurs lisent.

**Ce que la session fait.** `Maree::vitesse(t)` et `CarteCotidale::niveau_et_vitesse(x, y, t)` : `∂η/∂t = −Σ Aₖ·ωₖ·sin(ωₖt − gₖ)` (la
pulsation de la fréquence **arrondie**, celle des phases). `maree::avec_maree(échantillon, niveau, vitesse)` : `η += τ`, `∂η/∂t += τ̇`, et la
vitesse verticale de surface `w += τ̇` (la condition cinématique) ; le reste de l'échantillon inchangé. Une marée nulle laisse
l'échantillon de B tel quel. **Ne fait pas** : le courant horizontal de marée (il demande la profondeur : `u = η·√(g/h)` pour une onde
progressive), la pente de la marée (k·A ≈ 10⁻⁵, sous le visible), l'adoption par défaut.

**Références, calculées avant** (ce script les écrit). M2, 1 m : la fréquence arrondie `96054` (Q32), `ω` = **1.405191332e-04 rad/s**,
`|∂η/∂t|` ≤ **1.405191e-04 m/s**.

**Quantum** (ADR-236 D1) : la vitesse en f32, ulp ≈ 1.5e-11 m/s ; la somme B + marée, un ulp de `|η|` (≈ 2.4e-07 m). **Critères,
écrits avant.** (1) `vitesse(t)` contre `−A·ω·sin(ωt)` (la même `ω`, en f64) à 10⁻⁹ m/s sur 25 h (rapport ≈ 70 à l'ulp) ; la carte de S578
au nœud contre la même forme ; (2) sur B réel (un fond de S259 ou le plus simple disponible), `η_total − η_B` à 2 ulp de `|η|` de la marée
et `w_total − w_B` à 2 ulp ; (3) une marée nulle : l'échantillon identique (champ à champ) ; (4) la composition B + W (`compose`) accepte
l'échantillon avec marée et rend `η` = celui d'avant + τ, à 2 ulp.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — la vitesse, `avec_maree` ; (1)–(4).
- [ ] **P3** — preuve ; liste 2.2 ; rituel.

### Notes de reprise
