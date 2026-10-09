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

Session : S720 — **terminée**. En autonomie, sans arrêt (l'utilisateur dort). **Le jalon visuel de la phase A**, préparé pour la séance R43
que l'utilisateur jugera à son réveil : la vague de bout en bout (S718) contre le tout-3D, rendues côte à côte.

**Ce que la session fait.**
- **L'enregistrement** (`Enregistrement::film`). Une image tous les 1/30 s :
  - la surface de la 2D là où elle est active (SGN au large, Saint-Venant au rivage, puis la plage entière après la mort) ;
  - les particules d'une rangée de la 3D (la première, `y < dx`), leur vitesse.
  Les deux calculs sont enregistrés : le tout-3D (`AucunJusqua5`) et la vague de bout en bout (`BoutEnBout`), l'onde de départ de
  référence (Boussinesq).
- **Le rendu** (`outils/rendu_bout_en_bout.py`, d'après `rendu_rouleau.py` de S658), numpy et PIL. Deux GIF, le tout-3D en haut, de bout
  en bout en bas, à la même heure :
  - la plage entière ;
  - le déferlement de près.
  Un PNG de quatre instants sert à l'aperçu.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-276, ADR-277, ADR-281)

- **témoin** : le tout-3D lui-même, rendu à côté.
- **instrument** : l'image. Elle est contrôlée par les nombres de S717–S718, qui doivent se lire sur elle :
  - le retournement près de 2,6 s et de 9,9–10 m ;
  - la lame la plus haute vers 3,9 s ;
  - la 3D éteinte après 3,2 s, en bas.
- **calcul** :
  - la taille de l'enregistrement, une rangée sur quatre : ≈ 60 000 particules × 12 octets × 150 images ≈ 110 Mo pour le tout-3D,
    ≈ 50 Mo pour l'autre, dans `calculs/` (hors du dépôt) ;
  - le coût : ≈ 20 + 6 min.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-276 D1 : la même fonction pour les deux calculs ;
  - ADR-281 D1 : l'enregistrement s'écrit au fil du calcul.
- **pièges** :
  - les x de la bande, décalés de `x_r` ;
  - après la mort, il n'y a plus de particules ;
  - la mémoire des GIF (≈ 150 images de 1 300 × 300 px, en palette).

**Critères.** Les deux GIF et le PNG produits, les nombres de contrôle lisibles sur eux ; le livrable déposé pour l'utilisateur.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'enregistrement ; les deux calculs.
- [x] **P3** — le rendu ; le contrôle ; l'envoi.
- [x] **P4** — preuve ; rituel.

### Notes de reprise
- **P2–P3 finis** — les deux films (31 et 106 Mo), au bit des nombres de S717–S718 ; le rendu (deux GIF, un PNG) envoyé à l'utilisateur ; R43 posée.
