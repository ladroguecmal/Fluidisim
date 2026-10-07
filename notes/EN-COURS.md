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

Session : S653 — **en cours**. En autonomie vers la v2 (ADR-247) ; 6.7 : « le corps libre que le rouleau emporte ».

**Ce que la session fait.** La sphère d'APIC devient **libre** (`Apic3::set_body_mass`) : à chaque pas, après la projection, la force de
pression sur elle — l'instrument de S652 passé dans le cœur (`Apic3::body_force`) — et son poids (`g_eff`) changent sa vitesse
(`v += dt·(F/m + g)`) ; le pas suivant impose cette vitesse à l'eau. Le couplage est **explicite** : il est connu pour être instable pour
un corps plus léger que sa masse ajoutée (½ρV pour une sphère) ; le corps essayé (densité 500) pèse le double de sa masse ajoutée. Un
**contact** simple : le corps ne descend pas sous le fond de sa colonne ni ne sort du domaine (vitesse normale annulée). Sans masse, le
corps est imposé comme avant — au bit.

**Références, calculées avant** (ce script). La sphère `r` = 0,1 m, densité 500 : `V` = 0.004189 m³, `m` = 2.0944 kg. À l'équilibre,
Archimède la met à mi-immersion (centre au niveau) ; l'instrument lit 1,13 × Archimède (S652) : le corps doit flotter environ 0.9
cm trop haut.

**Critères, écrits avant.** (1) **La flottaison** : la sphère lâchée 3 cm sous son équilibre, en eau au repos (40 × 8 × 20 mailles de 5 cm,
eau à 0,4 m), oscille et se pose — entre 3 et 4 s, l'amplitude de sa vitesse verticale sous 2 cm/s, son centre à 2 cm du niveau ; aucune
vitesse non finie. (2) **Le rouleau l'emporte** : la même sphère libre, posée à x = 10,4 m dans le relais de S652 (sur sa marche, à
mi-immersion), avance vers la plage de plus de 0,5 m dans les 1,5 s qui suivent le retournement ; sa vitesse au plus celle de la colonne
de la sonde ; la masse de l'eau exacte. (3) Refus : une masse nulle, négative ou non finie. (4) Sans masse, les essais d'APIC 3D inchangés.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — le corps libre dans le cœur ; (1)–(4).
- [ ] **P3** — preuve ; liste 6.7, 6.4 ; rituel.

### Notes de reprise
