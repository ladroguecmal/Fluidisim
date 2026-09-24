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
- [ ] **P2** — l'éponge du mode linéaire ; critère 1.
- [ ] **P3** — le fondu de composition dans le rendu de la porte D ; critère 2.
- [ ] **P4** — la scène de la porte D, éponge et fondu, 12 s ; critère 3 ; images (critère 4).
- [ ] **P5** — preuve ; R15 au registre des revues ; file, feuille de route.
- [ ] **P6** — rituel ; arrêt pour le verdict.

### Notes de reprise

