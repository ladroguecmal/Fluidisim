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

Session : S689 — **terminée**. En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage, l'étape 3 (le reflux).

**Ce que la session fait.** **La dette remboursée au quantum.** S685 laissait la dette (ce que Saint-Venant a pris moins ce qu'APIC a
laissé sortir) s'accumuler : 2 à 3 quanta par rangée en 3 s. Désormais :

- quand elle atteint un quantum, APIC rend la particule de sa dernière colonne la plus proche du bord (`take_right`) ;
- quand elle descend sous moins un quantum, Saint-Venant reçoit ce volume dans sa première maille.

La masse reste exacte, et la dette sous un quantum par rangée. Puis l'onde de S685 est suivie 10 s : elle monte, redescend, repasse
dans la 3D, et ressort encore.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273)

- **témoin** : S685 sans remboursement (la dette à 2 à 3 quanta par rangée en 3 s).
- **instrument** : ce que rendrait chaque hypothèse.
  - Si le reflux passe bien, la masse reste exacte, la dette sous un quantum, la colonne du bord peu tassée, et aucune vitesse
    parasite.
  - Une entrée mal placée tasse la colonne du bord, ou lance des particules.
  - Un remboursement faux fait dériver la masse.
- **calcul** (ce script) : la dette de S685, 2,1 quanta par rangée à 5 cm et 2,9 à 2,5 cm. Le remboursement a donc de quoi agir.
- **ADR** : ADR-271, ADR-273.
- **pièges** :
  - rendre une particule d'une colonne sans eau (aucune : la dette attend) ;
  - ajouter un volume à Saint-Venant dans une maille sèche (une hauteur positive, au bit du volume) ;
  - le signe de la dette.

**Critères, écrits avant.**

1. Sur 10 s, à 5 cm : la masse à 10⁻¹² près.
2. La dette de chaque rangée sous un quantum, à chaque pas.
3. La dernière colonne sous 10 particules par maille mouillée.
4. La vitesse des particules sous 1 m/s (la vitesse de l'onde au raccord, `√(g·h)` ≈ 1,3 m/s, n'est pas une vitesse de particule).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le remboursement ; l'essai ; (1)–(4).
- [x] **P3** — preuve ; rituel.

### Notes de reprise
- **P2 fini** — (1) 2,8·10⁻¹⁶ ; (2) 0,998 quantum ; (3) 9,20 ; (4) 0,539 m/s. La sortie de la 3D surtout par remboursement (20,1 L sur 25,4).
