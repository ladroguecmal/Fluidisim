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

Session : S337 — **en cours**. **La coupure au bord de δ** : le verdict R15 de la porte D la voit encore ;
chemin de la porte D ([ADR-189](../docs/adr/ADR-189-la-v1-d-abord.md)).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — **verdict R15 de l'utilisateur**, sur les images de S336 : *« 1. Oui »* — le bateau lâché qui se pose en
3 à 4 s sur la houle est juste ; *« 2. On voit la coupure encore »* — les anneaux s'arrêtent sur le bord droit de
la grille de δ, visible au loin ; *« 3. Pas forcément »* — pas d'autre défaut.

**Ce que la session doit rendre possible.** Un δ dont le bord ne se voit pas, dans la scène de la porte D. La
production l'a déjà fait, et R11 l'a jugé : **éponge** au bord du domaine et **fondu de 3 m** à la composition
— *« pas de problème sur la transition »* (S303). Le banc de la porte D n'a ni l'un ni l'autre : le mode
linéaire de δ a des murs, qui renvoient les anneaux, et le rendu y ajoute δ jusqu'au dernier rang de mailles.
Le pli n'est pas de la physique de l'eau : c'est le bord d'un domaine local, qu'ADR-001 veut invisible. Le
rendre à W — le retour δ → W — reste bloqué par A289 ; ici, l'éponge éteint les anneaux avant le mur.

Critères, écrits avant le code :
1. **L'éponge du mode linéaire** (`Sponge3`, celle du pas couplé) : une bosse lâchée au centre d'un δ linéaire
   ne revient pas — hauteur au centre après le passage des ondes sous 20 % de celle que les murs renvoient —,
   et le volume qu'elle retire est compté : volume de δ plus volume retiré constant au plancher d'arrondi.
   Éponge absente par défaut : essais du cœur verts, valeurs S3xx au bit.
2. **Le fondu de composition** : δ pèse zéro au bord de la grille et un à 3 m de lui, comme la production.
3. **La scène de la porte D**, 12 s, avec éponge et fondu : critères 3 et 5 de S333 ; volume suivi en comptant
   l'éponge ; anneaux qui sortent sans revenir.
4. **Images** refaites, puis **arrêt pour le verdict** (ADR-189 D3).

### Plan

- [x] **P1** — jeton, plan seul, verdict R15 consigné ici.
- [x] **P2** — l'éponge du mode linéaire ; critère 1.
- [x] **P3** — le fondu de composition dans le rendu de la porte D ; critère 2.
- [x] **P4** — la scène de la porte D, éponge et fondu, 12 s ; critère 3 ; images (critère 4).
- [ ] **P5** — preuve ; R15 au registre des revues ; file, feuille de route.
- [ ] **P6** — rituel ; arrêt pour le verdict.

### Notes de reprise
- **P2, critère 1 tenu.** `Volume3::set_linear_sponge` : l'éponge du pas couplé (vitesses prédites amorties, hauteur
  rappelée au repos, volume retiré compté par `linear_sponge_removed`). Tranche 32 m, crête 10 cm, 20 s, éponge 4 m à
  2 /s : énergie de surface intérieure **0,4 %** de celle des murs ; volume de δ plus retiré, écart 2,2·10⁻⁹ m³ pour une
  borne d'arrondi de 1,0·10⁻⁶ (murs : 6,8·10⁻¹⁰). Le plan disait « au plancher d'arrondi » : l'essai compare à la
  borne cumulée, non aux 10⁻⁹ d'abord codés. Défaut sans éponge : S3xx au bit.
- **P3, critère 2** : `fondu` de `porte_d` = `delta_fade` de `water.wgsl` — `½ − ½·cos(π·s/w)`, produit des
  fondus en x et en y ; `--fondu 3` : zéro au bord, un à 3 m. Sans l'option, les images de S333–S336 inchangées.
- **P4, critère 3 tenu** (16 s au lieu des 12 prévues : les anneaux mettent ~13 s à revenir d'un mur). Scène
  `--couvercle-partiel --archetype --eponge --fondu 3 --pas 1600` : critères 3 (au bit) et 5 de S333 ; volume de δ
  plus retiré suit la coque, 1,95·10⁻⁹ m³, au plancher (1,9·10⁻⁵) ; **l'éponge retire 0,49 m³** — l'eau que la coque
  déplace en se posant, qu'une mer ouverte étale. Agitation du centre après 12 s : **4,03 mm** avec éponge, **8,19 mm**
  avec murs ; le reste vient de la coque, qui remue encore l'eau sur la houle — son tangage n'est pas amorti. Énergie
  0,981 (murs 0,956). Plus de pli au bord dans les images. `viewer/captures/s337`, scène / carte : 2 s
  `0x28e004971a52f585` / `0x2cf65f51fae160b1` ; 4 s `0xc33c5065fb79fcff` / `0xba406b73662214c1` ; 6 s
  `0x7cd8db9d4c41f85e` / `0x8a41b853a480b80d` ; 8 s `0x21d2ccff6a239506` / `0x11e4c93ea60047cf` ; 10 s
  `0xc4f0a58ae159079e` / `0x199806393a2489a3` ; 12 s `0xb42556af028c4e0c` / `0xef75f9c6f4391133` ; 14 s
  `0xbfc0e4afe228ce0a` / `0xf10f55ab736a94df` ; 16 s `0xe315eb83a715c715` / `0x871e3986adbe3d09`.
