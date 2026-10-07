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

Session : S622 — **en cours**. En autonomie (ADR-247 : la physique des partiels). **11.3** — un manque nommé en S614 : *le niveau du large
imposé au bord* du domaine local (en S614, l'onde était posée en condition initiale).

**Ce que la session fait.** `SaintVenant2D::pas_avec_bord(dt, t, extérieur)` : la face gauche devient une **frontière caractéristique** —
l'invariant entrant de l'extérieur `w⁺ = u_e + 2√(g·h_e)`, le sortant de la maille de bord `w⁻ = u₀ − 2√(g·h₀)`, l'état fantôme
`c = (w⁺ − w⁻)/4`, `h = c²/g`, `u = (w⁺ + w⁻)/2`, `v = v₀` ; le flux de Rusanov entre le fantôme et la maille de bord remplace la pression de
paroi. Ordre un et ordre deux (Heun : l'extérieur à `t`, puis à `t + dt`). Ne fait pas : les trois autres faces, une frontière oblique,
l'extérieur lu dans W en 2D.

**Références, calculées avant** (`s622_ref.py`, numpy). Une impulsion d'onde longue (2 mm, `σ` = 20 m) sur 10 m de fond ; un bassin plat de
400 m fermé à droite ; ordre deux. **Forcé** (le domaine commence au bord, l'impulsion entre par lui) contre **étendu** (le domaine commence
900 m plus tôt, l'impulsion posée dedans) : l'écart à la jauge (200 m) jusqu'à 120 s, rapporté à la crête — maille 1, ½, ¼ m :
**0.029137, 0.013123, 0.005931** — il converge avec la maille (l'étendu porte 150 m de diffusion numérique en plus : une première
mesure de 2 cm à l'onde solitaire, plus large que le bassin, et une seconde à 2 cm, où la propagation non linéaire s'ajoutait, l'ont montré
au plan). **L'absorption** : après la sortie de l'onde réfléchie par le mur, il reste dans le domaine forcé
**1.218e-08, 1.577e-09, 1.038e-09 m** — moins de 10⁻⁵ de l'amplitude.

**Quantum** : f64. **Critères, écrits avant.** (1) aux trois mailles, l'écart et le reste égaux aux références à 10⁻¹² m ; (2) l'écart relatif
divisé par au moins 2 à chaque raffinement, sous 1 % à ¼ m ; (3) le reste sous 10⁻⁵ de l'amplitude ; (4) un bord forcé par un extérieur au
repos garde le bassin au repos (vitesse sous 10⁻¹⁴ m/s, 500 pas : assemblage) ; (5) les essais de S613–S620 inchangés.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `pas_avec_bord` et ses essais ; (1)–(5).
- [ ] **P3** — preuve ; liste 11.3 ; rituel.

### Notes de reprise
