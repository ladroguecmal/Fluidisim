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

Session : S533 — **terminée**. En autonomie ; deux maillons : une capacité. **5.5 — la pluie hors contenant** : la pluie qui tombe sur le
sol (pas dans un contenant) s'infiltre, puis, quand le sol ne suit plus, remplit la rétention de surface et ruisselle.

**Ce que la session fait.** Le sol à ciel ouvert en trois pièces de V, sans loi nouvelle : un nœud de **rétention de surface** (la lame
que les creux du sol retiennent, 0,5 mm) qui reçoit la pluie (`Rain`) ; l'**infiltration** de Green–Ampt (S530) vers le **sol** ; le
**débordement** de la rétention vers l'extérieur (`Spill`, S489) — le **ruissellement**, que l'hôte dépose où le terrain le mène (δ, la
mer). Aucune flaque n'est posée par l'auteur : avant la submersion, toute la pluie entre.

**Ordre de grandeur, calculé.** Limon sableux (K = 10,9 mm/h, ψ = 11 cm, Δθ = 0,3 : M = 33 mm), pluie de 30 mm/h : **submersion de
Mein–Larson** `F_p = M K/(i − K)` = **18,84 mm** à `t_p = F_p/i` = **37,67 min** ; ensuite Green–Ampt décalé (`t_s` = 1 299 s) — `F` =
**48,88 mm** à 2 h, sur 60 mm de pluie : 11,12 mm de rétention et de ruissellement. La rétention (0,5 mm) déplace `M` de 0,45 % (la lame
`h₀` que la loi compte et que Mein–Larson néglige).

**Critères, écrits avant.** (1) Le début de la submersion (la rétention passe 1 ml) à 1 % de `t_p`. (2) `F` à 2 h à 0,5 % de Green–Ampt
décalé. (3) La masse exacte : pluie = sol + rétention + ruissellement, au millilitre ; le ruissellement nul avant la rétention pleine.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'essai ; (1)–(3).
- [x] **P3** — preuve ; liste 5.5 ; rituel.

### Notes de reprise
- **P2 fini** — submersion lue à 29,1 min (−23 % : le seuil d'1 ml est le quantum, (1) manqué ; 10 ml à 39,13 min, diagnostic après
  coup) ; F à 2 h 48,928 mm pour 48,880 (9,8·10⁻⁴) ; masse exacte, ruissellement dès la rétention pleine (50 min).
- **P3** — preuve PLUIE-SOL-S533 ; liste 5.5 ; index ; journal.

