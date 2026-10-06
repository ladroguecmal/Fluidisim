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

Session : S570 — **en cours**. En autonomie, **7.7 — danger et traversabilité** (absent ; ADR-018, SPEC-006 §5). Première pièce :
**l'échantillon** et **le prochain franchissement de seuil**.

**Ce que la session fait.** `traversabilite.rs` : `echantillon(profondeur, courant, glace…)` — le produit de danger **`HR = d·(v + 0,5)`**
(ADR-018 §3), sa classe (faible < 0,75 ≤ dangereux pour certains < 1,25 ≤ pour la plupart < 2,5 ≤ pour tous ; bornes basses incluses :
« 50 cm à 2 m/s, HR = 1,25, emporte déjà un adulte »), la classe de profondeur d'un humanoïde (§2 : 0,15 ; 0,50 ; 1,00 ; 1,30 m) ;
`prochain_franchissement(profondeur(t), t₀, horizon)` — le temps avant que la profondeur franchisse l'un des seuils, par un balayage au
pas donné puis une bissection, et le sens (`trend`) ; la cause supposée (`CrossCause` : Aucune, Marée, Débit, Commande — SPEC-006 §5.4)
portée telle quelle. Le courant est celui de surface, jamais l'orbitale (§5.5) : l'appelant le fournit. Les tuiles et la publication
(§5.2) viendront ensuite ; la marée de B n'existe pas encore (2.2) — l'essai prend une marée analytique.

**Références, calculées avant** (ce script les écrit). HR : 0,5 m à 2 m/s → **1.25** (dangereux pour la plupart) ; 0,3 m à
0,5 m/s → **0.30** (faible) ; 1,0 m à 1,0 m/s → **1.50** (pour la plupart) ; 1,2 m à 2,0 m/s → **3.00**
(pour tous). Une marée M2, `d(t) = 0,8 + 0,4·sin(2πt/T)`, `T` = 12,42 h : depuis la mi-marée montante, 1,0 m franchi dans **3726.000 s**
(`T/12`) ; depuis la pleine mer, 1,0 m redescendu dans **7452.000 s** (`T/6`) ; depuis la mi-marée descendante, 0,5 m dans **6034.925 s**
(`asin(0,75)·T/2π`).

**Quantum** (ADR-236 D1) : la bissection à 1 ms ; `HR` en f32 (`half` dans SPEC-006 : 10⁻³ relatif). **Critères, écrits avant.** (1)
chaque `HR` à 10⁻⁶ et sa classe ; les classes de profondeur aux bornes ; (2) chaque franchissement à 10 ms (rapport 10), le bon seuil et le
bon sens ; aucun franchissement dans l'horizon → `None` ; (3) refus : profondeur ou courant négatifs ou non finis, pas ou horizon non
positifs.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `traversabilite.rs` et ses essais ; (1)–(3).
- [ ] **P3** — preuve ; liste 7.7 ; rituel.

### Notes de reprise
