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

Session : S718 — **terminée**. En autonomie, sans arrêt (l'utilisateur dort) ; session longue. LOD-ETAPE-2-S705, **E1 : la vague de bout
en bout**. N1, N2 et M1 sont acquis (S707, S715, S717).

**Ce que la session fait.** `Large::BoutEnBout`, un mode nommé (ADR-277 D2) :
- **au large** (0 à 5 m), SGN porte l'onde ;
- **la bande 3D** (5 à 10,775 m) est nourrie au bord gauche par la pose par la grille, d'après le profil de SGN (S703) ; le relais
  Saint-Venant tient le rivage ;
- **à 3,2 s**, après le déferlement et l'air, la 3D meurt (M1). Un seul Saint-Venant reprend la plage entière jusqu'à 5 s :
  - le large, depuis l'état de SGN ;
  - la bande, depuis sa surface ;
  - le rivage, depuis le relais.

**Le déclencheur, ici.** Pour l'onde de référence (x₁ = 3,4 m), le front touche déjà la bande au départ (η = 2,6 cm à 5 m) : la bande naît
à t = 0. Le déclencheur qui décide *s'il faut* la 3D (la prédiction du déferlement) attend, à la demande de l'utilisateur.

**L'essai et ses critères, écrits avant.** Contre le tout-3D jusqu'à 5 s (`Large::AucunJusqua5`, S717 : le même montage, la même fonction,
déterministe au bit, donc non relancé) :
1. le retournement à 0,1 s et 0,15 m du tout-3D (2,637 s, 9,988 m ; ADR-278 D2), l'air après lui ;
2. la remontée maximale à 1,25 cm et 0,1 s (0,3387 m à 3,917 s) ;
3. le volume rendu à la mort à 0,5 % ; la masse au bit avant ;
4. le coût jusqu'à 5 s, mesuré et comparé (le tout-3D : 1 191 s).

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-275, ADR-276, ADR-277, ADR-278, ADR-280, ADR-281)

- **témoin** : le tout-3D de S717, par la même fonction.
- **instrument** : le lecteur de retournement (S647), l'air (S648), la remontée sous la maille (S688), recalculée à chaque pas (ADR-281
  D2). Ce que rendrait chaque hypothèse :
  - les pièces s'assemblent sans faute nouvelle : (1) comme S703 (−0,068 s), (2) comme S717 ;
  - une faute de l'assemblage : un écart de remontée au-delà de 1,25 cm.
- **calcul** : le coût attendu. La bande 3D seule jusqu'à 3,2 s fait ≈ 93 000 à 109 000 particules, soit ≈ 7 min (S703). Puis
  Saint-Venant, presque rien. Soit ≈ 8 min, contre ≈ 20 min pour le tout-3D.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-275 D3 : chaque étape jugée sur le déferlement, l'air, la masse et le coût ;
  - ADR-278 D1 : le raccord retenu ;
  - ADR-280 D1 : la surface à la mort.
- **pièges** :
  - à la mort, la bande commence à x_r = 5 m : la fonction de fond de la mort prend `x + x_r` ;
  - le large de Saint-Venant vient de SGN, cellule à cellule (le même pas d'espace depuis 0) ;
  - le volume d'avant la mort compte aussi le large.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le mode ; l'essai ; (1)–(4).
- [x] **P3** — preuve ; rituel.

### Notes de reprise
- **L'essai : (2) échoue** — le retournement 2,569 s, 9,863 m (−0,068 s, dans la tolérance, comme S703) ; l'air 2,747 s ; **la remontée 0,3714 m à 3,859 s** (le tout-3D : 0,3387 m ; +3,3 cm) ; le volume rendu −3,8·10⁻⁴ ; **302 s contre 1 191 s** (4 fois moins). Le témoin, une seule cause (la mort) : `Large::BandeJusqua5`, la même bande nourrie par SGN jusqu'à 5 s sans la mort. Si sa remontée est aussi vers 0,37 m, la mort est innocente, et le large (SGN, l'onde 10 % plus haute) est en cause.
- **P2 fini** — le témoin sans la mort : 0,3714 m à 3,856 s, la même remontée ; la mort est innocente, le large (SGN) en cause (ADR-278 D3). Suite : S719, la même onde (le profil de Rayleigh) pour SGN et la 3D.
