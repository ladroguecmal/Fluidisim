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

Session : S614 — **terminée**. En autonomie (ADR-247) : **le lot** (dû ; feuille de route S611–S613), puis **11.3 — les très grands
événements : macroscopiques au large, locaux à l'interaction** (absent). Le tsunami : le modèle macroscopique (S582, levée de Green) donne la
hauteur au bord d'un domaine local ; le domaine local (Saint-Venant 2D de S613, une bande) calcule la remontée sur la plage.

**Ce que la session fait.** Un module `grand_evenement.rs` : `hauteur_au_bord(rayon, A₀, s)` (la levée de Green du rayon de S582) ;
`OndeSolitaire` (`η = H·sech²(γ(x − x₁))`, `γ = √(3H/4d³)`, `u = c·η/(d + η)`) ; `remontee_synolakis(H, d, cot β)` = `2,831·d·√cot β·(H/d)^(5/4)`
(Synolakis 1987, onde non déferlante) ; `Plage` — une bande de trois mailles, fond plat à `d` puis pente `1/cot β`, l'onde posée à la distance
canonique `arccosh(√20)/γ` du pied ; `remontee` — la plus haute cote mouillée (`h > 1 mm`). **En route, avant ce plan** : les murs de S613
n'exerçaient aucune pression — sans effet à bords secs (Thacker, le lac : S613 inchangé, vérifié), faux à bord mouillé (la maille du bord
accélérait) ; le flux de paroi `(0, ½gh², 0)` est ajouté (référence et code). Ne fait pas : l'entrée du niveau macroscopique au bord comme
condition aux limites (ici, une onde solitaire de la hauteur donnée), le déferlement, le 3D local, le crash et le très grand navire.

**Références, calculées avant** (`s614_ref.py`, numpy). Un tsunami de 4,14 cm à 4 000 m de fond, levé jusqu'à 10 m : **H = 0.185146429 m**
(`H/d` = 0.018515, sous le déferlement de Synolakis, 0,044). Plage 1:19,85 ; Synolakis : **R = 0.861418706 m**. Le domaine local, maille de
1, ½, ¼ m (pas 0,04/k s) : **R = 0.705289673, 0.793450882, 0.850125945 m** — la remontée converge vers la loi (-1.31 % au plus
fin) ; masse exacte, `h ≥ 0`, Courant ≤ 0.4071.

**Quantum** : la remontée se lit à la cote d'une maille — `dx/cot β`, 0.012594 m au plus fin ; le seuil de l'écart à Synolakis, dix quanta,
**0.125945 m** (asserté). **Critères, écrits avant.** (1) `H` de la levée égal à la référence à 10⁻¹² ; (2) les trois remontées égales aux
références au bit (des cotes de mailles) ; (3) croissantes, et la plus fine à moins de 0.1259 m de Synolakis ; (4) masse à 10⁻¹³, `h ≥ 0`,
aux trois mailles ; (5) un bord mouillé au repos (un bassin plat à 10 m, murs) ne bouge pas : vitesse sous 10⁻¹² m/s après 500 pas — le
défaut des murs ; (6) refus : `H`, `d` ou `cot β` non positifs.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [x] **P2** — les murs de `saint_venant_2d.rs`, `grand_evenement.rs` et leurs essais ; (1)–(6).
- [x] **P3** — preuve ; liste 11.3 ; rituel (`--lot`).

### Notes de reprise
- **P2 fini** — (1)–(6) tenus du premier essai ; S613 inchangé avec les murs corrigés. Suite : 803 essais listés.
- **P3** — preuve GRAND-EVENEMENT-S614 ; note datée à THACKER-S613 ; liste 11.3 (absent → partiel) et décompte ; index ; journal ; le lot.
