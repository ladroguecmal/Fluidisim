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

Session : S538 — **en cours**. En autonomie, **5.9 — compartiments et inondation limitée par l'air** (C17, ADR-015 T2) : dans V, l'air
est implicite (T0) ; un compartiment étanche se remplit comme s'il avait un évent — « tous les temps d'avarie du jeu sont trop courts ».

**Ce que la session fait.** `hydro_network::step_air` : le pas de V avec, par nœud, un **état d'air** — ouvert (l'air à la pression
atmosphérique, T0 : le pas d'avant au bit) ou **scellé** (une poche isotherme, `p·V_air` constant, ADR-015 §3 « lente »). La pression de
jauge d'une poche entre dans les charges des arêtes (`(p − p_atm)/ρg`) — l'eau qui entre comprime l'air, qui la retient. Les têtes de
pression dans un tampon de l'appelant (I-06). Ni évent à débit limité (un nœud est scellé ou ouvert), ni effet sur les pompes.

**Ordre de grandeur, calculé.** C17 : un compartiment de 10 m³ (5 m² × 2 m, son plafond à la flottaison), une brèche de 1 dm² à 2 m sous
la flottaison, `C_d` = 0,62, ρ = 1 025 kg/m³ : **sans évent**, l'équilibre `p_atm·2/u = p_atm + ρg·u` donne `u` = 1,710 m d'air, **0,290 m
d'eau** (14,5 %), l'air à 118,5 kPa ; **avec évent**, Torricelli `t = (2A/(C_d·a·√(2g)))·(√H − √(H − h))` : 99 % en 467 s, plein en
518 s.

**Critères, écrits avant.** (1) Sans air scellé, `step_air` rend `step_meteo` au bit. (2) Sans évent, la hauteur finale à 0,5 % de 0,290 m
(le quantum : 1 ml sur 5 m², 0,2 µm — rapport 10⁴, ADR-236 D1). (3) Avec évent, 99 % à 1 % de la loi de Torricelli. (4) L'assertion de
C17 : le rapport des temps de remplissage supérieur à 5 (sans évent, il ne se remplit jamais).

### Plan

- [ ] **P1** — jeton, plan seul.
- [ ] **P2** — `step_air`, les essais ; (1)–(4).
- [ ] **P3** — preuve ; liste 5.9 ; C17 ; rituel.

### Notes de reprise
