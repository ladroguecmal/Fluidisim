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

Session : S592 — **en cours**. En autonomie (ADR-247), **2.5 — les canaux** (absent) ; la base de 2.4 (les rivières).

**Ce que la session fait.** Une loi d'arête de V : **`Flow::Manning { width_mm, length_mm, roughness_e6, outlet_slope_e6 }`** — un bief de
canal rectangulaire entre deux nœuds : `Q = (1/n)·A·R^(2/3)·√S_f`, `A = b·ȳ`, `R = A/(b + 2ȳ)`, la profondeur moyenne `ȳ` des deux côtés au
seuil, la pente de frottement `S_f = Δh/L` entre les deux surfaces ; vers dehors (`to = None`), la sortie en régime uniforme : `S_f` = la
pente du lit donnée, `ȳ` la profondeur amont. Paramètres entiers (I-10 ; ADR-249 D2) : `n·10⁶`, la pente `·10⁶`. Ajoutée à la validation,
à l'instantané (son empreinte) et au pas ; les liquides de V l'héritent (la couche au seuil).

**Le montage.** Dix biefs de 100 × 5 m, le lit descendant de `S·L` = 0,1 m de bief en bief (`S` = 10⁻³), chaque arête au milieu de la
marche (la profondeur moyenne y vaut celle des biefs en régime uniforme) ; 5 m³/s apportés au premier (la pluie sur 1,8 km² à 10 mm/h) ;
la sortie en régime uniforme ; `n` = 0,015 ; trois heures au pas d'une seconde.

**Références, calculées avant** (ce script les écrit et vérifie ses rapports seuil/quantum, ADR-249 D1). La **hauteur normale** (bissection
sur Manning) : **`y_n` = 0.706106 m** ; la vitesse 1.4162 m/s, le Froude 0.538 (fluvial) ; la constante de temps d'un bief ≈ 13.6 s
(le pas d'une seconde est stable) ; le remplissage ≈ 706 s ; l'essai dure 10800 s.

**Quantum** (ADR-236 D1) : 1 ml sur 500 m² (2e-09 m) ; le débit, 1 ml par seconde (2·10⁻⁷ relatif). **Critères, écrits avant.**
(1) la profondeur de chacun des dix biefs à 1 mm de `y_n` ; (2) le débit de chaque arête, en moyenne sur la dernière minute, à 10⁻³ de
5 m³/s ; (3) le bilan exact au millilitre ; (4) l'instantané de V accepte la loi et la restaure au bit (l'empreinte la porte) ; la suite
entière inchangée.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — la loi, l'instantané, l'essai ; (1)–(4).
- [ ] **P3** — preuve ; liste 2.5 ; rituel.

### Notes de reprise
