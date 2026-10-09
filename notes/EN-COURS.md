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

Session : S734 — **terminée**. En autonomie ; session longue. SELECTEUR-DOMAINES-S732, **P2 : les témoins**. **La question** : les scènes
S2, S3 et S4 en tout-3D, sans aucun raccord, que donnent-elles (le retournement, la crête, la remontée), et que prévoit le prédicteur de
S733 sur chacune ? Ce sont les données du calibrage du prédicteur (S735).

**Ce que la session fait.**
- **`temoin_plage_s734`**, un témoin tout-3D générique, sur le modèle de S712. L'onde solitaire (`OndeSolitaire`) part centrée à
  `L + approche` du pied, où `L = arccosh(√20)/γ·d` est la distance où elle tombe à 5 % ; l'approche donne au prédicteur au moins 2 s (le
  piège de S722).
  - Le domaine : un mur 8 d derrière, la plage ; quatre rangées, 2,5 cm ; aucun raccord, aucune sortie (ADR-285 D1).
  - À chaque pas : le retournement (S647), l'air (S648), la crête par la surface (`volume_surface_s708`, ADR-280 D1), le front d'eau (la
    dernière colonne où l'épaisseur lue dépasse 5 mm). Tous les 0,1 s, la crête est affichée (ADR-281 D1).
- **Le prédicteur sur la même scène** (`porteur_plage_s734`, SGN à 5 cm sur la plage en miroir, le fond arrêté à 4 cm d'eau) : les trois
  critères publiés et les six variantes de S733, rapportés.
- Les scènes (`python outils/selecteur_nombres.py` pour S2 à S4 ; les dimensions par le script du plan) :

| scène | `d`, `H/d`, pente | la plage du domaine | durée | particules, calcul |
|---|---|---|---|---|
| **S2** — Synolakis | 0,5 m, 0,3, 1:19,85 ; l'approche 2,5 m | jusqu'à 8 d au-delà du rivage (S712) | 7 s (t·√(g/d) ≈ 25 depuis le pied, S712) | ≈ 350 k ; ≈ 40 min |
| **S3** — glissante | 0,3 m, 0,5, 1:90 ; 3 m | **un mur à 16 m du pied** (0,12 m d'eau) : le rivage est à 27 m, 23 s de trajet | 12 s | ≈ 270 k ; ≈ 1 h |
| **S4** — sans déferlement | 0,5 m, **0,2, 1:3** ; 3 m | jusqu'à 8 d au-delà du rivage | 6 s | ≈ 270 k ; ≈ 25 min |

S4 change de pente par rapport au registre (0,03 sur 1:12) : une onde de 15 mm y ferait 0,6 maille. À 1:3, le seuil de Synolakis est
`H/d` = 0,241, et `S₀` = 1,13 ne prévoit aucun déferlement. La remontée exacte (Synolakis, la loi des ondes non déferlantes) vaut
**0,328 m**.

**Les critères, écrits avant.**
- Chaque témoin (ADR-285 D1) : aucun raccord ; le nombre de particules constant (aucune sortie) ; **le front d'eau jamais à moins de 1 m du
  mur de droite** (S3 : le mur à 16 m ne doit rien voir).
- **S4** : (1) **aucun retournement** en 6 s ; (2) la remontée maximale (la cote du fond sous le front, moins `d`) à **15 %** de 0,328 m
  (le pas d'une colonne sur 1:3 vaut 8 mm, 2,5 %).
- **S2** : (3) un retournement ; son instant et son lieu rapportés. Les profils de laboratoire à t·√(g/d) = 15, 20, 25 (comptés depuis la
  place de départ de S712) comparés et rapportés.
- **S3** : (4) le retournement, rapporté **qu'il ait lieu ou non** : c'est la question de la scène (faut-il la 3D pour un déferlement
  glissant ?).
- **Le prédicteur** : rapporté sur chaque scène, aucun critère ici. Le calibrage se fait en S735, sur S1 à S4 ensemble.

**Contrôles du plan** (ADR-266, ADR-273, ADR-276, ADR-279, ADR-280, ADR-281, ADR-285)

- **témoin** : ces scènes **sont** les témoins. Contrôlés comme des objets (ADR-285 D1) : aucun raccord, les murs mesurés par le front.
  S2 contre les mesures de laboratoire (S712) ; S4 contre la loi exacte.
- **instrument** :
  - la crête et le front lus par la surface (ADR-280 D1) ;
  - le retournement et l'air, juges éprouvés (S647, S648) ;
  - le plancher du front : une colonne, 8 mm sur 1:3.
- **calcul** : S4 (25 min), S2 (40 min), S3 (1 h), en série, en arrière-plan (`essai.py`, chacun par son nom exact, ADR-285 D2).
- **ADR** :
  - ADR-276 D1 : chaque scène part d'une seule fonction d'onde (`OndeSolitaire`), pour la 3D et pour SGN ;
  - ADR-279 D1 : la convergence de ces témoins n'est pas mesurée ici ; elle est rapportée comme manquante ;
  - ADR-285 D1.
- **pièges** :
  - la lame mince sur sable sec fait tomber le pas (S712 : 0,34 ms à t·√(g/d) = 30), d'où les durées bornées ;
  - le prédicteur sur 1:3 sort de la pente douce : rapporté comme tel.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le témoin générique, le porteur générique ; la compilation ; S4.
- [x] **P3** — S2 et S3 (en arrière-plan) ; les nombres au fil du calcul.
- [x] **P3c** — **ajouté après S4** : S4 sur le **fond lisse** (S640), les mêmes critères (1) et (2) ; la comparaison des deux fonds localise
  l'écart (ADR-226 D1).
- [x] **P4** — preuve ; fermeture.

### Notes de reprise
- **Écart au plan, S2** : sans approche ajoutée (le départ de S712) ; le retournement vient vers 3,3 s, ce qui laisse au prédicteur plus de
  2 s ; les instants du laboratoire restent ceux de S712.
- **P2 fini** — **S4 en escalier : (1) et (2) manqués.**
  - La remontée est de 0,4835 m à 4,48 s contre 0,328 m (+47 %).
  - Un « retournement » est vu à 5,913 s, à 10,34 m, au reflux ; de l'air enfermé à 3,11 s, à 11,34 m.
  - Le front reste figé à 12,763 m dès 4,7 s : l'eau est piégée dans les marches. À 1:3 et 2,5 cm, le fond monte d'une maille toutes les
    trois colonnes.
  - Le témoin lui-même (aucune sortie, le front à 2,5 m du mur) tient. 1 965 s.
  - **Le prédicteur sur S4** : Kennedy 0,65 ne se déclenche pas, ce qui est juste. Les huit autres se déclenchent tous à 11,2–11,6 m, au
    rivage et dans la zone de jet de rive (le fond arrêté à 4 cm commence à 11,19 m).
  - **S2 en cours** : le retournement à 3,290 s, à 12,013 m (0,21 m d'eau).
- **S3, un défaut du montage, trouvé avant son calcul** : son mur (16 m après le pied) est dans 0,12 m d'eau. Le critère « le front à plus de
  1 m du mur » y est faux par construction. **Corrigé avant le calcul** : le niveau de la colonne du mur reste à 5 mm du repos.
- **Reprise** (2026-10-09) : le travail fait après `44807cd2` a été effacé à la demande de l'utilisateur. S2, S3 et S4 sur fond lisse
  tournent en série, seuls.
- **P3 et P3c finis** :
  - S2 **tenu** (3,290 s ; 12,013 m ; 1 681 s) ;
  - S3 **manqué** sur le mur : la vague l'atteint, 307 mm, parce que la durée était trop longue ; le déferlement à 3,488 s, à 9,54 m ; 4 012 s ;
  - S4 sur fond lisse **manqué** : la remontée 0,600 m (+83 %) ; le front figé à 13,113 m dès 4,1 s ; un « retournement » à 4,14 m ;
    2 113 s.
- Le prédicteur rapporté sur chaque scène. L'utilisateur accepte de fermer ainsi ; le diagnostic de S4 en S735.
