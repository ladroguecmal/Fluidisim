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

Session : S742 — **terminée**. En autonomie ; session longue. ADR-287 D1 : **le banc canonique de la 3D**, avec la projection de densité
(S740). **La question** : la 3D corrigée passe-t-elle les essais de base d'un modèle de vagues, sans perdre ce qui marchait ?

**Le banc** (2,5 cm) :
- **B1 — l'onde solitaire sur un canal plat** : fait en S740, E7 (la largeur à 92 %, le creux à 9 % de `H`). Repris tel quel.
- **B2 — la levée sur une pente douce** (`levee_s742`) :
  - une bosse gaussienne de 15 mm (σ = 1 m) sur 0,30 m d'eau, une pente de 1:30 jusqu'à 0,12 m, un plateau de 7 m, des murs ;
  - la 3D avec et sans la projection, et deux témoins sur la même bosse : Saint-Venant et SGN (S733, sur fond doux) ;
  - la grandeur : la plus haute crête lue sur le plateau, de 11,9 m jusqu'à l'arrivée du front au mur, rapportée à sa crête de départ (à
    0,25 s, ADR-287 D3).
- **B4 — la remontée de S645 avec la projection** : le montage de S645 tel quel, `d` = 0,35 m, `H/d` = 0,2, 1:3 ; la particule la plus haute
  contre la loi (0,2295 m ; S645 sans projection : 0,2307 m).

**Les critères, écrits avant.**
- B2 :
  1. la 3D avec la projection à **10 %** du témoin Saint-Venant (la même bosse, non linéaire ; la loi de Green, linéaire, vaut 1,257 et
     est rapportée) ;
  2. la 3D sans la projection, rapportée.
- B4 : 3. la remontée avec la projection à **10 %** de la loi.

**Les bornes** (ADR-286 D1, `python` au plan) : 18,4 m en tout (6 m plats, 5,4 m de pente, 7 m de plateau). Le front de la bosse (3σ devant
la crête) touche le mur quand la crête est à 15,4 m : la lecture s'arrête là. La bosse part à 3,5 m, à 3σ + 0,5 m du mur de gauche.

**Contrôles du plan** (ADR-276, ADR-280, ADR-286, ADR-287)

- **témoin** :
  - Saint-Venant et SGN sur la même bosse (S733) ;
  - la loi de Green (linéaire, rapportée) ;
  - la loi de Synolakis pour B4 ;
  - la 3D sans projection, comme seconde référence.
- **instrument** : la crête par la surface lissée sur 10 cm, et, en seconde lecture, par les particules (ADR-286 D2).
- **calcul** : B2, deux fois ≈ 10 min ; B4 ≈ 7 min ; en série.
- **ADR** : ADR-276 D1 (une seule fonction pour la bosse) ; ADR-287 D1, D3.
- **pièges** :
  - la bosse de 15 mm ne fait que 0,6 maille : la lecture par la surface est lissée, et la seconde lecture la contrôle ;
  - la projection a lissé le plongeon en S709 : B4 dira si elle lisse aussi la remontée.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — B2 (la levée) : le montage, les témoins, la 3D avec et sans projection.
- [x] **P3** — B4 (la remontée de S645 avec la projection).
- [x] **P4** — preuve ; fermeture.

### Notes de reprise
- **P2** — B2 sans conclusion. Saint-Venant 1,152, SGN 1,271 ; la 3D avec projection 3,33, sans projection aucune crête lue. Le profil,
  regardé (ADR-287 D4) : la bosse sous le quantum de pose (une couche de 12,5 mm), des dents de scie de ±10 mm sur la pente lisse dès le
  départ, le bassin qui oscille de ±20 mm. Les profils dans `captures/s742_levee_profils.png`.
- **P3** — B4 avec la projection : **0,2020 m (−12 %), manqué** ; sans projection 0,2307 m. La projection freine la lame.
