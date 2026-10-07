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

Session : S647 — **en cours**. En autonomie vers la v2 ; la campagne du rouleau 3D. **Étape 3 — une onde qui déferle sur la pente**, dans
APIC 3D, fond en escalier, l'air balistique actif (S645).

**Le jugement : la classification de Grilli, Svendsen et Subramanya (1997)**, vérifiée en ligne : le paramètre de pente
`S₀ = 1,521·s/√(H₀/h₀)` ; glissant si `S₀ < 0,025`, plongeant si `0,025 < S₀ < 0,30`, frontal si `0,30 < S₀ < 0,37`, pas de déferlement
au-delà. Leurs formules empiriques de la profondeur et de l'indice au déferlement n'ont pas pu être relues : elles ne jugent rien ici,
`H_b/h_b` est seulement rapporté.

**Le lecteur (ADR-263 D2), éprouvé d'abord.** Le retournement : dans une colonne de la rangée médiane, en montant depuis le fond, de l'eau,
puis de l'air, puis de l'eau (`labels`) — la surface n'est plus un graphe ; l'écart d'air en mailles. Éprouvé sur deux cas posés à la main,
reconstruits sans mouvement : une couche d'eau plate (aucun retournement) ; la même avec une lèvre d'eau posée au-dessus d'un vide d'air
(retournement trouvé, à l'abscisse et avec l'écart posés).

**Références, calculées avant** (ce script). Montage : `d` = 0.5 m, `H` = 0.15 m (`H/d` = 0.3), pente 1:12 : **`S₀` = 0.231 —
plongeant**. L'onde à 2.30 m du pied (pied à 5.696 m), le rivage au repos à 11.70 m, domaine 12.8 × 1.0 m. Le cas de S644 :
`S₀` = 1.13 > 0,37, pas de déferlement (vérifié en S644 à l'œil, ici par le lecteur). **Bornes assertées** (ADR-257 D1) : la queue
de l'onde, le rivage dans le domaine, la crête sous le couvercle, aucune égalité centre/fond.

**Critères, écrits avant.** (1) le lecteur : aucun retournement sur la couche plate, le retournement posé trouvé ; (2) les particules
gardées, aucune sous le fond ; (3) **à 2,5 cm, l'onde déferle en plongeant** — un retournement d'au moins une maille d'air sous de l'eau,
avant le rivage au repos ; (4) l'onde de S644 (`S₀` > 0,37) **ne déferle pas** (aucun retournement) ; (5) rapportés : le point de
déferlement, `h_b`, `H_b/h_b`, la maille de 5 cm.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — le lecteur et son épreuve ; le déferlement ; (1)–(5).
- [ ] **P3** — preuve ; liste 4.14, 4.16 ; rituel.

### Notes de reprise
